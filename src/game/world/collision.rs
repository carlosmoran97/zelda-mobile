//! World collision.
use super::*;

/// Can player move. Original reference: `j.a(IIILh;Z)Z`.
#[rustfmt::skip]
pub(in crate::game) fn can_player_move(
    r: &mut Runtime,
    world: u32,
    mut x: i32,
    mut y: i32,
    direction: i32,
    player: u32,
    check_objects: i32,
) -> Result<i32, u32> {
    let mut number_0: i32 = 0;
    let mut number_6: i32 = 0;
    let mut number_7: i32 = 0;
    let mut object_8: u32 = 0;
    let mut number_9: i32 = 0;
    let mut number_10: i32 = 0;
    'check_world_objects: {
    y = y.wrapping_sub(32);
    number_6 = 0;
    number_7 = 0;
    x = x.wrapping_add(12);
    y = y.wrapping_add(16);
    let world_switch_cooldown = r.get(world, field::world::SWITCH_COOLDOWN)?.i();
    if world_switch_cooldown > 0 {
        let world_switch_cooldown_2 = r.get(world, field::world::SWITCH_COOLDOWN)?.i();
        r.set(
            world,
            field::world::SWITCH_COOLDOWN,
            V::I(world_switch_cooldown_2.wrapping_sub(1)),
        )?;
    }
    let player_state_g = r.get(player, field::player::STATE_G)?.i();
    if player_state_g != 0 {
        number_6 = 0;
        loop {
            let world_overlay_objects = r.get(world, field::world::OVERLAY_OBJECTS)?.r();
            let vector_size_value = host::vector_size(r, world_overlay_objects)?;
            if number_6 >= vector_size_value {
                number_6 = 0;
                loop {
                    let world_objects_3 = r.get(world, field::world::OBJECTS)?.r();
                    let vector_size_value_3 = host::vector_size(r, world_objects_3)?;
                    if number_6 >= vector_size_value_3 {
                        return Ok(0);
                    }
                    let world_objects_4 = r.get(world, field::world::OBJECTS)?.r();
                    let vector_element_at_value_3 =
                        host::vector_element_at(r, world_objects_4, number_6)?;
                    host::check_cast(r, vector_element_at_value_3, "n")?;
                    object_8 = vector_element_at_value_3;
                    let world_alternate_map = r.get(world, field::world::ALTERNATE_MAP)?.i();
                    r.set(
                        vector_element_at_value_3,
                        field::map_object::STATE_C,
                        V::I(world_alternate_map),
                    )?;
                    let drawable_x_3 = r.get(object_8, field::drawable::X)?.i();
                    let map_object_state_d_upper_3 =
                        r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
                    if !(drawable_x_3.wrapping_add(map_object_state_d_upper_3) > x) {
                        let drawable_x_5 = r.get(object_8, field::drawable::X)?.i();
                        let map_object_width_4 = r.get(object_8, field::map_object::WIDTH)?.i();
                        let map_object_state_d_upper_5 =
                            r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
                        if !((drawable_x_5.wrapping_add(map_object_width_4))
                            .wrapping_add(map_object_state_d_upper_5)
                            < x)
                        {
                            let drawable_y_3 = r.get(object_8, field::drawable::Y)?.i();
                            let map_object_state_e_upper_3 =
                                r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
                            if !(drawable_y_3.wrapping_add(map_object_state_e_upper_3)
                                > y.wrapping_add(32))
                            {
                                let drawable_y_5 = r.get(object_8, field::drawable::Y)?.i();
                                let map_object_height_3 =
                                    r.get(object_8, field::map_object::HEIGHT)?.i();
                                let map_object_state_e_upper_5 =
                                    r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
                                if !((drawable_y_5.wrapping_add(map_object_height_3))
                                    .wrapping_add(map_object_state_e_upper_5)
                                    < y.wrapping_add(32))
                                {
                                    let map_object_kind_5 =
                                        r.get(object_8, field::map_object::KIND)?.i();
                                    if map_object_kind_5 == 17 {
                                        r.set(player, field::player::STATE_G, V::I(0))?;
                                        return Ok(1);
                                    }
                                }
                            }
                        }
                    }
                    number_6 = number_6.wrapping_add(1);
                }
            }
            let world_overlay_objects_2 = r.get(world, field::world::OVERLAY_OBJECTS)?.r();
            let vector_element_at_value =
                host::vector_element_at(r, world_overlay_objects_2, number_6)?;
            host::check_cast(r, vector_element_at_value, "n")?;
            object_8 = vector_element_at_value;
            let drawable_x = r.get(vector_element_at_value, field::drawable::X)?.i();
            let map_object_state_d_upper = r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
            if !(drawable_x.wrapping_add(map_object_state_d_upper) > x) {
                let drawable_x_2 = r.get(object_8, field::drawable::X)?.i();
                let map_object_width_3 = r.get(object_8, field::map_object::WIDTH)?.i();
                let map_object_state_d_upper_2 =
                    r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
                if !((drawable_x_2.wrapping_add(map_object_width_3))
                    .wrapping_add(map_object_state_d_upper_2)
                    < x)
                {
                    let drawable_y = r.get(object_8, field::drawable::Y)?.i();
                    let map_object_state_e_upper =
                        r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
                    if !(drawable_y.wrapping_add(map_object_state_e_upper) > y.wrapping_add(32)) {
                        let drawable_y_2 = r.get(object_8, field::drawable::Y)?.i();
                        let map_object_height_2 = r.get(object_8, field::map_object::HEIGHT)?.i();
                        let map_object_state_e_upper_2 =
                            r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
                        if !((drawable_y_2.wrapping_add(map_object_height_2))
                            .wrapping_add(map_object_state_e_upper_2)
                            < y.wrapping_add(32))
                        {
                            let state_out_value = r.static_get(field::system::STATE_OUT).r();
                            let text_value = r.intern("Standing on something..");
                            host::print_stream_println(r, state_out_value, text_value)?;
                            return Ok(1);
                        }
                    }
                }
            }
            number_6 = number_6.wrapping_add(1);
        }
    }
    let world_width = r.get(world, field::world::WIDTH)?.i();
    number_6 = ((y.wrapping_shr((4) as u32)).wrapping_mul(world_width))
        .wrapping_add(x.wrapping_shr((4) as u32));
    let world_width_2 = r.get(world, field::world::WIDTH)?.i();
    let world_height = r.get(world, field::world::HEIGHT)?.i();
    if (!(((y.wrapping_shr((4) as u32)).wrapping_mul(world_width))
        .wrapping_add(x.wrapping_shr((4) as u32))
        > world_width_2.wrapping_mul(world_height)))
        && (number_6 > 0)
    {
        let world_tiles = r.get(world, field::world::TILES)?.r();
        let element_value = r.array_get(world_tiles, number_6)?.i();
        number_6 = element_value;
        if !((element_value & 64) != 64) {
            return Ok(0);
        }

        if !((number_6 & 128) != 128) {
            let player_on_raft_2 = r.get(player, field::player::ON_RAFT)?.i();
            if player_on_raft_2 == 0 {
                return Ok(0);
            }
        }
    }
    number_6 = 0;
    } // check_world_objects
    'can_player_move_loop_2: loop {
        'advance_index: {
        'check_map_object_kind_9: {
        'check_player_on_raft: {
        'check_map_object_kind_10: {
        'check_map_object_kind_5: {
        'check_drawable_y_2: {
        'check_drawable_y: {
        'check_drawable_x_2: {
        'check_drawable_x: {
        'check_map_object_solid: {
        'check_map_object_kind_8: {
        'check_map_object_kind_6: {
        'check_map_object_kind_7: {
        'check_map_object_kind_4: {
        'check_map_object_kind_3: {
        'check_map_object_kind_2: {
        'check_map_object_kind: {
        'check_map_object_trigger_kind: {
        let world_objects = r.get(world, field::world::OBJECTS)?.r();
        let vector_size_value_2 = host::vector_size(r, world_objects)?;
        if number_6 >= vector_size_value_2 {
let player_on_raft = r.get(player, field::player::ON_RAFT)?.i();
if player_on_raft == 0 {
return Ok(1);
}
number_6 = 0;
loop {
let world_objects_5 = r.get(world, field::world::OBJECTS)?.r();
let vector_size_value_4 = host::vector_size(r, world_objects_5)?;
if number_6 >= vector_size_value_4 {
let player_facing = r.get(player, field::player::FACING)?.i();
if player_facing == 0 {
y = y.wrapping_add(-4);
}
let player_facing_2 = r.get(player, field::player::FACING)?.i();
if player_facing_2 == 1 {
x = x.wrapping_add(4);
}
let player_facing_3 = r.get(player, field::player::FACING)?.i();
if player_facing_3 == 2 {
y = y.wrapping_add(4);
}
let player_facing_4 = r.get(player, field::player::FACING)?.i();
if player_facing_4 == 3 {
x = x.wrapping_add(-4);
}
let world_width_8 = r.get(world, field::world::WIDTH)?.i();
number_6 = ((y.wrapping_shr((4) as u32)).wrapping_mul(world_width_8)).wrapping_add(x.wrapping_shr((4) as u32));
let world_width_9 = r.get(world, field::world::WIDTH)?.i();
let world_height_5 = r.get(world, field::world::HEIGHT)?.i();
if ((y.wrapping_shr((4) as u32)).wrapping_mul(world_width_8)).wrapping_add(x.wrapping_shr((4) as u32)) > world_width_9.wrapping_mul(world_height_5) {
return Ok(1);
}

if number_6 <= 0 {
return Ok(1);
}
let world_tiles_2 = r.get(world, field::world::TILES)?.r();
let element_value_2 = r.array_get(world_tiles_2, number_6)?.i();
number_6 = element_value_2;
if (element_value_2 & 128) == 128 {
return Ok(1);
}
return Ok(0);
}
let world_objects_6 = r.get(world, field::world::OBJECTS)?.r();
let vector_element_at_value_4 = host::vector_element_at(r, world_objects_6, number_6)?;
host::check_cast(r, vector_element_at_value_4, "n")?;
object_8 = vector_element_at_value_4;
let drawable_x_6 = r.get(vector_element_at_value_4, field::drawable::X)?.i();
let map_object_state_d_upper_6 = r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
if !(drawable_x_6.wrapping_add(map_object_state_d_upper_6) > x.wrapping_add(16)) {
let drawable_x_8 = r.get(object_8, field::drawable::X)?.i();
let map_object_width_6 = r.get(object_8, field::map_object::WIDTH)?.i();
let map_object_state_d_upper_8 = r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
if !((drawable_x_8.wrapping_add(map_object_width_6)).wrapping_add(map_object_state_d_upper_8) < x.wrapping_sub(16)) {
let drawable_x_9 = r.get(object_8, field::drawable::X)?.i();
let map_object_state_d_upper_9 = r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
if !((drawable_x_9.wrapping_add(map_object_state_d_upper_9)).wrapping_sub(4) > x) {
let drawable_x_10 = r.get(object_8, field::drawable::X)?.i();
let map_object_width_7 = r.get(object_8, field::map_object::WIDTH)?.i();
let map_object_state_d_upper_10 = r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
if !(((drawable_x_10.wrapping_add(map_object_width_7)).wrapping_add(map_object_state_d_upper_10)).wrapping_add(4) < x) {
let drawable_y_7 = r.get(object_8, field::drawable::Y)?.i();
let map_object_state_e_upper_7 = r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
if !((drawable_y_7.wrapping_add(map_object_state_e_upper_7)).wrapping_sub(4) > y.wrapping_add(32)) {
let drawable_y_8 = r.get(object_8, field::drawable::Y)?.i();
let map_object_height_6 = r.get(object_8, field::map_object::HEIGHT)?.i();
let map_object_state_e_upper_8 = r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
if !(((drawable_y_8.wrapping_add(map_object_height_6)).wrapping_add(map_object_state_e_upper_8)).wrapping_add(4) < y.wrapping_add(32)) {
return Ok(1);
}
}
}
}
}
}
number_6 = number_6.wrapping_add(1);
}
}
        let world_objects_2 = r.get(world, field::world::OBJECTS)?.r();
        let vector_element_at_value_2 = host::vector_element_at(r, world_objects_2, number_6)?;
        host::check_cast(r, vector_element_at_value_2, "n")?;
        object_8 = vector_element_at_value_2;
        let map_object_state_e = r.get(vector_element_at_value_2, field::map_object::STATE_E)?.i();
        if map_object_state_e
            != 0
        {
            break 'check_map_object_solid;
        }
        } // check_map_object_trigger_kind
        let map_object_trigger_kind = r.get(object_8, field::map_object::TRIGGER_KIND)?.i();
        if map_object_trigger_kind
            >= 2
        {
            break 'check_map_object_solid;
        }
        } // check_map_object_kind
        let map_object_kind = r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind == 18 {
            break 'check_map_object_solid;
        }
        } // check_map_object_kind_2
        let kind_value_2 = r
            .get(
                object_8,
                field::map_object::KIND,
            )?
            .i();
        if kind_value_2 == 185 {
            break 'check_map_object_solid;
        }
        } // check_map_object_kind_3
        let kind_value_3 = r
            .get(
                object_8,
                field::map_object::KIND,
            )?
            .i();
        if kind_value_3 == 17 {
            break 'check_map_object_solid;
        }
        } // check_map_object_kind_4
        let kind_value_4 = r
            .get(
                object_8,
                field::map_object::KIND,
            )?
            .i();
        if kind_value_4 < 21 {
            break 'check_map_object_kind_6;
        }
        } // check_map_object_kind_7
        let kind_value_8 = r
            .get(object_8, field::map_object::KIND)?
            .i();
        if kind_value_8 <= 24 {
            break 'check_map_object_solid;
        }
        } // check_map_object_kind_6
        let kind_value_7 = r
            .get(object_8, field::map_object::KIND)?
            .i();
        if kind_value_7 < 208 {
            break 'advance_index;
        }
        } // check_map_object_kind_8
        let map_object_kind_10 =
            r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind_10 > 215 {
            break 'advance_index;
        }
        } // check_map_object_solid
        let map_object_solid =
            r.get(object_8, field::map_object::SOLID)?.i();
        if map_object_solid == 0 {
            break 'advance_index;
        }
        } // check_drawable_x
        let drawable_x_4 = r.get(object_8, field::drawable::X)?.i();
        let map_object_state_d_upper_4 =
            r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
        if drawable_x_4.wrapping_add(map_object_state_d_upper_4) > x {
            break 'advance_index;
        }
        } // check_drawable_x_2
        let drawable_x_7 = r.get(object_8, field::drawable::X)?.i();
        let map_object_width_5 =
            r.get(object_8, field::map_object::WIDTH)?.i();
        let map_object_state_d_upper_7 =
            r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
        if (drawable_x_7.wrapping_add(map_object_width_5))
            .wrapping_add(map_object_state_d_upper_7)
            < x
        {
            break 'advance_index;
        }
        } // check_drawable_y
        let drawable_y_4 = r.get(object_8, field::drawable::Y)?.i();
        let map_object_state_e_upper_4 =
            r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
        if drawable_y_4.wrapping_add(map_object_state_e_upper_4)
            > y.wrapping_add(32)
        {
            break 'advance_index;
        }
        } // check_drawable_y_2
        let drawable_y_6 = r.get(object_8, field::drawable::Y)?.i();
        let map_object_height_4 = r.get(object_8, field::map_object::HEIGHT)?.i();
        let map_object_state_e_upper_6 =
            r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
        if (drawable_y_6.wrapping_add(map_object_height_4))
            .wrapping_add(map_object_state_e_upper_6)
            < y.wrapping_add(32)
        {
            break 'advance_index;
        }
        } // check_map_object_kind_5
        let map_object_kind_6 = r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind_6 == 18 {
            r.set(player, field::player::STATE_G, V::I(1))?;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let text_value_2 = r.intern("Layer changed to 1");
            host::print_stream_println(r, state_out_value_2, text_value_2)?;
            return Ok(1);
        }
        let map_object_kind_9 = r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind_9 == 17 {
            r.set(player, field::player::STATE_G, V::I(0))?;
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let text_value_3 = r.intern("Layer changed to 0");
            host::print_stream_println(r, state_out_value_3, text_value_3)?;
            return Ok(1);
        }
        let map_object_kind_11 = r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind_11 >= 21 {
            let map_object_kind_13 = r.get(object_8, field::map_object::KIND)?.i();
            if map_object_kind_13 <= 24 {
                let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
                let text_value_4 = r.intern("standing on it..");
                host::print_stream_println(r, state_out_value_4, text_value_4)?;
                number_9 = 0;
                number_10 = 0;
                let player_facing_5 = r.get(player, field::player::FACING)?.i();
                if player_facing_5 == 0 {
                    number_10 = -4;
                    let player_on_raft_4 =
                        r.get(player, field::player::ON_RAFT)?.i();
                    if player_on_raft_4 != 0 {
                        number_10 = 0;
                    }
                }
                let player_facing_6 = r.get(player, field::player::FACING)?.i();
                if player_facing_6 == 1 {
                    number_9 = 4;
                    let player_on_raft_5 =
                        r.get(player, field::player::ON_RAFT)?.i();
                    if player_on_raft_5 != 0 {
                        number_9 = 0;
                    }
                }
                let player_facing_7 = r.get(player, field::player::FACING)?.i();
                if player_facing_7 == 2 {
                    number_10 = 4;
                    let player_on_raft_6 =
                        r.get(player, field::player::ON_RAFT)?.i();
                    if player_on_raft_6 != 0 {
                        number_10 = 0;
                    }
                }
                let player_facing_9 = r.get(player, field::player::FACING)?.i();
                if player_facing_9 == 3 {
                    number_9 = -4;
                    let player_on_raft_7 =
                        r.get(player, field::player::ON_RAFT)?.i();
                    if player_on_raft_7 != 0 {
                        number_9 = 0;
                    }
                }
                let world_width_12 = r.get(world, field::world::WIDTH)?.i();
                number_6 = (((y.wrapping_add(number_10))
                    .wrapping_shr((4) as u32))
                .wrapping_mul(world_width_12))
                .wrapping_add(
                    (x.wrapping_add(number_9)).wrapping_shr((4) as u32),
                );
                let world_width_13 = r.get(world, field::world::WIDTH)?.i();
                let world_height_8 = r.get(world, field::world::HEIGHT)?.i();
                if (((y.wrapping_add(number_10)).wrapping_shr((4) as u32))
                    .wrapping_mul(world_width_12))
                .wrapping_add((x.wrapping_add(number_9)).wrapping_shr((4) as u32))
                    > world_width_13.wrapping_mul(world_height_8)
                {
                    let state_out_value_5 =
                        r.static_get(field::system::STATE_OUT).r();
                    let text_value_5 = r.intern("test");
                    host::print_stream_println(r, state_out_value_5, text_value_5)?;
                    return Ok(1);
                }

                if number_6 <= 0 {
                    let state_out_value_5 =
                        r.static_get(field::system::STATE_OUT).r();
                    let text_value_5 = r.intern("test");
                    host::print_stream_println(r, state_out_value_5, text_value_5)?;
                    return Ok(1);
                }
                let world_tiles_4 = r.get(world, field::world::TILES)?.r();
                let element_value_4 = r.array_get(world_tiles_4, number_6)?.i();
                number_0 = element_value_4;
                if (element_value_4 & 128) != 128 {
                    let drawable_x_18 = r.get(object_8, field::drawable::X)?.i();
                    let map_object_state_d_upper_13 =
                        r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
                    if (drawable_x_18.wrapping_add(map_object_state_d_upper_13))
                        .wrapping_sub(6)
                        > x
                    {
                        return Ok(0);
                    }
                    let drawable_x_24 = r.get(object_8, field::drawable::X)?.i();
                    let map_object_width_22 =
                        r.get(object_8, field::map_object::WIDTH)?.i();
                    let map_object_state_d_upper_14 =
                        r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
                    if ((drawable_x_24.wrapping_add(map_object_width_22))
                        .wrapping_add(map_object_state_d_upper_14))
                    .wrapping_add(6)
                        < x
                    {
                        return Ok(0);
                    }
                    let drawable_y_25 = r.get(object_8, field::drawable::Y)?.i();
                    let map_object_state_e_upper_11 =
                        r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
                    if (drawable_y_25.wrapping_add(map_object_state_e_upper_11))
                        .wrapping_sub(6)
                        > y.wrapping_add(32)
                    {
                        return Ok(0);
                    }
                    let drawable_y_26 = r.get(object_8, field::drawable::Y)?.i();
                    let map_object_height_13 =
                        r.get(object_8, field::map_object::HEIGHT)?.i();
                    let map_object_state_e_upper_12 =
                        r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
                    if ((drawable_y_26.wrapping_add(map_object_height_13))
                        .wrapping_add(map_object_state_e_upper_12))
                    .wrapping_add(6)
                        < y.wrapping_add(32)
                    {
                        return Ok(0);
                    }
                    r.set(player, field::player::ON_RAFT, V::I(0))?;
                    let state_out_value_7 =
                        r.static_get(field::system::STATE_OUT).r();
                    let text_value_7 = r.intern("NOT Rafting");
                    host::print_stream_println(r, state_out_value_7, text_value_7)?;
                    return Ok(1);
                }
                let player_has_hookshot =
                    r.get(player, field::player::HAS_HOOKSHOT)?.i();
                if player_has_hookshot != 0 {
                    r.set(player, field::player::ON_RAFT, V::I(1))?;
                }
                let state_out_value_6 = r.static_get(field::system::STATE_OUT).r();
                let text_value_6 = r.intern("Rafting");
                host::print_stream_println(r, state_out_value_6, text_value_6)?;
                return Ok(1);
            }
        }
        let map_object_kind_12 = r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind_12 < 212 {
            break 'check_map_object_kind_9;
        }
        } // check_map_object_kind_10
        let map_object_kind_15 = r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind_15 > 215 {
            break 'check_map_object_kind_9;
        }
        } // check_player_on_raft
        let player_on_raft_3 = r.get(player, field::player::ON_RAFT)?.i();
        if (player_on_raft_3 != 0) && (number_7 == 0) {
            let map_object_kind_17 = r.get(object_8, field::map_object::KIND)?.i();
            if map_object_kind_17 == 212 {
                let world_width_10 = r.get(world, field::world::WIDTH)?.i();
                number_9 = (((y.wrapping_add(2)).wrapping_shr((4) as u32))
                    .wrapping_mul(world_width_10))
                .wrapping_add(x.wrapping_shr((4) as u32));
                let world_width_11 = r.get(world, field::world::WIDTH)?.i();
                let world_height_7 = r.get(world, field::world::HEIGHT)?.i();
                if (!((((y.wrapping_add(2)).wrapping_shr((4) as u32))
                    .wrapping_mul(world_width_10))
                .wrapping_add(x.wrapping_shr((4) as u32))
                    > world_width_11.wrapping_mul(world_height_7)))
                    && (number_9 > 0)
                {
                    let world_tiles_3 = r.get(world, field::world::TILES)?.r();
                    let element_value_3 = r.array_get(world_tiles_3, number_9)?.i();
                    number_10 = element_value_3;
                    if !((element_value_3 & 128) != 128) {
                        let drawable_y_19 = r.get(player, field::drawable::Y)?.i();
                        r.set(
                            player,
                            field::drawable::Y,
                            V::I(drawable_y_19.wrapping_add(2)),
                        )?;
                        number_7 = 1;
                    }
                }
            }
            let map_object_kind_18 = r.get(object_8, field::map_object::KIND)?.i();
            if map_object_kind_18 == 213 {
                let world_width_15 = r.get(world, field::world::WIDTH)?.i();
                number_9 = ((y.wrapping_shr((4) as u32)).wrapping_mul(world_width_15))
                    .wrapping_add((x.wrapping_sub(2)).wrapping_shr((4) as u32));
                let world_width_16 = r.get(world, field::world::WIDTH)?.i();
                let world_height_9 = r.get(world, field::world::HEIGHT)?.i();
                if (!(((y.wrapping_shr((4) as u32)).wrapping_mul(world_width_15))
                    .wrapping_add((x.wrapping_sub(2)).wrapping_shr((4) as u32))
                    > world_width_16.wrapping_mul(world_height_9)))
                    && (number_9 > 0)
                {
                    let world_tiles_5 = r.get(world, field::world::TILES)?.r();
                    let element_value_5 = r.array_get(world_tiles_5, number_9)?.i();
                    number_10 = element_value_5;
                    if !((element_value_5 & 128) != 128) {
                        let drawable_x_23 = r.get(player, field::drawable::X)?.i();
                        r.set(
                            player,
                            field::drawable::X,
                            V::I(drawable_x_23.wrapping_sub(2)),
                        )?;
                        number_7 = 1;
                    }
                }
            }
            let map_object_kind_19 = r.get(object_8, field::map_object::KIND)?.i();
            if map_object_kind_19 == 214 {
                let world_width_17 = r.get(world, field::world::WIDTH)?.i();
                number_9 = (((y.wrapping_sub(2)).wrapping_shr((4) as u32))
                    .wrapping_mul(world_width_17))
                .wrapping_add(x.wrapping_shr((4) as u32));
                let world_width_18 = r.get(world, field::world::WIDTH)?.i();
                let world_height_11 = r.get(world, field::world::HEIGHT)?.i();
                if (!((((y.wrapping_sub(2)).wrapping_shr((4) as u32))
                    .wrapping_mul(world_width_17))
                .wrapping_add(x.wrapping_shr((4) as u32))
                    > world_width_18.wrapping_mul(world_height_11)))
                    && (number_9 > 0)
                {
                    let world_tiles_6 = r.get(world, field::world::TILES)?.r();
                    let element_value_6 = r.array_get(world_tiles_6, number_9)?.i();
                    number_10 = element_value_6;
                    if !((element_value_6 & 128) != 128) {
                        let drawable_y_24 = r.get(player, field::drawable::Y)?.i();
                        r.set(
                            player,
                            field::drawable::Y,
                            V::I(drawable_y_24.wrapping_sub(2)),
                        )?;
                        number_7 = 1;
                    }
                }
            }
            let map_object_kind_20 = r.get(object_8, field::map_object::KIND)?.i();
            if map_object_kind_20 == 215 {
                let world_width_20 = r.get(world, field::world::WIDTH)?.i();
                number_9 = ((y.wrapping_shr((4) as u32)).wrapping_mul(world_width_20))
                    .wrapping_add((x.wrapping_add(2)).wrapping_shr((4) as u32));
                let world_width_21 = r.get(world, field::world::WIDTH)?.i();
                let world_height_12 = r.get(world, field::world::HEIGHT)?.i();
                if (!(((y.wrapping_shr((4) as u32)).wrapping_mul(world_width_20))
                    .wrapping_add((x.wrapping_add(2)).wrapping_shr((4) as u32))
                    > world_width_21.wrapping_mul(world_height_12)))
                    && (number_9 > 0)
                {
                    let world_tiles_7 = r.get(world, field::world::TILES)?.r();
                    let element_value_7 = r.array_get(world_tiles_7, number_9)?.i();
                    number_10 = element_value_7;
                    if !((element_value_7 & 128) != 128) {
                        let drawable_x_28 = r.get(player, field::drawable::X)?.i();
                        r.set(
                            player,
                            field::drawable::X,
                            V::I(drawable_x_28.wrapping_add(2)),
                        )?;
                        number_7 = 1;
                    }
                }
            }
        }
        break 'advance_index;
        } // check_map_object_kind_9
        let map_object_kind_14 = r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind_14 == 185 {
            let player_hurt_timer = r.get(player, field::player::HURT_TIMER)?.i();
            if player_hurt_timer > 0 {
                return Ok(0);
            }
            let player_health = r.get(player, field::player::HEALTH)?.i();
            r.set(
                player,
                field::player::HEALTH,
                V::I(player_health.wrapping_sub(1)),
            )?;
            let player_health_2 = r.get(player, field::player::HEALTH)?.i();
            if player_health_2 < 0 {
                r.set(player, field::player::HEALTH, V::I(0))?;
            }
            let player_facing_8 = r.get(player, field::player::FACING)?.i();
            if player_facing_8 == 0 {
                let drawable_y_10 = r.get(player, field::drawable::Y)?.i();
                r.set(
                    player,
                    field::drawable::Y,
                    V::I(drawable_y_10.wrapping_add(5)),
                )?;
            }
            let player_facing_10 = r.get(player, field::player::FACING)?.i();
            if player_facing_10 == 1 {
                let drawable_x_13 = r.get(player, field::drawable::X)?.i();
                r.set(
                    player,
                    field::drawable::X,
                    V::I(drawable_x_13.wrapping_sub(5)),
                )?;
            }
            let player_facing_12 = r.get(player, field::player::FACING)?.i();
            if player_facing_12 == 2 {
                let drawable_y_14 = r.get(player, field::drawable::Y)?.i();
                r.set(
                    player,
                    field::drawable::Y,
                    V::I(drawable_y_14.wrapping_sub(5)),
                )?;
            }
            let player_facing_15 = r.get(player, field::player::FACING)?.i();
            if player_facing_15 == 3 {
                let drawable_x_17 = r.get(player, field::drawable::X)?.i();
                r.set(
                    player,
                    field::drawable::X,
                    V::I(drawable_x_17.wrapping_add(5)),
                )?;
            }
            r.set(player, field::player::HURT_TIMER, V::I(20))?;
            let world_canvas = r.get(world, field::world::CANVAS)?.r();
            r.set(world_canvas, field::canvas::REDRAW_WORLD, V::I(1))?;
            return Ok(0);
        }
        let map_object_state_v = r.get(object_8, field::map_object::STATE_V)?.i();
        if map_object_state_v != 0 {
            let map_object_kind_16 = r.get(object_8, field::map_object::KIND)?.i();
            if map_object_kind_16 == 61 {
                let player_shield_raised = r.get(player, field::player::SHIELD_RAISED)?.i();
                if player_shield_raised == 0 {
                    let player_hurt_timer_2 = r.get(player, field::player::HURT_TIMER)?.i();
                    if player_hurt_timer_2 > 0 {
                        return Ok(0);
                    }
                    let player_health_3 = r.get(player, field::player::HEALTH)?.i();
                    r.set(
                        player,
                        field::player::HEALTH,
                        V::I(player_health_3.wrapping_sub(1)),
                    )?;
                    let player_health_4 = r.get(player, field::player::HEALTH)?.i();
                    if player_health_4 < 0 {
                        r.set(player, field::player::HEALTH, V::I(0))?;
                    }
                    let player_facing_14 = r.get(player, field::player::FACING)?.i();
                    if player_facing_14 == 0 {
                        let drawable_y_18 = r.get(player, field::drawable::Y)?.i();
                        r.set(
                            player,
                            field::drawable::Y,
                            V::I(drawable_y_18.wrapping_add(5)),
                        )?;
                    }
                    let player_facing_17 = r.get(player, field::player::FACING)?.i();
                    if player_facing_17 == 1 {
                        let drawable_x_22 = r.get(player, field::drawable::X)?.i();
                        r.set(
                            player,
                            field::drawable::X,
                            V::I(drawable_x_22.wrapping_sub(5)),
                        )?;
                    }
                    let player_facing_19 = r.get(player, field::player::FACING)?.i();
                    if player_facing_19 == 2 {
                        let drawable_y_23 = r.get(player, field::drawable::Y)?.i();
                        r.set(
                            player,
                            field::drawable::Y,
                            V::I(drawable_y_23.wrapping_sub(5)),
                        )?;
                    }
                    let player_facing_20 = r.get(player, field::player::FACING)?.i();
                    if player_facing_20 == 3 {
                        let drawable_x_27 = r.get(player, field::drawable::X)?.i();
                        r.set(
                            player,
                            field::drawable::X,
                            V::I(drawable_x_27.wrapping_add(5)),
                        )?;
                    }
                    r.set(player, field::player::HURT_TIMER, V::I(20))?;
                    let world_canvas_2 = r.get(world, field::world::CANVAS)?.r();
                    r.set(world_canvas_2, field::canvas::REDRAW_WORLD, V::I(1))?;
                    return Ok(0);
                }
            }

            if direction == 0 {
                let drawable_y_9 = r.get(object_8, field::drawable::Y)?.i();
                let map_object_state_e_upper_9 =
                    r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
                if !(drawable_y_9.wrapping_add(map_object_state_e_upper_9) >= y.wrapping_add(36)) {
                    let drawable_x_12 = r.get(object_8, field::drawable::X)?.i();
                    let drawable_y_11 = r.get(object_8, field::drawable::Y)?.i();
                    let player_facing_11 = r.get(player, field::player::FACING)?.i();
                    let can_entity_move_value = super::world::can_entity_move(
                        r,
                        world,
                        drawable_x_12,
                        drawable_y_11.wrapping_sub(1),
                        player_facing_11,
                        100,
                        number_6,
                    )?;
                    if can_entity_move_value != 0 {
                        if check_objects == 0 {
                            return Ok(1);
                        }
                        let drawable_y_16 = r.get(object_8, field::drawable::Y)?.i();
                        r.set(
                            object_8,
                            field::drawable::Y,
                            V::I(drawable_y_16.wrapping_sub(1)),
                        )?;
                        let drawable_y_17 = r.get(player, field::drawable::Y)?.i();
                        r.set(
                            player,
                            field::drawable::Y,
                            V::I(drawable_y_17.wrapping_sub(1)),
                        )?;
                    }
                }
            }

            if direction == 1 {
                let drawable_x_11 = r.get(object_8, field::drawable::X)?.i();
                let map_object_state_d_upper_11 =
                    r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
                let map_object_width_14 = r.get(object_8, field::map_object::WIDTH)?.i();
                if !((drawable_x_11.wrapping_add(map_object_state_d_upper_11))
                    .wrapping_add(map_object_width_14)
                    <= x.wrapping_sub(4))
                {
                    let drawable_x_14 = r.get(object_8, field::drawable::X)?.i();
                    let drawable_y_13 = r.get(object_8, field::drawable::Y)?.i();
                    let player_facing_13 = r.get(player, field::player::FACING)?.i();
                    let can_entity_move_value_2 = super::world::can_entity_move(
                        r,
                        world,
                        drawable_x_14,
                        drawable_y_13,
                        player_facing_13,
                        100,
                        number_6,
                    )?;
                    if can_entity_move_value_2 != 0 {
                        if check_objects == 0 {
                            return Ok(1);
                        }
                        let drawable_x_20 = r.get(object_8, field::drawable::X)?.i();
                        r.set(
                            object_8,
                            field::drawable::X,
                            V::I(drawable_x_20.wrapping_add(1)),
                        )?;
                        let drawable_x_21 = r.get(player, field::drawable::X)?.i();
                        r.set(
                            player,
                            field::drawable::X,
                            V::I(drawable_x_21.wrapping_add(1)),
                        )?;
                    }
                }
            }

            if direction == 2 {
                let drawable_y_12 = r.get(object_8, field::drawable::Y)?.i();
                let map_object_state_e_upper_10 =
                    r.get(object_8, field::map_object::STATE_E_UPPER)?.i();
                let map_object_height_10 = r.get(object_8, field::map_object::HEIGHT)?.i();
                if !((drawable_y_12.wrapping_add(map_object_state_e_upper_10))
                    .wrapping_add(map_object_height_10)
                    <= y.wrapping_add(28))
                {
                    let drawable_x_16 = r.get(object_8, field::drawable::X)?.i();
                    let drawable_y_15 = r.get(object_8, field::drawable::Y)?.i();
                    let player_facing_16 = r.get(player, field::player::FACING)?.i();
                    let can_entity_move_value_3 = super::world::can_entity_move(
                        r,
                        world,
                        drawable_x_16,
                        drawable_y_15.wrapping_add(1),
                        player_facing_16,
                        100,
                        number_6,
                    )?;
                    if can_entity_move_value_3 != 0 {
                        if check_objects == 0 {
                            return Ok(1);
                        }
                        let drawable_y_21 = r.get(object_8, field::drawable::Y)?.i();
                        r.set(
                            object_8,
                            field::drawable::Y,
                            V::I(drawable_y_21.wrapping_add(1)),
                        )?;
                        let drawable_y_22 = r.get(player, field::drawable::Y)?.i();
                        r.set(
                            player,
                            field::drawable::Y,
                            V::I(drawable_y_22.wrapping_add(1)),
                        )?;
                    }
                }
            }

            if direction == 3 {
                let drawable_x_15 = r.get(object_8, field::drawable::X)?.i();
                let map_object_width_19 = r.get(object_8, field::map_object::WIDTH)?.i();
                let map_object_state_d_upper_12 =
                    r.get(object_8, field::map_object::STATE_D_UPPER)?.i();
                if !((drawable_x_15.wrapping_add(map_object_width_19))
                    .wrapping_add(map_object_state_d_upper_12)
                    >= x.wrapping_add(4))
                {
                    let drawable_x_19 = r.get(object_8, field::drawable::X)?.i();
                    let drawable_y_20 = r.get(object_8, field::drawable::Y)?.i();
                    let player_facing_18 = r.get(player, field::player::FACING)?.i();
                    let can_entity_move_value_4 = super::world::can_entity_move(
                        r,
                        world,
                        drawable_x_19.wrapping_sub(1),
                        drawable_y_20,
                        player_facing_18,
                        100,
                        number_6,
                    )?;
                    if can_entity_move_value_4 != 0 {
                        if check_objects == 0 {
                            return Ok(1);
                        }
                        let drawable_x_25 = r.get(object_8, field::drawable::X)?.i();
                        r.set(
                            object_8,
                            field::drawable::X,
                            V::I(drawable_x_25.wrapping_sub(1)),
                        )?;
                        let drawable_x_26 = r.get(player, field::drawable::X)?.i();
                        r.set(
                            player,
                            field::drawable::X,
                            V::I(drawable_x_26.wrapping_sub(1)),
                        )?;
                    }
                }
            }
        }
        let map_object_trigger_kind_2 = r.get(object_8, field::map_object::TRIGGER_KIND)?.i();
        if map_object_trigger_kind_2 != 2 {
            let map_object_trigger_kind_3 = r.get(object_8, field::map_object::TRIGGER_KIND)?.i();
            if map_object_trigger_kind_3 != 3 {
                return Ok(0);
            }
            r.set(object_8, field::map_object::TRIGGER_ACTIVE, V::I(1))?;
            let map_object_state_e_3 = r.get(object_8, field::map_object::STATE_E)?.i();
            if map_object_state_e_3 != 0 {
                return Ok(0);
            }
        } else {
            r.set(object_8, field::map_object::TRIGGER_ACTIVE, V::I(1))?;
            let map_object_state_e_2 = r.get(object_8, field::map_object::STATE_E)?.i();
            if map_object_state_e_2 != 0 {
                return Ok(0);
            }
        }
        } // advance_index
        number_6 = number_6.wrapping_add(1);
        continue 'can_player_move_loop_2;
    }
}

