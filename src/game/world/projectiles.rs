//! World projectiles.
use super::*;

/// Update projectiles. Original reference: `j.a(Ljavax/microedition/lcdui/Graphics;IIIILh;Lb;)Z`.
pub(in crate::game) fn update_projectiles(
    r: &mut Runtime,
    world: u32,
    graphics: u32,
    camera_x: i32,
    camera_y: i32,
    width: i32,
    height: i32,
    player: u32,
    sound: u32,
) -> Result<i32, u32> {
    use field::{
        drawable as body, enemy as foe, item as pickup, map_object as scenery, player as hero,
        projectile as shot, world as room,
    };
    macro_rules! read {
        ($object:expr, $key:path) => {
            r.get($object, $key)?.i()
        };
    }
    macro_rules! reference {
        ($object:expr, $key:path) => {
            r.get($object, $key)?.r()
        };
    }
    macro_rules! write {
        ($object:expr, $key:path, $value:expr) => {{
            let value = $value;
            r.set($object, $key, V::I(value))?;
        }};
    }
    macro_rules! reverse {
        ($projectile:expr) => {{
            write!(
                $projectile,
                shot::VELOCITY_X,
                read!($projectile, shot::VELOCITY_X).wrapping_neg()
            );
            write!(
                $projectile,
                shot::VELOCITY_Y,
                read!($projectile, shot::VELOCITY_Y).wrapping_neg()
            );
        }};
    }
    macro_rules! die {
        ($enemy:expr) => {{
            write!($enemy, foe::DEATH_FRAME, 0);
            write!($enemy, foe::DYING, 1);
            let image = reference!(world, room::DEATH_IMAGE);
            r.set($enemy, foe::IMAGE, V::R(image))?;
            write!($enemy, foe::ANIMATION, 0);
        }};
    }
    let mut changed = false;
    let cooldown = read!(player, hero::PROJECTILE_COOLDOWN);
    if cooldown > 0 {
        write!(
            player,
            hero::PROJECTILE_COOLDOWN,
            cooldown.wrapping_sub(1).max(0)
        );
    }
    let projectiles = reference!(world, room::PROJECTILES);
    let enemies = reference!(world, room::ENEMIES);
    let items = reference!(world, room::ITEMS);
    let objects = reference!(world, room::OBJECTS);
    let mut next_index = 0;
    'projectiles: while next_index < host::vector_size(r, projectiles)? {
        let index = next_index;
        next_index += 1;
        let projectile = host::vector_element_at(r, projectiles, index)?;
        // The beta increments its index after removals, deliberately retaining
        // its original update order even when the next entry shifts left.
        for (outside, permits_stone) in [
            (read!(projectile, body::X) > 256, false),
            (read!(projectile, body::X) < 0, true),
            (read!(projectile, body::Y) > 240, false),
            (read!(projectile, body::Y) < 32, false),
        ] {
            if !outside {
                continue;
            }
            let kind = read!(projectile, shot::KIND);
            if matches!(kind, 2 | 8) || permits_stone && kind == 0 {
                if read!(projectile, shot::ENEMY_OWNED) != 0 {
                    let owner_index = read!(projectile, shot::OWNER_INDEX);
                    let owner = host::vector_element_at(r, enemies, owner_index)?;
                    write!(owner, foe::HAS_PROJECTILE, 0);
                    host::vector_remove_element_at(r, projectiles, index)?;
                    continue 'projectiles;
                }
                write!(projectile, shot::RETURNING, 1);
            } else {
                host::vector_remove_element_at(r, projectiles, index)?;
                continue 'projectiles;
            }
        }
        let tile_width = read!(world, room::WIDTH);
        let tile_index = (read!(projectile, body::Y).wrapping_sub(32) >> 4)
            .wrapping_mul(tile_width)
            .wrapping_add(read!(projectile, body::X) >> 4);
        if tile_index <= tile_width.wrapping_mul(read!(world, room::HEIGHT)) && tile_index > 0 {
            let tiles = reference!(world, room::TILES);
            if r.array_get(tiles, tile_index)?.i() & 0x40 == 0x40 {
                let kind = read!(projectile, shot::KIND);
                if matches!(kind, 2 | 8) {
                    if read!(projectile, shot::RETURNING) == 0 {
                        reverse!(projectile);
                        write!(projectile, shot::RETURNING, 1);
                    }
                } else if !matches!(kind, 3 | 5 | 0 | 11) {
                    host::vector_remove_element_at(r, projectiles, index)?;
                }
            }
        }
        if read!(projectile, shot::RETURNING) != 0 {
            let x = read!(projectile, body::X).wrapping_add(5);
            let y = read!(projectile, body::Y).wrapping_add(5);
            if read!(projectile, shot::ENEMY_OWNED) != 0 {
                let owner_index = read!(projectile, shot::OWNER_INDEX);
                let owner = host::vector_element_at(r, enemies, owner_index);
                match owner {
                    Ok(owner) => {
                        let owner_x = read!(owner, body::X);
                        let owner_y = read!(owner, body::Y);
                        if owner_x <= x
                            && owner_x.wrapping_add(16) >= x
                            && owner_y <= y
                            && owner_y.wrapping_add(16) >= y
                        {
                            host::vector_remove_element_at(r, projectiles, index)?;
                            write!(owner, foe::HAS_PROJECTILE, 0);
                        }
                        write!(
                            projectile,
                            shot::VELOCITY_X,
                            match owner_x.wrapping_add(8).cmp(&x) {
                                std::cmp::Ordering::Greater => 1,
                                std::cmp::Ordering::Less => -1,
                                _ => 0,
                            }
                        );
                        write!(
                            projectile,
                            shot::VELOCITY_Y,
                            match owner_y.wrapping_add(8).cmp(&y) {
                                std::cmp::Ordering::Greater => 1,
                                std::cmp::Ordering::Less => -1,
                                _ => 0,
                            }
                        );
                    }
                    Err(error)
                        if r.is_instance(error, "java/lang/ArrayIndexOutOfBoundsException") =>
                    {
                        host::vector_remove_element_at(r, projectiles, index)?;
                    }
                    Err(error) => return Err(error),
                }
            } else {
                let player_x = read!(player, body::X);
                let player_y = read!(player, body::Y);
                if player_x.wrapping_sub(1) <= x
                    && player_x.wrapping_add(16) >= x
                    && player_y.wrapping_sub(1) <= y
                    && player_y.wrapping_add(24) >= y
                {
                    write!(player, hero::BOOMERANG_ACTIVE, 0);
                    host::vector_remove_element_at(r, projectiles, index)?;
                }
                write!(
                    projectile,
                    shot::VELOCITY_X,
                    match player_x.wrapping_add(8).cmp(&x) {
                        std::cmp::Ordering::Greater => 1,
                        std::cmp::Ordering::Less => -1,
                        _ => 0,
                    }
                );
                write!(
                    projectile,
                    shot::VELOCITY_Y,
                    match player_y.wrapping_add(8).cmp(&y) {
                        std::cmp::Ordering::Greater => 1,
                        std::cmp::Ordering::Less => -1,
                        _ => 0,
                    }
                );
            }
        }
        super::projectile::draw(r, projectile, graphics, camera_x, camera_y, width, height)?;
        let x = read!(projectile, body::X);
        let y = read!(projectile, body::Y);
        let kind = read!(projectile, shot::KIND);
        if read!(projectile, shot::ENEMY_OWNED) != 0 {
            let player_x = read!(player, body::X);
            let player_y = read!(player, body::Y);
            if player_x.wrapping_add(20) < x
                || player_x.wrapping_add(4) > x.wrapping_add(read!(projectile, shot::WIDTH))
                || player_y.wrapping_add(read!(player, hero::COLLISION_HEIGHT)) < y
                || player_y > y.wrapping_add(read!(projectile, shot::HEIGHT))
            {
                continue;
            }
            if read!(player, hero::SHIELD_RAISED) != 0
                && matches!(
                    (read!(projectile, shot::FACING), read!(player, hero::FACING)),
                    (0, 2) | (1, 3) | (2, 0) | (3, 1)
                )
            {
                if kind != 5 {
                    host::vector_remove_element_at(r, projectiles, index)?;
                } else {
                    write!(projectile, shot::ENEMY_OWNED, 0);
                    reverse!(projectile);
                    write!(
                        projectile,
                        shot::DAMAGE,
                        read!(projectile, shot::DAMAGE).wrapping_shl(1)
                    );
                }
                continue;
            }
            if read!(player, hero::ATTACKING) != 0 && kind == 7 {
                write!(projectile, shot::ENEMY_OWNED, 0);
                write!(
                    projectile,
                    shot::DAMAGE,
                    read!(projectile, shot::DAMAGE).wrapping_add(1)
                );
                reverse!(projectile);
                continue;
            }
            if read!(player, hero::HURT_TIMER) > 0 {
                continue;
            }
            write!(player, hero::HURT_TIMER, 20);
            let base_damage = read!(projectile, shot::DAMAGE);
            let defense = read!(player, hero::DEFENSE);
            let damage = arithmetic::divide_int(r, base_damage, defense)?.wrapping_add(1);
            write!(
                player,
                hero::HEALTH,
                read!(player, hero::HEALTH).wrapping_sub(damage).max(0)
            );
            if kind == 7 {
                for enemy_index in 0..host::vector_size(r, enemies)? {
                    let enemy = host::vector_element_at(r, enemies, enemy_index)?;
                    if read!(enemy, foe::KIND) == 23 && read!(enemy, foe::BEHAVIOR_STATE) == 5 {
                        write!(enemy, foe::STATE_TIMER, 60);
                    }
                }
            }
            if kind != 2 {
                host::vector_remove_element_at(r, projectiles, index)?;
            } else if read!(projectile, shot::RETURNING) == 0 {
                reverse!(projectile);
                write!(projectile, shot::RETURNING, 1);
            }
            changed = true;
            continue;
        }
        // Player projectiles apply each boss's original vulnerability rules.
        for enemy_index in 0..host::vector_size(r, enemies)? {
            let enemy = host::vector_element_at(r, enemies, enemy_index)?;
            let enemy_kind = read!(enemy, foe::KIND);
            let enemy_x = read!(enemy, body::X);
            let enemy_y = read!(enemy, body::Y);
            if read!(enemy, foe::ACTIVE) == 0 && !matches!(enemy_kind, 22 | 23 | 24 | 38)
                || read!(enemy, foe::DYING) != 0
                || enemy_x.wrapping_add(read!(enemy, foe::WIDTH)) < x
                || enemy_x > x.wrapping_add(read!(projectile, shot::WIDTH))
                || enemy_y.wrapping_add(read!(enemy, foe::HEIGHT)) < y
                || enemy_y > y.wrapping_add(read!(projectile, shot::HEIGHT))
            {
                continue;
            }
            if read!(enemy, foe::HURT_TIMER) <= 0 {
                if matches!(enemy_kind, 30 | 31) && kind == 8 {
                    write!(enemy, foe::BEHAVIOR_STATE, 4);
                    write!(enemy, foe::STATE_TIMER, 0);
                    if read!(projectile, shot::RETURNING) == 0 && read!(enemy, foe::DYING) == 0 {
                        reverse!(projectile);
                        write!(projectile, shot::RETURNING, 1);
                    }
                    continue;
                }
                if enemy_kind == 24 {
                    if !matches!(kind, 11 | 7)
                        || matches!(read!(enemy, foe::BEHAVIOR_STATE), 20 | 3 | 6)
                    {
                        continue;
                    }
                    write!(
                        enemy,
                        foe::HIT_COUNT,
                        read!(enemy, foe::HIT_COUNT).wrapping_add(1)
                    );
                    host::vector_remove_element_at(r, projectiles, index)?;
                    write!(enemy, foe::HURT_TIMER, 10);
                    if read!(enemy, foe::HIT_COUNT) == 3 {
                        write!(enemy, foe::HIT_COUNT, 0);
                        write!(enemy, foe::BEHAVIOR_STATE, 20);
                        write!(enemy, foe::HURT_TIMER, 0);
                    }
                    continue;
                }
                if enemy_kind == 38 {
                    if kind != 3
                        || x <= enemy_x.wrapping_add(16)
                        || x >= enemy_x.wrapping_add(32)
                        || read!(enemy, foe::BEHAVIOR_STATE) <= 0
                    {
                        continue;
                    }
                    write!(
                        enemy,
                        foe::HEALTH,
                        read!(enemy, foe::HEALTH).wrapping_sub(1)
                    );
                    if read!(enemy, foe::HEALTH) <= 0 {
                        die!(enemy);
                    } else {
                        write!(enemy, foe::BEHAVIOR_STATE, 0);
                        write!(enemy, foe::HURT_TIMER, 10);
                    }
                    continue;
                }
                if enemy_kind == 23 {
                    if kind != 7 || read!(enemy, foe::BEHAVIOR_STATE) != 5 {
                        continue;
                    }
                    let random = r.alloc("java/util/Random", Native::Empty);
                    host::random_initialize(r, random)?;
                    if host::random_next_int(r, random)?
                        .wrapping_rem(2)
                        .wrapping_abs()
                        != 1
                    {
                        write!(enemy, foe::BEHAVIOR_STATE, 6);
                        write!(projectile, shot::ENEMY_OWNED, 1);
                        write!(
                            projectile,
                            shot::DAMAGE,
                            read!(projectile, shot::DAMAGE).wrapping_add(1)
                        );
                        reverse!(projectile);
                    } else {
                        write!(enemy, foe::BEHAVIOR_STATE, 7);
                        host::vector_remove_element_at(r, projectiles, index)?;
                    }
                    continue;
                }
                if enemy_kind == 10 {
                    if kind == 3 {
                        write!(
                            enemy,
                            foe::HEALTH,
                            read!(enemy, foe::HEALTH).wrapping_sub(1)
                        );
                    }
                } else {
                    if matches!(enemy_kind, 17 | 38) {
                        continue;
                    }
                    write!(
                        enemy,
                        foe::HEALTH,
                        read!(enemy, foe::HEALTH).wrapping_sub(read!(projectile, shot::DAMAGE))
                    );
                }
                if read!(enemy, foe::HEALTH) <= 0 {
                    die!(enemy);
                } else {
                    write!(enemy, foe::HURT_TIMER, 10);
                }
            }
            if !matches!(kind, 2 | 8) {
                if !matches!(kind, 3 | 5) {
                    host::vector_remove_element_at(r, projectiles, index)?;
                }
            } else if read!(projectile, shot::RETURNING) == 0 && read!(enemy, foe::DYING) == 0 {
                reverse!(projectile);
                write!(projectile, shot::RETURNING, 1);
            }
        }
        let mut item_index = 0;
        while item_index < host::vector_size(r, items)? {
            let index = item_index;
            item_index += 1;
            let item = host::vector_element_at(r, items, index)?;
            if read!(item, body::X) > x.wrapping_add(10)
                || read!(item, body::X).wrapping_add(5) < x
                || read!(item, body::Y) > y.wrapping_add(10)
                || read!(item, body::Y).wrapping_add(5) < y
                || read!(item, pickup::KIND) >= 100
            {
                continue;
            }
            let canvas = reference!(world, room::CANVAS);
            if super::item::collect(r, item, player, sound, canvas, 0)? == 1 {
                host::vector_remove_element_at(r, items, index)?;
                changed = true;
                write!(canvas, field::canvas::REDRAW_WORLD, 1);
            }
        }
        if !matches!(kind, 3 | 8) {
            continue;
        }
        for object_index in 0..host::vector_size(r, objects)? {
            let object = host::vector_element_at(r, objects, object_index)?;
            if read!(object, body::X) > x.wrapping_add(10)
                || read!(object, body::X).wrapping_add(5) < x
                || read!(object, body::Y) > y.wrapping_add(10)
                || read!(object, body::Y).wrapping_add(5) < y
            {
                continue;
            }
            let object_kind = read!(object, scenery::KIND);
            if object_kind == 60 {
                write!(object, scenery::TRIGGER_ACTIVE, 1);
            }
            if object_kind != 30 || kind != 8 || read!(world, room::SWITCH_COOLDOWN) != 0 {
                continue;
            }
            let alternate = read!(world, room::ALTERNATE_MAP).wrapping_add(1);
            let alternate = if alternate > 1 { 0 } else { alternate };
            write!(world, room::ALTERNATE_MAP, alternate);
            let mask = 1_i32.wrapping_shl(read!(world, room::ZONE) as u32);
            if alternate == 0 {
                write!(
                    player,
                    hero::WORLD_FLAGS,
                    read!(player, hero::WORLD_FLAGS) & !mask
                );
            }
            if alternate == 1 {
                write!(
                    player,
                    hero::WORLD_FLAGS,
                    read!(player, hero::WORLD_FLAGS) | mask
                );
            }
            write!(object, scenery::STATE_C, alternate);
            write!(world, room::SWITCH_COOLDOWN, 15);
        }
    }
    Ok(i32::from(changed))
}

