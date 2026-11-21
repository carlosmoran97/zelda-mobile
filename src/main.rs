use zelda_mobile::{audio, engine, input, maps, storage};
mod desktop;
use zelda_mobile::graphics;

fn main() {
    if let Err(e) = run() {
        eprintln!("Zelda Mobile: {e}");
        let headless = std::env::args().any(|a| {
            matches!(
                a.as_str(),
                "--smoke" | "--replay" | "--audit" | "--audit-rooms" | "--audit-saves"
            )
        });
        if !headless {
            let _ = sdl3::messagebox::show_simple_message_box(
                sdl3::messagebox::MessageBoxFlag::ERROR,
                "Zelda Mobile",
                &e,
                None,
            );
        }
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<_>>();
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let option = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .map(std::path::PathBuf::from)
    };
    let bundle = std::env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .unwrap()
        .join("../Resources/assets");
    let assets = option("--assets").unwrap_or_else(|| {
        if bundle.exists() {
            bundle
        } else {
            root.join("assets")
        }
    });
    if args.iter().any(|a| a == "--audit") {
        println!("{}", maps::audit(&assets)?);
        println!(
            "{} MIDI tracks synthesized successfully",
            audio::AudioCore::new(&assets)?.audit()?
        );
        return Ok(());
    }
    let headless = args.iter().any(|a| {
        matches!(
            a.as_str(),
            "--smoke" | "--replay" | "--audit-rooms" | "--audit-saves"
        )
    });
    let saves = option("--save-dir").unwrap_or_else(|| {
        if headless {
            root.join("target/smoke-saves")
        } else {
            storage::data_dir()
        }
    });
    let mut game = engine::Game::new(assets, saves, args.iter().any(|a| a == "--verbose"))?;
    if args.iter().any(|a| a == "--audit-rooms") {
        return zelda_mobile::validation::audit_rooms(
            &mut game,
            &root.join("target/room-audit.json"),
        );
    }
    if args.iter().any(|a| a == "--audit-saves") {
        return zelda_mobile::validation::save_round_trips(&mut game);
    }
    if headless {
        let replay = option("--replay");
        if let Some(path) = replay {
            let steps: Vec<ReplayStep> =
                serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let mut now = 0;
            for step in steps {
                match step {
                    ReplayStep::Wait { ms } => {
                        let end = now + ms;
                        while now < end {
                            now = (now + 10).min(end);
                            game.advance(now)?;
                        }
                    }
                    ReplayStep::Press { action } => game.action(action, true)?,
                    ReplayStep::Release { action } => game.action(action, false)?,
                    ReplayStep::Command { label } => {
                        if !game.command(&label)? {
                            return Err(format!("Command unavailable: {label}"));
                        }
                    }
                    ReplayStep::Screenshot { path } => game.screen.borrow().save(&path)?,
                    ReplayStep::AssertState { state } => {
                        let actual = game.state();
                        if actual != state {
                            return Err(format!("Expected state {state}, found {actual}"));
                        }
                    }
                }
            }
        } else {
            for i in 0..30 {
                game.advance(i * 70)?;
            }
            game.action(input::Action::Primary, true)?;
            for i in 30..60 {
                game.advance(i * 70)?;
            }
            game.action(input::Action::Primary, true)?;
            for i in 60..100 {
                game.advance(i * 70)?;
            }
        }
        println!("State: {}", game.state());
        game.screen.borrow().save(
            &option("--screenshot").unwrap_or_else(|| root.join("target/native-screen.png")),
        )?;
        return Ok(());
    }
    let close_after = args
        .iter()
        .position(|a| a == "--close-after")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok());
    desktop::run(game, close_after, option("--screenshot"))
}

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ReplayStep {
    Wait { ms: u64 },
    Press { action: input::Action },
    Release { action: input::Action },
    Command { label: String },
    Screenshot { path: std::path::PathBuf },
    AssertState { state: i32 },
}
