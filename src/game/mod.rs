//! Independently maintained native gameplay.
//! Start with `canvas`, `world`, and `player`; `compat` isolates legacy identifiers.
//!
//! Shared labeled exits in a few large routines are deliberately aligned rather
//! than indented once per exit. This retains one copy of each gameplay branch.
//! Those functions opt out of rustfmt; other code uses normal Rust formatting.
#![allow(
    unused_imports,
    unused_variables,
    unused_mut,
    unused_assignments,
    unused_parens,
    unused_labels,
    unreachable_code,
    dead_code
)]
mod arithmetic;
mod canvas;
mod compat;
mod drawable;
mod enemy;
pub(crate) mod fields;
mod frame_timer;
mod host;
mod item;
mod lifecycle;
mod map_object;
mod motion;
mod player;
mod player_save;
mod portal;
mod projectile;
mod resources;
mod room_history;
mod second_timer;
pub(crate) mod sound;
mod trail_point;
mod world;
pub(crate) use compat::{FIELDS, METHODS, dispatch, parent};
#[cfg(test)]
mod projectile_tests;
