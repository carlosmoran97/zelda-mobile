//! Zelda Mobile game logic and native platform services.
pub mod audio;
pub mod assets;
pub mod engine;
mod game;
pub mod graphics;
pub mod input;
pub mod maps;
pub mod runtime;
pub mod storage;
pub mod validation;
#[cfg(target_arch = "wasm32")]
mod web;
