//! Browser bindings. The game and synth remain independent Wasm instances.
use crate::{
    assets::EmbeddedAssets,
    audio::{AudioCommand, AudioCore},
    engine::Game,
    input::{Action, Input},
    storage::{self, RecordStore},
};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};
use wasm_bindgen::prelude::*;

// Keep storage keys stable across repository/URL renames to retain existing saves.
const PREFIX: &str = "zelda-mobile-rust:v1:";

struct BrowserRecords {
    storage: RefCell<Option<web_sys::Storage>>,
    records: RefCell<BTreeMap<String, Vec<u8>>>,
    warnings: RefCell<Vec<String>>,
}
impl BrowserRecords {
    fn new() -> Self {
        let mut records = BTreeMap::new();
        let mut warnings = Vec::new();
        let storage = (|| -> Result<web_sys::Storage, JsValue> {
            let store = web_sys::window()
                .ok_or_else(|| JsValue::from_str("No window"))?
                .local_storage()?
                .ok_or_else(|| JsValue::from_str("Storage unavailable"))?;
            for i in 0..store.length()? {
                if let Some(key) = store.key(i)? {
                    if let Some(name) = key.strip_prefix(PREFIX) {
                        if let Some(value) = store.get_item(&key)? {
                            match serde_json::from_str::<Vec<u8>>(&value) {
                                Ok(bytes) => {
                                    records.insert(name.to_owned(), bytes);
                                }
                                Err(_) => warnings.push(format!(
                                    "Saved record {name} is damaged and could not be loaded."
                                )),
                            }
                        }
                    }
                }
            }
            // A full or read-only store must still load existing adventures.
            let probe = format!("{PREFIX}probe");
            if store.set_item(&probe, "1").is_err() {
                warnings.push("Browser storage cannot accept new saves. Existing saves are available; new progress may last only for this session.".into());
            } else {
                let _ = store.remove_item(&probe);
            }
            Ok(store)
        })()
        .ok();
        if storage.is_none() {
            warnings.push(
                "Browser storage is unavailable. Progress will last only for this session.".into(),
            );
        }
        Self {
            storage: RefCell::new(storage),
            records: RefCell::new(records),
            warnings: RefCell::new(warnings),
        }
    }
    fn warn(&self, message: String) {
        let mut warnings = self.warnings.borrow_mut();
        if !warnings.contains(&message) {
            warnings.push(message);
        }
    }
}
impl RecordStore for BrowserRecords {
    fn read(&self, name: &str) -> Result<Option<Vec<u8>>, String> {
        let record = self.records.borrow().get(name).cloned();
        if let Some(bytes) = &record {
            if let Err(e) = storage::decode_record(bytes) {
                self.warn(format!("Saved record {name} could not be loaded: {e}"));
                return Err(e);
            }
        }
        Ok(record)
    }
    fn write(&self, name: &str, bytes: &[u8]) -> Result<(), String> {
        self.records
            .borrow_mut()
            .insert(name.to_owned(), bytes.to_vec());
        if let Some(store) = self.storage.borrow().as_ref() {
            if store
                .set_item(
                    &format!("{PREFIX}{name}"),
                    &serde_json::to_string(bytes).unwrap(),
                )
                .is_err()
            {
                self.warn("Progress could not be saved to browser storage. Keep this page open to retain this session.".into());
            }
        }
        Ok(())
    }
    fn delete(&self, name: &str) -> Result<(), String> {
        // Only forget the in-memory record after the durable deletion succeeds.
        if let Some(store) = self.storage.borrow().as_ref() {
            if store.remove_item(&format!("{PREFIX}{name}")).is_err() {
                self.warn("The save could not be deleted from browser storage.".into());
                return Err("Could not delete save".into());
            }
        }
        self.records.borrow_mut().remove(name);
        Ok(())
    }
    fn list(&self) -> Result<Vec<String>, String> {
        Ok(self.records.borrow().keys().cloned().collect())
    }
}

#[wasm_bindgen]
pub struct BrowserGame {
    game: Game,
    input: Input,
    records: Rc<BrowserRecords>,
}

#[wasm_bindgen]
impl BrowserGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<BrowserGame, JsValue> {
        let records = Rc::new(BrowserRecords::new());
        let game = Game::with_services(Rc::new(EmbeddedAssets), records.clone(), false)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(Self {
            game,
            input: Input::default(),
            records,
        })
    }
    pub fn advance(&mut self, elapsed_ms: f64) -> Result<(), JsValue> {
        self.game
            .advance(elapsed_ms.max(self.game.runtime.now as f64) as u64)
            .map_err(|e| JsValue::from_str(&e))
    }
    /// Action indices follow Action::ALL; -1 releases the source.
    pub fn input(&mut self, source: String, action: i32) -> Result<(), JsValue> {
        let action = if action == -1 {
            None
        } else {
            Some(
                *Action::ALL
                    .get(action as usize)
                    .ok_or_else(|| JsValue::from_str("Unknown action"))?,
            )
        };
        let edges = self.input.update(source, action);
        self.send(edges)
    }
    pub fn clear_input(&mut self) -> Result<(), JsValue> {
        let edges = self.input.clear();
        self.send(edges)
    }
    pub fn back(&mut self) -> Result<(), JsValue> {
        self.game.back().map_err(|e| JsValue::from_str(&e))
    }
    pub fn pixels(&self) -> Vec<u8> {
        self.game
            .screen
            .borrow()
            .pixels
            .iter()
            .flat_map(|p| p.to_le_bytes())
            .collect()
    }
    pub fn commands(&self) -> String {
        serde_json::to_string(
            &self
                .game
                .commands()
                .iter()
                .map(|(_, s)| s)
                .collect::<Vec<_>>(),
        )
        .unwrap()
    }
    pub fn drain_audio(&mut self) -> String {
        serde_json::to_string(&self.game.runtime.audio.drain(..).collect::<Vec<_>>()).unwrap()
    }
    pub fn checkpoint(&mut self) -> Result<(), JsValue> {
        self.game.checkpoint().map_err(|e| JsValue::from_str(&e))
    }
    pub fn warnings(&self) -> String {
        serde_json::to_string(&self.records.warnings.take()).unwrap()
    }
    pub fn state(&mut self) -> i32 {
        self.game.state()
    }
    pub fn has_quit(&self) -> bool {
        self.game.runtime.quit
    }
}
impl BrowserGame {
    fn send(&mut self, edges: Vec<(Action, bool)>) -> Result<(), JsValue> {
        for (action, down) in edges {
            self.game
                .action(action, down)
                .map_err(|e| JsValue::from_str(&e))?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
pub struct BrowserAudio(AudioCore);
#[wasm_bindgen]
impl BrowserAudio {
    #[wasm_bindgen(constructor)]
    pub fn new(soundfont: &[u8], sample_rate: i32) -> Result<BrowserAudio, JsValue> {
        AudioCore::from_assets(&EmbeddedAssets, soundfont, sample_rate)
            .map(Self)
            .map_err(|e| JsValue::from_str(&e))
    }
    pub fn command(&mut self, json: &str) -> Result<(), JsValue> {
        let commands: Vec<AudioCommand> =
            serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        for command in commands {
            self.0.command(command);
        }
        Ok(())
    }
    pub fn render(&mut self, frames: usize) -> Result<Vec<f32>, JsValue> {
        if frames > 8192 {
            return Err(JsValue::from_str("Audio block too large"));
        }
        Ok(self.0.render(frames))
    }
}
