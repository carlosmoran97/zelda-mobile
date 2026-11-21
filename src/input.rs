use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Primary,
    Secondary,
    Menu,
    SoftLeft,
    SoftRight,
}
impl Action {
    pub const ALL: [Self; 9] = [
        Self::Up,
        Self::Down,
        Self::Left,
        Self::Right,
        Self::Primary,
        Self::Secondary,
        Self::Menu,
        Self::SoftLeft,
        Self::SoftRight,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Primary => "Primary / confirm",
            Self::Secondary => "Secondary / back",
            Self::Menu => "Game menu",
            Self::SoftLeft => "Left command",
            Self::SoftRight => "Right command",
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Bindings {
    pub keyboard: HashMap<String, Action>,
    pub controller: HashMap<String, Action>,
}
impl Default for Bindings {
    fn default() -> Self {
        use Action::*;
        Self {
            keyboard: [
                ("Up", Up),
                ("Down", Down),
                ("Left", Left),
                ("Right", Right),
                ("Z", Primary),
                ("X", Secondary),
                ("Return", Menu),
                ("Q", SoftLeft),
                ("W", SoftRight),
            ]
            .into_iter()
            .map(|(s, a)| (s.into(), a))
            .collect(),
            controller: [
                ("dpup", Up),
                ("dpdown", Down),
                ("dpleft", Left),
                ("dpright", Right),
                ("a", Primary),
                ("b", Secondary),
                ("start", Menu),
                ("leftshoulder", SoftLeft),
                ("rightshoulder", SoftRight),
            ]
            .into_iter()
            .map(|(s, a)| (s.into(), a))
            .collect(),
        }
    }
}
impl Bindings {
    pub fn load(path: &std::path::Path) -> Self {
        std::fs::read(path)
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or_default()
    }
    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        crate::storage::atomic_write(
            path,
            &serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }
    pub fn bind(&mut self, controller: bool, key: String, action: Action) {
        let table = if controller {
            &mut self.controller
        } else {
            &mut self.keyboard
        };
        table.retain(|_, a| *a != action);
        table.insert(key, action);
    }
    pub fn description(&self, controller: bool, action: Action) -> String {
        let table = if controller {
            &self.controller
        } else {
            &self.keyboard
        };
        let mut names = table
            .iter()
            .filter(|(_, a)| **a == action)
            .map(|(k, _)| k.clone())
            .collect::<Vec<_>>();
        names.sort();
        names.join(" / ")
    }
}
#[derive(Default)]
pub struct Input {
    sources: HashMap<String, (Action, u64)>,
    active: HashSet<Action>,
    sequence: u64,
}
impl Input {
    pub fn update(&mut self, source: String, action: Option<Action>) -> Vec<(Action, bool)> {
        if let Some(a) = action {
            if self.sources.get(&source).is_some_and(|x| x.0 == a) {
                return Vec::new();
            }
            self.sequence += 1;
            self.sources.insert(source, (a, self.sequence));
        } else {
            self.sources.remove(&source);
        }
        self.reconcile()
    }
    pub fn clear(&mut self) -> Vec<(Action, bool)> {
        self.sources.clear();
        self.reconcile()
    }
    pub fn clear_device(&mut self, prefix: &str) -> Vec<(Action, bool)> {
        self.sources.retain(|s, _| !s.starts_with(prefix));
        self.reconcile()
    }
    fn reconcile(&mut self) -> Vec<(Action, bool)> {
        let mut next = self.sources.values().map(|x| x.0).collect::<HashSet<_>>();
        for (a, b) in [(Action::Up, Action::Down), (Action::Left, Action::Right)] {
            if next.contains(&a) && next.contains(&b) {
                let latest = |x| {
                    self.sources
                        .values()
                        .filter(|p| p.0 == x)
                        .map(|p| p.1)
                        .max()
                        .unwrap_or(0)
                };
                next.remove(if latest(a) > latest(b) { &b } else { &a });
            }
        }
        let mut out = Action::ALL
            .into_iter()
            .filter(|a| self.active.contains(a) && !next.contains(a))
            .map(|a| (a, false))
            .collect::<Vec<_>>();
        out.extend(
            Action::ALL
                .into_iter()
                .filter(|a| !self.active.contains(a) && next.contains(a))
                .map(|a| (a, true)),
        );
        self.active = next;
        out
    }
}
pub fn stick(x: i16, y: i16) -> Option<Action> {
    let x = x as i32;
    let y = y as i32;
    if x.abs().max(y.abs()) < 9000 {
        None
    } else if x.abs() > y.abs() {
        Some(if x < 0 { Action::Left } else { Action::Right })
    } else {
        Some(if y < 0 { Action::Up } else { Action::Down })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_inputs_and_disconnect() {
        let mut i = Input::default();
        assert_eq!(
            i.update("key".into(), Some(Action::Up)),
            vec![(Action::Up, true)]
        );
        assert!(i.update("pad:1:a".into(), Some(Action::Up)).is_empty());
        assert!(i.update("key".into(), None).is_empty());
        assert_eq!(i.clear_device("pad:1:"), vec![(Action::Up, false)]);
    }
    #[test]
    fn opposite_directions_restore_held_key() {
        let mut i = Input::default();
        i.update("left".into(), Some(Action::Left));
        assert_eq!(
            i.update("right".into(), Some(Action::Right)),
            vec![(Action::Left, false), (Action::Right, true)]
        );
        assert_eq!(
            i.update("right".into(), None),
            vec![(Action::Right, false), (Action::Left, true)]
        );
        assert_eq!(stick(200, 300), None);
        assert_eq!(stick(-20000, 10000), Some(Action::Left));
    }
}
