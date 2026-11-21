//! Exhaustive resource loading and representative interaction checks. These
//! developer commands use isolated saves and never modify the user's progress.
use crate::{
    engine::Game,
    input::Action,
    runtime::{Native, V},
};
use std::path::Path;

pub fn audit_rooms(game: &mut Game, output: &Path) -> Result<(), String> {
    game.action(Action::Primary, true)?;
    game.advance(140)?;
    game.action(Action::Primary, true)?;
    game.advance(280)?;
    let canvas = game.canvas;
    let player = game.runtime.field(canvas, "e", "Lh;").r();
    let world = game.runtime.field(canvas, "d", "Lj;").r();
    let image = game
        .runtime
        .field(canvas, "R", "Ljavax/microedition/lcdui/Image;")
        .r();
    let graphics = game
        .runtime
        .call(
            image,
            "getGraphics",
            "()Ljavax/microedition/lcdui/Graphics;",
            &[],
        )
        .map_err(|e| game.runtime.string(e))?
        .r();
    game.runtime.roots.push(graphics);
    let mut paths = std::fs::read_dir(game.runtime.assets.join("Maps"))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect::<Vec<_>>();
    paths.sort();
    let mut rows = Vec::new();
    let mut failures = Vec::new();
    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let ids = name
            .strip_prefix("Map")
            .and_then(|s| s.split_once('-'))
            .and_then(|(m, w)| {
                Some((
                    m.parse::<i32>().ok()?,
                    w.split('.')
                        .next()?
                        .split('-')
                        .next()?
                        .parse::<i32>()
                        .ok()?,
                ))
            });
        let numbered = ids.is_some();
        let (map, zone) = ids.unwrap_or((0, 0));
        let alternate = name.contains("-9-1.") as i32;
        game.runtime.put(player, "P", "I", V::I(map));
        game.runtime.put(player, "O", "I", V::I(zone));
        game.runtime.put(player, "H", "I", V::I(12));
        game.runtime.put(player, "i", "I", V::I(75));
        game.runtime.put(player, "j", "I", V::I(85));
        game.runtime.put(player, "E", "Z", V::I(0));
        game.runtime.put(canvas, "ay", "I", V::I(map));
        game.runtime.put(canvas, "az", "I", V::I(zone));
        game.runtime.put(canvas, "p", "I", V::I(1));
        game.runtime.put(canvas, "m", "Z", V::I(0));
        game.runtime.put(world, "g", "I", V::I(alternate));
        game.runtime.put(world, "h", "I", V::I(alternate));
        let resource = game.runtime.text(format!("/Maps/{name}"));
        let before = game.runtime.errors.len();
        let result = game.runtime.call(
            world,
            "a",
            "(Ljava/lang/String;Ljavax/microedition/lcdui/Graphics;Lh;IIIIZ)Z",
            &[
                V::R(resource),
                V::R(graphics),
                V::R(player),
                V::I(map),
                V::I(zone),
                V::I(240),
                V::I(320),
                V::I(numbered as i32),
            ],
        );
        let ok = match result {
            Ok(v) => v.i() != 0,
            Err(e) => {
                failures.push(format!("{name}: {}", game.runtime.string(e)));
                false
            }
        };
        if ok {
            game.runtime.put(canvas, "f", "Z", V::I(1));
            game.runtime.put(canvas, "g", "Z", V::I(1));
            if let Err(e) = game.paint() {
                failures.push(format!("{name} paint: {e}"));
            }
        } else if failures.last().is_none_or(|s| !s.starts_with(&name)) {
            failures.push(format!("{name}: loader returned false"));
        }
        rows.push(serde_json::json!({"map":name,"loaded":ok,"reported_exceptions":game.runtime.errors.iter().skip(before).cloned().collect::<Vec<_>>() }));
        game.runtime.collect();
    }
    let report = serde_json::json!({"rooms":rows.len(),"failures":failures,"results":rows});
    crate::storage::atomic_write(output, &serde_json::to_vec_pretty(&report).unwrap())
        .map_err(|e| e.to_string())?;
    println!(
        "{} map variants loaded through original game logic; {} failures. Report: {}",
        rows.len(),
        failures.len(),
        output.display()
    );
    if !failures.is_empty() {
        return Err(failures.join("\n"));
    }
    Ok(())
}

pub fn save_round_trips(game: &mut Game) -> Result<(), String> {
    game.action(Action::Primary, true)?;
    game.advance(140)?;
    game.action(Action::Primary, true)?;
    game.advance(280)?;
    let player = game.runtime.field(game.canvas, "e", "Lh;").r();
    for (slot, name) in ["ZeldaSave", "ZeldaSave2", "ZeldaSave3"].iter().enumerate() {
        game.runtime.put(game.canvas, "aA", "I", V::I(slot as i32));
        game.runtime.put(player, "J", "I", V::I(23 + slot as i32));
        game.runtime
            .call(game.canvas, "d", "()V", &[])
            .map_err(|e| game.runtime.string(e))?;
        let saved = crate::storage::decode_record(
            &std::fs::read(game.runtime.saves.join(format!("{name}.rms")))
                .map_err(|e| e.to_string())?,
        )?;
        let object = game.runtime.alloc("h", Native::Empty);
        game.runtime
            .call(object, "<init>", "()V", &[])
            .map_err(|e| game.runtime.string(e))?;
        let bytes = game.runtime.byte_array(&saved);
        game.runtime
            .call(
                object,
                "a",
                "([BZLk;)V",
                &[V::R(bytes), V::I(0), V::R(game.canvas)],
            )
            .map_err(|e| game.runtime.string(e))?;
        if game.runtime.field(object, "J", "I").i() != 23 + slot as i32
            || game.runtime.field(object, "P", "I").i() != 120
        {
            return Err(format!("Save slot {} failed to reload", slot + 1));
        }
    }
    println!("All three original save payloads round-trip through the original encoder/decoder");
    Ok(())
}
