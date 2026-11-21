use crate::{
    audio::{AudioCore, Mixer},
    engine::Game,
    graphics::{Bitmap, Graphics, HEIGHT, WIDTH},
    input::{Action, Bindings, Input, stick},
};
use sdl3::{
    event::{Event, WindowEvent},
    gamepad::{Axis, Button},
    keyboard::Keycode,
    pixels::PixelFormat,
    rect::Rect,
    render::ScaleMode,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::mpsc,
    time::{Duration, Instant},
};

#[derive(Default)]
struct Controls {
    open: bool,
    row: usize,
    controller: bool,
    capturing: bool,
    message: String,
}
impl Controls {
    fn key(
        &mut self,
        key: Keycode,
        bindings: &mut Bindings,
        path: &std::path::Path,
    ) -> Result<(), String> {
        if self.capturing {
            if key == Keycode::Escape {
                self.capturing = false;
            } else if !self.controller {
                bindings.bind(false, format!("{key:?}"), Action::ALL[self.row]);
                bindings.save(path)?;
                self.capturing = false;
            }
            return Ok(());
        }
        match key {
            Keycode::Escape | Keycode::F2 => self.open = false,
            Keycode::Up => self.row = (self.row + 8) % 9,
            Keycode::Down => self.row = (self.row + 1) % 9,
            Keycode::Tab | Keycode::Left | Keycode::Right => self.controller = !self.controller,
            Keycode::Z | Keycode::Return => self.capturing = true,
            Keycode::R => {
                *bindings = Bindings::default();
                bindings.save(path)?;
                self.message = "Defaults restored".into();
            }
            _ => {}
        }
        Ok(())
    }
    fn button(
        &mut self,
        button: Button,
        bindings: &mut Bindings,
        path: &std::path::Path,
    ) -> Result<(), String> {
        if self.capturing {
            if self.controller {
                bindings.bind(true, button.string(), Action::ALL[self.row]);
                bindings.save(path)?;
                self.capturing = false;
            }
            return Ok(());
        }
        match button {
            Button::Back | Button::Start => self.open = false,
            Button::DPadUp => self.row = (self.row + 8) % 9,
            Button::DPadDown => self.row = (self.row + 1) % 9,
            Button::DPadLeft | Button::DPadRight | Button::North => {
                self.controller = !self.controller
            }
            Button::South => self.capturing = true,
            _ => {}
        }
        Ok(())
    }
}
fn send(game: &mut Game, edges: Vec<(Action, bool)>) -> Result<(), String> {
    for (a, down) in edges {
        game.action(a, down)?;
    }
    Ok(())
}
pub fn run(
    mut game: Game,
    close_after: Option<u64>,
    screenshot: Option<std::path::PathBuf>,
) -> Result<(), String> {
    let sdl = sdl3::init().map_err(|e| e.to_string())?;
    let video = sdl.video().map_err(|e| e.to_string())?;
    let window = video
        .window("Zelda Mobile — Rust", 480, 640)
        .position_centered()
        .resizable()
        .high_pixel_density()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas();
    canvas
        .window_mut()
        .set_minimum_size(240, 320)
        .map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_streaming(PixelFormat::RGBA32, WIDTH as u32, HEIGHT as u32)
        .map_err(|e| e.to_string())?;
    texture.set_scale_mode(ScaleMode::Nearest);
    let gamepads = sdl.gamepad().map_err(|e| e.to_string())?;
    let mut pads = HashMap::new();
    for id in gamepads.gamepads().map_err(|e| e.to_string())? {
        if let Ok(pad) = gamepads.open(id) {
            println!("Controller connected: {}", pad.name().unwrap_or_default());
            pads.insert(id, (pad, 0i16, 0i16));
        }
    }
    let audio_sub = sdl.audio().map_err(|e| e.to_string())?;
    let audio_core = AudioCore::new(&game.runtime.assets)?;
    let (tx, rx) = mpsc::channel();
    let spec = sdl3::audio::AudioSpec {
        freq: Some(AudioCore::RATE),
        channels: Some(2),
        format: Some(sdl3::audio::AudioFormat::f32_sys()),
    };
    let audio_stream = audio_sub
        .open_playback_stream(
            &spec,
            Mixer {
                core: audio_core,
                commands: rx,
            },
        )
        .map_err(|e| e.to_string())?;
    audio_stream.resume().map_err(|e| e.to_string())?;
    let bindings_path = game.runtime.saves.join("controls.json");
    let mut bindings = Bindings::load(&bindings_path);
    let mut input = Input::default();
    let mut controls = Controls::default();
    let mut events = sdl.event_pump().map_err(|e| e.to_string())?;
    let start = Instant::now();
    let mut focused = true;
    let mut full = false;
    let mut elapsed = Duration::ZERO;
    let mut last = Instant::now();
    let mut audio_active = true;
    'running: loop {
        let now = Instant::now();
        if focused && !controls.open {
            elapsed += now.duration_since(last);
        }
        last = now;
        for e in events.poll_iter() {
            match e {
                Event::Quit { .. }
                | Event::Window {
                    win_event: WindowEvent::CloseRequested,
                    ..
                } => break 'running,
                Event::Window {
                    win_event: WindowEvent::FocusLost,
                    ..
                } => {
                    focused = false;
                    send(&mut game, input.clear())?;
                }
                Event::Window {
                    win_event: WindowEvent::FocusGained,
                    ..
                } => focused = true,
                Event::KeyDown {
                    keycode: Some(key),
                    repeat: false,
                    ..
                } => {
                    if controls.open {
                        controls.key(key, &mut bindings, &bindings_path)?;
                        continue;
                    }
                    match key {
                        Keycode::F2 => {
                            send(&mut game, input.clear())?;
                            controls.open = true;
                        }
                        Keycode::F11 => {
                            full = !full;
                            canvas
                                .window_mut()
                                .set_fullscreen(full)
                                .map_err(|e| e.to_string())?;
                        }
                        Keycode::Escape => game.back()?,
                        Keycode::F12 => {
                            let p = game.runtime.saves.join("Screenshot.png");
                            game.screen.borrow().save(&p)?;
                            println!("Screenshot: {}", p.display());
                        }
                        _ => {
                            let key = format!("{key:?}");
                            if let Some(a) = bindings.keyboard.get(&key) {
                                send(&mut game, input.update(format!("key:{key}"), Some(*a)))?;
                            }
                        }
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    send(&mut game, input.update(format!("key:{key:?}"), None))?;
                }
                Event::GamepadAdded { which, .. } => {
                    if let std::collections::hash_map::Entry::Vacant(entry) = pads.entry(which) {
                        if let Ok(p) = gamepads.open(which) {
                            println!("Controller connected: {}", p.name().unwrap_or_default());
                            entry.insert((p, 0i16, 0i16));
                        }
                    }
                }
                Event::GamepadRemoved { which, .. } => {
                    pads.remove(&which);
                    send(&mut game, input.clear_device(&format!("pad:{which:?}:")))?;
                }
                Event::GamepadButtonDown { which, button, .. } => {
                    if controls.open {
                        controls.button(button, &mut bindings, &bindings_path)?;
                        continue;
                    }
                    if button == Button::Back {
                        send(&mut game, input.clear())?;
                        controls.open = true;
                        continue;
                    }
                    if let Some(action) = bindings.controller.get(&button.string()) {
                        send(
                            &mut game,
                            input.update(
                                format!("pad:{which:?}:{}", button.string()),
                                Some(*action),
                            ),
                        )?;
                    }
                }
                Event::GamepadButtonUp { which, button, .. } => {
                    send(
                        &mut game,
                        input.update(format!("pad:{which:?}:{}", button.string()), None),
                    )?;
                }
                Event::GamepadAxisMotion {
                    which, axis, value, ..
                } => {
                    if let Some((_, x, y)) = pads.get_mut(&which) {
                        match axis {
                            Axis::LeftX => *x = value,
                            Axis::LeftY => *y = value,
                            _ => continue,
                        }
                        if !controls.open && focused {
                            send(
                                &mut game,
                                input.update(format!("pad:{which:?}:stick"), stick(*x, *y)),
                            )?;
                        }
                    }
                }
                _ => {}
            }
        }
        if focused && !controls.open {
            game.advance(elapsed.as_millis() as u64)?;
        }
        let should_play = focused && !controls.open;
        if should_play != audio_active {
            if should_play {
                audio_stream.resume()
            } else {
                audio_stream.pause()
            }
            .map_err(|e| e.to_string())?;
            audio_active = should_play;
        }
        for c in game.runtime.audio.drain(..) {
            tx.send(c).map_err(|e| e.to_string())?;
        }
        if game.runtime.quit {
            break;
        }
        // The game updates at its original cadence; drawing the unchanged native
        // surface more frequently has no gameplay side effects.
        let frame = decorate(&mut game, &controls, &bindings, pads.len());
        let bytes: Vec<u8> = frame.pixels.iter().flat_map(|p| p.to_le_bytes()).collect();
        texture
            .update(None, &bytes, WIDTH as usize * 4)
            .map_err(|e| e.to_string())?;
        let (ow, oh) = canvas.output_size().map_err(|e| e.to_string())?;
        let scale = (ow / WIDTH as u32).min(oh / HEIGHT as u32).max(1);
        let (w, h) = (WIDTH as u32 * scale, HEIGHT as u32 * scale);
        let dest = Rect::new((ow as i32 - w as i32) / 2, (oh as i32 - h as i32) / 2, w, h);
        canvas.set_draw_color(sdl3::pixels::Color::RGB(8, 10, 12));
        canvas.clear();
        canvas
            .copy(&texture, None, dest)
            .map_err(|e| e.to_string())?;
        canvas.present();
        if close_after.is_some_and(|ms| start.elapsed().as_millis() >= ms as u128) {
            if let Some(p) = &screenshot {
                frame.save(p)?;
            }
            break;
        }
        std::thread::sleep(Duration::from_millis(8));
    }
    game.shutdown()?;
    Ok(())
}
fn decorate(game: &mut Game, controls: &Controls, bindings: &Bindings, pads: usize) -> Bitmap {
    let target = Rc::new(RefCell::new(game.screen.borrow().clone()));
    let mut g = Graphics::new(target.clone());
    if controls.open {
        g.color(0x10191c);
        g.fill_rect(0, 0, 240, 320);
        g.color(0xf1d477);
        game.runtime
            .fonts
            .draw(&mut g, "Controls", 12, 10, 0, 18, true);
        g.color(0xc7d6ce);
        game.runtime.fonts.draw(
            &mut g,
            if controls.controller {
                "Xbox controller"
            } else {
                "MacBook keyboard"
            },
            12,
            37,
            0,
            12,
            true,
        );
        for (i, a) in Action::ALL.iter().enumerate() {
            let y = 65 + i as i32 * 20;
            if i == controls.row {
                g.color(0x30483d);
                g.fill_rect(7, y - 1, 226, 20);
            }
            g.color(0xf5f0dc);
            game.runtime
                .fonts
                .draw(&mut g, a.label(), 12, y, 0, 12, false);
            g.color(0xa9d4ae);
            let label = bindings.description(controls.controller, *a);
            game.runtime
                .fonts
                .draw(&mut g, &label, 227, y, 8, 12, false);
        }
        g.color(0xc7d6ce);
        let lines = if controls.capturing {
            vec!["Press a new key or button", "Escape cancels"]
        } else {
            vec![
                "Z / A: change    Tab / Y: device",
                "Esc / View: close    R: defaults",
            ]
        };
        for (i, line) in lines.iter().enumerate() {
            game.runtime
                .fonts
                .draw(&mut g, line, 12, 260 + i as i32 * 17, 0, 12, false);
        }
        game.runtime.fonts.draw(
            &mut g,
            &format!("{pads} controller(s) connected"),
            12,
            300,
            0,
            12,
            false,
        );
    } else {
        let commands = game.commands();
        g.color(0x16231c);
        g.fill_rect(0, 300, 240, 20);
        g.color(0xe8dec3);
        for (i, (_, label)) in commands.iter().take(2).enumerate() {
            game.runtime.fonts.draw(
                &mut g,
                label,
                if i == 0 { 5 } else { 235 },
                302,
                if i == 0 { 0 } else { 8 },
                12,
                false,
            );
        }
    }
    drop(g);
    Rc::try_unwrap(target).ok().unwrap().into_inner()
}
