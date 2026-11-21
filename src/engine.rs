use crate::{
    graphics::{Bitmap, Graphics, Surface},
    runtime::{Native, Runtime, V},
};
use std::{cell::RefCell, collections::HashSet, path::PathBuf, rc::Rc};

pub struct Game {
    pub runtime: Runtime,
    pub zelda: u32,
    pub canvas: u32,
    pub screen: Surface,
    pub graphics: u32,
    started_threads: HashSet<u32>,
    last_gc: u64,
}
impl Game {
    pub fn new(assets: PathBuf, saves: PathBuf, verbose: bool) -> Result<Self, String> {
        let mut r = Runtime::new(assets, saves)?;
        r.verbose = verbose;
        let zelda = r.alloc("Zelda", Native::Empty);
        r.call(zelda, "<init>", "()V", &[]).map_err(|e| {
            format!(
                "Game initialization: {}\n{}",
                r.string(e),
                r.errors.join("\n")
            )
        })?;
        r.call(zelda, "startApp", "()V", &[])
            .map_err(|e| r.string(e))?;
        let canvas = r.field(zelda, "a", "Lk;").r();
        // Phone builds force key configuration when no RMS exists. The native
        // host already supplies complete keyboard/controller bindings.
        if r.field(canvas, "aG", "Z").i() != 0 {
            r.put(canvas, "aG", "Z", V::I(0));
            r.put(canvas, "aH", "I", V::I(0));
        }
        let screen = Rc::new(RefCell::new(Bitmap::new(240, 320)));
        let graphics = r.alloc(
            "javax/microedition/lcdui/Graphics",
            Native::Graphics(Graphics::new(screen.clone())),
        );
        r.roots.extend([zelda, canvas, graphics]);
        let mut g = Self {
            runtime: r,
            zelda,
            canvas,
            screen,
            graphics,
            started_threads: HashSet::new(),
            last_gc: 0,
        };
        g.paint()?;
        Ok(g)
    }
    fn error(&self, e: u32) -> String {
        format!(
            "{}: {}\n{}",
            self.runtime.obj(e).class,
            self.runtime.string(e),
            self.runtime
                .errors
                .iter()
                .rev()
                .take(6)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
    pub fn state(&mut self) -> i32 {
        self.runtime.field(self.canvas, "p", "I").i()
    }
    pub fn paint(&mut self) -> Result<(), String> {
        self.runtime.with_graphics(self.graphics, |g, _| g.reset());
        self.runtime
            .call(
                self.canvas,
                "paint",
                "(Ljavax/microedition/lcdui/Graphics;)V",
                &[V::R(self.graphics)],
            )
            .map_err(|e| self.error(e))?;
        self.runtime.repaint = false;
        Ok(())
    }
    pub fn advance(&mut self, now: u64) -> Result<(), String> {
        self.runtime.now = now;
        let mut due = Vec::new();
        for t in &mut self.runtime.timers {
            let mut n = 0;
            while t.next <= now && n < 4 {
                due.push(t.task);
                t.next += t.period;
                n += 1;
            }
            if t.next <= now {
                t.next = now + t.period;
            }
        }
        for task in due {
            self.runtime
                .call(task, "run", "()V", &[])
                .map_err(|e| self.error(e))?;
        }
        if self.runtime.repaint && !self.runtime.quit {
            self.paint()?;
        }
        self.poll_sound()?;
        if now - self.last_gc >= 1000 {
            self.runtime.collect();
            self.last_gc = now;
        }
        Ok(())
    }
    pub fn key(&mut self, code: i32, down: bool) -> Result<(), String> {
        let name = if down { "keyPressed" } else { "keyReleased" };
        self.runtime
            .call(self.canvas, name, "(I)V", &[V::I(code)])
            .map_err(|e| self.error(e))?;
        Ok(())
    }
    pub fn commands(&self) -> Vec<(u32, String)> {
        let mut items = self
            .runtime
            .commands
            .iter()
            .filter_map(|id| {
                if let Native::Command { label, priority } = &self.runtime.obj(*id).native {
                    Some((*priority, *id, label.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        items.sort_by_key(|x| x.0);
        items.into_iter().map(|(_, id, s)| (id, s)).collect()
    }
    pub fn command(&mut self, label: &str) -> Result<bool, String> {
        if let Some((id, _)) = self
            .commands()
            .into_iter()
            .find(|(_, s)| s.eq_ignore_ascii_case(label))
        {
            self.runtime
                .call(
                    self.zelda,
                    "commandAction",
                    "(Ljavax/microedition/lcdui/Command;Ljavax/microedition/lcdui/Displayable;)V",
                    &[V::R(id), V::R(self.canvas)],
                )
                .map_err(|e| self.error(e))?;
            self.runtime.repaint = true;
            return Ok(true);
        }
        Ok(false)
    }
    pub fn action(&mut self, action: crate::input::Action, down: bool) -> Result<(), String> {
        use crate::input::Action::*;
        if down
            && matches!(action, Primary)
            && self.state() == 0
            && self.runtime.field(self.canvas, "aH", "I").i() == 0
        {
            if self.command("Start")? {
                return Ok(());
            }
        }
        if down && matches!(action, Primary) && self.state() == 100 {
            self.command("Main Menu")?;
            return Ok(());
        }
        if down
            && action == Secondary
            && !(20..=26).contains(&self.state())
            && (self.state() != 1 || self.runtime.field(self.canvas, "m", "Z").i() != 0)
        {
            return self.back();
        }
        match action {
            SoftLeft | SoftRight => {
                if down {
                    let commands = self.commands();
                    let i = if action == SoftLeft { 0 } else { 1 };
                    if let Some((_, label)) = commands.get(i) {
                        self.command(label)?;
                    }
                }
            }
            _ => {
                let field = match action {
                    Up => "y",
                    Down => "z",
                    Left => "A",
                    Right => "B",
                    Primary => "C",
                    Secondary => "D",
                    Menu => "E",
                    _ => unreachable!(),
                };
                let code = self.runtime.field(self.canvas, field, "I").i();
                // Native logical controls remain usable when the old phone's
                // "UseJavaButtons" option is disabled or keypad codes change.
                self.runtime
                    .call(
                        self.canvas,
                        if down { "a" } else { "c" },
                        "(I)V",
                        &[V::I(code)],
                    )
                    .map_err(|e| self.error(e))?;
            }
        }
        Ok(())
    }
    pub fn back(&mut self) -> Result<(), String> {
        let state = self.state();
        let parent = match state {
            8 => Some((7, 0)),
            9 => Some((7, 2)),
            12 => Some((7, 1)),
            13 => Some((7, 3)),
            20..=26 => Some((8, 0)),
            10 | 30 => Some((0, 0)),
            7 => Some((self.runtime.field(self.canvas, "q", "I").i(), 0)),
            _ => None,
        };
        if let Some((parent, selection)) = parent {
            self.runtime.put(self.canvas, "p", "I", V::I(parent));
            self.runtime.put(self.canvas, "o", "I", V::I(selection));
            self.runtime.put(self.canvas, "r", "Z", V::I(1));
            self.runtime.put(self.canvas, "f", "Z", V::I(1));
            self.runtime.repaint = true;
            return Ok(());
        }
        if self.command("Return")? || self.command("Main Menu")? {
            return Ok(());
        }
        if state == 1 {
            if self.runtime.field(self.canvas, "m", "Z").i() != 0 {
                self.runtime
                    .call(self.canvas, "a", "(Z)V", &[V::I(0)])
                    .map_err(|e| self.error(e))?;
                self.runtime.put(self.canvas, "n", "I", V::I(0));
                self.runtime.put(self.canvas, "r", "Z", V::I(1));
                self.runtime.put(self.canvas, "f", "Z", V::I(1));
            } else {
                self.action(crate::input::Action::Menu, true)?;
            }
        }
        Ok(())
    }
    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.runtime.quit {
            let player = self.runtime.field(self.canvas, "e", "Lh;").r();
            if player != 0 {
                self.runtime
                    .call(self.canvas, "d", "()V", &[])
                    .map_err(|e| self.error(e))?;
            }
            self.runtime
                .call(self.canvas, "a", "()V", &[])
                .map_err(|e| self.error(e))?;
        }
        Ok(())
    }
    fn poll_sound(&mut self) -> Result<(), String> {
        // The original b.run() owns a 10ms polling thread. Its finite work is
        // performed cooperatively; MIDI/WAV rendering runs on SDL's audio thread.
        for o in self.runtime.threads.clone() {
            if self.runtime.obj(o).class != "b" {
                return Err("Unexpected game worker".into());
            }
            if self.started_threads.insert(o) {
                let ok = self
                    .runtime
                    .call(o, "a", "()Z", &[])
                    .map_err(|e| self.error(e))?;
                self.runtime.put(o, "k", "Z", ok);
                self.runtime
                    .put(o, "b", "Ljavax/microedition/media/Player;", V::R(0));
                self.runtime.put(o, "l", "I", V::I(-10));
                self.runtime
                    .call(o, "a", "(II)V", &[V::I(-1), V::I(0)])
                    .map_err(|e| self.error(e))?;
            }
            if self.runtime.field(o, "e", "Z").i() != 0 {
                continue;
            }
            let mut active = self.runtime.field(o, "o", "I").i();
            let began = self.runtime.field(o, "p", "J").j() as u64;
            let fx = self
                .runtime
                .field(o, "c", "Ljavax/microedition/media/Player;")
                .r();
            let durations = [0, 375, 450, 320, 500, 100, 1300, 2300, 500, 1100];
            if active > 0 && self.runtime.now.saturating_sub(began) > durations[active as usize] {
                if fx != 0 {
                    self.runtime
                        .call(fx, "stop", "()V", &[])
                        .map_err(|e| self.error(e))?;
                }
                active = 0;
                self.runtime.put(o, "o", "I", V::I(0));
            }
            let mut request = self.runtime.field(o, "d", "I").i();
            let pending = self.runtime.field(o, "i", "I").i();
            if request == 0 && pending != 0 && active == 0 {
                request = pending;
                self.runtime.put(o, "d", "I", V::I(request));
                self.runtime.put(o, "i", "I", V::I(0));
            }
            if request == 0
                || self.runtime.field(o, "a", "Z").i() == 0
                || self.runtime.field(o, "k", "Z").i() == 0
                || self.runtime.field(o, "n", "I").i() == 0
            {
                continue;
            }
            if active == 6 || active == 7 || active == 9 {
                if pending == 0 && request == 7 && active != 9 {
                    self.runtime.put(o, "i", "I", V::I(request));
                }
                self.runtime.put(o, "d", "I", V::I(0));
                continue;
            }
            if active != 0 && fx != 0 {
                self.runtime
                    .call(fx, "stop", "()V", &[])
                    .map_err(|e| self.error(e))?;
            }
            let (effect, position) = match request {
                1 => match active {
                    1 => (2, 585000),
                    2 => (3, 1145000),
                    _ => (1, 39000),
                },
                2 => (4, 1511000),
                3 => (5, 2057000),
                4 => (6, 2252000),
                5 => (7, 5000000),
                6 => (8, 7450000),
                7 => (9, 3760000),
                _ => {
                    self.runtime.put(o, "d", "I", V::I(0));
                    continue;
                }
            };
            if fx != 0 {
                self.runtime.put(o, "o", "I", V::I(effect));
                self.runtime.put(o, "p", "J", V::J(self.runtime.now as i64));
                self.runtime
                    .call(fx, "setMediaTime", "(J)J", &[V::J(position)])
                    .map_err(|e| self.error(e))?;
                self.runtime
                    .call(fx, "start", "()V", &[])
                    .map_err(|e| self.error(e))?;
            }
            self.runtime.put(o, "d", "I", V::I(0));
        }
        Ok(())
    }
}
