//! World interactions.
use super::*;

/// Count active triggers. Original reference: `j.a()I`.
#[rustfmt::skip]
pub(in crate::game) fn count_active_triggers(r: &mut Runtime, world: u32) -> Result<i32, u32> {
    let mut number_1: i32 = 0;
    let mut number_2: i32 = 0;
    let mut object_3: u32 = 0;
    let mut number_4: i32 = 0;
    let mut number_5: i32 = 0;
    let mut number_6: i32 = 0;
    let mut object_7: u32 = 0;
    'check_world_objects: {
    number_1 = 0;
    number_2 = 0;
    } // check_world_objects
    'count_active_triggers_loop_2: loop {
        'advance_index: {
        'next_step_3: {
        'check_world_enemies_2: {
        'next_step_2: {
        'check_map_object_trigger_value_3: {
        'check_world_enemies: {
        'check_map_object_state_p_2: {
        'check_map_object_trigger_value_2: {
        'check_map_object_trigger_active_2: {
        'check_map_object_trigger_kind_2: {
        'check_map_object_trigger_value: {
        'check_map_object_trigger_active: {
        'check_map_object_trigger_kind: {
        'check_map_object_state_p: {
        let objects_value = r
            .get(
                world,
                field::world::OBJECTS,
            )?
            .r();
        let vector_size_value =
            host::vector_size(
                r,
                objects_value,
            )?;
        if number_2 >= vector_size_value {
            return Ok(number_1);
        }
        let objects_value_2 = r
            .get(
                world,
                field::world::OBJECTS,
            )?
            .r();
        let vector_element_at_value =
            host::vector_element_at(
                r,
                objects_value_2,
                number_2,
            )?;
        host::check_cast(
            r,
            vector_element_at_value,
            "n",
        )?;
        object_3 = vector_element_at_value;
        let map_object_trigger_kind = r.get(vector_element_at_value, field::map_object::TRIGGER_KIND)?.i();
        if map_object_trigger_kind != 0 {
            break 'check_map_object_trigger_kind;
        }
        } // check_map_object_state_p
        let state_p_value = r
            .get(
                object_3,
                field::map_object::STATE_P,
            )?
            .i();
        if state_p_value == 0 {
            break 'advance_index;
        }
        } // check_map_object_trigger_kind
        let trigger_kind_value_2 = r
            .get(
                object_3,
                field::map_object::TRIGGER_KIND,
            )?
            .i();
        if trigger_kind_value_2 != 3 {
            break 'check_map_object_trigger_kind_2;
        }
        } // check_map_object_trigger_active
        let trigger_active_value = r
            .get(
                object_3,
                field::map_object::TRIGGER_ACTIVE,
            )?
            .i();
        if trigger_active_value == 0 {
            break 'check_map_object_trigger_kind_2;
        }
        } // check_map_object_trigger_value
        let trigger_value_value = r
            .get(
                object_3,
                field::map_object::TRIGGER_VALUE,
            )?
            .i();
        number_1 =
            number_1.wrapping_add(trigger_value_value);
        break 'advance_index;
        } // check_map_object_trigger_kind_2
        let trigger_kind_value_3 = r
            .get(object_3, field::map_object::TRIGGER_KIND)?
            .i();
        if trigger_kind_value_3 != 2 {
            break 'check_map_object_state_p_2;
        }
        } // check_map_object_trigger_active_2
        let map_object_trigger_active_2 =
            r.get(object_3, field::map_object::TRIGGER_ACTIVE)?.i();
        if map_object_trigger_active_2 == 0 {
            break 'check_map_object_state_p_2;
        }
        } // check_map_object_trigger_value_2
        let map_object_trigger_value_2 =
            r.get(object_3, field::map_object::TRIGGER_VALUE)?.i();
        number_1 = number_1.wrapping_add(map_object_trigger_value_2);
        break 'advance_index;
        } // check_map_object_state_p_2
        let map_object_state_p_2 =
            r.get(object_3, field::map_object::STATE_P)?.i();
        if map_object_state_p_2 == 0 {
            break 'advance_index;
        }
        } // check_world_enemies
        let world_enemies = r.get(world, field::world::ENEMIES)?.r();
        let vector_size_value_2 = host::vector_size(r, world_enemies)?;
        if vector_size_value_2 != 0 {
            break 'next_step_2;
        }
        } // check_map_object_trigger_value_3
        let map_object_trigger_value_3 =
            r.get(object_3, field::map_object::TRIGGER_VALUE)?.i();
        number_1 = number_1.wrapping_add(map_object_trigger_value_3);
        break 'advance_index;
        } // next_step_2
        number_4 = 0;
        number_5 = 0;
        number_6 = 0;
        } // check_world_enemies_2
        'count_active_triggers_loop_14: loop {
            'advance_index_3: {
            'advance_index_2: {
            'check_enemy_kind: {
            'check_world_enemies_3: {
            let world_enemies_2 = r.get(world, field::world::ENEMIES)?.r();
            let vector_size_value_3 =
                host::vector_size(r, world_enemies_2)?;
            if number_6 >= vector_size_value_3 {
                break 'next_step_3;
            }
            } // check_world_enemies_3
            let world_enemies_3 = r.get(world, field::world::ENEMIES)?.r();
            let vector_element_at_value_2 =
                host::vector_element_at(r, world_enemies_3, number_6)?;
            host::check_cast(r, vector_element_at_value_2, "l")?;
            object_7 = vector_element_at_value_2;
            let enemy_kind =
                r.get(vector_element_at_value_2, field::enemy::KIND)?.i();
            if enemy_kind != 9 {
                number_4 = 1;
            }
            let enemy_dead = r.get(object_7, field::enemy::DEAD)?.i();
            if enemy_dead != 0 {
                break 'advance_index_2;
            }
            } // check_enemy_kind
            let enemy_kind_2 = r.get(object_7, field::enemy::KIND)?.i();
            if enemy_kind_2 != 36 {
                break 'advance_index_3;
            }
            } // advance_index_2
            number_5 = number_5.wrapping_add(1);
            } // advance_index_3
            number_6 = number_6.wrapping_add(1);
            continue 'count_active_triggers_loop_14;
        }
        } // next_step_3

        if number_4 == 0 {
            let map_object_trigger_value_4 = r.get(object_3, field::map_object::TRIGGER_VALUE)?.i();
            number_1 = number_1.wrapping_add(map_object_trigger_value_4);
        }
        let world_enemies_4 = r.get(world, field::world::ENEMIES)?.r();
        let vector_size_value_4 = host::vector_size(r, world_enemies_4)?;
        if vector_size_value_4 == number_5 {
            let map_object_trigger_value_5 = r.get(object_3, field::map_object::TRIGGER_VALUE)?.i();
            number_1 = number_1.wrapping_add(map_object_trigger_value_5);
        }
        } // advance_index
        number_2 = number_2.wrapping_add(1);
        continue 'count_active_triggers_loop_2;
    }
}

/// Update triggers. Original reference: `j.a(Lh;)V`.
pub(in crate::game) fn update_triggers(
    r: &mut Runtime,
    world: u32,
    player: u32,
) -> Result<(), u32> {
    let mut number_2: i32 = 0;
    let mut number_3: i32 = 0;
    let mut number_4: i32 = 0;
    let mut object_5: u32 = 0;
    let mut number_6: i32 = 0;
    let mut object_6: u32 = 0;
    let mut number_7: i32 = 0;
    let mut object_7: u32 = 0;
    let drawable_x = r.get(player, field::drawable::X)?.i();
    number_2 = drawable_x;
    let drawable_y = r.get(player, field::drawable::Y)?.i();
    number_3 = drawable_y.wrapping_sub(32);
    number_2 = number_2.wrapping_add(12);
    number_3 = number_3.wrapping_add(16);
    number_4 = 0;
    'update_triggers_loop_2: loop {
        let world_objects = r.get(world, field::world::OBJECTS)?.r();
        let vector_size_value = host::vector_size(r, world_objects)?;
        if number_4 >= vector_size_value {
            number_4 = 0;
            loop {
                let world_overlay_objects = r.get(world, field::world::OVERLAY_OBJECTS)?.r();
                let vector_size_value_2 = host::vector_size(r, world_overlay_objects)?;
                if number_4 >= vector_size_value_2 {
                    return Ok(());
                }
                let world_overlay_objects_2 = r.get(world, field::world::OVERLAY_OBJECTS)?.r();
                let vector_element_at_value_2 =
                    host::vector_element_at(r, world_overlay_objects_2, number_4)?;
                host::check_cast(r, vector_element_at_value_2, "n")?;
                object_5 = vector_element_at_value_2;
                let trigger_kind_value_2 = r
                    .get(vector_element_at_value_2, field::map_object::TRIGGER_KIND)?
                    .i();
                if trigger_kind_value_2 == 1 {
                    let count_active_triggers_value =
                        super::world::count_active_triggers(r, world)?;
                    let map_object_trigger_value =
                        r.get(object_5, field::map_object::TRIGGER_VALUE)?.i();
                    if count_active_triggers_value < map_object_trigger_value {
                        r.set(object_5, field::map_object::SOLID, V::I(1))?;
                        r.set(object_5, field::map_object::STATE_Y, V::I(1))?;
                    } else {
                        r.set(object_5, field::map_object::SOLID, V::I(0))?;
                        let map_object_persistence_id =
                            r.get(object_5, field::map_object::PERSISTENCE_ID)?.i();
                        if map_object_persistence_id > 0 {
                            let map_object_persistence_id_2 =
                                r.get(object_5, field::map_object::PERSISTENCE_ID)?.i();
                            let world_map = r.get(world, field::world::MAP)?.i();
                            super::world::mark_persistent_event(
                                r,
                                world,
                                map_object_persistence_id_2,
                                world_map,
                                player,
                            )?;
                            let world_canvas = r.get(world, field::world::CANVAS)?.r();
                            let object_get_class_value = host::object_get_class(r, world_canvas)?;
                            let map_object_state_y =
                                r.get(object_5, field::map_object::STATE_Y)?.i();
                            if map_object_state_y != 0 {
                                let world_canvas_2 = r.get(world, field::world::CANVAS)?.r();
                                let canvas_sound = r.get(world_canvas_2, field::canvas::SOUND)?.r();
                                number_7 = 7;
                                object_6 = canvas_sound;
                                r.set(canvas_sound, field::sound::PENDING_EFFECT, V::I(number_7))?;
                            }
                        }
                        r.set(object_5, field::map_object::STATE_Y, V::I(0))?;
                    }
                }
                number_4 = number_4.wrapping_add(1);
            }
        }
        let world_objects_2 = r.get(world, field::world::OBJECTS)?.r();
        let vector_element_at_value = host::vector_element_at(r, world_objects_2, number_4)?;
        host::check_cast(r, vector_element_at_value, "n")?;
        object_5 = vector_element_at_value;
        let map_object_kind = r.get(vector_element_at_value, field::map_object::KIND)?.i();
        if map_object_kind == 218 {
            number_6 = 0;
            'update_triggers_loop_4: loop {
                let world_items = r.get(world, field::world::ITEMS)?.r();
                let vector_size_value_3 = host::vector_size(r, world_items)?;
                if number_6 >= vector_size_value_3 {
                    break 'update_triggers_loop_4;
                }
                let world_items_2 = r.get(world, field::world::ITEMS)?.r();
                let vector_element_at_value_3 =
                    host::vector_element_at(r, world_items_2, number_6)?;
                host::check_cast(r, vector_element_at_value_3, "c")?;
                object_7 = vector_element_at_value_3;
                let drawable_x_3 = r.get(vector_element_at_value_3, field::drawable::X)?.i();
                let drawable_x_4 = r.get(object_5, field::drawable::X)?.i();
                if drawable_x_3 <= drawable_x_4 {
                    let drawable_x_6 = r.get(object_7, field::drawable::X)?.i();
                    let drawable_x_7 = r.get(object_5, field::drawable::X)?.i();
                    if !(drawable_x_6.wrapping_add(16) < drawable_x_7) {
                        let drawable_y_3 = r.get(object_7, field::drawable::Y)?.i();
                        let drawable_y_4 = r.get(object_5, field::drawable::Y)?.i();
                        if drawable_y_3 <= drawable_y_4 {
                            let drawable_y_6 = r.get(object_7, field::drawable::Y)?.i();
                            let drawable_y_7 = r.get(object_5, field::drawable::Y)?.i();
                            if !(drawable_y_6.wrapping_add(16) < drawable_y_7) {
                                r.set(object_7, field::item::ACTIVE, V::I(0))?;
                            }
                        }
                    }
                }
                number_6 = number_6.wrapping_add(1);
                continue 'update_triggers_loop_4;
            }
        }
        let map_object_trigger_kind = r.get(object_5, field::map_object::TRIGGER_KIND)?.i();
        if map_object_trigger_kind != 0 {
            let map_object_trigger_kind_3 = r.get(object_5, field::map_object::TRIGGER_KIND)?.i();
            if map_object_trigger_kind_3 != 3 {
                let map_object_trigger_kind_4 =
                    r.get(object_5, field::map_object::TRIGGER_KIND)?.i();
                if map_object_trigger_kind_4 == 1 {
                    let count_active_triggers_value_2 =
                        super::world::count_active_triggers(r, world)?;
                    let map_object_trigger_value_2 =
                        r.get(object_5, field::map_object::TRIGGER_VALUE)?.i();
                    if count_active_triggers_value_2 < map_object_trigger_value_2 {
                        r.set(object_5, field::map_object::SOLID, V::I(1))?;
                        r.set(object_5, field::map_object::STATE_Y, V::I(1))?;
                    } else {
                        r.set(object_5, field::map_object::SOLID, V::I(0))?;
                        let map_object_persistence_id_3 =
                            r.get(object_5, field::map_object::PERSISTENCE_ID)?.i();
                        if map_object_persistence_id_3 > 0 {
                            let map_object_persistence_id_4 =
                                r.get(object_5, field::map_object::PERSISTENCE_ID)?.i();
                            let world_map_2 = r.get(world, field::world::MAP)?.i();
                            super::world::mark_persistent_event(
                                r,
                                world,
                                map_object_persistence_id_4,
                                world_map_2,
                                player,
                            )?;
                            let world_canvas_3 = r.get(world, field::world::CANVAS)?.r();
                            let object_get_class_value_2 =
                                host::object_get_class(r, world_canvas_3)?;
                            let map_object_state_y_2 =
                                r.get(object_5, field::map_object::STATE_Y)?.i();
                            if map_object_state_y_2 != 0 {
                                let world_canvas_4 = r.get(world, field::world::CANVAS)?.r();
                                let canvas_sound_2 =
                                    r.get(world_canvas_4, field::canvas::SOUND)?.r();
                                number_7 = 7;
                                object_6 = canvas_sound_2;
                                r.set(
                                    canvas_sound_2,
                                    field::sound::PENDING_EFFECT,
                                    V::I(number_7),
                                )?;
                            }
                        }
                        r.set(object_5, field::map_object::STATE_Y, V::I(0))?;
                    }
                }
            } else {
                r.set(object_5, field::map_object::TRIGGER_ACTIVE, V::I(0))?;
                let drawable_x_2 = r.get(object_5, field::drawable::X)?.i();
                let drawable_y_2 = r.get(object_5, field::drawable::Y)?.i();
                let can_entity_move_value = super::world::can_entity_move(
                    r,
                    world,
                    drawable_x_2,
                    drawable_y_2,
                    4,
                    100,
                    number_4,
                )?;
                if can_entity_move_value == 0 {
                    r.set(object_5, field::map_object::TRIGGER_ACTIVE, V::I(1))?;
                }
                let drawable_x_5 = r.get(object_5, field::drawable::X)?.i();
                if drawable_x_5 <= number_2 {
                    let drawable_x_8 = r.get(object_5, field::drawable::X)?.i();
                    if !(drawable_x_8.wrapping_add(16) < number_2) {
                        let drawable_y_5 = r.get(object_5, field::drawable::Y)?.i();
                        if !(drawable_y_5 > number_3.wrapping_add(32)) {
                            let drawable_y_8 = r.get(object_5, field::drawable::Y)?.i();
                            if !(drawable_y_8.wrapping_add(16) < number_3.wrapping_add(32)) {
                                r.set(object_5, field::map_object::TRIGGER_ACTIVE, V::I(1))?;
                            }
                        }
                    }
                }
            }
        }
        number_4 = number_4.wrapping_add(1);
        continue 'update_triggers_loop_2;
    }
}