/// Update hookshot. Original reference: `j.c(Lh;)V`.
pub(in crate::game) fn update_hookshot(
    r: &mut Runtime,
    world: u32,
    player: u32,
) -> Result<(), u32> {
    let mut number_2: i32 = 0;
    let mut object_2: u32 = 0;
    let mut object_3: u32 = 0;
    number_2 = 0;
    let player_action_timer = r.get(player, field::player::ACTION_TIMER)?.i();
    if player_action_timer != 0 {
        return Ok(());
    }
    let player_hookshot_extending = r.get(player, field::player::HOOKSHOT_EXTENDING)?.i();
    if player_hookshot_extending == 0 {
        return Ok(());
    }
    let player_hookshot_returning = r.get(player, field::player::HOOKSHOT_RETURNING)?.i();
    if player_hookshot_returning != 0 {
        let player_hookshot_trail = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value = host::vector_size(r, player_hookshot_trail)?;
        if vector_size_value < 1 {
            r.set(player, field::player::HOOKSHOT_EXTENDING, V::I(0))?;
            let player_hookshot_trail_3_read = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
            host::vector_remove_all_elements(r, player_hookshot_trail_3_read)?;
            r.set(player, field::player::HOOKSHOT_RETURNING, V::I(0))?;
            r.set(player, field::player::ACTION_TIMER, V::I(0))?;
            r.set(player, field::player::STATE_AH, V::I(0))?;
            return Ok(());
        }
        let player_hookshot_trail_4 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let player_hookshot_trail_5 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_3 = host::vector_size(r, player_hookshot_trail_5)?;
        let vector_element_at_value = host::vector_element_at(
            r,
            player_hookshot_trail_4,
            vector_size_value_3.wrapping_sub(1),
        )?;
        host::check_cast(r, vector_element_at_value, "g")?;
        object_3 = vector_element_at_value;
        let trail_point_x = r.get(object_3, field::trail_point::X)?.i();
        let trail_point_y = r.get(object_3, field::trail_point::Y)?.i();
        let check_hookshot_contact_value =
            super::world::check_hookshot_contact(r, world, trail_point_x, trail_point_y, player)?;
        let player_state_ah = r.get(player, field::player::STATE_AH)?.i();
        if player_state_ah != 0 {
            let player_hookshot_trail_8 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
            let vector_element_at_value_3 = host::vector_element_at(r, player_hookshot_trail_8, 0)?;
            host::check_cast(r, vector_element_at_value_3, "g")?;
            object_2 = vector_element_at_value_3;
            let trail_point_x_2 = r.get(object_2, field::trail_point::X)?.i();
            r.set(player, field::drawable::X, V::I(trail_point_x_2))?;
            let trail_point_y_2 = r.get(object_2, field::trail_point::Y)?.i();
            r.set(player, field::drawable::Y, V::I(trail_point_y_2))?;
            let player_facing_2 = r.get(player, field::player::FACING)?.i();
            if player_facing_2 == 0 {
                let drawable_x_4 = r.get(player, field::drawable::X)?.i();
                r.set(
                    player,
                    field::drawable::X,
                    V::I(drawable_x_4.wrapping_sub(8)),
                )?;
                let drawable_y_4 = r.get(player, field::drawable::Y)?.i();
                r.set(
                    player,
                    field::drawable::Y,
                    V::I(drawable_y_4.wrapping_sub(8)),
                )?;
            }
            let player_facing_5 = r.get(player, field::player::FACING)?.i();
            if player_facing_5 == 1 {
                let drawable_x_7 = r.get(player, field::drawable::X)?.i();
                r.set(
                    player,
                    field::drawable::X,
                    V::I(drawable_x_7.wrapping_sub(4)),
                )?;
                let drawable_y_7 = r.get(player, field::drawable::Y)?.i();
                r.set(
                    player,
                    field::drawable::Y,
                    V::I(drawable_y_7.wrapping_sub(12)),
                )?;
            }
            let player_facing_8 = r.get(player, field::player::FACING)?.i();
            if player_facing_8 == 2 {
                let drawable_x_10 = r.get(player, field::drawable::X)?.i();
                r.set(
                    player,
                    field::drawable::X,
                    V::I(drawable_x_10.wrapping_sub(8)),
                )?;
                let drawable_y_10 = r.get(player, field::drawable::Y)?.i();
                r.set(
                    player,
                    field::drawable::Y,
                    V::I(drawable_y_10.wrapping_sub(12)),
                )?;
            }
            let player_facing_11 = r.get(player, field::player::FACING)?.i();
            if player_facing_11 == 3 {
                let drawable_x_13 = r.get(player, field::drawable::X)?.i();
                r.set(
                    player,
                    field::drawable::X,
                    V::I(drawable_x_13.wrapping_sub(12)),
                )?;
                let drawable_y_13 = r.get(player, field::drawable::Y)?.i();
                r.set(
                    player,
                    field::drawable::Y,
                    V::I(drawable_y_13.wrapping_sub(8)),
                )?;
            }
            let player_hookshot_trail_12 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
            host::vector_remove_element_at(r, player_hookshot_trail_12, 0)?;
        } else {
            let player_hookshot_trail_9 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
            let player_hookshot_trail_10 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
            let vector_size_value_5 = host::vector_size(r, player_hookshot_trail_10)?;
            host::vector_remove_element_at(
                r,
                player_hookshot_trail_9,
                vector_size_value_5.wrapping_sub(1),
            )?;
        }
        let player_hookshot_trail_11 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_6 = host::vector_size(r, player_hookshot_trail_11)?;
        if vector_size_value_6 != 0 {
            return Ok(());
        }
        r.set(player, field::player::HOOKSHOT_EXTENDING, V::I(0))?;
        let player_hookshot_trail_3_read = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        host::vector_remove_all_elements(r, player_hookshot_trail_3_read)?;
        r.set(player, field::player::HOOKSHOT_RETURNING, V::I(0))?;
        r.set(player, field::player::ACTION_TIMER, V::I(0))?;
        r.set(player, field::player::STATE_AH, V::I(0))?;
        return Ok(());
    }
    let trail_point_value = r.alloc("g", Native::Empty);
    super::trail_point::initialize(r, trail_point_value)?;
    object_3 = trail_point_value;
    let player_hookshot_trail_2 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
    let vector_size_value_2 = host::vector_size(r, player_hookshot_trail_2)?;
    if vector_size_value_2 != 0 {
        let player_hookshot_trail_6 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let player_hookshot_trail_7 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_4 = host::vector_size(r, player_hookshot_trail_7)?;
        let vector_element_at_value_2 = host::vector_element_at(
            r,
            player_hookshot_trail_6,
            vector_size_value_4.wrapping_sub(1),
        )?;
        host::check_cast(r, vector_element_at_value_2, "g")?;
        object_2 = vector_element_at_value_2;
    } else {
        let trail_point_value_2 = r.alloc("g", Native::Empty);
        super::trail_point::initialize(r, trail_point_value_2)?;
        object_2 = trail_point_value_2;
        let player_facing = r.get(player, field::player::FACING)?.i();
        if player_facing == 0 {
            let drawable_x_3 = r.get(player, field::drawable::X)?.i();
            r.set(
                object_2,
                field::trail_point::X,
                V::I(drawable_x_3.wrapping_add(8)),
            )?;
            let drawable_y_3 = r.get(player, field::drawable::Y)?.i();
            r.set(
                object_2,
                field::trail_point::Y,
                V::I(drawable_y_3.wrapping_add(8)),
            )?;
        }
        let player_facing_4 = r.get(player, field::player::FACING)?.i();
        if player_facing_4 == 1 {
            let drawable_x_6 = r.get(player, field::drawable::X)?.i();
            r.set(
                object_2,
                field::trail_point::X,
                V::I(drawable_x_6.wrapping_add(12)),
            )?;
            let drawable_y_6 = r.get(player, field::drawable::Y)?.i();
            r.set(
                object_2,
                field::trail_point::Y,
                V::I(drawable_y_6.wrapping_add(12)),
            )?;
        }
        let player_facing_7 = r.get(player, field::player::FACING)?.i();
        if player_facing_7 == 2 {
            let drawable_x_9 = r.get(player, field::drawable::X)?.i();
            r.set(
                object_2,
                field::trail_point::X,
                V::I(drawable_x_9.wrapping_add(8)),
            )?;
            let drawable_y_9 = r.get(player, field::drawable::Y)?.i();
            r.set(
                object_2,
                field::trail_point::Y,
                V::I(drawable_y_9.wrapping_add(12)),
            )?;
        }
        let player_facing_10 = r.get(player, field::player::FACING)?.i();
        if player_facing_10 == 3 {
            let drawable_x_12 = r.get(player, field::drawable::X)?.i();
            r.set(
                object_2,
                field::trail_point::X,
                V::I(drawable_x_12.wrapping_add(12)),
            )?;
            let drawable_y_12 = r.get(player, field::drawable::Y)?.i();
            r.set(
                object_2,
                field::trail_point::Y,
                V::I(drawable_y_12.wrapping_add(8)),
            )?;
        }
    }
    let player_facing_3 = r.get(player, field::player::FACING)?.i();
    if player_facing_3 == 0 {
        let trail_point_y_5 = r.get(object_2, field::trail_point::Y)?.i();
        r.set(
            object_3,
            field::trail_point::Y,
            V::I(trail_point_y_5.wrapping_sub(8)),
        )?;
        let trail_point_x_5 = r.get(object_2, field::trail_point::X)?.i();
        r.set(object_3, field::trail_point::X, V::I(trail_point_x_5))?;
    }
    let player_facing_6 = r.get(player, field::player::FACING)?.i();
    if player_facing_6 == 1 {
        let trail_point_y_8 = r.get(object_2, field::trail_point::Y)?.i();
        r.set(object_3, field::trail_point::Y, V::I(trail_point_y_8))?;
        let trail_point_x_8 = r.get(object_2, field::trail_point::X)?.i();
        r.set(
            object_3,
            field::trail_point::X,
            V::I(trail_point_x_8.wrapping_add(8)),
        )?;
    }
    let player_facing_9 = r.get(player, field::player::FACING)?.i();
    if player_facing_9 == 2 {
        let trail_point_y_11 = r.get(object_2, field::trail_point::Y)?.i();
        r.set(
            object_3,
            field::trail_point::Y,
            V::I(trail_point_y_11.wrapping_add(8)),
        )?;
        let trail_point_x_11 = r.get(object_2, field::trail_point::X)?.i();
        r.set(object_3, field::trail_point::X, V::I(trail_point_x_11))?;
    }
    let player_facing_12 = r.get(player, field::player::FACING)?.i();
    if player_facing_12 == 3 {
        let trail_point_y_15 = r.get(object_2, field::trail_point::Y)?.i();
        r.set(object_3, field::trail_point::Y, V::I(trail_point_y_15))?;
        let trail_point_x_15 = r.get(object_2, field::trail_point::X)?.i();
        r.set(
            object_3,
            field::trail_point::X,
            V::I(trail_point_x_15.wrapping_sub(8)),
        )?;
    }
    let trail_point_x_14 = r.get(object_3, field::trail_point::X)?.i();
    let trail_point_y_14 = r.get(object_3, field::trail_point::Y)?.i();
    let check_hookshot_contact_value_2 =
        super::world::check_hookshot_contact(r, world, trail_point_x_14, trail_point_y_14, player)?;
    number_2 = check_hookshot_contact_value_2;
    if check_hookshot_contact_value_2 != 0 {
        if number_2 == 2 {
            r.set(player, field::player::STATE_AH, V::I(1))?;
        }
        r.set(player, field::player::HOOKSHOT_RETURNING, V::I(1))?;
        return Ok(());
    }
    let player_hookshot_trail_13 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
    host::vector_add_element(r, player_hookshot_trail_13, object_3)?;
    let player_hookshot_trail_14 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
    let vector_size_value_7 = host::vector_size(r, player_hookshot_trail_14)?;
    if vector_size_value_7 <= 12 {
        return Ok(());
    }
    r.set(player, field::player::HOOKSHOT_RETURNING, V::I(1))?;
    return Ok(());
}

