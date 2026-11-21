//! Compare sprite pixels, shared cache slots and aging with CatalogOracle.java.
use serde_json::{Value, json};
use zelda_mobile::{
    engine::Game,
    input::Action,
    runtime::{Native, Runtime, V},
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut game = Game::new(
        root.join("assets"),
        root.join("target/catalog-saves"),
        false,
    )?;
    game.action(Action::Primary, true)?;
    game.advance(140)?;
    game.action(Action::Primary, true)?;
    for _ in 0..20 {
        game.paint()?;
    }
    let r = &mut game.runtime;
    let world = r.field(game.canvas, "d", "Lj;").r();
    let player = r.field(game.canvas, "e", "Lh;").r();
    let mut rows = Vec::new();
    for (classic, color) in [(true, 0), (false, 2)] {
        r.put(game.canvas, "a", "Z", V::I(classic as i32));
        r.put(player, "V", "B", V::I(color));
        for (family, (method, image_key, age_key, used_key, age_method)) in [
            ("a", "d", "g", "k", "a"),
            ("b", "e", "i", "m", "c"),
            ("c", "c", "f", "j", "b"),
            ("d", "a", "h", "l", "d"),
        ]
        .into_iter()
        .enumerate()
        {
            for kind in 0..256 {
                let resources = r.alloc("o", Native::Empty);
                r.call(resources, "<init>", "(Lj;)V", &[V::R(world)])
                    .map_err(|e| r.string(e))?;
                let image = r
                    .call(
                        resources,
                        method,
                        "(I)Ljavax/microedition/lcdui/Image;",
                        &[V::I(kind)],
                    )
                    .map_err(|e| r.string(e))?
                    .r();
                let (width, height, hash) = if image == 0 {
                    (0, 0, 0)
                } else {
                    let Native::Image(surface) = &r.obj(image).native else {
                        return Err("Invalid sprite".into());
                    };
                    let bitmap = surface.borrow();
                    let mut hash = 2166136261_u32;
                    for &pixel in &bitmap.pixels {
                        // Java's buffered image rounds translucent RGB. Check exact
                        // opaque colors and every alpha value without codec noise.
                        let [red, green, blue, alpha] = pixel.to_le_bytes();
                        let bytes = if alpha == 255 {
                            [red, green, blue, alpha]
                        } else {
                            [0, 0, 0, alpha]
                        };
                        for byte in bytes {
                            hash = (hash ^ byte as u32).wrapping_mul(16777619);
                        }
                    }
                    (bitmap.w, bitmap.h, hash)
                };
                let images = r
                    .field(resources, image_key, "[Ljavax/microedition/lcdui/Image;")
                    .r();
                let slot = if image == 0 {
                    -1
                } else {
                    (0..r.array_len(images).unwrap())
                        .find(|&i| r.array_get(images, i).unwrap().r() == image)
                        .unwrap()
                };
                let again = r
                    .call(
                        resources,
                        method,
                        "(I)Ljavax/microedition/lcdui/Image;",
                        &[V::I(kind)],
                    )
                    .map_err(|e| r.string(e))?
                    .r();
                let ages = r.field(resources, age_key, "[I").r();
                let used = r.field(resources, used_key, "[Z").r();
                let age_values = nonzero(r, ages);
                let used_values = nonzero(r, used);
                let mut retained = Vec::new();
                for _ in 0..3 {
                    r.call(resources, age_method, "()V", &[])
                        .map_err(|e| r.string(e))?;
                    retained.push(slot >= 0 && r.array_get(images, slot).unwrap().r() != 0);
                }
                rows.push(json!([
                    classic,
                    color,
                    family,
                    kind,
                    width,
                    height,
                    hash,
                    slot,
                    image == again,
                    age_values,
                    used_values,
                    retained
                ]));
                r.collect();
            }
        }
    }
    std::fs::write(
        root.join("target/native-catalog.json"),
        serde_json::to_vec(&rows).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    println!("{} sprite/cache cases checked", rows.len());
    Ok(())
}
fn nonzero(r: &mut Runtime, array: u32) -> Vec<Value> {
    (0..r.array_len(array).unwrap())
        .filter_map(|index| {
            let value = r.array_get(array, index).unwrap().i();
            (value != 0).then(|| json!([index, value]))
        })
        .collect()
}