/// Can entity move. Original reference: `j.a(IIIII)Z`.
pub(in crate::game) fn can_entity_move(
    r: &mut Runtime,
    world: u32,
    mut x: i32,
    mut y: i32,
    direction: i32,
    kind: i32,
    object_index: i32,
) -> Result<i32, u32> {
    let mut number_6: i32 = 0;
    let mut number_7: i32 = 0;
    let mut number_8: i32 = 0;
    let mut object_9: u32 = 0;
    'check_world_objects: {
        y = y.wrapping_sub(32);
        number_6 = 0;
        number_7 = 0;
        number_8 = 0;
        if direction == 0 {
            number_8 = -8;
        }

        if direction == 1 {
            number_7 = 8;
        }

        if direction == 2 {
            number_8 = 8;
        }

        if direction == 3 {
            number_7 = -8;
        }

        if kind == 0 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 1 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 2 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 3 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 4 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 5 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 6 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 7 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 8 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 9 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 10 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 11 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 12 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 13 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 15 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 16 {
            x = x.wrapping_add(30);
            y = y.wrapping_add(32);
        }

        if kind == 19 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 25 {
            x = x.wrapping_add(48);
            y = y.wrapping_add(48);
        }

        if kind == 26 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }

        if kind == 28 {
            x = x.wrapping_add(16);
            y = y.wrapping_add(16);
        }

        if kind == 27 {
            x = x.wrapping_add(16);
            y = y.wrapping_add(16);
        }

        if kind == 29 {
            x = x.wrapping_add(48);
            y = y.wrapping_add(48);
        }

        if kind == 30 {
            x = x.wrapping_add(12);
            y = y.wrapping_add(12);
        }

        if kind == 31 {
            x = x.wrapping_add(12);
            y = y.wrapping_add(12);
        }

        if kind == 32 {
            x = x.wrapping_add(12);
            y = y.wrapping_add(12);
        }

        if kind == 100 {
            x = x.wrapping_add(8);
            y = y.wrapping_add(8);
        }
        let world_width = r.get(world, field::world::WIDTH)?.i();
        number_6 = (((y.wrapping_add(number_8)).wrapping_shr((4) as u32))
            .wrapping_mul(world_width))
        .wrapping_add((x.wrapping_add(number_7)).wrapping_shr((4) as u32));
        let world_width_2 = r.get(world, field::world::WIDTH)?.i();
        let world_height = r.get(world, field::world::HEIGHT)?.i();
        if (!((((y.wrapping_add(number_8)).wrapping_shr((4) as u32)).wrapping_mul(world_width))
            .wrapping_add((x.wrapping_add(number_7)).wrapping_shr((4) as u32))
            > world_width_2.wrapping_mul(world_height)))
            && (number_6 > 0)
        {
            let world_tiles = r.get(world, field::world::TILES)?.r();
            let element_value = r.array_get(world_tiles, number_6)?.i();
            number_6 = element_value;
            if !((element_value & 64) != 64) {
                return Ok(0);
            }

            if (number_6 & 128) != 128 {
                if kind == 26 {
                    return Ok(0);
                }

                if kind == 33 {
                    return Ok(0);
                }
            } else {
                if ((kind != 18) && (kind != 26)) && (kind != 33) {
                    return Ok(0);
                }
            }
        }
        y = y.wrapping_add(32);
        number_6 = 0;
    }
    'can_entity_move_loop_2: loop {
        'advance_index: {
            'next_step_2: {
                'check_map_object_trigger_kind: {
                    let world_objects = r.get(world, field::world::OBJECTS)?.r();
                    let vector_size_value = host::vector_size(r, world_objects)?;
                    if number_6 >= vector_size_value {
                        if kind != 100 {
                            let find_portal_value = super::world::find_portal(r, world, x, y, 1)?;
                            if find_portal_value <= 0 {
                                return Ok(1);
                            }
                            return Ok(0);
                        }
                        number_6 = 0;
                        loop {
                            let world_enemies = r.get(world, field::world::ENEMIES)?.r();
                            let vector_size_value_2 = host::vector_size(r, world_enemies)?;
                            if number_6 >= vector_size_value_2 {
                                return Ok(1);
                            }
                            let world_enemies_2 = r.get(world, field::world::ENEMIES)?.r();
                            let vector_element_at_value_2 =
                                host::vector_element_at(r, world_enemies_2, number_6)?;
                            host::check_cast(r, vector_element_at_value_2, "l")?;
                            object_9 = vector_element_at_value_2;
                            let drawable_x =
                                r.get(vector_element_at_value_2, field::drawable::X)?.i();
                            if !(drawable_x > x.wrapping_add(number_7)) {
                                let drawable_x_3 = r.get(object_9, field::drawable::X)?.i();
                                let enemy_width_3 = r.get(object_9, field::enemy::WIDTH)?.i();
                                if !(drawable_x_3.wrapping_add(enemy_width_3)
                                    < x.wrapping_add(number_7))
                                {
                                    let drawable_y = r.get(object_9, field::drawable::Y)?.i();
                                    if !(drawable_y > y.wrapping_add(number_8)) {
                                        let drawable_y_3 = r.get(object_9, field::drawable::Y)?.i();
                                        let enemy_height_2 =
                                            r.get(object_9, field::enemy::HEIGHT)?.i();
                                        if !(drawable_y_3.wrapping_add(enemy_height_2)
                                            < y.wrapping_add(number_8))
                                        {
                                            if direction == 4 {
                                                return Ok(0);
                                            }

                                            if direction == 0 {
                                                let drawable_x_5 =
                                                    r.get(object_9, field::drawable::X)?.i();
                                                let drawable_y_5 =
                                                    r.get(object_9, field::drawable::Y)?.i();
                                                let enemy_kind =
                                                    r.get(object_9, field::enemy::KIND)?.i();
                                                let can_entity_move_value =
                                                    super::world::can_entity_move(
                                                        r,
                                                        world,
                                                        drawable_x_5,
                                                        drawable_y_5.wrapping_sub(5),
                                                        0,
                                                        enemy_kind,
                                                        0,
                                                    )?;
                                                if can_entity_move_value == 0 {
                                                    return Ok(0);
                                                }
                                                let drawable_y_7 =
                                                    r.get(object_9, field::drawable::Y)?.i();
                                                r.set(
                                                    object_9,
                                                    field::drawable::Y,
                                                    V::I(drawable_y_7.wrapping_sub(1)),
                                                )?;
                                                let enemy_kind_3 =
                                                    r.get(object_9, field::enemy::KIND)?.i();
                                                if enemy_kind_3 == 9 {
                                                    r.set(
                                                        object_9,
                                                        field::enemy::STATE_Q,
                                                        V::I(1),
                                                    )?;
                                                }
                                            }

                                            if direction == 1 {
                                                let drawable_x_6 =
                                                    r.get(object_9, field::drawable::X)?.i();
                                                let drawable_y_6 =
                                                    r.get(object_9, field::drawable::Y)?.i();
                                                let enemy_kind_2 =
                                                    r.get(object_9, field::enemy::KIND)?.i();
                                                let can_entity_move_value_2 =
                                                    super::world::can_entity_move(
                                                        r,
                                                        world,
                                                        drawable_x_6.wrapping_add(5),
                                                        drawable_y_6,
                                                        1,
                                                        enemy_kind_2,
                                                        0,
                                                    )?;
                                                if can_entity_move_value_2 == 0 {
                                                    return Ok(0);
                                                }
                                                let drawable_x_8 =
                                                    r.get(object_9, field::drawable::X)?.i();
                                                r.set(
                                                    object_9,
                                                    field::drawable::X,
                                                    V::I(drawable_x_8.wrapping_add(1)),
                                                )?;
                                                let enemy_kind_5 =
                                                    r.get(object_9, field::enemy::KIND)?.i();
                                                if enemy_kind_5 == 9 {
                                                    r.set(
                                                        object_9,
                                                        field::enemy::STATE_P,
                                                        V::I(1),
                                                    )?;
                                                }
                                            }

                                            if direction == 2 {
                                                let drawable_x_7 =
                                                    r.get(object_9, field::drawable::X)?.i();
                                                let drawable_y_8 =
                                                    r.get(object_9, field::drawable::Y)?.i();
                                                let enemy_kind_4 =
                                                    r.get(object_9, field::enemy::KIND)?.i();
                                                let can_entity_move_value_3 =
                                                    super::world::can_entity_move(
                                                        r,
                                                        world,
                                                        drawable_x_7,
                                                        drawable_y_8.wrapping_add(5),
                                                        2,
                                                        enemy_kind_4,
                                                        0,
                                                    )?;
                                                if can_entity_move_value_3 == 0 {
                                                    return Ok(0);
                                                }
                                                let drawable_y_10 =
                                                    r.get(object_9, field::drawable::Y)?.i();
                                                r.set(
                                                    object_9,
                                                    field::drawable::Y,
                                                    V::I(drawable_y_10.wrapping_add(1)),
                                                )?;
                                                let enemy_kind_7 =
                                                    r.get(object_9, field::enemy::KIND)?.i();
                                                if enemy_kind_7 == 9 {
                                                    r.set(
                                                        object_9,
                                                        field::enemy::STATE_Q,
                                                        V::I(1),
                                                    )?;
                                                }
                                            }

                                            if direction == 3 {
                                                let drawable_x_9 =
                                                    r.get(object_9, field::drawable::X)?.i();
                                                let drawable_y_9 =
                                                    r.get(object_9, field::drawable::Y)?.i();
                                                let enemy_kind_6 =
                                                    r.get(object_9, field::enemy::KIND)?.i();
                                                let can_entity_move_value_4 =
                                                    super::world::can_entity_move(
                                                        r,
                                                        world,
                                                        drawable_x_9.wrapping_sub(5),
                                                        drawable_y_9,
                                                        3,
                                                        enemy_kind_6,
                                                        0,
                                                    )?;
                                                if can_entity_move_value_4 == 0 {
                                                    return Ok(0);
                                                }
                                                let drawable_x_10 =
                                                    r.get(object_9, field::drawable::X)?.i();
                                                r.set(
                                                    object_9,
                                                    field::drawable::X,
                                                    V::I(drawable_x_10.wrapping_sub(1)),
                                                )?;
                                                let enemy_kind_8 =
                                                    r.get(object_9, field::enemy::KIND)?.i();
                                                if enemy_kind_8 == 9 {
                                                    r.set(
                                                        object_9,
                                                        field::enemy::STATE_P,
                                                        V::I(1),
                                                    )?;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            number_6 = number_6.wrapping_add(1);
                        }
                    }
                    let world_objects_2 = r.get(world, field::world::OBJECTS)?.r();
                    let vector_element_at_value =
                        host::vector_element_at(r, world_objects_2, number_6)?;
                    host::check_cast(r, vector_element_at_value, "n")?;
                    object_9 = vector_element_at_value;
                    let state_e_value = r
                        .get(vector_element_at_value, field::map_object::STATE_E)?
                        .i();
                    if state_e_value != 0 {
                        break 'next_step_2;
                    }
                }
                let map_object_trigger_kind = r.get(object_9, field::map_object::TRIGGER_KIND)?.i();
                if map_object_trigger_kind < 2 {
                    break 'advance_index;
                }
            }

            if !((object_index == number_6) && (kind == 100)) {
                let map_object_solid = r.get(object_9, field::map_object::SOLID)?.i();
                if map_object_solid != 0 {
                    let drawable_x_2 = r.get(object_9, field::drawable::X)?.i();
                    let map_object_state_d_upper =
                        r.get(object_9, field::map_object::STATE_D_UPPER)?.i();
                    if !(drawable_x_2.wrapping_add(map_object_state_d_upper) > x.wrapping_add(6)) {
                        let drawable_x_4 = r.get(object_9, field::drawable::X)?.i();
                        let map_object_state_d_upper_2 =
                            r.get(object_9, field::map_object::STATE_D_UPPER)?.i();
                        let map_object_width_4 = r.get(object_9, field::map_object::WIDTH)?.i();
                        if !((drawable_x_4.wrapping_add(map_object_state_d_upper_2))
                            .wrapping_add(map_object_width_4)
                            < x.wrapping_sub(6))
                        {
                            let drawable_y_2 = r.get(object_9, field::drawable::Y)?.i();
                            let map_object_state_e_upper =
                                r.get(object_9, field::map_object::STATE_E_UPPER)?.i();
                            if !(drawable_y_2.wrapping_add(map_object_state_e_upper)
                                > y.wrapping_add(6))
                            {
                                let drawable_y_4 = r.get(object_9, field::drawable::Y)?.i();
                                let map_object_state_e_upper_2 =
                                    r.get(object_9, field::map_object::STATE_E_UPPER)?.i();
                                let map_object_height_3 =
                                    r.get(object_9, field::map_object::HEIGHT)?.i();
                                if !((drawable_y_4.wrapping_add(map_object_state_e_upper_2))
                                    .wrapping_add(map_object_height_3)
                                    < y.wrapping_sub(6))
                                {
                                    let map_object_trigger_kind_2 =
                                        r.get(object_9, field::map_object::TRIGGER_KIND)?.i();
                                    if map_object_trigger_kind_2 != 2 {
                                        let map_object_trigger_kind_3 =
                                            r.get(object_9, field::map_object::TRIGGER_KIND)?.i();
                                        if map_object_trigger_kind_3 != 3 {
                                            return Ok(0);
                                        }
                                        r.set(
                                            object_9,
                                            field::map_object::TRIGGER_ACTIVE,
                                            V::I(1),
                                        )?;
                                    } else {
                                        r.set(
                                            object_9,
                                            field::map_object::TRIGGER_ACTIVE,
                                            V::I(1),
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        number_6 = number_6.wrapping_add(1);
        continue 'can_entity_move_loop_2;
    }
}

/// Find portal. Original reference: `j.a(IIZ)I`.
pub(in crate::game) fn find_portal(
    r: &mut Runtime,
    world: u32,
    mut x: i32,
    mut y: i32,
    mut expanded: i32,
) -> Result<i32, u32> {
    let mut object_4: u32 = 0;

    y = y.wrapping_sub(32);
    if expanded == 0 {
        x = x.wrapping_add(12);
        y = y.wrapping_add(16);
    }

    if expanded == 0 {
        expanded = 0;
        loop {
            let world_portals = r.get(world, field::world::PORTALS)?.r();
            let vector_size_value = host::vector_size(r, world_portals)?;
            if expanded >= vector_size_value {
                return Ok(-1);
            }
            let world_portals_3 = r.get(world, field::world::PORTALS)?.r();
            let vector_element_at_value = host::vector_element_at(r, world_portals_3, expanded)?;
            host::check_cast(r, vector_element_at_value, "e")?;
            object_4 = vector_element_at_value;
            let portal_tile_x = r.get(vector_element_at_value, field::portal::TILE_X)?.i();
            if !(portal_tile_x.wrapping_shl((4) as u32) > x) {
                let portal_tile_x_3 = r.get(object_4, field::portal::TILE_X)?.i();
                if !((portal_tile_x_3.wrapping_add(1)).wrapping_shl((4) as u32) < x) {
                    let portal_tile_y = r.get(object_4, field::portal::TILE_Y)?.i();
                    if !(portal_tile_y.wrapping_shl((4) as u32) > y) {
                        let portal_tile_y_3 = r.get(object_4, field::portal::TILE_Y)?.i();
                        if !((portal_tile_y_3.wrapping_add(1)).wrapping_shl((4) as u32) < y) {
                            return Ok(expanded.wrapping_add(1));
                        }
                    }
                }
            }
            expanded = expanded.wrapping_add(1);
        }
    }
    expanded = 0;
    loop {
        let world_portals_2 = r.get(world, field::world::PORTALS)?.r();
        let vector_size_value_2 = host::vector_size(r, world_portals_2)?;
        if expanded >= vector_size_value_2 {
            return Ok(-1);
        }
        let world_portals_4 = r.get(world, field::world::PORTALS)?.r();
        let vector_element_at_value_2 = host::vector_element_at(r, world_portals_4, expanded)?;
        host::check_cast(r, vector_element_at_value_2, "e")?;
        object_4 = vector_element_at_value_2;
        let portal_tile_x_2 = r.get(vector_element_at_value_2, field::portal::TILE_X)?.i();
        if !((portal_tile_x_2.wrapping_shl((4) as u32)).wrapping_sub(4) > x) {
            let portal_tile_x_4 = r.get(object_4, field::portal::TILE_X)?.i();
            if !(((portal_tile_x_4.wrapping_add(1)).wrapping_shl((4) as u32)).wrapping_add(4) < x) {
                let portal_tile_y_2 = r.get(object_4, field::portal::TILE_Y)?.i();
                if !((portal_tile_y_2.wrapping_shl((4) as u32)).wrapping_sub(4) > y) {
                    let portal_tile_y_4 = r.get(object_4, field::portal::TILE_Y)?.i();
                    if !(((portal_tile_y_4.wrapping_add(1)).wrapping_shl((4) as u32))
                        .wrapping_add(4)
                        < y)
                    {
                        return Ok(expanded.wrapping_add(1));
                    }
                }
            }
        }
        expanded = expanded.wrapping_add(1);
    }
}
