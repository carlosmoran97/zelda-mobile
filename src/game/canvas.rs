//! Menus, input, progression and save-slot management.
use super::{arithmetic, fields as field, host};
use crate::game::{
    canvas, item, lifecycle, map_object, player, projectile, resources, sound, trail_point, world,
};
use crate::runtime::{JResult, Native, Runtime, V};
#[path = "canvas/initialization.rs"]
mod initialization;
pub(super) use initialization::{initialize, initialize_empty};
#[path = "canvas/saves.rs"]
mod saves;
pub(super) use saves::{delete_save_slot, load_save_slots, save_game, save_settings, start_game};
#[path = "canvas/inventory.rs"]
mod inventory;
pub(super) use inventory::{
    draw_inventory, draw_map_marker, load_inventory_images, remove_menu_entry, set_inventory_open,
};
#[path = "canvas/rendering.rs"]
mod rendering;
pub(super) use rendering::{draw_hud, draw_key_prompt, draw_title, paint};
#[path = "canvas/controls.rs"]
mod controls;
pub(super) use controls::{
    center_camera, key_pressed, key_released, pointer_dragged, pointer_pressed, pointer_released,
    press_control, release_control,
};
#[path = "canvas/settings.rs"]
mod settings;
pub(super) use settings::draw_settings;