/// Interact. Original reference: `j.a(Lh;Lb;Z)I`.
#[rustfmt::skip]
pub(in crate::game) fn interact(
    r: &mut Runtime,
    world: u32,
    player: u32,
    sound: u32,
    attacking: i32,
) -> Result<i32, u32> {
    let mut number_4: i32 = 0;
    let mut number_5: i32 = 0;
    let mut number_6: i32 = 0;
    let mut number_7: i32 = 0;
    let mut number_8: i32 = 0;
    let mut number_9: i32 = 0;
    let mut object_10: u32 = 0;
    let mut number_11: i32 = 0;
    let mut object_11: u32 = 0;
    let mut number_12: i32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut carry_1_i: i32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    'check_world_objects: {
    'next_step_2: {
    'check_world_enemies: {
    'next_step_3: {
    'check_world_items: {
    number_4 = 0;
    let drawable_x = r.get(player, field::drawable::X)?.i();
    number_5 = drawable_x;
    let drawable_y = r.get(player, field::drawable::Y)?.i();
    number_6 = drawable_y;
    number_7 = 0;
    number_8 = 0;
    let player_facing = r.get(player, field::player::FACING)?.i();
    if player_facing == 0 {
        number_5 = number_5.wrapping_add(6);
        number_6 = number_6.wrapping_add(-9);
        number_7 = 9;
        number_8 = 18;
    }
    let player_facing_2 = r.get(player, field::player::FACING)?.i();
    if player_facing_2 == 1 {
        number_5 = number_5.wrapping_add(17);
        number_6 = number_6.wrapping_add(12);
        number_7 = 18;
        number_8 = 9;
    }
    let player_facing_3 = r.get(player, field::player::FACING)?.i();
    if player_facing_3 == 2 {
        number_5 = number_5.wrapping_add(12);
        number_6 = number_6.wrapping_add(20);
        number_7 = 9;
        number_8 = 18;
    }
    let player_facing_4 = r.get(player, field::player::FACING)?.i();
    if player_facing_4 == 3 {
        number_5 = number_5.wrapping_add(-12);
        number_6 = number_6.wrapping_add(12);
        number_7 = 18;
        number_8 = 9;
    }
    number_9 = 0;
    } // check_world_items
    'interact_loop_6: loop {
        'check_world_items_2: {
        let world_items = r.get(world, field::world::ITEMS)?.r();
        let vector_size_value_3 = host::vector_size(r, world_items)?;
        if number_9 >= vector_size_value_3 {
            break 'next_step_3;
        }
        } // check_world_items_2
        let world_items_2 = r.get(world, field::world::ITEMS)?.r();
        let vector_element_at_value_3 =
            host::vector_element_at(r, world_items_2, number_9)?;
        host::check_cast(r, vector_element_at_value_3, "c")?;
        object_10 = vector_element_at_value_3;
        let drawable_x_2 = r.get(vector_element_at_value_3, field::drawable::X)?.i();
        if !(drawable_x_2 > number_5.wrapping_add(number_7)) {
            let drawable_x_3 = r.get(object_10, field::drawable::X)?.i();
            let drawable_half_width =
                r.get(object_10, field::drawable::HALF_WIDTH)?.i();
            if !((drawable_x_3)
                .wrapping_add(drawable_half_width.wrapping_shl((1) as u32))
                < number_5)
            {
                let drawable_y_2 = r.get(object_10, field::drawable::Y)?.i();
                if !(drawable_y_2 > number_6.wrapping_add(number_8)) {
                    let drawable_y_3 = r.get(object_10, field::drawable::Y)?.i();
                    let drawable_half_height =
                        r.get(object_10, field::drawable::HALF_HEIGHT)?.i();
                    if !((drawable_y_3)
                        .wrapping_add(drawable_half_height.wrapping_shl((1) as u32))
                        < number_6)
                    {
                        let item_kind_7 = r.get(object_10, field::item::KIND)?.i();
                        if item_kind_7 != 128 {
                            let item_kind_9 =
                                r.get(object_10, field::item::KIND)?.i();
                            if item_kind_9 != 129 {
                                let world_canvas_28 =
                                    r.get(world, field::world::CANVAS)?.r();
                                let collect_value = super::item::collect(
                                    r,
                                    object_10,
                                    player,
                                    sound,
                                    world_canvas_28,
                                    1,
                                )?;
                                number_4 = collect_value;
                                if collect_value == 1 {
                                    let world_items_3 =
                                        r.get(world, field::world::ITEMS)?.r();
                                    host::vector_remove_element_at(
                                        r,
                                        world_items_3,
                                        number_9,
                                    )?;
                                    let world_canvas_31 =
                                        r.get(world, field::world::CANVAS)?.r();
                                    r.set(
                                        world_canvas_31,
                                        field::canvas::REDRAW_WORLD,
                                        V::I(1),
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        }
        number_9 = number_9.wrapping_add(1);
        continue 'interact_loop_6;
    }
    } // next_step_3
    number_9 = 0;
    } // check_world_enemies
    'interact_loop_9: loop {
        'advance_index: {
        'check_enemy_state_n_2: {
        'check_enemy_state_n: {
        'check_enemy_facing: {
        'check_enemy_kind_8: {
        'next_step_7: {
        'check_enemy_state_t_lower: {
        'check_world_canvas_7: {
        'check_enemy_kind_7: {
        'check_enemy_kind_6: {
        'check_drawable_y_4: {
        'check_drawable_y_3: {
        'check_drawable_x_4: {
        'check_drawable_x_3: {
        'check_enemy_state_f_lower: {
        'check_world_canvas_3: {
        'handle_result: {
        'system_gc: {
        'handle_result_3: {
        'handle_result_2: {
        'check_world_canvas_6: {
        'check_world_canvas_5: {
        'check_world_canvas_4: {
        'check_world_canvas_2: {
        'check_enemy_kind_3: {
        'check_world_canvas: {
        'check_enemy_kind: {
        'check_enemy_kind_5: {
        'check_enemy_kind_4: {
        'check_enemy_kind_2: {
        'check_enemy_active: {
        'check_world_enemies_2: {
        let world_enemies = r.get(world, field::world::ENEMIES)?.r();
        let vector_size_value = host::vector_size(r, world_enemies)?;
        if number_9 >= vector_size_value { break 'next_step_2; }
        } // check_world_enemies_2
        let world_enemies_2 = r.get(world, field::world::ENEMIES)?.r();
        let vector_element_at_value = host::vector_element_at(r, world_enemies_2, number_9)?;
        host::check_cast(r, vector_element_at_value, "l")?;
        object_10 = vector_element_at_value;
        if attacking == 0 { break 'advance_index; }
        } // check_enemy_active
        let enemy_active = r.get(object_10, field::enemy::ACTIVE)?.i();
        if enemy_active != 0 { break 'check_enemy_kind; }
        } // check_enemy_kind_2
        let enemy_kind_2 = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_2 == 34 { break 'check_enemy_kind; }
        } // check_enemy_kind_4
        let enemy_kind_4 = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_4 == 135 { break 'check_enemy_kind; }
        } // check_enemy_kind_5
        let enemy_kind_5 = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_5 != 132 { break 'advance_index; }
        } // check_enemy_kind
        let enemy_kind = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind != 132 { break 'check_enemy_kind_3; }
        } // check_world_canvas
        let world_canvas = r.get(world, field::world::CANVAS)?.r();
        let canvas_player = r.get(world_canvas, field::canvas::PLAYER)?.r();
        let player_has_hookshot = r.get(canvas_player, field::player::HAS_HOOKSHOT)?.i();
        if player_has_hookshot == 0 {
let world_canvas_3 = r.get(world, field::world::CANVAS)?.r();
r.set(world_canvas_3, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
let world_canvas_4 = r.get(world, field::world::CANVAS)?.r();
r.set(world_canvas_4, field::canvas::ITEM_MESSAGE, V::I(30))?;
} else {
let world_canvas_5 = r.get(world, field::world::CANVAS)?.r();
let canvas_player_3 = r.get(world_canvas_5, field::canvas::PLAYER)?.r();
let player_has_fish = r.get(canvas_player_3, field::player::HAS_FISH)?.i();
if player_has_fish == 0 {
let world_canvas_8 = r.get(world, field::world::CANVAS)?.r();
r.set(world_canvas_8, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
let world_canvas_9 = r.get(world, field::world::CANVAS)?.r();
r.set(world_canvas_9, field::canvas::ITEM_MESSAGE, V::I(31))?;
let world_canvas_10 = r.get(world, field::world::CANVAS)?.r();
let canvas_player_6 = r.get(world_canvas_10, field::canvas::PLAYER)?.r();
let player_progression_flags_2 = r.get(canvas_player_6, field::player::PROGRESSION_FLAGS)?.i();
r.set(canvas_player_6, field::player::PROGRESSION_FLAGS, V::I((player_progression_flags_2 | 268435456)))?;
let world_canvas_11 = r.get(world, field::world::CANVAS)?.r();
let canvas_player_7 = r.get(world_canvas_11, field::canvas::PLAYER)?.r();
r.set(canvas_player_7, field::player::HAS_FISH, V::I(1))?;
} else {
let world_canvas_12 = r.get(world, field::world::CANVAS)?.r();
r.set(world_canvas_12, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
let world_canvas_13 = r.get(world, field::world::CANVAS)?.r();
r.set(world_canvas_13, field::canvas::ITEM_MESSAGE, V::I(32))?;
}
}
        break 'advance_index;
        } // check_enemy_kind_3
        let enemy_kind_3 = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_3 != 135 { break 'check_enemy_state_f_lower; }
        } // check_world_canvas_2
        let world_canvas_2 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_2 = r.get(world_canvas_2, field::canvas::PLAYER)?.r();
        let player_progression_flags = r.get(canvas_player_2, field::player::PROGRESSION_FLAGS)?.i();
        if (player_progression_flags & 67108864) <= 0 { break 'check_world_canvas_3; }
        } // check_world_canvas_4
        let world_canvas_7 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_5 = r.get(world_canvas_7, field::canvas::PLAYER)?.r();
        let player_tunic_color_2 = r.get(canvas_player_5, field::player::TUNIC_COLOR)?.i();
        if player_tunic_color_2 == 2 { break 'check_world_canvas_3; }
        } // check_world_canvas_5
        let world_canvas_18 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_8 = r.get(world_canvas_18, field::canvas::PLAYER)?.r();
        let player_progression_flags_3 = r.get(canvas_player_8, field::player::PROGRESSION_FLAGS)?.i();
        r.set(canvas_player_8, field::player::PROGRESSION_FLAGS, V::I((player_progression_flags_3 | 134217728)))?;
        let world_canvas_19 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_9 = r.get(world_canvas_19, field::canvas::PLAYER)?.r();
        let player_tunic_color_3 = r.get(canvas_player_9, field::player::TUNIC_COLOR)?.i();
        if player_tunic_color_3 > 2 { break 'advance_index; }
        } // check_world_canvas_6
        let world_canvas_20 = r.get(world, field::world::CANVAS)?.r();
        r.set(world_canvas_20, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
        let world_canvas_21 = r.get(world, field::world::CANVAS)?.r();
        r.set(world_canvas_21, field::canvas::ITEM_MESSAGE, V::I(27))?;
        let world_canvas_22 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_10 = r.get(world_canvas_22, field::canvas::PLAYER)?.r();
        r.set(canvas_player_10, field::player::TUNIC_COLOR, V::I(2))?;
        let world_canvas_23 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_11 = r.get(world_canvas_23, field::canvas::PLAYER)?.r();
        r.set(canvas_player_11, field::player::DEFENSE, V::I(4))?;
        let world_canvas_24 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_12 = r.get(world_canvas_24, field::canvas::PLAYER)?.r();
        r.set(canvas_player_12, field::player::SPRITE, V::R(0))?;
        let world_canvas_25 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_13 = r.get(world_canvas_25, field::canvas::PLAYER)?.r();
        r.set(canvas_player_13, field::player::SPRITE, V::R(0))?;
        let world_canvas_26 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_14 = r.get(world_canvas_26, field::canvas::PLAYER)?.r();
        let text_value = r.intern("/Images/LinkWalk_Red.png");
        outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value)?)) })();
        (carry_0_r,) = (canvas_player_14,);
        if outcome.is_err() { break 'handle_result; }
        } // handle_result_2
        let image_create_image_value = outcome.unwrap().r();
        r.set(carry_0_r, field::player::SPRITE, V::R(image_create_image_value))?;
        let world_canvas_27 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_15 = r.get(world_canvas_27, field::canvas::PLAYER)?.r();
        r.set(canvas_player_15, field::player::DEFENSE, V::I(4))?;
        let world_resources = r.get(world, field::world::RESOURCES)?.r();
        let resources_enemy_images = r.get(world_resources, field::resources::ENEMY_IMAGES)?.r();
        r.array_set(resources_enemy_images, 135, V::R(0))?;
        let world_resources_2 = r.get(world, field::world::RESOURCES)?.r();
        let resources_enemy_images_2 = r.get(world_resources_2, field::resources::ENEMY_IMAGES)?.r();
        let text_value_2 = r.intern("/Images/GreenTingle.png");
        outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_2)?)) })();
        (carry_0_r, carry_1_i,) = (resources_enemy_images_2, 135,);
        if outcome.is_err() { break 'handle_result; }
        } // handle_result_3
        let image_create_image_value_2 = outcome.unwrap().r();
        r.array_set(carry_0_r, carry_1_i, V::R(image_create_image_value_2))?;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("tingle test");
        outcome = (|| -> JResult { host::print_stream_println(r, state_out_value, text_value_3)?; Ok(V::I(0)) })();
        if outcome
            .is_err(
            )
        {
            break 'handle_result;
        }
        } // system_gc
        host::system_gc(r)?;

        break 'advance_index;
        } // handle_result
        carry_0_r = outcome
            .unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}

        break 'advance_index;
        } // check_world_canvas_3
        let world_canvas_6 = r.get(world, field::world::CANVAS)?.r();
        let canvas_player_4 = r.get(world_canvas_6, field::canvas::PLAYER)?.r();
        let player_tunic_color = r.get(canvas_player_4, field::player::TUNIC_COLOR)?.i();
        if player_tunic_color
            != 2
        {
            let world_canvas_14 = r.get(world, field::world::CANVAS)?.r();
            r.set(world_canvas_14, field::canvas::ITEM_MESSAGE, V::I(28))?;
            let world_canvas_15 = r.get(world, field::world::CANVAS)?.r();
            r.set(world_canvas_15, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
        } else {
            let world_canvas_16 = r.get(world, field::world::CANVAS)?.r();
            r.set(world_canvas_16, field::canvas::ITEM_MESSAGE, V::I(29))?;
            let world_canvas_17 = r.get(world, field::world::CANVAS)?.r();
            r.set(world_canvas_17, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
        }
        break 'advance_index;
        } // check_enemy_state_f_lower
        let state_f_lower_value = r
            .get(
                object_10,
                field::enemy::DYING,
            )?
            .i();
        if state_f_lower_value != 0
        {
            break 'advance_index;
        }
        } // check_drawable_x_3
        let x_value_6 = r
            .get(
                object_10,
                field::drawable::X,
            )?
            .i();
        if x_value_6
            > (number_5)
                .wrapping_add(number_7)
        {
            break 'advance_index;
        }
        } // check_drawable_x_4
        let x_value_7 = r
            .get(
                object_10,
                field::drawable::X,
            )?
            .i();
        let width_value_2 = r
            .get(
                object_10,
                field::enemy::WIDTH,
            )?
            .i();
        if (x_value_7)
            .wrapping_add(width_value_2)
            < number_5
        {
            break 'advance_index;
        }
        } // check_drawable_y_3
        let y_value_6 = r
            .get(object_10, field::drawable::Y)?
            .i();
        if y_value_6
            > number_6.wrapping_add(number_8)
        {
            break 'advance_index;
        }
        } // check_drawable_y_4
        let y_value_7 = r
            .get(object_10, field::drawable::Y)?
            .i();
        let height_value_2 = r
            .get(object_10, field::enemy::HEIGHT)?
            .i();
        if y_value_7.wrapping_add(height_value_2)
            < number_6
        {
            break 'advance_index;
        }
        } // check_enemy_kind_6
        let enemy_kind_11 =
            r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_11 == 17 {
            break 'advance_index;
        }
        } // check_enemy_kind_7
        let enemy_kind_12 =
            r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_12 != 34 {
            break 'check_enemy_state_t_lower;
        }
        } // check_world_canvas_7
        let world_canvas_32 =
            r.get(world, field::world::CANVAS)?.r();
        r.set(
            world_canvas_32,
            field::canvas::SHOW_ITEM_MESSAGE,
            V::I(1),
        )?;
        let world_canvas_33 =
            r.get(world, field::world::CANVAS)?.r();
        r.set(
            world_canvas_33,
            field::canvas::ITEM_MESSAGE,
            V::I(25),
        )?;
        let world_canvas_34 =
            r.get(world, field::world::CANVAS)?.r();
        let canvas_player_16 =
            r.get(world_canvas_34, field::canvas::PLAYER)?.r();
        let world_canvas_35 =
            r.get(world, field::world::CANVAS)?.r();
        let canvas_player_17 =
            r.get(world_canvas_35, field::canvas::PLAYER)?.r();
        let max_health_value = r
            .get(canvas_player_17, field::player::MAX_HEALTH)?
            .i();
        let quotient_value =
            arithmetic::divide_int(r, max_health_value, 4)?;
        r.set(
            canvas_player_16,
            field::player::HEALTH,
            V::I(quotient_value.wrapping_shl((2) as u32)),
        )?;
        let world_canvas_36 =
            r.get(world, field::world::CANVAS)?.r();
        r.set(
            world_canvas_36,
            field::canvas::REDRAW_WORLD,
            V::I(1),
        )?;
        break 'advance_index;
        } // check_enemy_state_t_lower
        let enemy_hurt_timer =
            r.get(object_10, field::enemy::HURT_TIMER)?.i();
        if enemy_hurt_timer > 0 {
            break 'advance_index;
        }
        } // next_step_7
        number_11 = 0;
        let enemy_kind_17 = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_17 == 10 {
            break 'check_enemy_facing;
        }
        } // check_enemy_kind_8
        let enemy_kind_20 = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_20 != 27 {
            break 'check_enemy_state_n;
        }
        } // check_enemy_facing
        let enemy_facing_5 = r.get(object_10, field::enemy::FACING)?.i();
        if enemy_facing_5 == 0 {
            let player_facing_7 = r.get(player, field::player::FACING)?.i();
            if player_facing_7 == 2 {
                r.set(object_10, field::enemy::VELOCITY_Y, V::I(5))?;
                r.set(object_10, field::enemy::VELOCITY_X, V::I(0))?;
                r.set(object_10, field::enemy::KNOCKBACK_TIMER, V::I(2))?;
            } else {
                let enemy_health_2 =
                    r.get(object_10, field::enemy::HEALTH)?.i();
                let player_sword_damage_2 =
                    r.get(player, field::player::SWORD_DAMAGE)?.i();
                r.set(
                    object_10,
                    field::enemy::HEALTH,
                    V::I(enemy_health_2.wrapping_sub(player_sword_damage_2)),
                )?;
                number_11 = 1;
            }
        }
        let enemy_facing_6 = r.get(object_10, field::enemy::FACING)?.i();
        if enemy_facing_6 == 1 {
            let player_facing_9 = r.get(player, field::player::FACING)?.i();
            if player_facing_9 == 3 {
                r.set(object_10, field::enemy::VELOCITY_X, V::I(-5))?;
                r.set(object_10, field::enemy::VELOCITY_Y, V::I(0))?;
                r.set(object_10, field::enemy::KNOCKBACK_TIMER, V::I(2))?;
            } else {
                let enemy_health_3 =
                    r.get(object_10, field::enemy::HEALTH)?.i();
                let player_sword_damage_3 =
                    r.get(player, field::player::SWORD_DAMAGE)?.i();
                r.set(
                    object_10,
                    field::enemy::HEALTH,
                    V::I(enemy_health_3.wrapping_sub(player_sword_damage_3)),
                )?;
                number_11 = 1;
            }
        }
        let enemy_facing_8 = r.get(object_10, field::enemy::FACING)?.i();
        if enemy_facing_8 == 2 {
            let player_facing_11 = r.get(player, field::player::FACING)?.i();
            if player_facing_11 == 0 {
                r.set(object_10, field::enemy::VELOCITY_X, V::I(0))?;
                r.set(object_10, field::enemy::VELOCITY_Y, V::I(-5))?;
                r.set(object_10, field::enemy::KNOCKBACK_TIMER, V::I(2))?;
            } else {
                let enemy_health_5 =
                    r.get(object_10, field::enemy::HEALTH)?.i();
                let player_sword_damage_4 =
                    r.get(player, field::player::SWORD_DAMAGE)?.i();
                r.set(
                    object_10,
                    field::enemy::HEALTH,
                    V::I(enemy_health_5.wrapping_sub(player_sword_damage_4)),
                )?;
                number_11 = 1;
            }
        }
        let enemy_facing_10 = r.get(object_10, field::enemy::FACING)?.i();
        if enemy_facing_10 == 3 {
            let player_facing_12 = r.get(player, field::player::FACING)?.i();
            if player_facing_12 == 1 {
                r.set(object_10, field::enemy::VELOCITY_X, V::I(5))?;
                r.set(object_10, field::enemy::VELOCITY_Y, V::I(0))?;
                r.set(object_10, field::enemy::KNOCKBACK_TIMER, V::I(2))?;
            } else {
                let enemy_health_6 =
                    r.get(object_10, field::enemy::HEALTH)?.i();
                let player_sword_damage_5 =
                    r.get(player, field::player::SWORD_DAMAGE)?.i();
                r.set(
                    object_10,
                    field::enemy::HEALTH,
                    V::I(enemy_health_6.wrapping_sub(player_sword_damage_5)),
                )?;
                number_11 = 1;
            }
        }
        break 'check_enemy_state_n_2;
        } // check_enemy_state_n
        let enemy_health = r.get(object_10, field::enemy::HEALTH)?.i();
        let player_sword_damage = r.get(player, field::player::SWORD_DAMAGE)?.i();
        r.set(
            object_10,
            field::enemy::HEALTH,
            V::I(enemy_health.wrapping_sub(player_sword_damage)),
        )?;
        number_11 = 1;
        let enemy_kind_23 = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_23 == 28 {
            let enemy_behavior_state = r.get(object_10, field::enemy::BEHAVIOR_STATE)?.i();
            if enemy_behavior_state == 3 {
                r.set(object_10, field::enemy::BEHAVIOR_STATE, V::I(4))?;
                r.set(object_10, field::enemy::STATE_TIMER, V::I(0))?;
            }
        }
        let enemy_kind_25 = r.get(object_10, field::enemy::KIND)?.i();
        if enemy_kind_25 == 29 {
            let enemy_behavior_state_2 =
                r.get(object_10, field::enemy::BEHAVIOR_STATE)?.i();
            if enemy_behavior_state_2 == 3 {
                r.set(object_10, field::enemy::BEHAVIOR_STATE, V::I(4))?;
                r.set(object_10, field::enemy::STATE_TIMER, V::I(0))?;
            }
        }
        } // check_enemy_state_n_2
        let enemy_health_4 = r.get(object_10, field::enemy::HEALTH)?.i();
        if enemy_health_4 > 0 {
            if number_11 != 0 {
                r.set(object_10, field::enemy::HURT_TIMER, V::I(5))?;
                let enemy_kind_26 = r.get(object_10, field::enemy::KIND)?.i();
                if enemy_kind_26 != 20 {
                    let enemy_kind_27 = r.get(object_10, field::enemy::KIND)?.i();
                    if enemy_kind_27 != 21 {
                        let enemy_kind_28 = r.get(object_10, field::enemy::KIND)?.i();
                        if enemy_kind_28 != 23 {
                            let enemy_kind_29 =
                                r.get(object_10, field::enemy::KIND)?.i();
                            if enemy_kind_29 != 24 {
                                let enemy_kind_30 =
                                    r.get(object_10, field::enemy::KIND)?.i();
                                if enemy_kind_30 != 28 {
                                    let enemy_kind_31 =
                                        r.get(object_10, field::enemy::KIND)?.i();
                                    if enemy_kind_31 != 29 {
                                        let enemy_kind_32 =
                                            r.get(object_10, field::enemy::KIND)?.i();
                                        if enemy_kind_32 != 30 {
                                            let kind_value_33 = r
                                                .get(object_10, field::enemy::KIND)?
                                                .i();
                                            if kind_value_33 != 31 {
                                                let kind_value_34 = r
                                                    .get(object_10, field::enemy::KIND)?
                                                    .i();
                                                if kind_value_34 != 25 {
                                                    let facing_value_13 = r
                                                        .get(
                                                            player,
                                                            field::player::FACING,
                                                        )?
                                                        .i();
                                                    if facing_value_13 == 0 {
                                                        r.set(
                                                            object_10,
                                                            field::enemy::VELOCITY_Y,
                                                            V::I(-5),
                                                        )?;
                                                        r.set(
                                                            object_10,
                                                            field::enemy::VELOCITY_X,
                                                            V::I(0),
                                                        )?;
                                                    }
                                                    let facing_value_14 = r
                                                        .get(
                                                            player,
                                                            field::player::FACING,
                                                        )?
                                                        .i();
                                                    if facing_value_14 == 1 {
                                                        r.set(
                                                            object_10,
                                                            field::enemy::VELOCITY_X,
                                                            V::I(5),
                                                        )?;
                                                        r.set(
                                                            object_10,
                                                            field::enemy::VELOCITY_Y,
                                                            V::I(0),
                                                        )?;
                                                    }
                                                    let facing_value_15 = r
                                                        .get(
                                                            player,
                                                            field::player::FACING,
                                                        )?
                                                        .i();
                                                    if facing_value_15 == 2 {
                                                        r.set(
                                                            object_10,
                                                            field::enemy::VELOCITY_X,
                                                            V::I(0),
                                                        )?;
                                                        r.set(
                                                            object_10,
                                                            field::enemy::VELOCITY_Y,
                                                            V::I(5),
                                                        )?;
                                                    }
                                                    let facing_value_16 = r
                                                        .get(
                                                            player,
                                                            field::player::FACING,
                                                        )?
                                                        .i();
                                                    if facing_value_16 == 3 {
                                                        r.set(
                                                            object_10,
                                                            field::enemy::VELOCITY_X,
                                                            V::I(-5),
                                                        )?;
                                                        r.set(
                                                            object_10,
                                                            field::enemy::VELOCITY_Y,
                                                            V::I(0),
                                                        )?;
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
        } else {
            r.set(object_10, field::enemy::DEATH_FRAME, V::I(0))?;
            r.set(object_10, field::enemy::DYING, V::I(1))?;
            let world_death_image = r.get(world, field::world::DEATH_IMAGE)?.r();
            r.set(object_10, field::enemy::IMAGE, V::R(world_death_image))?;
            r.set(object_10, field::enemy::ANIMATION, V::I(0))?;
        }
        } // advance_index
        number_9 = number_9.wrapping_add(1);
        continue 'interact_loop_9;
    }
    } // next_step_2
    number_9 = 0;
    } // check_world_objects
    'interact_loop_39: loop {
        'advance_index_2: {
        'check_map_object_state_x: {
        'check_map_object_kind_2: {
        'check_map_object_kind: {
        'next_step_4: {
        'check_map_object_kind_4: {
        'check_player_keys: {
        'check_world_canvas_8: {
        'check_world_map_2: {
        'check_map_object_kind_11: {
        'check_player_progression_flags_2: {
        'check_world_map: {
        'check_map_object_kind_9: {
        'check_player_progression_flags: {
        'check_map_object_kind_7: {
        'check_map_object_kind_5: {
        'check_map_object_state_o: {
        'check_world_objects_2: {
        'check_map_object_state_x_2: {
        'check_map_object_kind_10: {
        'check_map_object_kind_8: {
        'check_map_object_kind_6: {
        'next_step_6: {
        'check_map_object_state_g: {
        'check_map_object_trigger_active: {
        'check_map_object_kind_3: {
        'next_step_5: {
        'check_map_object_has_dialogue_2: {
        'check_drawable_y_2: {
        'check_drawable_y: {
        'check_drawable_x_2: {
        'check_drawable_x: {
        'check_map_object_solid: {
        'check_map_object_has_dialogue: {
        let world_objects = r.get(world, field::world::OBJECTS)?.r();
        let vector_size_value_2 = host::vector_size(r, world_objects)?;
        if number_9 >= vector_size_value_2 {
return Ok(number_4);
}
        let world_objects_2 = r.get(world, field::world::OBJECTS)?.r();
        let vector_element_at_value_2 = host::vector_element_at(r, world_objects_2, number_9)?;
        host::check_cast(r, vector_element_at_value_2, "n")?;
        object_10 = vector_element_at_value_2;
        let world_alternate_map = r.get(world, field::world::ALTERNATE_MAP)?.i();
        r.set(vector_element_at_value_2, field::map_object::STATE_C, V::I(world_alternate_map))?;
        let map_object_state_f = r.get(object_10, field::map_object::STATE_F)?.i();
        if map_object_state_f != 0 { break 'check_map_object_solid; }
        } // check_map_object_has_dialogue
        let map_object_has_dialogue = r.get(object_10, field::map_object::HAS_DIALOGUE)?.i();
        if map_object_has_dialogue <= 0 { break 'advance_index_2; }
        } // check_map_object_solid
        let map_object_solid = r.get(object_10, field::map_object::SOLID)?.i();
        if map_object_solid == 0 { break 'advance_index_2; }
        } // check_drawable_x
        let drawable_x_4 = r.get(object_10, field::drawable::X)?.i();
        let map_object_state_d_upper = r.get(object_10, field::map_object::STATE_D_UPPER)?.i();
        if drawable_x_4.wrapping_add(map_object_state_d_upper) > number_5.wrapping_add(number_7) { break 'next_step_4; }
        } // check_drawable_x_2
        let drawable_x_5 = r.get(object_10, field::drawable::X)?.i();
        let map_object_width = r.get(object_10, field::map_object::WIDTH)?.i();
        let map_object_state_d_upper_2 = r.get(object_10, field::map_object::STATE_D_UPPER)?.i();
        if (drawable_x_5.wrapping_add(map_object_width)).wrapping_add(map_object_state_d_upper_2) < number_5 { break 'next_step_4; }
        } // check_drawable_y
        let drawable_y_4 = r.get(object_10, field::drawable::Y)?.i();
        let map_object_state_e_upper = r.get(object_10, field::map_object::STATE_E_UPPER)?.i();
        if drawable_y_4.wrapping_add(map_object_state_e_upper) > number_6.wrapping_add(number_8) { break 'next_step_4; }
        } // check_drawable_y_2
        let drawable_y_5 = r.get(object_10, field::drawable::Y)?.i();
        let map_object_height = r.get(object_10, field::map_object::HEIGHT)?.i();
        let map_object_state_e_upper_2 = r.get(object_10, field::map_object::STATE_E_UPPER)?.i();
        if (drawable_y_5.wrapping_add(map_object_height)).wrapping_add(map_object_state_e_upper_2) < number_6 { break 'next_step_4; }
        } // check_map_object_has_dialogue_2
        let map_object_has_dialogue_2 = r.get(object_10, field::map_object::HAS_DIALOGUE)?.i();
        if map_object_has_dialogue_2 > 0 {
let world_canvas_29 = r.get(world, field::world::CANVAS)?.r();
r.set(world_canvas_29, field::canvas::STATE_L, V::I(number_9))?;
let world_canvas_30 = r.get(world, field::world::CANVAS)?.r();
r.set(world_canvas_30, field::canvas::STATE_I, V::I(1))?;
r.set(player, field::player::ATTACKING, V::I(0))?;
return Ok(0);
}
        let map_object_state_h = r.get(object_10, field::map_object::STATE_H)?.i();
        if map_object_state_h == 0 { break 'check_map_object_state_g; }
        } // next_step_5

        if attacking == 0 { break 'advance_index_2; }
        } // check_map_object_kind_3
        let map_object_kind_13 = r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_13 != 11 { break 'check_map_object_state_g; }
        } // check_map_object_trigger_active
        r.set(object_10, field::map_object::TRIGGER_ACTIVE, V::I(1))?;
        r.set(object_10, field::map_object::TRIGGER_KIND, V::I(2))?;
        r.set(object_10, field::map_object::KIND, V::I(8))?;
        let world_resources_3 = r.get(world, field::world::RESOURCES)?.r();
        let object_image_value = super::resources::object_image(r, world_resources_3, 8)?;
        r.set(object_10, field::map_object::IMAGE, V::R(object_image_value))?;
        r.set(object_10, field::map_object::STATE_E, V::I(1))?;
        r.set(object_10, field::map_object::STATE_F, V::I(0))?;
        break 'advance_index_2;
        } // check_map_object_state_g
        let map_object_state_g = r.get(object_10, field::map_object::STATE_G)?.i();
        if map_object_state_g == 0 { break 'check_map_object_state_o; }
        } // next_step_6

        if attacking == 0 { break 'advance_index_2; }
        } // check_map_object_kind_6
        let map_object_kind_16 = r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_16 == 3 { break 'check_map_object_state_x_2; }
        } // check_map_object_kind_8
        let map_object_kind_19 = r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_19 == 4 { break 'check_map_object_state_x_2; }
        } // check_map_object_kind_10
        let map_object_kind_22 = r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_22
            != 15
        {
            break 'check_world_objects_2;
        }
        } // check_map_object_state_x_2
        let map_object_state_x_2 = r.get(object_10, field::map_object::STATE_X)?.i();
        if map_object_state_x_2
            == 0
        {
            r.set(object_10, field::map_object::STATE_X, V::I(1))?;
            r.set(object_10, field::map_object::STATE_A, V::I(5))?;
        }
        break 'check_map_object_state_o;
        } // check_world_objects_2
        let world_objects_5 = r.get(world, field::world::OBJECTS)?.r();
        host::vector_remove_element_at(r, world_objects_5, number_9)?;
        } // check_map_object_state_o
        let map_object_state_o = r.get(object_10, field::map_object::STATE_O)?.i();
        if map_object_state_o == 0 {
            break 'check_map_object_kind_4;
        }
        } // check_map_object_kind_5
        let kind_value_15 = r
            .get(
                object_10,
                field::map_object::KIND,
            )?
            .i();
        if kind_value_15 == 78 {
            break 'check_player_progression_flags;
        }
        } // check_map_object_kind_7
        let kind_value_18 = r
            .get(
                object_10,
                field::map_object::KIND,
            )?
            .i();
        if kind_value_18 != 77 {
            break 'check_player_keys;
        }
        } // check_player_progression_flags
        let player_progression_flags_4 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
        if (player_progression_flags_4 & 1048576)
            <= 0
        {
            break 'check_player_progression_flags_2;
        }
        } // check_map_object_kind_9
        let kind_value_21 = r
            .get(
                object_10,
                field::map_object::KIND,
            )?
            .i();
        if kind_value_21 != 77 {
            break 'check_player_progression_flags_2;
        }
        } // check_world_map
        let world_map =
            r.get(world, field::world::MAP)?.i();
        let quotient_value_2 =
            arithmetic::divide_int(r, world_map, 32)?;
        number_11 = quotient_value_2;
        let persistence_id_value = r
            .get(
                object_10,
                field::map_object::PERSISTENCE_ID,
            )?
            .i();
        let remainder_value =
            arithmetic::remainder_int(
                r,
                persistence_id_value,
                32,
            )?;
        number_4 = remainder_value;
        let dungeon_8_events_value = r
            .get(
                player,
                field::player::DUNGEON_8_EVENTS,
            )?
            .r();
        let element_value = r
            .array_get(
                dungeon_8_events_value,
                number_11,
            )?
            .i();
        r.array_set(
            dungeon_8_events_value,
            number_11,
            V::I(
                (element_value
                    | (1_i32).wrapping_shl(
                        (number_4) as u32,
                    )),
            ),
        )?;
        r.set(
            player,
            field::player::ATTACKING,
            V::I(0),
        )?;
        let world_objects_4 =
            r.get(world, field::world::OBJECTS)?.r();
        host::vector_remove_element_at(
            r,
            world_objects_4,
            number_9,
        )?;
        number_4 = 1;
        let world_canvas_39 =
            r.get(world, field::world::CANVAS)?.r();
        r.set(
            world_canvas_39,
            field::canvas::REDRAW_WORLD,
            V::I(1),
        )?;
        let world_canvas_40 =
            r.get(world, field::world::CANVAS)?.r();
        let object_get_class_value =
            host::object_get_class(r, world_canvas_40)?;
        number_12 = 6;
        object_11 = sound;
        r.set(
            sound,
            field::sound::PENDING_EFFECT,
            V::I(number_12),
        )?;
        break 'advance_index_2;
        } // check_player_progression_flags_2
        let progression_flags_value_5 = r
            .get(player, field::player::PROGRESSION_FLAGS)?
            .i();
        if (progression_flags_value_5 & 2097152) <= 0 {
            break 'check_world_canvas_8;
        }
        } // check_map_object_kind_11
        let map_object_kind_24 =
            r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_24 != 78 {
            break 'check_world_canvas_8;
        }
        } // check_world_map_2
        let world_map_3 = r.get(world, field::world::MAP)?.i();
        let quotient_value_4 =
            arithmetic::divide_int(r, world_map_3, 32)?;
        number_11 = quotient_value_4;
        let persistence_id_value_3 = r
            .get(object_10, field::map_object::PERSISTENCE_ID)?
            .i();
        let remainder_value_3 = arithmetic::remainder_int(
            r,
            persistence_id_value_3,
            32,
        )?;
        number_4 = remainder_value_3;
        let player_dungeon_7_events =
            r.get(player, field::player::DUNGEON_7_EVENTS)?.r();
        let element_value_2 =
            r.array_get(player_dungeon_7_events, number_11)?.i();
        r.array_set(
            player_dungeon_7_events,
            number_11,
            V::I(
                (element_value_2
                    | (1_i32).wrapping_shl((number_4) as u32)),
            ),
        )?;
        r.set(player, field::player::ATTACKING, V::I(0))?;
        let world_objects_6 =
            r.get(world, field::world::OBJECTS)?.r();
        host::vector_remove_element_at(
            r,
            world_objects_6,
            number_9,
        )?;
        number_4 = 1;
        let world_canvas_41 =
            r.get(world, field::world::CANVAS)?.r();
        r.set(
            world_canvas_41,
            field::canvas::REDRAW_WORLD,
            V::I(1),
        )?;
        let world_canvas_42 =
            r.get(world, field::world::CANVAS)?.r();
        let object_get_class_value_2 =
            host::object_get_class(r, world_canvas_42)?;
        number_12 = 6;
        object_11 = sound;
        r.set(
            sound,
            field::sound::PENDING_EFFECT,
            V::I(number_12),
        )?;
        break 'advance_index_2;
        } // check_world_canvas_8
        let world_canvas_37 = r.get(world, field::world::CANVAS)?.r();
        r.set(world_canvas_37, field::canvas::ITEM_MESSAGE, V::I(9))?;
        let world_canvas_38 = r.get(world, field::world::CANVAS)?.r();
        r.set(
            world_canvas_38,
            field::canvas::SHOW_ITEM_MESSAGE,
            V::I(1),
        )?;
        break 'advance_index_2;
        } // check_player_keys
        let player_keys = r.get(player, field::player::KEYS)?.i();
        if player_keys > 0 {
            let world_map_2 = r.get(world, field::world::MAP)?.i();
            let quotient_value_3 =
                arithmetic::divide_int(r, world_map_2, 32)?;
            number_11 = quotient_value_3;
            let map_object_persistence_id_2 =
                r.get(object_10, field::map_object::PERSISTENCE_ID)?.i();
            let remainder_value_2 =
                arithmetic::remainder_int(r, map_object_persistence_id_2, 32)?;
            number_4 = remainder_value_2;
            let world_zone = r.get(world, field::world::ZONE)?.i();
            if world_zone == 0 {
                let player_overworld_events =
                    r.get(player, field::player::OVERWORLD_EVENTS)?.r();
                let element_value_3 =
                    r.array_get(player_overworld_events, number_11)?.i();
                r.array_set(
                    player_overworld_events,
                    number_11,
                    V::I(
                        (element_value_3
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_3 = r.get(world, field::world::ZONE)?.i();
            if world_zone_3 == 1 {
                let player_cave_events =
                    r.get(player, field::player::CAVE_EVENTS)?.r();
                let element_value_4 =
                    r.array_get(player_cave_events, number_11)?.i();
                r.array_set(
                    player_cave_events,
                    number_11,
                    V::I(
                        (element_value_4
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_5 = r.get(world, field::world::ZONE)?.i();
            if world_zone_5 == 3 {
                let player_dungeon_1_events =
                    r.get(player, field::player::DUNGEON_1_EVENTS)?.r();
                let element_value_5 =
                    r.array_get(player_dungeon_1_events, number_11)?.i();
                r.array_set(
                    player_dungeon_1_events,
                    number_11,
                    V::I(
                        (element_value_5
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_6 = r.get(world, field::world::ZONE)?.i();
            if world_zone_6 == 4 {
                let player_dungeon_2_events =
                    r.get(player, field::player::DUNGEON_2_EVENTS)?.r();
                let element_value_6 =
                    r.array_get(player_dungeon_2_events, number_11)?.i();
                r.array_set(
                    player_dungeon_2_events,
                    number_11,
                    V::I(
                        (element_value_6
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_7 = r.get(world, field::world::ZONE)?.i();
            if world_zone_7 == 5 {
                let player_dungeon_3_events =
                    r.get(player, field::player::DUNGEON_3_EVENTS)?.r();
                let element_value_7 =
                    r.array_get(player_dungeon_3_events, number_11)?.i();
                r.array_set(
                    player_dungeon_3_events,
                    number_11,
                    V::I(
                        (element_value_7
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_8 = r.get(world, field::world::ZONE)?.i();
            if world_zone_8 == 6 {
                let player_dungeon_4_events =
                    r.get(player, field::player::DUNGEON_4_EVENTS)?.r();
                let element_value_8 =
                    r.array_get(player_dungeon_4_events, number_11)?.i();
                r.array_set(
                    player_dungeon_4_events,
                    number_11,
                    V::I(
                        (element_value_8
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
                let state_out_value_3 =
                    r.static_get(field::system::STATE_OUT).r();
                let object_value_2 =
                    r.alloc("java/lang/StringBuffer", Native::Empty);
                host::string_buffer_initialize(r, object_value_2)?;
                let text_value_5 = r.intern("Curent worl set ");
                let string_buffer_append_2_value_2 =
                    host::string_buffer_append_2(
                        r,
                        object_value_2,
                        text_value_5,
                    )?;
                let string_buffer_append_value_2 =
                    host::string_buffer_append(
                        r,
                        string_buffer_append_2_value_2,
                        number_4,
                    )?;
                let string_buffer_to_string_value_2 =
                    host::string_buffer_to_string(
                        r,
                        string_buffer_append_value_2,
                    )?;
                host::print_stream_println(
                    r,
                    state_out_value_3,
                    string_buffer_to_string_value_2,
                )?;
            }
            let world_zone_9 = r.get(world, field::world::ZONE)?.i();
            if world_zone_9 == 7 {
                let player_dungeon_5_events =
                    r.get(player, field::player::DUNGEON_5_EVENTS)?.r();
                let element_value_9 =
                    r.array_get(player_dungeon_5_events, number_11)?.i();
                r.array_set(
                    player_dungeon_5_events,
                    number_11,
                    V::I(
                        (element_value_9
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_10 = r.get(world, field::world::ZONE)?.i();
            if world_zone_10 == 8 {
                let player_dungeon_6_events =
                    r.get(player, field::player::DUNGEON_6_EVENTS)?.r();
                let element_value_10 =
                    r.array_get(player_dungeon_6_events, number_11)?.i();
                r.array_set(
                    player_dungeon_6_events,
                    number_11,
                    V::I(
                        (element_value_10
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_11 = r.get(world, field::world::ZONE)?.i();
            if world_zone_11 == 9 {
                let player_dungeon_7_events_2 =
                    r.get(player, field::player::DUNGEON_7_EVENTS)?.r();
                let element_value_11 =
                    r.array_get(player_dungeon_7_events_2, number_11)?.i();
                r.array_set(
                    player_dungeon_7_events_2,
                    number_11,
                    V::I(
                        (element_value_11
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_12 = r.get(world, field::world::ZONE)?.i();
            if world_zone_12 == 10 {
                let player_dungeon_8_events_2 =
                    r.get(player, field::player::DUNGEON_8_EVENTS)?.r();
                let element_value_12 =
                    r.array_get(player_dungeon_8_events_2, number_11)?.i();
                r.array_set(
                    player_dungeon_8_events_2,
                    number_11,
                    V::I(
                        (element_value_12
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            let world_zone_13 = r.get(world, field::world::ZONE)?.i();
            if world_zone_13 == 11 {
                let player_dungeon_9_events =
                    r.get(player, field::player::DUNGEON_9_EVENTS)?.r();
                let element_value_13 =
                    r.array_get(player_dungeon_9_events, number_11)?.i();
                r.array_set(
                    player_dungeon_9_events,
                    number_11,
                    V::I(
                        (element_value_13
                            | (1_i32).wrapping_shl((number_4) as u32)),
                    ),
                )?;
            }
            r.set(player, field::player::ATTACKING, V::I(0))?;
            let player_keys_2 = r.get(player, field::player::KEYS)?.i();
            r.set(
                player,
                field::player::KEYS,
                V::I(player_keys_2.wrapping_sub(1)),
            )?;
            let world_objects_7 = r.get(world, field::world::OBJECTS)?.r();
            host::vector_remove_element_at(r, world_objects_7, number_9)?;
            number_4 = 1;
            let world_canvas_43 = r.get(world, field::world::CANVAS)?.r();
            r.set(world_canvas_43, field::canvas::REDRAW_WORLD, V::I(1))?;
            let world_canvas_44 = r.get(world, field::world::CANVAS)?.r();
            let object_get_class_value_3 =
                host::object_get_class(r, world_canvas_44)?;
            number_12 = 6;
            object_11 = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_12))?;
        }
        } // check_map_object_kind_4
        let map_object_kind_14 = r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_14 == 30 {
            let world_switch_cooldown_2 =
                r.get(world, field::world::SWITCH_COOLDOWN)?.i();
            if world_switch_cooldown_2 == 0 {
                let world_alternate_map_2 =
                    r.get(world, field::world::ALTERNATE_MAP)?.i();
                r.set(
                    world,
                    field::world::ALTERNATE_MAP,
                    V::I(world_alternate_map_2.wrapping_add(1)),
                )?;
                let world_alternate_map_3 =
                    r.get(world, field::world::ALTERNATE_MAP)?.i();
                if world_alternate_map_3 > 1 {
                    r.set(world, field::world::ALTERNATE_MAP, V::I(0))?;
                }
                let world_alternate_map_4 =
                    r.get(world, field::world::ALTERNATE_MAP)?.i();
                if world_alternate_map_4 == 0 {
                    let player_world_flags =
                        r.get(player, field::player::WORLD_FLAGS)?.i();
                    let world_zone_2 = r.get(world, field::world::ZONE)?.i();
                    r.set(
                        player,
                        field::player::WORLD_FLAGS,
                        V::I(
                            (player_world_flags
                                & ((1_i32)
                                    .wrapping_shl((world_zone_2) as u32)
                                    ^ -1)),
                        ),
                    )?;
                }
                let world_alternate_map_5 =
                    r.get(world, field::world::ALTERNATE_MAP)?.i();
                if world_alternate_map_5 == 1 {
                    let player_world_flags_3 =
                        r.get(player, field::player::WORLD_FLAGS)?.i();
                    let world_zone_4 = r.get(world, field::world::ZONE)?.i();
                    r.set(
                        player,
                        field::player::WORLD_FLAGS,
                        V::I(
                            (player_world_flags_3
                                | (1_i32).wrapping_shl((world_zone_4) as u32)),
                        ),
                    )?;
                }
                let world_alternate_map_6 =
                    r.get(world, field::world::ALTERNATE_MAP)?.i();
                r.set(
                    object_10,
                    field::map_object::STATE_C,
                    V::I(world_alternate_map_6),
                )?;
                r.set(world, field::world::SWITCH_COOLDOWN, V::I(15))?;
                let state_out_value_2 =
                    r.static_get(field::system::STATE_OUT).r();
                let object_value =
                    r.alloc("java/lang/StringBuffer", Native::Empty);
                host::string_buffer_initialize(r, object_value)?;
                let text_value_4 = r.intern("p.WorldStates= ");
                let string_buffer_append_2_value =
                    host::string_buffer_append_2(
                        r,
                        object_value,
                        text_value_4,
                    )?;
                let player_world_flags_2 =
                    r.get(player, field::player::WORLD_FLAGS)?.i();
                let string_buffer_append_value = host::string_buffer_append(
                    r,
                    string_buffer_append_2_value,
                    player_world_flags_2,
                )?;
                let string_buffer_to_string_value =
                    host::string_buffer_to_string(
                        r,
                        string_buffer_append_value,
                    )?;
                host::print_stream_println(
                    r,
                    state_out_value_2,
                    string_buffer_to_string_value,
                )?;
            }
        }
        } // next_step_4

        if attacking == 0 {
            return Ok(number_4);
        }
        let map_object_kind_6 = r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_6 == 3 {
            break 'check_map_object_state_x;
        }
        } // check_map_object_kind
        let map_object_kind_8 = r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_8 == 4 {
            break 'check_map_object_state_x;
        }
        } // check_map_object_kind_2
        let map_object_kind_10 = r.get(object_10, field::map_object::KIND)?.i();
        if map_object_kind_10 != 15 {
            break 'advance_index_2;
        }
        } // check_map_object_state_x
        let map_object_state_x = r.get(object_10, field::map_object::STATE_X)?.i();
        if map_object_state_x != 0 {
            let map_object_state_a = r.get(object_10, field::map_object::STATE_A)?.i();
            r.set(
                object_10,
                field::map_object::STATE_A,
                V::I(map_object_state_a.wrapping_sub(1)),
            )?;
            let map_object_state_a_2 = r.get(object_10, field::map_object::STATE_A)?.i();
            if map_object_state_a_2 <= 0 {
                let drawable_x_8 = r.get(object_10, field::drawable::X)?.i();
                let drawable_y_8 = r.get(object_10, field::drawable::Y)?.i();
                let map_object_state_i = r.get(object_10, field::map_object::STATE_I)?.i();
                super::world::spawn_drops(
                    r,
                    world,
                    drawable_x_8,
                    drawable_y_8,
                    player,
                    map_object_state_i,
                )?;
                let world_objects_3 = r.get(world, field::world::OBJECTS)?.r();
                host::vector_remove_element_at(r, world_objects_3, number_9)?;
            }
        }
        } // advance_index_2
        number_9 = number_9.wrapping_add(1);
        continue 'interact_loop_39;
    }
}

/// Update contacts. Original reference: `j.a(Lh;Lb;)I`.
#[rustfmt::skip]
pub(in crate::game) fn update_contacts(
    r: &mut Runtime,
    world: u32,
    player: u32,
    sound: u32,
) -> Result<i32, u32> {
    let mut number_3: i32 = 0;
    let mut number_4: i32 = 0;
    let mut number_5: i32 = 0;
    let mut number_6: i32 = 0;
    let mut object_7: u32 = 0;
    let mut number_8: i32 = 0;
    let mut number_9: i32 = 0;
    let mut object_10: u32 = 0;
    let mut number_11: i32 = 0;
    let mut number_12: i32 = 0;
    let mut object_13: u32 = 0;
    let mut number_14: i32 = 0;
    'check_world_objects: {
    'next_step_2: {
    'check_world_items: {
    number_3 = 0;
    let drawable_x = r.get(player, field::drawable::X)?.i();
    number_4 = drawable_x;
    let drawable_y = r.get(player, field::drawable::Y)?.i();
    number_5 = drawable_y;
    number_4 = number_4.wrapping_add(12);
    number_5 = number_5.wrapping_add(16);
    let player_hurt_timer = r.get(player, field::player::HURT_TIMER)?.i();
    if player_hurt_timer > 0 {
        let player_hurt_timer_3 = r.get(player, field::player::HURT_TIMER)?.i();
        r.set(
            player,
            field::player::HURT_TIMER,
            V::I(player_hurt_timer_3.wrapping_sub(1)),
        )?;
    }
    let player_hurt_timer_2 = r.get(player, field::player::HURT_TIMER)?.i();
    if player_hurt_timer_2 < 0 {
        r.set(player, field::player::HURT_TIMER, V::I(0))?;
    }
    number_6 = 0;
    } // check_world_items
    'update_contacts_loop_2: loop {
        'advance_index: {
        'next_step_5: {
        'check_world_enemies: {
        'next_step_4: {
        'check_world_objects_2: {
        'next_step_3: {
        'check_item_explosion_timer: {
        'check_item_explosion_timer_2: {
        'check_item_lifetime_2: {
        'check_item_lifetime: {
        'check_item_armed: {
        'check_world_items_2: {
        let items_value = r
            .get(
                world,
                field::world::ITEMS,
            )?
            .r();
        let vector_size_value =
            host::vector_size(
                r,
                items_value,
            )?;
        if number_6 >= vector_size_value {
            break 'next_step_2;
        }
        } // check_world_items_2
        let items_value_2 = r
            .get(world, field::world::ITEMS)?
            .r();
        let vector_element_at_value =
            host::vector_element_at(
                r,
                items_value_2,
                number_6,
            )?;
        host::check_cast(
            r,
            vector_element_at_value,
            "c",
        )?;
        object_7 = vector_element_at_value;
        let x_value_2 = r
            .get(
                vector_element_at_value,
                field::drawable::X,
            )?
            .i();
        let half_width_value = r
            .get(
                object_7,
                field::drawable::HALF_WIDTH,
            )?
            .i();
        if !((x_value_2)
            .wrapping_add(half_width_value)
            > number_4.wrapping_add(3))
        {
            let x_value_3 = r
                .get(
                    object_7,
                    field::drawable::X,
                )?
                .i();
            let half_width_value_2 = r
                .get(
                    object_7,
                    field::drawable::HALF_WIDTH,
                )?
                .i();
            if !(x_value_3.wrapping_add(
                half_width_value_2,
            ) < (number_4)
                .wrapping_sub(10))
            {
                let y_value_2 = r
                    .get(
                        object_7,
                        field::drawable::Y,
                    )?
                    .i();
                let drawable_half_height = r.get(object_7, field::drawable::HALF_HEIGHT)?.i();
                if !(y_value_2.wrapping_add(
                    drawable_half_height,
                ) > number_5)
                {
                    let y_value_3 = r
                        .get(
                            object_7,
                            field::drawable::Y,
                        )?
                        .i();
                    let drawable_half_height_2 = r.get(object_7, field::drawable::HALF_HEIGHT)?.i();
                    if !((y_value_3)
                        .wrapping_add(
                            drawable_half_height_2,
                        )
                        < (number_5)
                            .wrapping_sub(16))
                    {
                        let world_canvas = r.get(world, field::world::CANVAS)?.r();
                        let collect_value = super::item::collect(r, object_7, player, sound, world_canvas, 0)?;
                        number_8 =
                            collect_value;
                        if collect_value != 1 {
                            if number_8 == 2 {
                                number_3 = 2;
                                let world_canvas_3 = r.get(world, field::world::CANVAS)?.r();
                                r.set(world_canvas_3, field::canvas::REDRAW_WORLD, V::I(1))?;
                            }
                        } else {
                            let world_items_3 = r.get(world, field::world::ITEMS)?.r();
                            host::vector_remove_element_at(r, world_items_3, number_6)?;
                            number_3 = 1;
                            let world_canvas_2 = r.get(world, field::world::CANVAS)?.r();
                            r.set(world_canvas_2, field::canvas::REDRAW_WORLD, V::I(1))?;
                        }
                    }
                }
            }
        }
        let kind_value = r
            .get(object_7, field::item::KIND)?
            .i();
        if kind_value != 4 {
            break 'advance_index;
        }
        } // check_item_armed
        let armed_value = r
            .get(object_7, field::item::ARMED)?
            .i();
        if armed_value == 0 {
            break 'advance_index;
        }
        } // check_item_lifetime
        let item_lifetime =
            r.get(object_7, field::item::LIFETIME)?.i();
        r.set(
            object_7,
            field::item::LIFETIME,
            V::I(item_lifetime.wrapping_sub(1)),
        )?;
        let item_lifetime_2 =
            r.get(object_7, field::item::LIFETIME)?.i();
        if item_lifetime_2 > 0 {
            break 'advance_index;
        }
        } // check_item_lifetime_2
        let item_lifetime_3 =
            r.get(object_7, field::item::LIFETIME)?.i();
        if item_lifetime_3 != 0 {
            break 'check_item_explosion_timer;
        }
        } // check_item_explosion_timer_2
        r.set(
            object_7,
            field::item::EXPLOSION_TIMER,
            V::I(10),
        )?;
        number_8 = 4;
        object_7 = sound;
        r.set(
            sound,
            field::sound::PENDING_EFFECT,
            V::I(number_8),
        )?;
        break 'advance_index;
        } // check_item_explosion_timer
        let item_explosion_timer =
            r.get(object_7, field::item::EXPLOSION_TIMER)?.i();
        r.set(
            object_7,
            field::item::EXPLOSION_TIMER,
            V::I(item_explosion_timer.wrapping_sub(1)),
        )?;
        let item_explosion_timer_2 =
            r.get(object_7, field::item::EXPLOSION_TIMER)?.i();
        if item_explosion_timer_2 != 0 {
            break 'advance_index;
        }
        } // next_step_3
        number_8 = 0;
        let world_items_4 = r.get(world, field::world::ITEMS)?.r();
        host::vector_remove_element_at(r, world_items_4, number_6)?;
        let drawable_x_8 = r.get(object_7, field::drawable::X)?.i();
        let drawable_y_7 = r.get(object_7, field::drawable::Y)?.i();
        object_10 = player;
        number_9 = drawable_y_7;
        number_8 = drawable_x_8;
        object_7 = world;
        number_11 = 0;
        number_12 = 0;
        } // check_world_objects_2
        'update_contacts_loop_10: loop {
            'advance_index_3: {
            'check_world_objects_4: {
            'check_map_object_state_x_2: {
            'check_map_object_kind_6: {
            'check_map_object_kind_5: {
            'check_map_object_kind_4: {
            'check_map_object_state_n: {
            'check_drawable_y_4: {
            'check_drawable_y_3: {
            'check_drawable_x_2: {
            'check_world_objects_3: {
            let world_objects_4 = r.get(object_7, field::world::OBJECTS)?.r();
            let vector_size_value_4 = host::vector_size(r, world_objects_4)?;
            if number_12 >= vector_size_value_4 { break 'next_step_4; }
            } // check_world_objects_3
            let world_objects_5 = r.get(object_7, field::world::OBJECTS)?.r();
            let vector_element_at_value_4 = host::vector_element_at(r, world_objects_5, number_12)?;
            host::check_cast(r, vector_element_at_value_4, "n")?;
            object_13 = vector_element_at_value_4;
            let drawable_x_10 = r.get(vector_element_at_value_4, field::drawable::X)?.i();
            if drawable_x_10
                > (number_8)
                    .wrapping_add(
                        24,
                    )
            {
                break 'advance_index_3;
            }
            } // check_drawable_x_2
            let x_value_11 = r
                .get(
                    object_13,
                    field::drawable::X,
                )?
                .i();
            let map_object_width_3 = r.get(object_13, field::map_object::WIDTH)?.i();
            if (x_value_11)
                .wrapping_add(
                    map_object_width_3,
                )
                < (number_8)
                    .wrapping_sub(24)
            {
                break 'advance_index_3;
            }
            } // check_drawable_y_3
            let y_value_10 = r
                .get(
                    object_13,
                    field::drawable::Y,
                )?
                .i();
            if y_value_10
                > (number_9)
                    .wrapping_add(24)
            {
                break 'advance_index_3;
            }
            } // check_drawable_y_4
            let y_value_11 = r
                .get(
                    object_13,
                    field::drawable::Y,
                )?
                .i();
            let height_value_3 = r
                .get(
                    object_13,
                    field::map_object::HEIGHT,
                )?
                .i();
            if (y_value_11)
                .wrapping_add(height_value_3)
                < number_9.wrapping_sub(24)
            {
                break 'advance_index_3;
            }
            } // check_map_object_state_n
            let state_n_value = r
                .get(
                    object_13,
                    field::map_object::STATE_N,
                )?
                .i();
            if state_n_value != 0 {
                let map_value = r
                    .get(
                        object_7,
                        field::world::MAP,
                    )?
                    .i();
                let quotient_value_2 =
                    arithmetic::divide_int(
                        r, map_value, 32,
                    )?;
                number_11 = quotient_value_2;
                let map_object_persistence_id = r.get(object_13, field::map_object::PERSISTENCE_ID)?.i();
                let remainder_value =
                    arithmetic::remainder_int(
                        r,
                        map_object_persistence_id,
                        32,
                    )?;
                number_14 = remainder_value;
                let zone_value = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value == 0 {
                    let player_overworld_events = r.get(object_10, field::player::OVERWORLD_EVENTS)?.r();
                    let element_value = r
                        .array_get(
                            player_overworld_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_overworld_events,
                        number_11,
                        V::I(
                            (element_value
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let zone_value_2 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_2 == 1 {
                    let player_cave_events = r.get(object_10, field::player::CAVE_EVENTS)?.r();
                    let element_value_2 = r
                        .array_get(
                            player_cave_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_cave_events,
                        number_11,
                        V::I(
                            (element_value_2
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let zone_value_3 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_3 == 3 {
                    let player_dungeon_1_events = r.get(object_10, field::player::DUNGEON_1_EVENTS)?.r();
                    let element_value_3 = r
                        .array_get(
                            player_dungeon_1_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_1_events,
                        number_11,
                        V::I(
                            (element_value_3
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                    let state_out_value = r.static_get(field::system::STATE_OUT).r();
                    let text_value =
                        r.intern("Curent worl set");
                    host::print_stream_println(
                        r,
                        state_out_value,
                        text_value,
                    )?;
                }
                let zone_value_4 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_4 == 4 {
                    let player_dungeon_2_events = r.get(object_10, field::player::DUNGEON_2_EVENTS)?.r();
                    let element_value_4 = r
                        .array_get(
                            player_dungeon_2_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_2_events,
                        number_11,
                        V::I(
                            (element_value_4
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let zone_value_5 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_5 == 5 {
                    let player_dungeon_3_events = r.get(object_10, field::player::DUNGEON_3_EVENTS)?.r();
                    let element_value_5 = r
                        .array_get(
                            player_dungeon_3_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_3_events,
                        number_11,
                        V::I(
                            (element_value_5
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let zone_value_6 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_6 == 6 {
                    let player_dungeon_4_events = r.get(object_10, field::player::DUNGEON_4_EVENTS)?.r();
                    let element_value_6 = r
                        .array_get(
                            player_dungeon_4_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_4_events,
                        number_11,
                        V::I(
                            (element_value_6
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                    let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
                    let text_value_2 =
                        r.intern("Curent worl set");
                    host::print_stream_println(
                        r,
                        state_out_value_2,
                        text_value_2,
                    )?;
                }
                let zone_value_7 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_7 == 7 {
                    let player_dungeon_5_events = r.get(object_10, field::player::DUNGEON_5_EVENTS)?.r();
                    let element_value_7 = r
                        .array_get(
                            player_dungeon_5_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_5_events,
                        number_11,
                        V::I(
                            (element_value_7
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let zone_value_8 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_8 == 8 {
                    let player_dungeon_6_events = r.get(object_10, field::player::DUNGEON_6_EVENTS)?.r();
                    let element_value_8 = r
                        .array_get(
                            player_dungeon_6_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_6_events,
                        number_11,
                        V::I(
                            (element_value_8
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let zone_value_9 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_9 == 9 {
                    let player_dungeon_7_events = r.get(object_10, field::player::DUNGEON_7_EVENTS)?.r();
                    let element_value_9 = r
                        .array_get(
                            player_dungeon_7_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_7_events,
                        number_11,
                        V::I(
                            (element_value_9
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let zone_value_10 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_10 == 10 {
                    let player_dungeon_8_events = r.get(object_10, field::player::DUNGEON_8_EVENTS)?.r();
                    let element_value_10 = r
                        .array_get(
                            player_dungeon_8_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_8_events,
                        number_11,
                        V::I(
                            (element_value_10
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let zone_value_11 = r
                    .get(
                        object_7,
                        field::world::ZONE,
                    )?
                    .i();
                if zone_value_11 == 11 {
                    let player_dungeon_9_events = r.get(object_10, field::player::DUNGEON_9_EVENTS)?.r();
                    let element_value_11 = r
                        .array_get(
                            player_dungeon_9_events,
                            number_11,
                        )?
                        .i();
                    r.array_set(
                        player_dungeon_9_events,
                        number_11,
                        V::I(
                            (element_value_11
                                | (1_i32)
                                    .wrapping_shl(
                                        (number_14)
                                            as u32,
                                    )),
                        ),
                    )?;
                }
                let objects_value_7 = r
                    .get(
                        object_7,
                        field::world::OBJECTS,
                    )?
                    .r();
                host::vector_remove_element_at(
                    r,
                    objects_value_7,
                    number_12,
                )?;
                number_11 = 1;
            }
            let state_g_value = r
                .get(
                    object_13,
                    field::map_object::STATE_G,
                )?
                .i();
            if state_g_value == 0 {
                break 'advance_index_3;
            }
            } // check_map_object_kind_4
            let kind_value_7 = r
                .get(
                    object_13,
                    field::map_object::KIND,
                )?
                .i();
            if kind_value_7 == 3 {
                break 'check_map_object_state_x_2;
            }
            } // check_map_object_kind_5
            let kind_value_8 = r
                .get(object_13, field::map_object::KIND)?
                .i();
            if kind_value_8 == 4 {
                break 'check_map_object_state_x_2;
            }
            } // check_map_object_kind_6
            let map_object_kind_9 =
                r.get(object_13, field::map_object::KIND)?.i();
            if map_object_kind_9 != 15 {
                break 'check_world_objects_4;
            }
            } // check_map_object_state_x_2
            let map_object_state_x_2 =
                r.get(object_13, field::map_object::STATE_X)?.i();
            if map_object_state_x_2 == 0 {
                r.set(
                    object_13,
                    field::map_object::STATE_X,
                    V::I(1),
                )?;
                r.set(
                    object_13,
                    field::map_object::STATE_A,
                    V::I(6),
                )?;
            }
            break 'advance_index_3;
            } // check_world_objects_4
            let world_objects_6 =
                r.get(object_7, field::world::OBJECTS)?.r();
            host::vector_remove_element_at(
                r,
                world_objects_6,
                number_12,
            )?;
            let drawable_x_28 = r.get(object_13, field::drawable::X)?.i();
            let drawable_y_25 = r.get(object_13, field::drawable::Y)?.i();
            let map_object_state_i_2 =
                r.get(object_13, field::map_object::STATE_I)?.i();
            super::world::spawn_drops(
                r,
                object_7,
                drawable_x_28,
                drawable_y_25,
                object_10,
                map_object_state_i_2,
            )?;
            } // advance_index_3
            number_12 = number_12.wrapping_add(1);
            continue 'update_contacts_loop_10;
        }
        } // next_step_4
        number_12 = 0;
        } // check_world_enemies
        'update_contacts_loop_23: loop {
            'advance_index_4: {
            'check_enemy_state_f_lower: {
            'check_enemy_state_z_2: {
            'check_enemy_state_z: {
            'check_enemy_kind: {
            'check_world_enemies_2: {
            let world_enemies_3 =
                r.get(object_7, field::world::ENEMIES)?.r();
            let vector_size_value_5 =
                host::vector_size(r, world_enemies_3)?;
            if number_12 >= vector_size_value_5 {
                break 'next_step_5;
            }
            } // check_world_enemies_2
            let world_enemies_4 =
                r.get(object_7, field::world::ENEMIES)?.r();
            let vector_element_at_value_5 =
                host::vector_element_at(
                    r,
                    world_enemies_4,
                    number_12,
                )?;
            host::check_cast(
                r,
                vector_element_at_value_5,
                "l",
            )?;
            object_13 = vector_element_at_value_5;
            let active_value = r
                .get(
                    vector_element_at_value_5,
                    field::enemy::ACTIVE,
                )?
                .i();
            if active_value != 0 {
                break 'check_enemy_state_f_lower;
            }
            } // check_enemy_kind
            let enemy_kind_5 =
                r.get(object_13, field::enemy::KIND)?.i();
            if enemy_kind_5 != 29 {
                break 'advance_index_4;
            }
            } // check_enemy_state_z
            let enemy_behavior_state =
                r.get(object_13, field::enemy::BEHAVIOR_STATE)?.i();
            if enemy_behavior_state != 8 {
                break 'check_enemy_state_f_lower;
            }
            } // check_enemy_state_z_2
            r.set(object_13, field::enemy::BEHAVIOR_STATE, V::I(9))?;
            r.set(object_13, field::enemy::STATE_TIMER, V::I(0))?;
            break 'advance_index_4;
            } // check_enemy_state_f_lower
            let enemy_dying_2 =
                r.get(object_13, field::enemy::DYING)?.i();
            if enemy_dying_2 == 0 {
                let drawable_x_12 = r.get(object_13, field::drawable::X)?.i();
                if !(drawable_x_12 > number_8.wrapping_add(24)) {
                    let drawable_x_14 = r.get(object_13, field::drawable::X)?.i();
                    let enemy_width_4 =
                        r.get(object_13, field::enemy::WIDTH)?.i();
                    if !(drawable_x_14.wrapping_add(enemy_width_4)
                        < number_8.wrapping_sub(24))
                    {
                        let drawable_y_15 =
                            r.get(object_13, field::drawable::Y)?.i();
                        if !(drawable_y_15 > number_9.wrapping_add(24)) {
                            let drawable_y_19 =
                                r.get(object_13, field::drawable::Y)?.i();
                            let enemy_height_4 =
                                r.get(object_13, field::enemy::HEIGHT)?.i();
                            if !(drawable_y_19.wrapping_add(enemy_height_4)
                                < number_9.wrapping_sub(24))
                            {
                                let state_t_lower_value = r
                                    .get(object_13, field::enemy::HURT_TIMER)?
                                    .i();
                                if state_t_lower_value <= 0 {
                                    let state_n_value_2 = r
                                        .get(object_13, field::enemy::HEALTH)?
                                        .i();
                                    r.set(
                                        object_13,
                                        field::enemy::HEALTH,
                                        V::I(state_n_value_2.wrapping_sub(2)),
                                    )?;
                                    let state_n_value_3 = r
                                        .get(object_13, field::enemy::HEALTH)?
                                        .i();
                                    if state_n_value_3 > 0 {
                                        r.set(
                                            object_13,
                                            field::enemy::HURT_TIMER,
                                            V::I(5),
                                        )?;
                                        let kind_value_10 = r
                                            .get(object_13, field::enemy::KIND)?
                                            .i();
                                        if kind_value_10 != 20 {
                                            let x_value_31 = r
                                                .get(
                                                    object_13,
                                                    field::drawable::X,
                                                )?
                                                .i();
                                            if x_value_31 < number_8 {
                                                r.set(
                                                    object_13,
                                                    field::enemy::VELOCITY_Y,
                                                    V::I(-10),
                                                )?;
                                                r.set(
                                                    object_13,
                                                    field::enemy::VELOCITY_X,
                                                    V::I(0),
                                                )?;
                                            }
                                            let y_value_26 = r
                                                .get(
                                                    object_13,
                                                    field::drawable::Y,
                                                )?
                                                .i();
                                            if y_value_26 > number_9 {
                                                r.set(
                                                    object_13,
                                                    field::enemy::VELOCITY_X,
                                                    V::I(10),
                                                )?;
                                                r.set(
                                                    object_13,
                                                    field::enemy::VELOCITY_Y,
                                                    V::I(0),
                                                )?;
                                            }
                                            let x_value_32 = r
                                                .get(
                                                    object_13,
                                                    field::drawable::X,
                                                )?
                                                .i();
                                            if x_value_32 > number_8 {
                                                r.set(
                                                    object_13,
                                                    field::enemy::VELOCITY_X,
                                                    V::I(0),
                                                )?;
                                                r.set(
                                                    object_13,
                                                    field::enemy::VELOCITY_Y,
                                                    V::I(10),
                                                )?;
                                            }
                                            let y_value_27 = r
                                                .get(
                                                    object_13,
                                                    field::drawable::Y,
                                                )?
                                                .i();
                                            if y_value_27 < number_9 {
                                                r.set(
                                                    object_13,
                                                    field::enemy::VELOCITY_X,
                                                    V::I(-10),
                                                )?;
                                                r.set(
                                                    object_13,
                                                    field::enemy::VELOCITY_Y,
                                                    V::I(0),
                                                )?;
                                            }
                                        }
                                    } else {
                                        r.set(
                                            object_13,
                                            field::enemy::DEATH_FRAME,
                                            V::I(0),
                                        )?;
                                        r.set(
                                            object_13,
                                            field::enemy::DYING,
                                            V::I(1),
                                        )?;
                                        let state_w_value = r
                                            .get(
                                                object_7,
                                                field::world::DEATH_IMAGE,
                                            )?
                                            .r();
                                        r.set(
                                            object_13,
                                            field::enemy::IMAGE,
                                            V::R(state_w_value),
                                        )?;
                                        r.set(
                                            object_13,
                                            field::enemy::ANIMATION,
                                            V::I(0),
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            } // advance_index_4
            number_12 = number_12.wrapping_add(1);
            continue 'update_contacts_loop_23;
        }
        } // next_step_5
        number_8 = number_11;
        if number_11 != 0 {
            number_8 = 7;
            object_7 = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_8))?;
        }
        } // advance_index
        number_6 = number_6.wrapping_add(1);
        continue 'update_contacts_loop_2;
    }
    } // next_step_2
    number_6 = 0;
    } // check_world_objects
    'update_contacts_loop_33: loop {
        'advance_index_2: {
        'check_map_object_state_x: {
        'check_map_object_kind_3: {
        'check_map_object_kind_2: {
        'check_map_object_kind: {
        'check_world_canvas: {
        'check_map_object_has_dialogue: {
        'check_drawable_y_2: {
        'check_drawable_y: {
        'check_drawable_x: {
        let world_objects =
            r.get(world, field::world::OBJECTS)?.r();
        let vector_size_value_2 =
            host::vector_size(r, world_objects)?;
        if number_6 >= vector_size_value_2 {
            number_6 = 0;
            loop {
                let enemies_value = r
                    .get(world, field::world::ENEMIES)?
                    .r();
                let vector_size_value_3 =
                    host::vector_size(r, enemies_value)?;
                if number_6 >= vector_size_value_3 {
                    return Ok(number_3);
                }
                let enemies_value_2 = r
                    .get(world, field::world::ENEMIES)?
                    .r();
                let vector_element_at_value_3 =
                    host::vector_element_at(
                        r,
                        enemies_value_2,
                        number_6,
                    )?;
                host::check_cast(
                    r,
                    vector_element_at_value_3,
                    "l",
                )?;
                object_7 = vector_element_at_value_3;
                let dead_value = r
                    .get(
                        vector_element_at_value_3,
                        field::enemy::DEAD,
                    )?
                    .i();
                if dead_value == 0 {
                    let state_f_lower_value = r
                        .get(object_7, field::enemy::DYING)?
                        .i();
                    if state_f_lower_value == 0 {
                        let x_value_6 = r
                            .get(
                                object_7,
                                field::drawable::X,
                            )?
                            .i();
                        if x_value_6 <= number_4 {
                            let x_value_9 = r
                                .get(
                                    object_7,
                                    field::drawable::X,
                                )?
                                .i();
                            let width_value_2 = r
                                .get(
                                    object_7,
                                    field::enemy::WIDTH,
                                )?
                                .i();
                            if !(x_value_9.wrapping_add(
                                width_value_2,
                            ) < number_4)
                            {
                                let y_value_8 = r
                                    .get(
                                        object_7,
                                        field::drawable::Y,
                                    )?
                                    .i();
                                if y_value_8 <= number_5 {
                                    let drawable_y_9 = r.get(object_7, field::drawable::Y)?.i();
                                    let enemy_height_2 = r.get(object_7, field::enemy::HEIGHT)?.i();
                                    if !((drawable_y_9)
                                        .wrapping_add(
                                            enemy_height_2,
                                        )
                                        < number_5)
                                    {
                                        let player_hurt_timer_4 = r.get(player, field::player::HURT_TIMER)?.i();
                                        if player_hurt_timer_4 <= 0 {
let player_health = r.get(player, field::player::HEALTH)?.i();
let enemy_state_o = r.get(object_7, field::enemy::STATE_O)?.i();
let player_defense = r.get(player, field::player::DEFENSE)?.i();
let quotient_value = arithmetic::divide_int(r, enemy_state_o, player_defense)?;
r.set(player, field::player::HEALTH, V::I(player_health.wrapping_sub(quotient_value)))?;
let player_health_2 = r.get(player, field::player::HEALTH)?.i();
if player_health_2 < 0 {
r.set(player, field::player::HEALTH, V::I(0))?;
}
let enemy_kind_6 = r.get(object_7, field::enemy::KIND)?.i();
if enemy_kind_6 == 21 {
let drawable_x_13 = r.get(player, field::drawable::X)?.i();
let drawable_y_12 = r.get(player, field::drawable::Y)?.i();
let player_facing = r.get(player, field::player::FACING)?.i();
let can_player_move_value = super::world::can_player_move(r, world, drawable_x_13.wrapping_add(50), drawable_y_12, player_facing, player, 1)?;
if can_player_move_value == 0 {
let drawable_x_15 = r.get(player, field::drawable::X)?.i();
let drawable_y_13 = r.get(player, field::drawable::Y)?.i();
let player_facing_3 = r.get(player, field::player::FACING)?.i();
let can_player_move_value_2 = super::world::can_player_move(r, world, drawable_x_15.wrapping_add(40), drawable_y_13, player_facing_3, player, 1)?;
if can_player_move_value_2 == 0 {
let drawable_x_18 = r.get(player, field::drawable::X)?.i();
let drawable_y_16 = r.get(player, field::drawable::Y)?.i();
let player_facing_6 = r.get(player, field::player::FACING)?.i();
let can_player_move_value_4 = super::world::can_player_move(r, world, drawable_x_18.wrapping_add(30), drawable_y_16, player_facing_6, player, 1)?;
if can_player_move_value_4 == 0 {
let drawable_x_21 = r.get(player, field::drawable::X)?.i();
let drawable_y_20 = r.get(player, field::drawable::Y)?.i();
let player_facing_9 = r.get(player, field::player::FACING)?.i();
let can_player_move_value_6 = super::world::can_player_move(r, world, drawable_x_21.wrapping_add(20), drawable_y_20, player_facing_9, player, 1)?;
if can_player_move_value_6 == 0 {
let drawable_x_25 = r.get(player, field::drawable::X)?.i();
let drawable_y_22 = r.get(player, field::drawable::Y)?.i();
let player_facing_12 = r.get(player, field::player::FACING)?.i();
let can_player_move_value_8 = super::world::can_player_move(r, world, drawable_x_25.wrapping_add(10), drawable_y_22, player_facing_12, player, 1)?;
if can_player_move_value_8 != 0 {
let drawable_x_29 = r.get(player, field::drawable::X)?.i();
r.set(player, field::drawable::X, V::I(drawable_x_29.wrapping_add(10)))?;
}
} else {
let drawable_x_26 = r.get(player, field::drawable::X)?.i();
r.set(player, field::drawable::X, V::I(drawable_x_26.wrapping_add(20)))?;
}
} else {
let drawable_x_22 = r.get(player, field::drawable::X)?.i();
r.set(player, field::drawable::X, V::I(drawable_x_22.wrapping_add(30)))?;
}
} else {
let drawable_x_19 = r.get(player, field::drawable::X)?.i();
r.set(player, field::drawable::X, V::I(drawable_x_19.wrapping_add(40)))?;
}
} else {
let drawable_x_16 = r.get(player, field::drawable::X)?.i();
r.set(player, field::drawable::X, V::I(drawable_x_16.wrapping_add(50)))?;
}
} else {
let player_facing_2 = r.get(player, field::player::FACING)?.i();
if player_facing_2 == 0 {
let drawable_x_17 = r.get(player, field::drawable::X)?.i();
let drawable_y_14 = r.get(player, field::drawable::Y)?.i();
let player_facing_5 = r.get(player, field::player::FACING)?.i();
let can_player_move_value_3 = super::world::can_player_move(r, world, drawable_x_17, drawable_y_14.wrapping_add(10), player_facing_5, player, 1)?;
if can_player_move_value_3 != 0 {
let drawable_y_18 = r.get(player, field::drawable::Y)?.i();
r.set(player, field::drawable::Y, V::I(drawable_y_18.wrapping_add(10)))?;
}
}
let player_facing_4 = r.get(player, field::player::FACING)?.i();
if player_facing_4 == 1 {
let drawable_x_20 = r.get(player, field::drawable::X)?.i();
let drawable_y_17 = r.get(player, field::drawable::Y)?.i();
let player_facing_8 = r.get(player, field::player::FACING)?.i();
let can_player_move_value_5 = super::world::can_player_move(r, world, drawable_x_20.wrapping_sub(10), drawable_y_17, player_facing_8, player, 1)?;
if can_player_move_value_5 != 0 {
let drawable_x_24 = r.get(player, field::drawable::X)?.i();
r.set(player, field::drawable::X, V::I(drawable_x_24.wrapping_sub(10)))?;
}
}
let player_facing_7 = r.get(player, field::player::FACING)?.i();
if player_facing_7 == 2 {
let drawable_x_23 = r.get(player, field::drawable::X)?.i();
let drawable_y_21 = r.get(player, field::drawable::Y)?.i();
let player_facing_11 = r.get(player, field::player::FACING)?.i();
let can_player_move_value_7 = super::world::can_player_move(r, world, drawable_x_23, drawable_y_21.wrapping_sub(10), player_facing_11, player, 1)?;
if can_player_move_value_7 != 0 {
let drawable_y_24 = r.get(player, field::drawable::Y)?.i();
r.set(player, field::drawable::Y, V::I(drawable_y_24.wrapping_sub(10)))?;
}
}
let player_facing_10 = r.get(player, field::player::FACING)?.i();
if player_facing_10 == 3 {
let drawable_x_27 = r.get(player, field::drawable::X)?.i();
let drawable_y_23 = r.get(player, field::drawable::Y)?.i();
let player_facing_13 = r.get(player, field::player::FACING)?.i();
let can_player_move_value_9 = super::world::can_player_move(r, world, drawable_x_27.wrapping_add(10), drawable_y_23, player_facing_13, player, 1)?;
if can_player_move_value_9 != 0 {
let drawable_x_30 = r.get(player, field::drawable::X)?.i();
r.set(player, field::drawable::X, V::I(drawable_x_30.wrapping_add(10)))?;
}
}
}
number_3 = 1;
r.set(player, field::player::HURT_TIMER, V::I(20))?;
}
                                    }
                                }
                            }
                        }
                    }
                }
                number_6 = number_6.wrapping_add(1);
            }
        }
        let world_objects_2 =
            r.get(world, field::world::OBJECTS)?.r();
        let vector_element_at_value_2 =
            host::vector_element_at(
                r,
                world_objects_2,
                number_6,
            )?;
        host::check_cast(
            r,
            vector_element_at_value_2,
            "n",
        )?;
        object_7 = vector_element_at_value_2;
        let x_value_4 = r
            .get(
                vector_element_at_value_2,
                field::drawable::X,
            )?
            .i();
        if x_value_4 > number_4 {
            break 'check_map_object_kind;
        }
        } // check_drawable_x
        let drawable_x_5 =
            r.get(object_7, field::drawable::X)?.i();
        let map_object_width =
            r.get(object_7, field::map_object::WIDTH)?.i();
        let state_d_upper_value = r
            .get(object_7, field::map_object::STATE_D_UPPER)?
            .i();
        if (drawable_x_5.wrapping_add(map_object_width))
            .wrapping_add(state_d_upper_value)
            < number_4
        {
            break 'check_map_object_kind;
        }
        } // check_drawable_y
        let drawable_y_4 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_4 > number_5 {
            break 'check_map_object_kind;
        }
        } // check_drawable_y_2
        let drawable_y_5 = r.get(object_7, field::drawable::Y)?.i();
        let map_object_height =
            r.get(object_7, field::map_object::HEIGHT)?.i();
        let map_object_state_e_upper =
            r.get(object_7, field::map_object::STATE_E_UPPER)?.i();
        if (drawable_y_5.wrapping_add(map_object_height))
            .wrapping_add(map_object_state_e_upper)
            < number_5
        {
            break 'check_map_object_kind;
        }
        } // check_map_object_has_dialogue
        let map_object_has_dialogue =
            r.get(object_7, field::map_object::HAS_DIALOGUE)?.i();
        if map_object_has_dialogue != 1 {
            break 'check_map_object_kind;
        }
        } // check_world_canvas
        let world_canvas_4 = r.get(world, field::world::CANVAS)?.r();
        r.set(world_canvas_4, field::canvas::STATE_I, V::I(1))?;
        let world_canvas_5 = r.get(world, field::world::CANVAS)?.r();
        r.set(world_canvas_5, field::canvas::STATE_L, V::I(number_6))?;
        break 'advance_index_2;
        } // check_map_object_kind
        let map_object_kind_2 = r.get(object_7, field::map_object::KIND)?.i();
        if map_object_kind_2 == 3 {
            break 'check_map_object_state_x;
        }
        } // check_map_object_kind_2
        let map_object_kind_3 = r.get(object_7, field::map_object::KIND)?.i();
        if map_object_kind_3 == 4 {
            break 'check_map_object_state_x;
        }
        } // check_map_object_kind_3
        let map_object_kind_4 = r.get(object_7, field::map_object::KIND)?.i();
        if map_object_kind_4 != 15 {
            break 'advance_index_2;
        }
        } // check_map_object_state_x
        let map_object_state_x = r.get(object_7, field::map_object::STATE_X)?.i();
        if map_object_state_x != 0 {
            let map_object_state_a = r.get(object_7, field::map_object::STATE_A)?.i();
            r.set(
                object_7,
                field::map_object::STATE_A,
                V::I(map_object_state_a.wrapping_sub(1)),
            )?;
            let map_object_state_a_2 = r.get(object_7, field::map_object::STATE_A)?.i();
            if map_object_state_a_2 < 0 {
                let drawable_x_7 = r.get(object_7, field::drawable::X)?.i();
                let drawable_y_6 = r.get(object_7, field::drawable::Y)?.i();
                let map_object_state_i = r.get(object_7, field::map_object::STATE_I)?.i();
                super::world::spawn_drops(
                    r,
                    world,
                    drawable_x_7,
                    drawable_y_6,
                    player,
                    map_object_state_i,
                )?;
                let world_objects_3 = r.get(world, field::world::OBJECTS)?.r();
                host::vector_remove_element_at(r, world_objects_3, number_6)?;
            }
        }
        } // advance_index_2
        number_6 = number_6.wrapping_add(1);
        continue 'update_contacts_loop_33;
    }
}

/// Spawn drops. Original reference: `j.a(IILh;I)V`.
pub(in crate::game) fn spawn_drops(
    r: &mut Runtime,
    world: u32,
    x: i32,
    y: i32,
    player: u32,
    drop_kind: i32,
) -> Result<(), u32> {
    let mut object_1: u32 = 0;
    let mut object_4: u32 = 0;
    let mut number_5: i32 = 0;
    let mut number_6: i32 = 0;
    number_5 = 0;
    let world_random = r.get(world, field::world::RANDOM)?.r();
    let random_next_int_value = host::random_next_int(r, world_random)?;
    let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 4)?;
    let math_abs_value = host::math_abs(r, remainder_value)?;
    if math_abs_value == 3 {
        return Ok(());
    }
    number_6 = 0;
    match drop_kind {
        1 => {
            let world_random_2 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_2 = host::random_next_int(r, world_random_2)?;
            let remainder_value_2 = arithmetic::remainder_int(r, random_next_int_value_2, 2)?;
            let math_abs_value_2 = host::math_abs(r, remainder_value_2)?;
            number_6 = math_abs_value_2;
            if math_abs_value_2 > 0 {
                number_6 = 3;
            }
        }
        2 => {
            let world_random_3 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_3 = host::random_next_int(r, world_random_3)?;
            let remainder_value_3 = arithmetic::remainder_int(r, random_next_int_value_3, 5)?;
            let math_abs_value_3 = host::math_abs(r, remainder_value_3)?;
            number_6 = math_abs_value_3;
            if math_abs_value_3 == 2 {
                number_6 = 5;
            }
        }
        3 => {
            let world_random_4 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_4 = host::random_next_int(r, world_random_4)?;
            let remainder_value_4 = arithmetic::remainder_int(r, random_next_int_value_4, 6)?;
            let math_abs_value_4 = host::math_abs(r, remainder_value_4)?;
            number_6 = math_abs_value_4;
            if math_abs_value_4 == 0 {
                number_6 = 1;
            }
            let world_random_5 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_5 = host::random_next_int(r, world_random_5)?;
            let remainder_value_5 = arithmetic::remainder_int(r, random_next_int_value_5, 6)?;
            let math_abs_value_5 = host::math_abs(r, remainder_value_5)?;
            if math_abs_value_5 == 2 {
                number_5 = 1;
            }
        }
        _ => {
            return Ok(());
        }
    }

    if number_6 == 6 {
        return Ok(());
    }

    if number_6 == 4 {
        let player_has_bombs = r.get(player, field::player::HAS_BOMBS)?.i();
        if player_has_bombs == 0 {
            number_6 = 1;
        }
    }

    if number_6 == 5 {
        let player_has_bow = r.get(player, field::player::HAS_BOW)?.i();
        if player_has_bow == 0 {
            number_6 = 1;
        }
    }
    let item_value = r.alloc("c", Native::Empty);
    let world_resources = r.get(world, field::world::RESOURCES)?.r();
    let item_image_value = super::resources::item_image(r, world_resources, number_6)?;
    super::item::initialize(
        r,
        item_value,
        item_image_value,
        number_6,
        x,
        y.wrapping_sub(32),
        0,
        player,
    )?;
    object_4 = item_value;
    let world_items = r.get(world, field::world::ITEMS)?.r();
    host::vector_add_element(r, world_items, object_4)?;
    if number_5 == 0 {
        return Ok(());
    }
    let item_value_2 = r.alloc("c", Native::Empty);
    let world_resources_2 = r.get(world, field::world::RESOURCES)?.r();
    let item_image_value_2 = super::resources::item_image(r, world_resources_2, number_6)?;
    super::item::initialize(
        r,
        item_value_2,
        item_image_value_2,
        number_6,
        x.wrapping_sub(16),
        y.wrapping_sub(32),
        0,
        player,
    )?;
    object_4 = item_value_2;
    let world_items_2 = r.get(world, field::world::ITEMS)?.r();
    host::vector_add_element(r, world_items_2, object_4)?;
    let item_value_3 = r.alloc("c", Native::Empty);
    let world_resources_3 = r.get(world, field::world::RESOURCES)?.r();
    let item_image_value_3 = super::resources::item_image(r, world_resources_3, number_6)?;
    super::item::initialize(
        r,
        item_value_3,
        item_image_value_3,
        number_6,
        x.wrapping_add(16),
        y.wrapping_sub(32),
        0,
        player,
    )?;
    object_4 = item_value_3;
    let world_items_3 = r.get(world, field::world::ITEMS)?.r();
    host::vector_add_element(r, world_items_3, object_4)?;
    let item_value_4 = r.alloc("c", Native::Empty);
    let world_resources_4 = r.get(world, field::world::RESOURCES)?.r();
    let item_image_value_4 = super::resources::item_image(r, world_resources_4, number_6)?;
    super::item::initialize(
        r,
        item_value_4,
        item_image_value_4,
        number_6,
        x,
        y.wrapping_sub(16),
        0,
        player,
    )?;
    object_4 = item_value_4;
    let world_items_4 = r.get(world, field::world::ITEMS)?.r();
    host::vector_add_element(r, world_items_4, object_4)?;
    let item_value_5 = r.alloc("c", Native::Empty);
    let world_resources_5 = r.get(world, field::world::RESOURCES)?.r();
    let item_image_value_5 = super::resources::item_image(r, world_resources_5, number_6)?;
    super::item::initialize(
        r,
        item_value_5,
        item_image_value_5,
        number_6,
        x,
        y,
        0,
        player,
    )?;
    object_1 = item_value_5;
    let world_items_5 = r.get(world, field::world::ITEMS)?.r();
    host::vector_add_element(r, world_items_5, object_1)?;
    return Ok(());
}

/// Dig. Original reference: `j.d(Lh;)V`.
pub(in crate::game) fn dig(r: &mut Runtime, world: u32, player: u32) -> Result<(), u32> {
    let mut number_1: i32 = 0;
    let mut number_2: i32 = 0;
    let mut number_3: i32 = 0;
    let mut object_3: u32 = 0;
    let mut object_4: u32 = 0;
    'check_world_enemies: {
        'next_step_2: {
            'check_world_objects: {
                number_2 = 0;
                number_3 = 0;
                let player_dig_timer = r.get(player, field::player::DIG_TIMER)?.i();
                if player_dig_timer <= 0 {
                    r.set(player, field::player::DIGGING, V::I(1))?;
                    r.set(player, field::player::DIG_TIMER, V::I(10))?;
                }
                let player_dig_timer_2 = r.get(player, field::player::DIG_TIMER)?.i();
                if player_dig_timer_2 != 6 {
                    return Ok(());
                }
                let player_facing = r.get(player, field::player::FACING)?.i();
                if player_facing == 0 {
                    let drawable_x = r.get(player, field::drawable::X)?.i();
                    number_2 = drawable_x.wrapping_add(12);
                    let drawable_y = r.get(player, field::drawable::Y)?.i();
                    number_3 = drawable_y.wrapping_sub(26);
                }
                let player_facing_2 = r.get(player, field::player::FACING)?.i();
                if player_facing_2 == 1 {
                    let drawable_x_2 = r.get(player, field::drawable::X)?.i();
                    number_2 = (drawable_x_2.wrapping_add(16)).wrapping_add(12);
                    let drawable_y_2 = r.get(player, field::drawable::Y)?.i();
                    number_3 = drawable_y_2.wrapping_sub(16);
                }
                let player_facing_3 = r.get(player, field::player::FACING)?.i();
                if player_facing_3 == 2 {
                    let drawable_x_3 = r.get(player, field::drawable::X)?.i();
                    number_2 = drawable_x_3.wrapping_add(12);
                    let drawable_y_3 = r.get(player, field::drawable::Y)?.i();
                    number_3 = drawable_y_3;
                }
                let player_facing_4 = r.get(player, field::player::FACING)?.i();
                if player_facing_4 == 3 {
                    let drawable_x_4 = r.get(player, field::drawable::X)?.i();
                    number_2 = drawable_x_4.wrapping_sub(10);
                    let drawable_y_4 = r.get(player, field::drawable::Y)?.i();
                    number_3 = drawable_y_4.wrapping_sub(16);
                }
                number_1 = 0;
            }
            'dig_loop_2: loop {
                'check_world_objects_2: {
                    let world_objects = r.get(world, field::world::OBJECTS)?.r();
                    let vector_size_value = host::vector_size(r, world_objects)?;
                    if number_1 >= vector_size_value {
                        break 'next_step_2;
                    }
                }
                let world_objects_2 = r.get(world, field::world::OBJECTS)?.r();
                let vector_element_at_value =
                    host::vector_element_at(r, world_objects_2, number_1)?;
                host::check_cast(r, vector_element_at_value, "n")?;
                object_4 = vector_element_at_value;
                let drawable_x_5 = r.get(vector_element_at_value, field::drawable::X)?.i();
                if drawable_x_5 <= number_2 {
                    let drawable_x_6 = r.get(object_4, field::drawable::X)?.i();
                    let map_object_width = r.get(object_4, field::map_object::WIDTH)?.i();
                    if !(drawable_x_6.wrapping_add(map_object_width) < number_2) {
                        let drawable_y_5 = r.get(object_4, field::drawable::Y)?.i();
                        if !(drawable_y_5 > number_3.wrapping_add(32)) {
                            let drawable_y_6 = r.get(object_4, field::drawable::Y)?.i();
                            let map_object_height = r.get(object_4, field::map_object::HEIGHT)?.i();
                            if !(drawable_y_6.wrapping_add(map_object_height)
                                < number_3.wrapping_add(32))
                            {
                                let map_object_kind_3 =
                                    r.get(object_4, field::map_object::KIND)?.i();
                                if map_object_kind_3 == 218 {
                                    number_2 = 0;
                                    loop {
                                        let world_items = r.get(world, field::world::ITEMS)?.r();
                                        let vector_size_value_3 =
                                            host::vector_size(r, world_items)?;
                                        if number_2 >= vector_size_value_3 {
                                            let world_objects_3 =
                                                r.get(world, field::world::OBJECTS)?.r();
                                            host::vector_remove_element_at(
                                                r,
                                                world_objects_3,
                                                number_1,
                                            )?;
                                            return Ok(());
                                        }
                                        let world_items_2 = r.get(world, field::world::ITEMS)?.r();
                                        let vector_element_at_value_3 =
                                            host::vector_element_at(r, world_items_2, number_2)?;
                                        host::check_cast(r, vector_element_at_value_3, "c")?;
                                        object_3 = vector_element_at_value_3;
                                        let x_value_9 = r
                                            .get(vector_element_at_value_3, field::drawable::X)?
                                            .i();
                                        let drawable_x_10 =
                                            r.get(object_4, field::drawable::X)?.i();
                                        if x_value_9 <= drawable_x_10 {
                                            let drawable_x_11 =
                                                r.get(object_3, field::drawable::X)?.i();
                                            let drawable_x_12 =
                                                r.get(object_4, field::drawable::X)?.i();
                                            if !(drawable_x_11.wrapping_add(16) < drawable_x_12) {
                                                let drawable_y_9 =
                                                    r.get(object_3, field::drawable::Y)?.i();
                                                let drawable_y_10 =
                                                    r.get(object_4, field::drawable::Y)?.i();
                                                if drawable_y_9 <= drawable_y_10 {
                                                    let drawable_y_11 =
                                                        r.get(object_3, field::drawable::Y)?.i();
                                                    let drawable_y_12 =
                                                        r.get(object_4, field::drawable::Y)?.i();
                                                    if !(drawable_y_11.wrapping_add(16)
                                                        < drawable_y_12)
                                                    {
                                                        r.set(
                                                            object_3,
                                                            field::item::ACTIVE,
                                                            V::I(1),
                                                        )?;
                                                    }
                                                }
                                            }
                                        }
                                        number_2 = number_2.wrapping_add(1);
                                    }
                                }
                            }
                        }
                    }
                }
                number_1 = number_1.wrapping_add(1);
                continue 'dig_loop_2;
            }
        }
        number_1 = 0;
    }
    'dig_loop_5: loop {
        'advance_index: {
            'check_drawable_x: {
                'check_enemy_kind: {
                    let world_enemies = r.get(world, field::world::ENEMIES)?.r();
                    let vector_size_value_2 = host::vector_size(r, world_enemies)?;
                    if number_1 >= vector_size_value_2 {
                        return Ok(());
                    }
                    let world_enemies_2 = r.get(world, field::world::ENEMIES)?.r();
                    let vector_element_at_value_2 =
                        host::vector_element_at(r, world_enemies_2, number_1)?;
                    host::check_cast(r, vector_element_at_value_2, "l")?;
                    object_4 = vector_element_at_value_2;
                    let enemy_kind = r.get(vector_element_at_value_2, field::enemy::KIND)?.i();
                    if enemy_kind == 28 {
                        break 'check_drawable_x;
                    }
                }
                let enemy_kind_2 = r.get(object_4, field::enemy::KIND)?.i();
                if enemy_kind_2 != 29 {
                    break 'advance_index;
                }
            }
            let drawable_x_7 = r.get(object_4, field::drawable::X)?.i();
            if drawable_x_7 <= number_2 {
                let drawable_x_8 = r.get(object_4, field::drawable::X)?.i();
                let enemy_width_2 = r.get(object_4, field::enemy::WIDTH)?.i();
                if !(drawable_x_8.wrapping_add(enemy_width_2) < number_2) {
                    let drawable_y_7 = r.get(object_4, field::drawable::Y)?.i();
                    if !(drawable_y_7 > number_3.wrapping_add(32)) {
                        let drawable_y_8 = r.get(object_4, field::drawable::Y)?.i();
                        let enemy_height_2 = r.get(object_4, field::enemy::HEIGHT)?.i();
                        if !(drawable_y_8.wrapping_add(enemy_height_2) < number_3.wrapping_add(32))
                        {
                            r.set(object_4, field::enemy::BEHAVIOR_STATE, V::I(3))?;
                        }
                    }
                }
            }
        }
        number_1 = number_1.wrapping_add(1);
        continue 'dig_loop_5;
    }
}