/// Check hookshot contact. Original reference: `j.a(IILh;)I`.
pub(in crate::game) fn check_hookshot_contact(
    r: &mut Runtime,
    world: u32,
    mut x: i32,
    mut y: i32,
    player: u32,
) -> Result<i32, u32> {
    let mut number_4: i32 = 0;
    let mut number_5: i32 = 0;
    let mut number_6: i32 = 0;
    let mut number_7: i32 = 0;
    let mut number_8: i32 = 0;
    let mut object_8: u32 = 0;
    let mut number_9: i32 = 0;
    let mut object_10: u32 = 0;
    number_4 = 0;

    number_5 = 0;
    number_6 = 0;
    number_7 = 0;
    number_8 = 0;
    number_9 = 0;
    let player_facing = r.get(player, field::player::FACING)?.i();
    if player_facing == 0 {
        y = y.wrapping_add(-4);
        number_6 = 12;
        number_7 = 8;
        number_9 = 8;
    }
    let player_facing_2 = r.get(player, field::player::FACING)?.i();
    if player_facing_2 == 1 {
        x = x.wrapping_add(4);
        number_6 = 8;
        number_7 = 12;
        number_8 = -8;
    }
    let player_facing_3 = r.get(player, field::player::FACING)?.i();
    if player_facing_3 == 2 {
        y = y.wrapping_add(4);
        number_6 = 12;
        number_7 = 8;
        number_9 = -8;
    }
    let player_facing_4 = r.get(player, field::player::FACING)?.i();
    if player_facing_4 == 3 {
        x = x.wrapping_add(-4);
        number_6 = 8;
        number_7 = 12;
        number_8 = 8;
    }
    let world_width = r.get(world, field::world::WIDTH)?.i();
    number_5 = (((y.wrapping_sub(32)).wrapping_shr((4) as u32)).wrapping_mul(world_width))
        .wrapping_add(x.wrapping_shr((4) as u32));
    let world_width_2 = r.get(world, field::world::WIDTH)?.i();
    let world_height = r.get(world, field::world::HEIGHT)?.i();
    if (!((((y.wrapping_sub(32)).wrapping_shr((4) as u32)).wrapping_mul(world_width))
        .wrapping_add(x.wrapping_shr((4) as u32))
        > world_width_2.wrapping_mul(world_height)))
        && (number_5 > 0)
    {
        let world_tiles = r.get(world, field::world::TILES)?.r();
        let element_value = r.array_get(world_tiles, number_5)?.i();
        number_5 = element_value;
        if !((element_value & 64) != 64) {
            return Ok(1);
        }
    }
    number_5 = 0;
    'check_hookshot_contact_loop_2: loop {
        let world_items = r.get(world, field::world::ITEMS)?.r();
        let vector_size_value = host::vector_size(r, world_items)?;
        if number_5 >= vector_size_value {
            break 'check_hookshot_contact_loop_2;
        }
        let world_items_2 = r.get(world, field::world::ITEMS)?.r();
        let vector_element_at_value = host::vector_element_at(r, world_items_2, number_5)?;
        host::check_cast(r, vector_element_at_value, "c")?;
        object_10 = vector_element_at_value;
        let drawable_x = r.get(vector_element_at_value, field::drawable::X)?.i();
        if !(drawable_x > x.wrapping_add(number_6)) {
            let drawable_x_2 = r.get(object_10, field::drawable::X)?.i();
            let drawable_half_width = r.get(object_10, field::drawable::HALF_WIDTH)?.i();
            if !(drawable_x_2.wrapping_add(drawable_half_width.wrapping_shl((1) as u32)) < x) {
                let drawable_y = r.get(object_10, field::drawable::Y)?.i();
                if !(drawable_y > y.wrapping_add(number_7)) {
                    let drawable_y_2 = r.get(object_10, field::drawable::Y)?.i();
                    let drawable_half_height = r.get(object_10, field::drawable::HALF_HEIGHT)?.i();
                    if !(drawable_y_2.wrapping_add(drawable_half_height.wrapping_shl((1) as u32))
                        < y)
                    {
                        let item_kind = r.get(object_10, field::item::KIND)?.i();
                        if item_kind < 128 {
                            let drawable_x_7 = r.get(object_10, field::drawable::X)?.i();
                            r.set(
                                object_10,
                                field::drawable::X,
                                V::I(drawable_x_7.wrapping_add(number_8)),
                            )?;
                            let drawable_y_3 = r.get(object_10, field::drawable::Y)?.i();
                            r.set(
                                object_10,
                                field::drawable::Y,
                                V::I(drawable_y_3.wrapping_add(number_9)),
                            )?;
                            number_4 = 1;
                        }
                    }
                }
            }
        }
        number_5 = number_5.wrapping_add(1);
        continue 'check_hookshot_contact_loop_2;
    }
    number_5 = 0;
    'check_hookshot_contact_loop_5: loop {
        let world_enemies = r.get(world, field::world::ENEMIES)?.r();
        let vector_size_value_2 = host::vector_size(r, world_enemies)?;
        if number_5 >= vector_size_value_2 {
            number_5 = 0;
            loop {
                let world_objects = r.get(world, field::world::OBJECTS)?.r();
                let vector_size_value_3 = host::vector_size(r, world_objects)?;
                if number_5 >= vector_size_value_3 {
                    return Ok(number_4);
                }
                let world_objects_2 = r.get(world, field::world::OBJECTS)?.r();
                let vector_element_at_value_3 =
                    host::vector_element_at(r, world_objects_2, number_5)?;
                host::check_cast(r, vector_element_at_value_3, "n")?;
                object_10 = vector_element_at_value_3;
                let drawable_x_3 = r.get(vector_element_at_value_3, field::drawable::X)?.i();
                if !(drawable_x_3 > x.wrapping_add(number_6)) {
                    let drawable_x_5 = r.get(object_10, field::drawable::X)?.i();
                    if !(drawable_x_5.wrapping_add(16) < x) {
                        let drawable_y_4 = r.get(object_10, field::drawable::Y)?.i();
                        if !(drawable_y_4 > y.wrapping_add(number_7)) {
                            let drawable_y_6 = r.get(object_10, field::drawable::Y)?.i();
                            if !(drawable_y_6.wrapping_add(16) < y) {
                                let map_object_kind_2 =
                                    r.get(object_10, field::map_object::KIND)?.i();
                                if map_object_kind_2 != 20 {
                                    let map_object_state_e =
                                        r.get(object_10, field::map_object::STATE_E)?.i();
                                    if map_object_state_e != 0 {
                                        let map_object_solid =
                                            r.get(object_10, field::map_object::SOLID)?.i();
                                        if map_object_solid != 0 {
                                            return Ok(1);
                                        }
                                    }
                                } else {
                                    number_4 = 2;
                                    let player_hookshot_trail =
                                        r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
                                    let vector_size_value_4 =
                                        host::vector_size(r, player_hookshot_trail)?;
                                    if vector_size_value_4 < 1 {
                                        return Ok(2);
                                    }
                                    let player_hookshot_trail_2 =
                                        r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
                                    let player_hookshot_trail_3 =
                                        r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
                                    let vector_size_value_5 =
                                        host::vector_size(r, player_hookshot_trail_3)?;
                                    let vector_element_at_value_4 = host::vector_element_at(
                                        r,
                                        player_hookshot_trail_2,
                                        vector_size_value_5.wrapping_sub(1),
                                    )?;
                                    host::check_cast(r, vector_element_at_value_4, "g")?;
                                    object_8 = vector_element_at_value_4;
                                    let player_facing_5 = r.get(player, field::player::FACING)?.i();
                                    if player_facing_5 == 0 {
                                        let drawable_y_8 =
                                            r.get(object_10, field::drawable::Y)?.i();
                                        r.set(
                                            object_8,
                                            field::trail_point::Y,
                                            V::I(drawable_y_8.wrapping_add(16)),
                                        )?;
                                    }
                                    let player_facing_6 = r.get(player, field::player::FACING)?.i();
                                    if player_facing_6 == 1 {
                                        let drawable_x_8 =
                                            r.get(object_10, field::drawable::X)?.i();
                                        r.set(
                                            object_8,
                                            field::trail_point::X,
                                            V::I(drawable_x_8.wrapping_sub(16)),
                                        )?;
                                    }
                                    let player_facing_7 = r.get(player, field::player::FACING)?.i();
                                    if player_facing_7 == 2 {
                                        let drawable_y_9 =
                                            r.get(object_10, field::drawable::Y)?.i();
                                        r.set(
                                            object_8,
                                            field::trail_point::Y,
                                            V::I(drawable_y_9.wrapping_sub(16)),
                                        )?;
                                    }
                                    let player_facing_8 = r.get(player, field::player::FACING)?.i();
                                    if player_facing_8 == 3 {
                                        let drawable_x_9 =
                                            r.get(object_10, field::drawable::X)?.i();
                                        r.set(
                                            object_8,
                                            field::trail_point::X,
                                            V::I(drawable_x_9.wrapping_add(20)),
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }
                number_5 = number_5.wrapping_add(1);
            }
        }
        let world_enemies_2 = r.get(world, field::world::ENEMIES)?.r();
        let vector_element_at_value_2 = host::vector_element_at(r, world_enemies_2, number_5)?;
        host::check_cast(r, vector_element_at_value_2, "l")?;
        object_10 = vector_element_at_value_2;
        let enemy_active = r.get(vector_element_at_value_2, field::enemy::ACTIVE)?.i();
        if enemy_active != 0 {
            let enemy_dying = r.get(object_10, field::enemy::DYING)?.i();
            if enemy_dying == 0 {
                let drawable_x_4 = r.get(object_10, field::drawable::X)?.i();
                if !(drawable_x_4 > x.wrapping_add(number_6)) {
                    let drawable_x_6 = r.get(object_10, field::drawable::X)?.i();
                    let enemy_width_3 = r.get(object_10, field::enemy::WIDTH)?.i();
                    if !(drawable_x_6.wrapping_add(enemy_width_3) < x) {
                        let drawable_y_5 = r.get(object_10, field::drawable::Y)?.i();
                        if !(drawable_y_5 > y.wrapping_add(number_7)) {
                            let drawable_y_7 = r.get(object_10, field::drawable::Y)?.i();
                            let enemy_height_2 = r.get(object_10, field::enemy::HEIGHT)?.i();
                            if !(drawable_y_7.wrapping_add(enemy_height_2) < y) {
                                let enemy_kind_3 = r.get(object_10, field::enemy::KIND)?.i();
                                if enemy_kind_3 == 16 {
                                    number_4 = 2;
                                } else {
                                    let enemy_kind_4 = r.get(object_10, field::enemy::KIND)?.i();
                                    if enemy_kind_4 == 17 {
                                        number_4 = 2;
                                    } else {
                                        let enemy_kind_5 =
                                            r.get(object_10, field::enemy::KIND)?.i();
                                        if enemy_kind_5 == 18 {
                                            number_4 = 2;
                                        } else {
                                            let enemy_kind_6 =
                                                r.get(object_10, field::enemy::KIND)?.i();
                                            if enemy_kind_6 == 19 {
                                                number_4 = 2;
                                            } else {
                                                let enemy_kind_7 =
                                                    r.get(object_10, field::enemy::KIND)?.i();
                                                if enemy_kind_7 == 20 {
                                                    number_4 = 2;
                                                } else {
                                                    let enemy_kind_8 =
                                                        r.get(object_10, field::enemy::KIND)?.i();
                                                    if enemy_kind_8 == 21 {
                                                        number_4 = 2;
                                                    } else {
                                                        let x_value_10 = r
                                                            .get(object_10, field::drawable::X)?
                                                            .i();
                                                        r.set(
                                                            object_10,
                                                            field::drawable::X,
                                                            V::I(x_value_10.wrapping_add(number_8)),
                                                        )?;
                                                        let y_value_10 = r
                                                            .get(object_10, field::drawable::Y)?
                                                            .i();
                                                        r.set(
                                                            object_10,
                                                            field::drawable::Y,
                                                            V::I(y_value_10.wrapping_add(number_9)),
                                                        )?;
                                                        number_4 = 1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        number_5 = number_5.wrapping_add(1);
        continue 'check_hookshot_contact_loop_5;
    }
}

/// Update hookshot target. Original reference: `j.a(II)Z`.
pub(in crate::game) fn update_hookshot_target(
    r: &mut Runtime,
    world: u32,
    x: i32,
    y: i32,
) -> Result<i32, u32> {
    let mut number_3: i32 = 0;
    let mut object_4: u32 = 0;
    number_3 = 0;
    loop {
        let world_projectiles = r.get(world, field::world::PROJECTILES)?.r();
        let vector_size_value = host::vector_size(r, world_projectiles)?;
        if number_3 >= vector_size_value {
            return Ok(0);
        }
        let world_projectiles_2 = r.get(world, field::world::PROJECTILES)?.r();
        let vector_element_at_value = host::vector_element_at(r, world_projectiles_2, number_3)?;
        host::check_cast(r, vector_element_at_value, "p")?;
        object_4 = vector_element_at_value;
        let projectile_kind = r.get(vector_element_at_value, field::projectile::KIND)?.i();
        if projectile_kind == 8 {
            let projectile_returning = r.get(object_4, field::projectile::RETURNING)?.i();
            if projectile_returning != 0 {
                let drawable_x = r.get(object_4, field::drawable::X)?.i();
                r.set(world, field::world::HOOKSHOT_X, V::I(drawable_x))?;
                let drawable_y = r.get(object_4, field::drawable::Y)?.i();
                r.set(world, field::world::HOOKSHOT_Y, V::I(drawable_y))?;
                return Ok(0);
            }
            r.set(object_4, field::projectile::VELOCITY_X, V::I(x))?;
            r.set(object_4, field::projectile::VELOCITY_Y, V::I(y))?;
            let drawable_x_2 = r.get(object_4, field::drawable::X)?.i();
            r.set(world, field::world::HOOKSHOT_X, V::I(drawable_x_2))?;
            let drawable_y_2 = r.get(object_4, field::drawable::Y)?.i();
            r.set(world, field::world::HOOKSHOT_Y, V::I(drawable_y_2))?;
            return Ok(1);
        }
        number_3 = number_3.wrapping_add(1);
    }
}
