use zelda_mobile::{
    engine::Game,
    input::Action,
    runtime::{Native, V},
};
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut game = Game::new(
        root.join("assets"),
        root.join("target/behavior-saves"),
        false,
    )?;
    game.action(Action::Primary, true)?;
    game.advance(140)?;
    game.action(Action::Primary, true)?;
    for _ in 0..20 {
        game.paint()?;
    }
    let player = game.runtime.field(game.canvas, "e", "Lh;").r();
    let sound = game.runtime.field(game.canvas, "H", "Lb;").r();
    let mut rows = Vec::new();
    fn snapshot(game: &mut Game, p: u32, label: &str) -> Result<serde_json::Value, String> {
        let data = game
            .runtime
            .call(p, "b", "()[B", &[])
            .map_err(|e| game.runtime.string(e))?
            .r();
        let bytes = game.runtime.bytes(data);
        Ok(
            serde_json::json!({"label":label,"state":game.state(),"x":game.runtime.field(p,"i","I").i(),"y":game.runtime.field(p,"j","I").i(),"health":game.runtime.field(p,"H","I").i(),"sword":game.runtime.field(p,"q","Z").i()!=0,"inventory":game.runtime.field(game.canvas,"m","Z").i()!=0,"save":bytes.iter().map(|b|format!("{b:02x}")).collect::<String>()}),
        )
    }
    rows.push(snapshot(&mut game, player, "opening")?);
    for (action, label) in [(Action::Down, "down"), (Action::Right, "right")] {
        game.action(action, true)?;
        for _ in 0..8 {
            game.paint()?;
        }
        game.action(action, false)?;
        rows.push(snapshot(&mut game, player, label)?);
    }
    for (kind, flag, label) in [(130, 1, "sword pickup"), (3, 0, "heart pickup")] {
        if kind == 3 {
            game.runtime.put(player, "H", "I", V::I(4));
        }
        let item = game.runtime.alloc("c", Native::Empty);
        game.runtime
            .call(
                item,
                "<init>",
                "(Ljavax/microedition/lcdui/Image;IIIILh;)V",
                &[
                    V::R(0),
                    V::I(kind),
                    V::I(0),
                    V::I(0),
                    V::I(flag),
                    V::R(player),
                ],
            )
            .map_err(|e| game.runtime.string(e))?;
        game.runtime
            .call(
                item,
                "a",
                "(Lh;Lb;Lk;Z)I",
                &[V::R(player), V::R(sound), V::R(game.canvas), V::I(0)],
            )
            .map_err(|e| game.runtime.string(e))?;
        for _ in 0..30 {
            game.paint()?;
        }
        rows.push(snapshot(&mut game, player, label)?);
    }
    game.key(53, true)?;
    game.key(53, false)?;
    game.key(53, true)?;
    for _ in 0..8 {
        game.paint()?;
    }
    game.key(53, false)?;
    rows.push(snapshot(&mut game, player, "sword action")?);
    game.key(57, true)?;
    for _ in 0..2 {
        game.paint()?;
    }
    game.key(57, false)?;
    rows.push(snapshot(&mut game, player, "inventory")?);
    let out = root.join("target/native-behavior.json");
    std::fs::write(&out, serde_json::to_vec_pretty(&rows).unwrap()).map_err(|e| e.to_string())?;
    println!("{} behavioral snapshots: {}", rows.len(), out.display());
    Ok(())
}
