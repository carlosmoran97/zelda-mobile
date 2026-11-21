//! Map loading, collision, transitions and entity updates.
use super::{arithmetic, fields as field, host};
use crate::game::{
    enemy, item, map_object, portal, projectile, resources, room_history, trail_point, world,
};
use crate::runtime::{JResult, Native, Runtime, V};
#[path = "world/rooms.rs"]
mod rooms;
pub(super) use rooms::{initialize, load_map, mark_persistent_event};
#[path = "world/collision.rs"]
mod collision;
pub(super) use collision::{can_entity_move, can_player_move, find_portal};
#[path = "world/interactions.rs"]
mod interactions;
pub(super) use interactions::{
    count_active_triggers, dig, interact, spawn_drops, update_contacts, update_triggers,
};
#[path = "world/enemies.rs"]
mod enemies;
pub(super) use enemies::update_enemies;
#[path = "world/projectiles.rs"]
mod projectiles;
pub(super) use projectiles::{
    check_hookshot_contact, update_hookshot, update_hookshot_target, update_projectiles,
};
#[path = "world/rendering.rs"]
mod rendering;
pub(super) use rendering::{draw, draw_dialogue, draw_item_message};
