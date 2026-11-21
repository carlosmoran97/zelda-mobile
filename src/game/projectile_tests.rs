//! Combat regression cases captured before retiring the mechanical Rust port.
use super::*;
use crate::{
    engine::Game,
    input::Action,
    runtime::{Native, V},
};
fn game() -> Game {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut game = Game::new(
        root.join("assets"),
        root.join("target/projectile-parity-saves"),
        false,
    )
    .unwrap();
    game.action(Action::Primary, true).unwrap();
    game.advance(140).unwrap();
    game.action(Action::Primary, true).unwrap();
    for _ in 0..20 {
        game.paint().unwrap();
    }
    game
}
fn prepare(
    game: &mut Game,
    kind: i32,
    enemy_kind: i32,
    mode: i32,
    position: (i32, i32),
) -> (u32, u32, u32) {
    use fields::{drawable as f, enemy as l, player as h, projectile as p, world as w};
    let r = &mut game.runtime;
    let world = r.get(game.canvas, fields::canvas::WORLD).unwrap().r();
    let player = r.get(game.canvas, fields::canvas::PLAYER).unwrap().r();
    let sound = r.get(game.canvas, fields::canvas::SOUND).unwrap().r();
    for key in [w::ENEMIES, w::PROJECTILES, w::ITEMS, w::OBJECTS] {
        let v = r.get(world, key).unwrap().r();
        host::vector_remove_all_elements(r, v).unwrap();
    }
    let tiles = r.get(world, w::TILES).unwrap().r();
    for index in 0..r.array_len(tiles).unwrap() {
        r.array_set(tiles, index, V::I(if mode == 15 { 64 } else { 0 }))
            .unwrap();
    }
    for (key, value) in [
        (f::X, 96),
        (f::Y, 80),
        (h::HEALTH, 12),
        (h::RUPEES, 0),
        (h::HURT_TIMER, 0),
        (h::PROJECTILE_COOLDOWN, 3),
        (h::SHIELD_RAISED, (mode >> 2) & 1),
        (h::ATTACKING, (mode >> 3) & 1),
        (h::FACING, 0),
        (h::DEFENSE, 1),
    ] {
        r.set(player, key, V::I(value)).unwrap();
    }
    let resource = r.get(world, w::RESOURCES).unwrap().r();
    let image = resources::enemy_image(r, resource, enemy_kind).unwrap();
    let enemy = r.alloc("l", Native::Empty);
    enemy::initialize(r, enemy, image, 96, 80, 1, enemy_kind).unwrap();
    for (key, value) in [
        (l::ACTIVE, 1),
        (l::DYING, 0),
        (l::HEALTH, 3),
        (l::HURT_TIMER, 0),
        (l::BEHAVIOR_STATE, 5),
        (l::WIDTH, 16),
        (l::HEIGHT, 16),
    ] {
        r.set(enemy, key, V::I(value)).unwrap();
    }
    let list = r.get(world, w::ENEMIES).unwrap().r();
    host::vector_add_element(r, list, enemy).unwrap();
    let image = resources::projectile_image(r, resource, kind).unwrap();
    let shot = r.alloc("p", Native::Empty);
    projectile::initialize(r, shot, image, kind, 0, 0).unwrap();
    for (key, value) in [
        (f::X, position.0),
        (f::Y, position.1),
        (p::ENEMY_OWNED, mode & 1),
        (p::RETURNING, (mode >> 1) & 1),
        (p::OWNER_INDEX, 0),
        (p::FACING, 2),
        (p::DAMAGE, 2),
    ] {
        r.set(shot, key, V::I(value)).unwrap();
    }
    let list = r.get(world, w::PROJECTILES).unwrap().r();
    host::vector_add_element(r, list, shot).unwrap();
    game.screen.borrow_mut().pixels.fill(0xffffffff);
    r.with_graphics(game.graphics, |g, _| g.reset());
    (world, player, sound)
}
fn summary(game: &mut Game, world: u32, player: u32) -> serde_json::Value {
    use fields::{drawable as f, enemy as l, player as h, projectile as p, world as w};
    let r = &mut game.runtime;
    let player_state = [
        h::HEALTH,
        h::HURT_TIMER,
        h::PROJECTILE_COOLDOWN,
        h::BOOMERANG_ACTIVE,
        h::WORLD_FLAGS,
    ]
    .map(|key| r.get(player, key).unwrap().i());
    let mut entities = Vec::new();
    for (list_key, keys) in [
        (
            w::PROJECTILES,
            vec![
                p::KIND,
                f::X,
                f::Y,
                p::VELOCITY_X,
                p::VELOCITY_Y,
                p::DAMAGE,
                p::ENEMY_OWNED,
                p::RETURNING,
                p::ANIMATION,
            ],
        ),
        (
            w::ENEMIES,
            vec![
                l::KIND,
                l::HEALTH,
                l::HURT_TIMER,
                l::BEHAVIOR_STATE,
                l::DYING,
                l::ANIMATION,
                l::HAS_PROJECTILE,
                l::HIT_COUNT,
                l::STATE_TIMER,
            ],
        ),
    ] {
        let list = r.get(world, list_key).unwrap().r();
        let mut rows = Vec::new();
        for index in 0..host::vector_size(r, list).unwrap() {
            let entity = host::vector_element_at(r, list, index).unwrap();
            rows.push(
                keys.iter()
                    .map(|&key| r.get(entity, key).unwrap().i())
                    .collect::<Vec<_>>(),
            );
        }
        entities.push(rows);
    }
    let hash = game
        .screen
        .borrow()
        .pixels
        .iter()
        .fold(0_u64, |h, &p| h.wrapping_mul(31).wrapping_add(p as u64));
    serde_json::json!([player_state, entities, hash])
}
#[test]
fn projectile_scenarios_match_the_original_behavior() {
    // Frozen during migration from the instruction-preserving implementation.
    // This fixture contains state and rendered-frame signatures, not game code.
    let cases: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("../../reference/projectile-behavior.json")).unwrap();
    let mut game = game();
    for case in cases {
        let kind = case[0].as_i64().unwrap() as i32;
        let enemy = case[1].as_i64().unwrap() as i32;
        let mode = case[2].as_i64().unwrap() as i32;
        let position = (
            case[3][0].as_i64().unwrap() as i32,
            case[3][1].as_i64().unwrap() as i32,
        );
        let (world, player, sound) = prepare(&mut game, kind, enemy, mode, position);
        let result = world::update_projectiles(
            &mut game.runtime,
            world,
            game.graphics,
            0,
            0,
            240,
            320,
            player,
            sound,
        );
        let result = match result {
            Ok(value) => serde_json::json!(value),
            Err(error) => serde_json::json!(game.runtime.obj(error).class),
        };
        assert_eq!(
            result, case[4],
            "outcome: kind={kind} enemy={enemy} mode={mode} position={position:?}"
        );
        assert_eq!(
            summary(&mut game, world, player),
            case[5],
            "state/frame: kind={kind} enemy={enemy} mode={mode} position={position:?}"
        );
        game.runtime.collect();
    }
}
