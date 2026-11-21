//! World enemies.
use super::*;

/// Update enemies. Original reference: `j.b(Lh;)V`.
#[rustfmt::skip]
pub(in crate::game) fn update_enemies(
    r: &mut Runtime,
    world: u32,
    player: u32,
) -> Result<(), u32> {
    let mut number_2: i32 = 0;
    let mut object_3: u32 = 0;
    let mut number_4: i32 = 0;
    let mut number_5: i32 = 0;
    let mut number_6: i32 = 0;
    let mut object_6: u32 = 0;
    let mut number_7: i32 = 0;
    let mut object_7: u32 = 0;
    let mut object_8: u32 = 0;
    let mut number_9: i32 = 0;
    let mut object_9: u32 = 0;
    let mut number_10: i32 = 0;
    let mut object_10: u32 = 0;
    let mut number_11: i32 = 0;
    let mut object_11: u32 = 0;
    let mut number_12: i32 = 0;
    let mut object_12: u32 = 0;
    'check_world_enemies: {
    number_2 = 0;
    } // check_world_enemies
    'update_enemies_loop_2: loop {
        'advance_index: {
        'check_world_enemies_4: {
        'next_step_28: {
        'next_step_27: {
        'check_enemy_kind_5: {
        'check_world_random: {
        'check_drawable_x_19: {
        'check_enemy_facing: {
        'check_enemy_state_h: {
        'check_enemy_state_e_lower_2: {
        'check_drawable_x_12: {
        'check_enemy_kind_3: {
        'check_enemy_kind_4: {
        'check_enemy_kind_2: {
        'check_enemy_state_e_lower: {
        'check_enemy_state_f_lower: {
        'check_enemy_facing_8: {
        'check_enemy_facing_14: {
        'check_enemy_state_z_36: {
        'check_enemy_facing_12: {
        'check_enemy_state_f_10: {
        'check_enemy_state_z_16: {
        'check_enemy_state_r_6: {
        'check_trail_point_x: {
        'check_enemy_facing_11: {
        'check_trail_point_y: {
        'check_enemy_state_r_2: {
        'next_step_8: {
        'check_enemy_state_z_17: {
        'check_enemy_state_f_11: {
        'next_step_22: {
        'next_step_18: {
        'check_enemy_state_f_9: {
        'check_enemy_state_z_29: {
        'check_enemy_facing_7: {
        'check_enemy_facing_4: {
        'check_enemy_facing_5: {
        'check_enemy_state_h_3: {
        'check_enemy_state_z_23: {
        'check_enemy_state_z_15: {
        'check_enemy_state_z_28: {
        'check_enemy_state_f_8: {
        'next_step_17: {
        'next_step_13: {
        'check_enemy_state_f_5: {
        'check_enemy_state_z_22: {
        'check_enemy_facing_6: {
        'check_enemy_facing_2: {
        'check_enemy_facing_3: {
        'check_enemy_state_h_2: {
        'check_drawable_x_3: {
        'check_enemy_state_z_14: {
        'next_step_7: {
        'check_enemy_state_z_13: {
        'check_enemy_state_z_12: {
        'check_enemy_state_z_11: {
        'check_enemy_state_z_10: {
        'check_enemy_state_z_9: {
        'check_enemy_state_d_lower: {
        'check_enemy_state_z_8: {
        'next_step_6: {
        'check_enemy_state_f_14: {
        'next_step_29: {
        'check_enemy_random: {
        'check_world_enemies_2: {
        'next_step_26: {
        'check_enemy_active_6: {
        'check_enemy_state_z_35: {
        'check_enemy_active_3: {
        'check_enemy_state_z_32: {
        'check_enemy_state_z_27: {
        'next_step_20: {
        'check_enemy_active: {
        'check_enemy_state_z_7: {
        'check_enemy_state_z_43: {
        'check_enemy_state_z_46: {
        'check_enemy_state_z_48: {
        'check_drawable_y_25: {
        'check_drawable_y_23: {
        'check_drawable_x_23: {
        'check_enemy_facing_17: {
        'check_enemy_state_z_45: {
        'check_enemy_state_z_47: {
        'check_drawable_y_24: {
        'check_drawable_y_22: {
        'check_drawable_x_22: {
        'check_enemy_active_5: {
        'check_enemy_state_z_37: {
        'check_enemy_width_2: {
        'check_enemy_width_3: {
        'check_drawable_x_21: {
        'check_enemy_width: {
        'check_enemy_state_o: {
        'check_enemy_active_4: {
        'check_enemy_state_z_31: {
        'check_enemy_state_f_13: {
        'check_enemy_state_z_41: {
        'check_enemy_state_z_44: {
        'check_drawable_y_20: {
        'check_drawable_y_15: {
        'check_drawable_y_16: {
        'check_drawable_x_17: {
        'check_enemy_facing_10: {
        'check_enemy_state_z_40: {
        'check_enemy_state_z_42: {
        'check_drawable_y_17: {
        'check_drawable_y_10: {
        'check_drawable_y_11: {
        'check_drawable_x_14: {
        'check_enemy_active_2: {
        'check_enemy_state_z_6: {
        'check_enemy_state_z_30: {
        'check_enemy_facing_15: {
        'check_enemy_state_f_7: {
        'check_enemy_state_z_26: {
        'check_enemy_facing_9: {
        'check_enemy_state_f_4: {
        'check_enemy_state_z_21: {
        'check_enemy_state_f_2: {
        'check_enemy_state_z_5: {
        'check_enemy_state_f_12: {
        'check_enemy_state_o_lower: {
        'check_enemy_state_o_lower_2: {
        'check_enemy_state_z_34: {
        'check_enemy_state_z_4: {
        'check_enemy_state_r_5: {
        'next_step_12: {
        'check_enemy_state_s: {
        'next_step_10: {
        'next_step_5: {
        'check_enemy_facing_16: {
        'next_step_23: {
        'next_step_24: {
        'check_enemy_state_f_6: {
        'check_enemy_state_z_25: {
        'check_enemy_facing_13: {
        'next_step_15: {
        'next_step_16: {
        'check_enemy_state_f_3: {
        'check_enemy_state_z_20: {
        'check_enemy_state_f: {
        'check_enemy_state_z_3: {
        'check_drawable_y_19: {
        'check_world_objects: {
        'next_step_4: {
        'check_enemy_state_z_19: {
        'check_drawable_y_21: {
        'check_drawable_x_20: {
        'check_drawable_x_18: {
        'next_step_19: {
        'check_drawable_y_18: {
        'check_drawable_y_13: {
        'check_drawable_x_15: {
        'next_step_14: {
        'check_drawable_y_14: {
        'check_drawable_x_16: {
        'check_drawable_x_13: {
        'next_step_11: {
        'check_drawable_y_9: {
        'check_drawable_y_6: {
        'check_drawable_x_9: {
        'random_next_int: {
        'next_step_9: {
        'check_enemy_state_z_2: {
        'check_enemy_state_z_24: {
        'check_enemy_state_z_33: {
        'check_drawable_x_11: {
        'check_drawable_x_7: {
        'check_enemy_state_z_38: {
        'check_drawable_y_12: {
        'check_drawable_y_7: {
        'check_enemy_state_z_39: {
        'check_drawable_y_8: {
        'check_drawable_y_5: {
        'check_drawable_x_8: {
        'check_drawable_x_6: {
        'check_enemy_state_z_18: {
        'check_drawable_x_5: {
        'check_enemy_state_z: {
        'next_step_3: {
        'check_drawable_y_2: {
        'check_drawable_x_10: {
        'check_enemy_state_q: {
        'check_drawable_y_3: {
        'check_drawable_y: {
        'check_drawable_y_4: {
        'check_enemy_state_p: {
        'check_drawable_x_4: {
        'check_drawable_x_2: {
        'check_enemy_state_r: {
        'next_step_2: {
        'check_drawable_x: {
        'check_enemy_kind: {
        'check_enemy_knockback_timer: {
        'check_enemy_knockback_timer_2: {
        let world_enemies = r.get(world, field::world::ENEMIES)?.r();
        let vector_size_value = host::vector_size(r, world_enemies)?;
        if number_2 >= vector_size_value {
return Ok(());
}
        let world_enemies_2 = r.get(world, field::world::ENEMIES)?.r();
        let vector_element_at_value = host::vector_element_at(r, world_enemies_2, number_2)?;
        host::check_cast(r, vector_element_at_value, "l")?;
        object_3 = vector_element_at_value;
        number_4 = 0;
        number_5 = 0;
        object_8 = world;
        object_7 = player;
        object_6 = object_3;
        let enemy_hurt_timer = r.get(object_3, field::enemy::HURT_TIMER)?.i();
        if enemy_hurt_timer > 0 { break 'check_enemy_knockback_timer; }
        } // check_enemy_knockback_timer_2
        let enemy_knockback_timer_3 = r.get(object_6, field::enemy::KNOCKBACK_TIMER)?.i();
        if enemy_knockback_timer_3 <= 0 { break 'check_enemy_kind; }
        } // check_enemy_knockback_timer
        let enemy_knockback_timer = r.get(object_6, field::enemy::KNOCKBACK_TIMER)?.i();
        r.set(object_6, field::enemy::KNOCKBACK_TIMER, V::I(enemy_knockback_timer.wrapping_sub(1)))?;
        let enemy_knockback_timer_2 = r.get(object_6, field::enemy::KNOCKBACK_TIMER)?.i();
        if enemy_knockback_timer_2 < 0 {
r.set(object_6, field::enemy::KNOCKBACK_TIMER, V::I(0))?;
}
        let drawable_x = r.get(object_6, field::drawable::X)?.i();
        let enemy_velocity_x = r.get(object_6, field::enemy::VELOCITY_X)?.i();
        let drawable_y = r.get(object_6, field::drawable::Y)?.i();
        let enemy_velocity_y = r.get(object_6, field::enemy::VELOCITY_Y)?.i();
        let enemy_facing = r.get(object_6, field::enemy::FACING)?.i();
        let enemy_kind = r.get(object_6, field::enemy::KIND)?.i();
        let can_entity_move_value = super::world::can_entity_move(r, object_8, drawable_x.wrapping_add(enemy_velocity_x), drawable_y.wrapping_add(enemy_velocity_y), enemy_facing, enemy_kind, 0)?;
        if can_entity_move_value != 0 {
let enemy_kind_3 = r.get(object_6, field::enemy::KIND)?.i();
if enemy_kind_3 != 16 {
let enemy_kind_4 = r.get(object_6, field::enemy::KIND)?.i();
if enemy_kind_4 != 20 {
let enemy_kind_5 = r.get(object_6, field::enemy::KIND)?.i();
if enemy_kind_5 != 37 {
let drawable_x_42 = r.get(object_6, field::drawable::X)?.i();
let enemy_velocity_x_2 = r.get(object_6, field::enemy::VELOCITY_X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_42.wrapping_add(enemy_velocity_x_2)))?;
let drawable_y_30 = r.get(object_6, field::drawable::Y)?.i();
let enemy_velocity_y_2 = r.get(object_6, field::enemy::VELOCITY_Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_30.wrapping_add(enemy_velocity_y_2)))?;
}
}
}
}
        } // check_enemy_kind
        let enemy_kind_2 = r.get(object_6, field::enemy::KIND)?.i();
        match enemy_kind_2 {
7 => {  },
8 => { break 'next_step_2; },
9 => { break 'check_drawable_x_2; },
10 => { break 'next_step_3; },
11 => { break 'check_enemy_state_z; },
13 => { break 'check_enemy_state_z_2; },
14 => { break 'next_step_4; },
19 => { break 'check_enemy_state_z_3; },
20 => { break 'next_step_5; },
21 => { break 'check_enemy_state_z_4; },
22 => { break 'check_enemy_state_z_5; },
23 => { break 'check_enemy_state_z_6; },
24 => { break 'check_enemy_state_z_7; },
25 => { break 'next_step_6; },
26 => { break 'check_enemy_state_z_8; },
27 => { break 'check_enemy_state_d_lower; },
28 => { break 'check_enemy_state_z_9; },
29 => { break 'check_enemy_state_z_10; },
30 | 31 => { break 'check_enemy_state_z_11; },
32 => { break 'check_enemy_state_z_12; },
33 => { break 'check_enemy_state_z_13; },
35 => { break 'next_step_7; },
36 => { break 'check_enemy_state_z_14; },
37 => { break 'check_drawable_x_3; },
38 => { break 'check_enemy_state_z_15; },
133 => { break 'next_step_8; },
134 => { break 'check_enemy_state_z_16; },
135 => { break 'check_enemy_state_z_17; },
_ => { break 'check_enemy_state_f_lower; },
}
        } // check_drawable_x
        let drawable_x_2 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_3 = r.get(object_6, field::drawable::X)?.i();
        if !(drawable_x_2.wrapping_add(24) <= drawable_x_3) {
let drawable_x_13 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_14 = r.get(object_6, field::drawable::X)?.i();
if !(drawable_x_13 >= drawable_x_14.wrapping_add(16)) {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(2))?;
let drawable_y_16 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_17 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_16 > drawable_y_17 {
let enemy_facing_12 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_12 == 2 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(3))?;
}
}
let drawable_y_31 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_32 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_31 < drawable_y_32 {
let enemy_facing_19 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_19 == 0 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(3))?;
}
}
}
}
        let drawable_y_7 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_8 = r.get(object_6, field::drawable::Y)?.i();
        if !(drawable_y_7.wrapping_add(24) <= drawable_y_8) {
let drawable_y_14 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_15 = r.get(object_6, field::drawable::Y)?.i();
if !(drawable_y_14 >= drawable_y_15.wrapping_add(16)) {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(2))?;
let drawable_x_43 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_44 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_43 > drawable_x_44 {
let enemy_facing_18 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_18 == 1 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(3))?;
}
}
let drawable_x_72 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_73 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_72 < drawable_x_73 {
let enemy_facing_26 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_26 == 3 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(3))?;
}
}
}
}
        break 'check_enemy_state_f_lower;
        } // next_step_2
        let trail_point_value = r.alloc("g", Native::Empty);
        super::trail_point::initialize(r, trail_point_value)?;
        object_9 = trail_point_value;
        let drawable_x_4 = r.get(object_6, field::drawable::X)?.i();
        r.set(trail_point_value, field::trail_point::X, V::I(drawable_x_4))?;
        let drawable_y_2 = r.get(object_6, field::drawable::Y)?.i();
        r.set(object_9, field::trail_point::Y, V::I(drawable_y_2))?;
        let enemy_state_t = r.get(object_6, field::enemy::STATE_T)?.r();
        host::vector_add_element(r, enemy_state_t, object_9)?;
        let enemy_state_t_2 = r.get(object_6, field::enemy::STATE_T)?.r();
        host::vector_remove_element_at(r, enemy_state_t_2, 0)?;
        number_10 = 0;
        } // check_enemy_state_r
        'update_enemies_loop_8: loop {
            'check_enemy_state_r_3: {
            let enemy_state_r_6 = r.get(object_6, field::enemy::STATE_R)?.r();
            let vector_size_value_5 = host::vector_size(r, enemy_state_r_6)?;
            if number_10 >= vector_size_value_5 { break 'check_enemy_state_f_lower; }
            } // check_enemy_state_r_3
            let enemy_state_r_8 = r.get(object_6, field::enemy::STATE_R)?.r();
            let vector_element_at_value_3 = host::vector_element_at(r, enemy_state_r_8, number_10)?;
            host::check_cast(r, vector_element_at_value_3, "g")?;
            object_11 = vector_element_at_value_3;
            let enemy_state_t_3 = r.get(object_6, field::enemy::STATE_T)?.r();
            let vector_element_at_value_4 = host::vector_element_at(r, enemy_state_t_3, number_10.wrapping_mul(5))?;
            host::check_cast(r, vector_element_at_value_4, "g")?;
            object_12 = vector_element_at_value_4;
            let trail_point_x_30 = r.get(object_12, field::trail_point::X)?.i();
            r.set(object_11, field::trail_point::X, V::I(trail_point_x_30))?;
            let trail_point_y_18 = r.get(object_12, field::trail_point::Y)?.i();
            r.set(object_11, field::trail_point::Y, V::I(trail_point_y_18))?;
            let enemy_state_r_9 = r.get(object_6, field::enemy::STATE_R)?.r();
            host::vector_set_element_at(r, enemy_state_r_9, object_11, number_10)?;
            number_10 = number_10.wrapping_add(1);
            continue 'update_enemies_loop_8;
        }
        } // check_drawable_x_2
        let drawable_x_5 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_6 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_5.wrapping_add(18) <= drawable_x_6 { break 'check_drawable_y; }
        } // check_drawable_x_4
        let drawable_x_15 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_16 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_15.wrapping_add(6) >= drawable_x_16.wrapping_add(16) { break 'check_drawable_y; }
        } // check_enemy_state_p
        let enemy_state_p = r.get(object_6, field::enemy::STATE_P)?.i();
        if enemy_state_p != 0 { break 'check_drawable_y; }
        } // check_drawable_y_4
        let drawable_y_35 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_36 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_35 > drawable_y_36 {
let drawable_x_78 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_65 = r.get(object_6, field::drawable::Y)?.i();
let can_entity_move_value_5 = super::world::can_entity_move(r, object_8, drawable_x_78, drawable_y_65.wrapping_add(5), 2, 9, 0)?;
if can_entity_move_value_5 != 0 {
let drawable_y_83 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_83.wrapping_add(3)))?;
r.set(object_6, field::enemy::STATE_Q, V::I(1))?;
}
}
        let drawable_y_63 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_64 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_63 < drawable_y_64 {
let drawable_x_107 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_82 = r.get(object_6, field::drawable::Y)?.i();
let can_entity_move_value_9 = super::world::can_entity_move(r, object_8, drawable_x_107, drawable_y_82.wrapping_sub(5), 0, 9, 0)?;
if can_entity_move_value_9 != 0 {
let drawable_y_127 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_127.wrapping_sub(3)))?;
r.set(object_6, field::enemy::STATE_Q, V::I(1))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_drawable_y
        let drawable_y_9 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_10 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_9.wrapping_add(18) <= drawable_y_10 { break 'check_drawable_y_2; }
        } // check_drawable_y_3
        let drawable_y_20 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_21 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_20.wrapping_add(6) >= drawable_y_21.wrapping_add(16) { break 'check_drawable_y_2; }
        } // check_enemy_state_q
        let enemy_state_q = r.get(object_6, field::enemy::STATE_Q)?.i();
        if enemy_state_q != 0 { break 'check_drawable_y_2; }
        } // check_drawable_x_10
        let drawable_x_76 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_77 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_76 > drawable_x_77 {
let drawable_x_106 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_81 = r.get(object_6, field::drawable::Y)?.i();
let can_entity_move_value_8 = super::world::can_entity_move(r, object_8, drawable_x_106.wrapping_add(5), drawable_y_81, 1, 9, 0)?;
if can_entity_move_value_8 != 0 {
let drawable_x_153 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_153.wrapping_add(3)))?;
r.set(object_6, field::enemy::STATE_P, V::I(1))?;
}
}
        let drawable_x_104 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_105 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_104 < drawable_x_105 {
let drawable_x_152 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_126 = r.get(object_6, field::drawable::Y)?.i();
let can_entity_move_value_12 = super::world::can_entity_move(r, object_8, drawable_x_152.wrapping_sub(5), drawable_y_126, 3, 9, 0)?;
if can_entity_move_value_12 != 0 {
let drawable_x_186 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_186.wrapping_sub(3)))?;
r.set(object_6, field::enemy::STATE_P, V::I(1))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_drawable_y_2
        let drawable_y_19 = r.get(object_6, field::drawable::Y)?.i();
        let enemy_state_e = r.get(object_6, field::enemy::STATE_E)?.i();
        if drawable_y_19 > enemy_state_e {
let drawable_x_45 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_34 = r.get(object_6, field::drawable::Y)?.i();
let can_entity_move_value_2 = super::world::can_entity_move(r, object_8, drawable_x_45, drawable_y_34.wrapping_sub(3), 0, 9, 0)?;
if can_entity_move_value_2 == 0 {
let drawable_y_61 = r.get(object_6, field::drawable::Y)?.i();
let enemy_state_e_4 = r.get(object_6, field::enemy::STATE_E)?.i();
if !(drawable_y_61.wrapping_sub(4) > enemy_state_e_4) {
let enemy_state_e_6 = r.get(object_6, field::enemy::STATE_E)?.i();
r.set(object_6, field::drawable::Y, V::I(enemy_state_e_6))?;
}
} else {
let drawable_y_62 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_62.wrapping_sub(1)))?;
}
}
        let drawable_y_33 = r.get(object_6, field::drawable::Y)?.i();
        let enemy_state_e_2 = r.get(object_6, field::enemy::STATE_E)?.i();
        if drawable_y_33 < enemy_state_e_2 {
let drawable_x_75 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_60 = r.get(object_6, field::drawable::Y)?.i();
let can_entity_move_value_4 = super::world::can_entity_move(r, object_8, drawable_x_75, drawable_y_60.wrapping_add(3), 2, 9, 0)?;
if can_entity_move_value_4 == 0 {
let drawable_y_79 = r.get(object_6, field::drawable::Y)?.i();
let enemy_state_e_5 = r.get(object_6, field::enemy::STATE_E)?.i();
if !(drawable_y_79.wrapping_add(4) < enemy_state_e_5) {
let enemy_state_e_11 = r.get(object_6, field::enemy::STATE_E)?.i();
r.set(object_6, field::drawable::Y, V::I(enemy_state_e_11))?;
}
} else {
let drawable_y_80 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_80.wrapping_add(1)))?;
}
}
        let drawable_x_74 = r.get(object_6, field::drawable::X)?.i();
        let enemy_state_d_2 = r.get(object_6, field::enemy::STATE_D)?.i();
        if drawable_x_74 > enemy_state_d_2 {
let drawable_x_103 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_78 = r.get(object_6, field::drawable::Y)?.i();
let can_entity_move_value_7 = super::world::can_entity_move(r, object_8, drawable_x_103.wrapping_sub(3), drawable_y_78, 3, 9, 0)?;
if can_entity_move_value_7 == 0 {
let drawable_x_150 = r.get(object_6, field::drawable::X)?.i();
let enemy_state_d_5 = r.get(object_6, field::enemy::STATE_D)?.i();
if !(drawable_x_150.wrapping_sub(4) > enemy_state_d_5) {
let enemy_state_d_8 = r.get(object_6, field::enemy::STATE_D)?.i();
r.set(object_6, field::drawable::X, V::I(enemy_state_d_8))?;
}
} else {
let drawable_x_151 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_151.wrapping_sub(1)))?;
}
}
        let drawable_x_102 = r.get(object_6, field::drawable::X)?.i();
        let enemy_state_d_4 = r.get(object_6, field::enemy::STATE_D)?.i();
        if drawable_x_102 < enemy_state_d_4 {
let drawable_x_149 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_125 = r.get(object_6, field::drawable::Y)?.i();
let can_entity_move_value_11 = super::world::can_entity_move(r, object_8, drawable_x_149.wrapping_add(3), drawable_y_125, 1, 9, 0)?;
if can_entity_move_value_11 == 0 {
let drawable_x_184 = r.get(object_6, field::drawable::X)?.i();
let enemy_state_d_7 = r.get(object_6, field::enemy::STATE_D)?.i();
if !(drawable_x_184.wrapping_add(4) < enemy_state_d_7) {
let enemy_state_d_9 = r.get(object_6, field::enemy::STATE_D)?.i();
r.set(object_6, field::drawable::X, V::I(enemy_state_d_9))?;
}
} else {
let drawable_x_185 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_185.wrapping_add(1)))?;
}
}
        let drawable_y_124 = r.get(object_6, field::drawable::Y)?.i();
        let enemy_state_e_10 = r.get(object_6, field::enemy::STATE_E)?.i();
        if drawable_y_124 == enemy_state_e_10 {
r.set(object_6, field::enemy::STATE_Q, V::I(0))?;
}
        let drawable_x_183 = r.get(object_6, field::drawable::X)?.i();
        let enemy_state_d_6 = r.get(object_6, field::enemy::STATE_D)?.i();
        if drawable_x_183 == enemy_state_d_6 {
r.set(object_6, field::enemy::STATE_P, V::I(0))?;
}
        break 'check_enemy_state_f_lower;
        } // next_step_3

        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z
        let enemy_behavior_state = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state != 0 { break 'check_enemy_state_z_18; }
        } // check_drawable_x_5
        let drawable_x_17 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_18 = r.get(object_6, field::drawable::X)?.i();
        if !(drawable_x_17 <= drawable_x_18.wrapping_sub(48)) {
let drawable_x_33 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_34 = r.get(object_6, field::drawable::X)?.i();
if !(drawable_x_33 >= drawable_x_34.wrapping_add(48)) {
let drawable_y_37 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_38 = r.get(object_6, field::drawable::Y)?.i();
if !(drawable_y_37 <= drawable_y_38.wrapping_sub(48)) {
let drawable_y_68 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_69 = r.get(object_6, field::drawable::Y)?.i();
if !(drawable_y_68 >= drawable_y_69.wrapping_add(48)) {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
}
}
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_18
        let enemy_behavior_state_19 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_19 != 1 { break 'check_enemy_state_z_24; }
        } // check_drawable_x_6
        let drawable_x_31 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_32 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_31 <= drawable_x_32.wrapping_sub(24) { break 'check_drawable_x_7; }
        } // check_drawable_x_8
        let drawable_x_50 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_51 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_50 >= drawable_x_51.wrapping_add(24) { break 'check_drawable_x_7; }
        } // check_drawable_y_5
        let drawable_y_66 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_67 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_66 <= drawable_y_67.wrapping_sub(24) { break 'check_drawable_y_7; }
        } // check_drawable_y_8
        let drawable_y_88 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_89 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_88 >= drawable_y_89.wrapping_add(24) { break 'check_drawable_y_7; }
        } // check_enemy_state_z_39
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(3))?;
        r.set(object_6, field::enemy::STATE_H, V::I(0))?;
        let drawable_x_154 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_155 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_154 > drawable_x_155 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
        let drawable_x_187 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_188 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_187 < drawable_x_188 {
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
        let drawable_y_177 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_178 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_177 > drawable_y_178 {
r.set(object_6, field::enemy::FACING, V::I(2))?;
}
        let drawable_y_193 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_194 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_193 < drawable_y_194 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
}
        break 'check_enemy_state_f_lower;
        } // check_drawable_y_7
        let drawable_y_86 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_87 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_86 <= drawable_y_87.wrapping_sub(48) { break 'check_enemy_state_z_38; }
        } // check_drawable_y_12
        let drawable_y_130 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_131 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_130 < drawable_y_131.wrapping_add(48) { break 'check_enemy_state_f_lower; }
        } // check_enemy_state_z_38
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        break 'check_enemy_state_f_lower;
        } // check_drawable_x_7
        let drawable_x_48 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_49 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_48 <= drawable_x_49.wrapping_sub(48) { break 'check_enemy_state_z_33; }
        } // check_drawable_x_11
        let drawable_x_81 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_82 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_81 < drawable_x_82.wrapping_add(48) { break 'check_enemy_state_f_lower; }
        } // check_enemy_state_z_33
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_24
        let enemy_behavior_state_35 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_35 == 3 {
number_9 = 0;
number_10 = 0;
let drawable_x_46 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_47 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_46 > drawable_x_47 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
let drawable_x_79 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_80 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_79 < drawable_x_80 {
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
let drawable_y_84 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_85 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_84 > drawable_y_85 {
r.set(object_6, field::enemy::FACING, V::I(2))?;
}
let drawable_y_128 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_129 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_128 < drawable_y_129 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_2
        let enemy_behavior_state_2 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_2 != 0 { break 'check_enemy_state_z_19; }
        } // next_step_9
        let object_value_2 = r.alloc("java/util/Random", Native::Empty);
        host::random_initialize(r, object_value_2)?;
        object_9 = object_value_2;
        let enemy_animation = r.get(object_6, field::enemy::ANIMATION)?.i();
        host::random_set_seed(r, object_value_2, (enemy_animation.wrapping_mul(3456)) as i64)?;
        let random_next_int_value_3 = host::random_next_int(r, object_9)?;
        let remainder_value = arithmetic::remainder_int(r, random_next_int_value_3, 3)?;
        if remainder_value == 1 { break 'check_enemy_state_f_lower; }
        } // random_next_int
        let random_next_int_value_7 = host::random_next_int(r, object_9)?;
        let remainder_value_5 = arithmetic::remainder_int(r, random_next_int_value_7, 4)?;
        number_10 = remainder_value_5;
        if remainder_value_5 != 0 { break 'next_step_11; }
        } // check_drawable_x_9
        let drawable_x_53 = r.get(object_6, field::drawable::X)?.i();
        let drawable_y_39 = r.get(object_6, field::drawable::Y)?.i();
        let can_entity_move_value_3 = super::world::can_entity_move(r, object_8, drawable_x_53, drawable_y_39.wrapping_sub(32), number_10, 13, 0)?;
        if can_entity_move_value_3 == 0 { break 'next_step_11; }
        } // check_drawable_y_6
        let drawable_y_72 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_72.wrapping_sub(32) < 32 { break 'check_enemy_state_f_lower; }
        } // check_drawable_y_9
        let drawable_y_95 = r.get(object_6, field::drawable::Y)?.i();
        r.set(object_6, field::enemy::STATE_E, V::I(drawable_y_95.wrapping_sub(32)))?;
        let drawable_x_113 = r.get(object_6, field::drawable::X)?.i();
        r.set(object_6, field::enemy::STATE_D, V::I(drawable_x_113))?;
        r.set(object_6, field::enemy::FACING, V::I((number_10) as i8 as i32))?;
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
        r.set(object_6, field::enemy::STATE_A, V::I(16))?;
        } // next_step_11

        if number_10 != 1 { break 'next_step_14; }
        } // check_drawable_x_13
        let drawable_x_85 = r.get(object_6, field::drawable::X)?.i();
        let drawable_y_71 = r.get(object_6, field::drawable::Y)?.i();
        let can_entity_move_value_6 = super::world::can_entity_move(r, object_8, drawable_x_85.wrapping_add(32), drawable_y_71, number_10, 13, 0)?;
        if can_entity_move_value_6 == 0 { break 'next_step_14; }
        } // check_drawable_x_16
        let drawable_x_112 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_112.wrapping_add(32) > 240 { break 'check_enemy_state_f_lower; }
        } // check_drawable_y_14
        let drawable_y_136 = r.get(object_6, field::drawable::Y)?.i();
        r.set(object_6, field::enemy::STATE_E, V::I(drawable_y_136))?;
        let drawable_x_158 = r.get(object_6, field::drawable::X)?.i();
        r.set(object_6, field::enemy::STATE_D, V::I(drawable_x_158.wrapping_add(32)))?;
        r.set(object_6, field::enemy::FACING, V::I((number_10) as i8 as i32))?;
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
        r.set(object_6, field::enemy::STATE_A, V::I(16))?;
        } // next_step_14

        if number_10 != 2 { break 'next_step_19; }
        } // check_drawable_x_15
        let drawable_x_111 = r.get(object_6, field::drawable::X)?.i();
        let drawable_y_94 = r.get(object_6, field::drawable::Y)?.i();
        let can_entity_move_value_10 = super::world::can_entity_move(r, object_8, drawable_x_111, drawable_y_94.wrapping_add(32), number_10, 13, 0)?;
        if can_entity_move_value_10 == 0 { break 'next_step_19; }
        } // check_drawable_y_13
        let drawable_y_135 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_135.wrapping_add(32) > 176 { break 'check_enemy_state_f_lower; }
        } // check_drawable_y_18
        let drawable_y_154 = r.get(object_6, field::drawable::Y)?.i();
        r.set(object_6, field::enemy::STATE_E, V::I(drawable_y_154.wrapping_add(32)))?;
        let drawable_x_192 = r.get(object_6, field::drawable::X)?.i();
        r.set(object_6, field::enemy::STATE_D, V::I(drawable_x_192))?;
        r.set(object_6, field::enemy::FACING, V::I((number_10) as i8 as i32))?;
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
        r.set(object_6, field::enemy::STATE_A, V::I(16))?;
        } // next_step_19

        if number_10 != 3 { break 'check_enemy_state_z_19; }
        } // check_drawable_x_18
        let drawable_x_157 = r.get(object_6, field::drawable::X)?.i();
        let drawable_y_134 = r.get(object_6, field::drawable::Y)?.i();
        let can_entity_move_value_14 = super::world::can_entity_move(r, object_8, drawable_x_157.wrapping_sub(32), drawable_y_134, number_10, 13, 0)?;
        if can_entity_move_value_14 == 0 { break 'check_enemy_state_z_19; }
        } // check_drawable_x_20
        let drawable_x_191 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_191.wrapping_sub(32) < 16 { break 'check_enemy_state_f_lower; }
        } // check_drawable_y_21
        let drawable_y_179 = r.get(object_6, field::drawable::Y)?.i();
        r.set(object_6, field::enemy::STATE_E, V::I(drawable_y_179))?;
        let drawable_x_221 = r.get(object_6, field::drawable::X)?.i();
        r.set(object_6, field::enemy::STATE_D, V::I(drawable_x_221.wrapping_sub(32)))?;
        r.set(object_6, field::enemy::FACING, V::I((number_10) as i8 as i32))?;
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
        r.set(object_6, field::enemy::STATE_A, V::I(16))?;
        } // check_enemy_state_z_19
        let enemy_behavior_state_20 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_20 == 1 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
let enemy_state_a = r.get(object_6, field::enemy::STATE_A)?.i();
r.set(object_6, field::enemy::STATE_A, V::I(enemy_state_a.wrapping_sub(1)))?;
let enemy_state_a_2 = r.get(object_6, field::enemy::STATE_A)?.i();
if enemy_state_a_2 < -16 {
r.set(object_6, field::enemy::STATE_A, V::I(-16))?;
}
number_9 = 0;
let drawable_x_52 = r.get(object_6, field::drawable::X)?.i();
let enemy_state_d = r.get(object_6, field::enemy::STATE_D)?.i();
if drawable_x_52 < enemy_state_d {
r.set(object_6, field::enemy::FACING, V::I(1))?;
number_9 = 1;
}
let drawable_x_84 = r.get(object_6, field::drawable::X)?.i();
let enemy_state_d_3 = r.get(object_6, field::enemy::STATE_D)?.i();
if drawable_x_84 > enemy_state_d_3 {
r.set(object_6, field::enemy::FACING, V::I(3))?;
number_9 = 1;
}
let drawable_y_93 = r.get(object_6, field::drawable::Y)?.i();
let enemy_state_e_7 = r.get(object_6, field::enemy::STATE_E)?.i();
if drawable_y_93 < enemy_state_e_7 {
r.set(object_6, field::enemy::FACING, V::I(2))?;
number_9 = 1;
}
let drawable_y_133 = r.get(object_6, field::drawable::Y)?.i();
let enemy_state_e_12 = r.get(object_6, field::enemy::STATE_E)?.i();
if drawable_y_133 > enemy_state_e_12 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
number_9 = 1;
}

if number_9 == 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::STATE_A, V::I(16))?;
}
}
        break 'check_enemy_state_f_lower;
        } // next_step_4
        number_9 = 0;
        number_10 = 0;
        let drawable_x_7 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_8 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_7 > drawable_x_8 {
number_9 = 1;
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
        let drawable_x_19 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_20 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_19 < drawable_x_20 {
number_9 = -1;
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
        let drawable_y_22 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_23 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_22 > drawable_y_23 {
number_10 = 1;
r.set(object_6, field::enemy::FACING, V::I(2))?;
}
        let drawable_y_40 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_41 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_40 < drawable_y_41 {
number_10 = -1;
r.set(object_6, field::enemy::FACING, V::I(0))?;
}
        let player_state_a_4 = r.get(object_7, field::player::STATE_A)?.i();
        if player_state_a_4 != 0 {
let drawable_x_114 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_115 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_116 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_117 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_96 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_97 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_98 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_99 = r.get(object_6, field::drawable::Y)?.i();
if !(((drawable_x_114.wrapping_sub(drawable_x_115)).wrapping_mul(drawable_x_116.wrapping_sub(drawable_x_117))).wrapping_add((drawable_y_96.wrapping_sub(drawable_y_97)).wrapping_mul(drawable_y_98.wrapping_sub(drawable_y_99))) > 1024) {
let drawable_x_159 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_160 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_159 >= drawable_x_160 {
number_9 = -3;
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
let drawable_x_194 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_195 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_194 < drawable_x_195 {
number_9 = 3;
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
let drawable_y_182 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_183 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_182 >= drawable_y_183 {
number_10 = -3;
r.set(object_6, field::enemy::FACING, V::I(0))?;
}
let drawable_y_196 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_197 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_196 < drawable_y_197 {
number_10 = 3;
r.set(object_6, field::enemy::FACING, V::I(2))?;
}
}
}
        number_11 = 0;
        } // check_world_objects
        'update_enemies_loop_55: loop {
            'check_world_objects_2: {
            let world_objects = r.get(object_8, field::world::OBJECTS)?.r();
            let vector_size_value_14 = host::vector_size(r, world_objects)?;
            if number_11 >= vector_size_value_14 { break 'check_drawable_y_19; }
            } // check_world_objects_2
            let world_objects_2 = r.get(object_8, field::world::OBJECTS)?.r();
            let vector_element_at_value_14 = host::vector_element_at(r, world_objects_2, number_11)?;
            host::check_cast(r, vector_element_at_value_14, "n")?;
            object_12 = vector_element_at_value_14;
            let map_object_kind_14 = r.get(vector_element_at_value_14, field::map_object::KIND)?.i();
            if map_object_kind_14 == 8 {
let drawable_x_222 = r.get(object_12, field::drawable::X)?.i();
let drawable_x_223 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_224 = r.get(object_12, field::drawable::X)?.i();
let drawable_x_225 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_226 = r.get(object_12, field::drawable::X)?.i();
let drawable_y_180 = r.get(object_6, field::drawable::Y)?.i();
let drawable_x_227 = r.get(object_12, field::drawable::X)?.i();
let drawable_y_181 = r.get(object_6, field::drawable::Y)?.i();
if !(((drawable_x_222.wrapping_sub(drawable_x_223)).wrapping_mul(drawable_x_224.wrapping_sub(drawable_x_225))).wrapping_add((drawable_x_226.wrapping_sub(drawable_y_180)).wrapping_mul(drawable_x_227.wrapping_sub(drawable_y_181))) > 1024) {
let enemy_facing_49 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_49 == 0 {
number_10 = 3;
number_9 = 0;
}
let enemy_facing_55 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_55 == 1 {
number_9 = -3;
number_10 = 0;
}
let enemy_facing_61 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_61 == 2 {
number_10 = -3;
number_9 = 0;
}
let enemy_facing_64 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_64 == 3 {
number_9 = 3;
number_10 = 0;
}
}
}
            number_11 = number_11.wrapping_add(1);
            continue 'update_enemies_loop_55;
        }
        } // check_drawable_y_19
        let drawable_y_155 = r.get(object_6, field::drawable::Y)?.i();
        r.set(object_6, field::drawable::Y, V::I(drawable_y_155.wrapping_add(number_10)))?;
        let drawable_x_193 = r.get(object_6, field::drawable::X)?.i();
        r.set(object_6, field::drawable::X, V::I(drawable_x_193.wrapping_add(number_9)))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_3
        let enemy_behavior_state_3 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_3 != 0 { break 'check_enemy_state_z_20; }
        } // check_enemy_state_f
        let enemy_state_timer = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer.wrapping_add(1)))?;
        let enemy_state_timer_2 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        let enemy_state_g = r.get(object_6, field::enemy::STATE_G)?.i();
        if enemy_state_timer_2 > enemy_state_g {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::STATE_G, V::I(15))?;
}
        r.set(object_6, field::enemy::STATE_D_LOWER, V::I(4))?;
        r.set(object_6, field::enemy::DEAD, V::I(1))?;
        r.set(object_6, field::enemy::STATE_H, V::I(0))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_20
        let enemy_behavior_state_21 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_21 != 1 { break 'check_enemy_state_z_25; }
        } // check_enemy_state_f_3
        let enemy_state_timer_18 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_18.wrapping_add(1)))?;
        r.set(object_6, field::enemy::DEAD, V::I(0))?;
        r.set(object_6, field::enemy::STATE_H, V::I(1))?;
        let enemy_state_timer_19 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        let enemy_state_g_3 = r.get(object_6, field::enemy::STATE_G)?.i();
        if enemy_state_timer_19 > enemy_state_g_3 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::STATE_G, V::I(50))?;
}
        r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
        let drawable_x_54 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_55 = r.get(object_6, field::drawable::X)?.i();
        number_9 = drawable_x_54.wrapping_sub(drawable_x_55);
        let drawable_y_42 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_43 = r.get(object_6, field::drawable::Y)?.i();
        number_10 = drawable_y_42.wrapping_sub(drawable_y_43);
        let math_abs_value_4 = host::math_abs(r, number_9)?;
        let math_abs_value_5 = host::math_abs(r, number_10)?;
        if math_abs_value_4 > math_abs_value_5 { break 'next_step_15; }
        } // next_step_16

        if number_10 < 0 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
} else {
r.set(object_6, field::enemy::FACING, V::I(2))?;
}
        break 'check_enemy_state_f_lower;
        } // next_step_15

        if number_9 >= 0 { break 'check_enemy_facing_8; }
        } // check_enemy_facing_13
        r.set(object_6, field::enemy::FACING, V::I(3))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_25
        let enemy_behavior_state_36 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_36 != 2 { break 'check_enemy_state_f_lower; }
        } // check_enemy_state_f_6
        let enemy_state_timer_39 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_39.wrapping_add(1)))?;
        r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
        r.set(object_6, field::enemy::DEAD, V::I(0))?;
        r.set(object_6, field::enemy::STATE_H, V::I(1))?;
        let enemy_state_timer_40 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        let enemy_state_g_5 = r.get(object_6, field::enemy::STATE_G)?.i();
        if enemy_state_timer_40 > enemy_state_g_5 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
let object_value_3 = r.alloc("java/util/Random", Native::Empty);
host::random_initialize(r, object_value_3)?;
object_9 = object_value_3;
let drawable_x_86 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_73 = r.get(object_6, field::drawable::Y)?.i();
let random_next_int_value_9 = host::random_next_int(r, object_9)?;
host::random_set_seed(r, object_value_3, ((drawable_x_86.wrapping_mul(drawable_y_73)).wrapping_add(random_next_int_value_9)) as i64)?;
let random_next_int_value_10 = host::random_next_int(r, object_9)?;
let remainder_value_9 = arithmetic::remainder_int(r, random_next_int_value_10, 100)?;
r.set(object_6, field::enemy::STATE_G, V::I(remainder_value_9.wrapping_add(50)))?;
let random_next_int_value_11 = host::random_next_int(r, object_9)?;
let remainder_value_10 = arithmetic::remainder_int(r, random_next_int_value_11, 4)?;
r.set(object_6, field::enemy::FACING, V::I((remainder_value_10) as i8 as i32))?;
}
        let enemy_state_timer_67 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_67 == 15 {
number_9 = 0;
number_10 = 0;
let enemy_facing_28 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_28 == 0 {
number_9 = -1;
}
let enemy_facing_33 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_33 == 1 {
number_10 = 1;
}
let enemy_facing_38 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_38 == 2 {
number_9 = 1;
}
let enemy_facing_44 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_44 == 3 {
number_10 = -1;
}
let projectile_value_10 = r.alloc("p", Native::Empty);
let world_resources_14 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_10 = super::resources::projectile_image(r, world_resources_14, 5)?;
super::projectile::initialize(r, projectile_value_10, projectile_image_value_10, 5, number_10, number_9)?;
object_11 = projectile_value_10;
let drawable_x_237 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_10, field::drawable::X, V::I(drawable_x_237.wrapping_add(number_10.wrapping_mul(16))))?;
let drawable_y_198 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I(drawable_y_198.wrapping_add(number_9.wrapping_mul(16))))?;
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_7 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_7, object_11)?;
}
        let drawable_x_118 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_119 = r.get(object_6, field::drawable::X)?.i();
        number_9 = drawable_x_118.wrapping_sub(drawable_x_119);
        let drawable_y_100 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_101 = r.get(object_6, field::drawable::Y)?.i();
        number_10 = drawable_y_100.wrapping_sub(drawable_y_101);
        let math_abs_value_10 = host::math_abs(r, number_9)?;
        let math_abs_value_11 = host::math_abs(r, number_10)?;
        if math_abs_value_10 > math_abs_value_11 { break 'next_step_23; }
        } // next_step_24

        if number_10 < 0 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
} else {
r.set(object_6, field::enemy::FACING, V::I(2))?;
}
        break 'check_enemy_state_f_lower;
        } // next_step_23

        if number_9 >= 0 { break 'check_enemy_facing_8; }
        } // check_enemy_facing_16
        r.set(object_6, field::enemy::FACING, V::I(3))?;
        break 'check_enemy_state_f_lower;
        } // next_step_5
        let object_value = r.alloc("java/util/Random", Native::Empty);
        host::random_initialize(r, object_value)?;
        object_9 = object_value;
        let drawable_x_9 = r.get(object_6, field::drawable::X)?.i();
        let drawable_y_3 = r.get(object_6, field::drawable::Y)?.i();
        let random_next_int_value = host::random_next_int(r, object_9)?;
        host::random_set_seed(r, object_value, ((drawable_x_9.wrapping_mul(drawable_y_3)).wrapping_add(random_next_int_value)) as i64)?;
        number_10 = 0;
        let random_next_int_value_2 = host::random_next_int(r, object_9)?;
        number_11 = 0;
        number_12 = 0;
        let enemy_state_r = r.get(object_6, field::enemy::STATE_R)?.r();
        let enemy_state_r_2 = r.get(object_6, field::enemy::STATE_R)?.r();
        let vector_size_value_2 = host::vector_size(r, enemy_state_r_2)?;
        let vector_element_at_value_2 = host::vector_element_at(r, enemy_state_r, vector_size_value_2.wrapping_sub(1))?;
        object_11 = 0;
        let enemy_state_r_3 = r.get(object_6, field::enemy::STATE_R)?.r();
        let vector_size_value_3 = host::vector_size(r, enemy_state_r_3)?;
        number_7 = vector_size_value_3.wrapping_sub(1);
        } // next_step_10
        'update_enemies_loop_71: loop {
            'check_enemy_state_r_4: {
            if number_7 <= 0 { break 'check_enemy_state_s; }
            } // check_enemy_state_r_4
            let enemy_state_r_10 = r.get(object_6, field::enemy::STATE_R)?.r();
            let vector_element_at_value_6 = host::vector_element_at(r, enemy_state_r_10, number_7)?;
            host::check_cast(r, vector_element_at_value_6, "g")?;
            object_10 = vector_element_at_value_6;
            let enemy_state_r_11 = r.get(object_6, field::enemy::STATE_R)?.r();
            let vector_size_value_8 = host::vector_size(r, enemy_state_r_11)?;
            if number_7 != vector_size_value_8.wrapping_sub(1) {
let enemy_state_r_12 = r.get(object_6, field::enemy::STATE_R)?.r();
let vector_element_at_value_7 = host::vector_element_at(r, enemy_state_r_12, number_7.wrapping_add(1))?;
host::check_cast(r, vector_element_at_value_7, "g")?;
object_11 = vector_element_at_value_7;
let enemy_state_r_13 = r.get(object_6, field::enemy::STATE_R)?.r();
let enemy_state_r_14 = r.get(object_6, field::enemy::STATE_R)?.r();
let vector_size_value_9 = host::vector_size(r, enemy_state_r_14)?;
let vector_element_at_value_8 = host::vector_element_at(r, enemy_state_r_13, vector_size_value_9.wrapping_sub(1))?;
host::check_cast(r, vector_element_at_value_8, "g")?;
object_12 = vector_element_at_value_8;
let trail_point_x_56 = r.get(object_11, field::trail_point::X)?.i();
let drawable_x_57 = r.get(object_6, field::drawable::X)?.i();
let trail_point_x_58 = r.get(object_12, field::trail_point::X)?.i();
let quotient_value = arithmetic::divide_int(r, drawable_x_57.wrapping_sub(trail_point_x_58), 4)?;
r.set(object_10, field::trail_point::X, V::I(trail_point_x_56.wrapping_add(quotient_value)))?;
let trail_point_y_44 = r.get(object_11, field::trail_point::Y)?.i();
let enemy_state_e_3 = r.get(object_6, field::enemy::STATE_E)?.i();
let trail_point_y_45 = r.get(object_12, field::trail_point::Y)?.i();
let quotient_value_2 = arithmetic::divide_int(r, enemy_state_e_3.wrapping_sub(trail_point_y_45), 4)?;
r.set(object_10, field::trail_point::Y, V::I(trail_point_y_44.wrapping_add(quotient_value_2)))?;
let enemy_state_r_15 = r.get(object_6, field::enemy::STATE_R)?.r();
host::vector_set_element_at(r, enemy_state_r_15, object_10, number_7)?;
} else {
let trail_point_y_46 = r.get(object_10, field::trail_point::Y)?.i();
let enemy_velocity_x_3 = r.get(object_6, field::enemy::VELOCITY_X)?.i();
r.set(object_10, field::trail_point::Y, V::I(trail_point_y_46.wrapping_add(enemy_velocity_x_3)))?;
let trail_point_x_59 = r.get(object_10, field::trail_point::X)?.i();
let drawable_x_60 = r.get(object_6, field::drawable::X)?.i();
if !(trail_point_x_59 >= drawable_x_60.wrapping_sub(48)) {
let drawable_x_89 = r.get(object_6, field::drawable::X)?.i();
r.set(object_10, field::trail_point::X, V::I(drawable_x_89.wrapping_sub(48)))?;
let enemy_velocity_x_4 = r.get(object_6, field::enemy::VELOCITY_X)?.i();
r.set(object_6, field::enemy::VELOCITY_X, V::I(enemy_velocity_x_4.wrapping_neg()))?;
}
let trail_point_x_87 = r.get(object_10, field::trail_point::X)?.i();
let drawable_x_88 = r.get(object_6, field::drawable::X)?.i();
if !(trail_point_x_87 <= drawable_x_88.wrapping_add(48)) {
let drawable_x_126 = r.get(object_6, field::drawable::X)?.i();
r.set(object_10, field::trail_point::X, V::I(drawable_x_126.wrapping_add(48)))?;
let enemy_velocity_x_5 = r.get(object_6, field::enemy::VELOCITY_X)?.i();
r.set(object_6, field::enemy::VELOCITY_X, V::I(enemy_velocity_x_5.wrapping_neg()))?;
}
let trail_point_y_106 = r.get(object_10, field::trail_point::Y)?.i();
let enemy_state_e_9 = r.get(object_6, field::enemy::STATE_E)?.i();
if !(trail_point_y_106 >= enemy_state_e_9.wrapping_sub(48)) {
let enemy_state_e_14 = r.get(object_6, field::enemy::STATE_E)?.i();
r.set(object_10, field::trail_point::Y, V::I(enemy_state_e_14.wrapping_sub(48)))?;
let enemy_velocity_x_6 = r.get(object_6, field::enemy::VELOCITY_X)?.i();
r.set(object_6, field::enemy::VELOCITY_X, V::I(enemy_velocity_x_6.wrapping_neg()))?;
}
let trail_point_y_138 = r.get(object_10, field::trail_point::Y)?.i();
let enemy_state_e_13 = r.get(object_6, field::enemy::STATE_E)?.i();
if !(trail_point_y_138 <= enemy_state_e_13.wrapping_add(48)) {
let enemy_state_e_16 = r.get(object_6, field::enemy::STATE_E)?.i();
r.set(object_10, field::trail_point::Y, V::I(enemy_state_e_16.wrapping_add(48)))?;
let enemy_velocity_x_7 = r.get(object_6, field::enemy::VELOCITY_X)?.i();
r.set(object_6, field::enemy::VELOCITY_X, V::I(enemy_velocity_x_7.wrapping_neg()))?;
}
let enemy_state_r_19 = r.get(object_6, field::enemy::STATE_R)?.r();
host::vector_set_element_at(r, enemy_state_r_19, object_10, number_7)?;
}
            number_7 = number_7.wrapping_add(-1);
            continue 'update_enemies_loop_71;
        }
        } // check_enemy_state_s
        let enemy_state_s = r.get(object_6, field::enemy::STATE_S)?.r();
        let enemy_state_s_2 = r.get(object_6, field::enemy::STATE_S)?.r();
        let vector_size_value_6 = host::vector_size(r, enemy_state_s_2)?;
        let vector_element_at_value_5 = host::vector_element_at(r, enemy_state_s, vector_size_value_6.wrapping_sub(1))?;
        let enemy_state_s_3 = r.get(object_6, field::enemy::STATE_S)?.r();
        let vector_size_value_7 = host::vector_size(r, enemy_state_s_3)?;
        number_7 = vector_size_value_7.wrapping_sub(1);
        } // next_step_12
        'update_enemies_loop_74: loop {
            'check_enemy_state_s_2: {
            if number_7 <= 0 { break 'check_enemy_state_r_5; }
            } // check_enemy_state_s_2
            let enemy_state_s_4 = r.get(object_6, field::enemy::STATE_S)?.r();
            let vector_element_at_value_10 = host::vector_element_at(r, enemy_state_s_4, number_7)?;
            host::check_cast(r, vector_element_at_value_10, "g")?;
            object_10 = vector_element_at_value_10;
            let enemy_state_s_5 = r.get(object_6, field::enemy::STATE_S)?.r();
            let vector_size_value_11 = host::vector_size(r, enemy_state_s_5)?;
            if number_7 != vector_size_value_11.wrapping_sub(1) {
let enemy_state_s_8 = r.get(object_6, field::enemy::STATE_S)?.r();
let vector_element_at_value_12 = host::vector_element_at(r, enemy_state_s_8, number_7.wrapping_add(1))?;
host::check_cast(r, vector_element_at_value_12, "g")?;
object_11 = vector_element_at_value_12;
let enemy_state_s_9 = r.get(object_6, field::enemy::STATE_S)?.r();
let enemy_state_s_10 = r.get(object_6, field::enemy::STATE_S)?.r();
let vector_size_value_13 = host::vector_size(r, enemy_state_s_10)?;
let vector_element_at_value_13 = host::vector_element_at(r, enemy_state_s_9, vector_size_value_13.wrapping_sub(1))?;
host::check_cast(r, vector_element_at_value_13, "g")?;
object_12 = vector_element_at_value_13;
let trail_point_x_121 = r.get(object_11, field::trail_point::X)?.i();
let drawable_x_122 = r.get(object_6, field::drawable::X)?.i();
let trail_point_x_123 = r.get(object_12, field::trail_point::X)?.i();
let quotient_value_3 = arithmetic::divide_int(r, drawable_x_122.wrapping_sub(trail_point_x_123), 4)?;
r.set(object_10, field::trail_point::X, V::I(trail_point_x_121.wrapping_add(quotient_value_3)))?;
let trail_point_y_103 = r.get(object_11, field::trail_point::Y)?.i();
let enemy_state_e_8 = r.get(object_6, field::enemy::STATE_E)?.i();
let trail_point_y_104 = r.get(object_12, field::trail_point::Y)?.i();
let quotient_value_4 = arithmetic::divide_int(r, enemy_state_e_8.wrapping_sub(trail_point_y_104), 4)?;
r.set(object_10, field::trail_point::Y, V::I(trail_point_y_103.wrapping_add(quotient_value_4)))?;
let enemy_state_s_11 = r.get(object_6, field::enemy::STATE_S)?.r();
host::vector_set_element_at(r, enemy_state_s_11, object_10, number_7)?;
} else {
let trail_point_y_105 = r.get(object_10, field::trail_point::Y)?.i();
let enemy_velocity_y_3 = r.get(object_6, field::enemy::VELOCITY_Y)?.i();
r.set(object_10, field::trail_point::Y, V::I(trail_point_y_105.wrapping_add(enemy_velocity_y_3)))?;
let trail_point_x_124 = r.get(object_10, field::trail_point::X)?.i();
let drawable_x_125 = r.get(object_6, field::drawable::X)?.i();
if !(trail_point_x_124 >= drawable_x_125.wrapping_sub(48)) {
let drawable_x_164 = r.get(object_6, field::drawable::X)?.i();
r.set(object_10, field::trail_point::X, V::I(drawable_x_164.wrapping_sub(48)))?;
let enemy_velocity_y_4 = r.get(object_6, field::enemy::VELOCITY_Y)?.i();
r.set(object_6, field::enemy::VELOCITY_Y, V::I(enemy_velocity_y_4.wrapping_neg()))?;
}
let trail_point_x_162 = r.get(object_10, field::trail_point::X)?.i();
let drawable_x_163 = r.get(object_6, field::drawable::X)?.i();
if !(trail_point_x_162 <= drawable_x_163.wrapping_add(48)) {
let drawable_x_196 = r.get(object_6, field::drawable::X)?.i();
r.set(object_10, field::trail_point::X, V::I(drawable_x_196.wrapping_add(48)))?;
let enemy_velocity_y_5 = r.get(object_6, field::enemy::VELOCITY_Y)?.i();
r.set(object_6, field::enemy::VELOCITY_Y, V::I(enemy_velocity_y_5.wrapping_neg()))?;
}
let trail_point_y_156 = r.get(object_10, field::trail_point::Y)?.i();
let enemy_state_e_15 = r.get(object_6, field::enemy::STATE_E)?.i();
if !(trail_point_y_156 >= enemy_state_e_15.wrapping_sub(48)) {
let enemy_state_e_18 = r.get(object_6, field::enemy::STATE_E)?.i();
r.set(object_10, field::trail_point::Y, V::I(enemy_state_e_18.wrapping_sub(48)))?;
let enemy_velocity_y_6 = r.get(object_6, field::enemy::VELOCITY_Y)?.i();
r.set(object_6, field::enemy::VELOCITY_Y, V::I(enemy_velocity_y_6.wrapping_neg()))?;
}
let trail_point_y_184 = r.get(object_10, field::trail_point::Y)?.i();
let enemy_state_e_17 = r.get(object_6, field::enemy::STATE_E)?.i();
if !(trail_point_y_184 <= enemy_state_e_17.wrapping_add(48)) {
let enemy_state_e_19 = r.get(object_6, field::enemy::STATE_E)?.i();
r.set(object_10, field::trail_point::Y, V::I(enemy_state_e_19.wrapping_add(48)))?;
let enemy_velocity_y_7 = r.get(object_6, field::enemy::VELOCITY_Y)?.i();
r.set(object_6, field::enemy::VELOCITY_Y, V::I(enemy_velocity_y_7.wrapping_neg()))?;
}
let enemy_state_s_12 = r.get(object_6, field::enemy::STATE_S)?.r();
host::vector_set_element_at(r, enemy_state_s_12, object_10, number_7)?;
}
            number_7 = number_7.wrapping_add(-1);
            continue 'update_enemies_loop_74;
        }
        } // check_enemy_state_r_5
        let enemy_state_r_16 = r.get(object_6, field::enemy::STATE_R)?.r();
        let enemy_state_r_17 = r.get(object_6, field::enemy::STATE_R)?.r();
        let vector_size_value_10 = host::vector_size(r, enemy_state_r_17)?;
        let vector_element_at_value_9 = host::vector_element_at(r, enemy_state_r_16, vector_size_value_10.wrapping_sub(1))?;
        host::check_cast(r, vector_element_at_value_9, "g")?;
        object_7 = vector_element_at_value_9;
        let random_next_int_value_12 = host::random_next_int(r, object_9)?;
        let remainder_value_11 = arithmetic::remainder_int(r, random_next_int_value_12, 30)?;
        if remainder_value_11 == 1 {
let projectile_value_2 = r.alloc("p", Native::Empty);
let world_resources_3 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_2 = super::resources::projectile_image(r, world_resources_3, 6)?;
super::projectile::initialize(r, projectile_value_2, projectile_image_value_2, 6, -1, 0)?;
object_10 = projectile_value_2;
let trail_point_x_120 = r.get(object_7, field::trail_point::X)?.i();
r.set(projectile_value_2, field::drawable::X, V::I(trail_point_x_120))?;
let trail_point_y_102 = r.get(object_7, field::trail_point::Y)?.i();
r.set(object_10, field::drawable::Y, V::I(trail_point_y_102))?;
r.set(object_7, field::trail_point::STATE_C, V::I(-10))?;
r.set(object_10, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_2 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_2, object_10)?;
}
        let enemy_state_s_6 = r.get(object_6, field::enemy::STATE_S)?.r();
        let enemy_state_s_7 = r.get(object_6, field::enemy::STATE_S)?.r();
        let vector_size_value_12 = host::vector_size(r, enemy_state_s_7)?;
        let vector_element_at_value_11 = host::vector_element_at(r, enemy_state_s_6, vector_size_value_12.wrapping_sub(1))?;
        host::check_cast(r, vector_element_at_value_11, "g")?;
        object_7 = vector_element_at_value_11;
        let random_next_int_value_19 = host::random_next_int(r, object_9)?;
        let remainder_value_19 = arithmetic::remainder_int(r, random_next_int_value_19, 30)?;
        let math_abs_value_12 = host::math_abs(r, remainder_value_19)?;
        if math_abs_value_12 == 5 {
let projectile_value_6 = r.alloc("p", Native::Empty);
let world_resources_8 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_6 = super::resources::projectile_image(r, world_resources_8, 6)?;
super::projectile::initialize(r, projectile_value_6, projectile_image_value_6, 6, -1, 0)?;
object_10 = projectile_value_6;
let trail_point_x_161 = r.get(object_7, field::trail_point::X)?.i();
r.set(projectile_value_6, field::drawable::X, V::I(trail_point_x_161))?;
let trail_point_y_137 = r.get(object_7, field::trail_point::Y)?.i();
r.set(object_10, field::drawable::Y, V::I(trail_point_y_137))?;
r.set(object_7, field::trail_point::STATE_C, V::I(-10))?;
r.set(object_10, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_4 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_4, object_10)?;
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_4
        let enemy_behavior_state_4 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_4 == 0 {
let drawable_x_21 = r.get(object_7, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_21.wrapping_sub(20)))?;
let drawable_y_11 = r.get(object_7, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_11.wrapping_sub(90)))?;
r.set(object_6, field::enemy::WIDTH, V::I(55))?;
r.set(object_6, field::enemy::HEIGHT, V::I(90))?;
r.set(object_6, field::drawable::HALF_WIDTH, V::I(25))?;
r.set(object_6, field::drawable::HALF_HEIGHT, V::I(45))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_O_LOWER, V::I(0))?;
let enemy_state_timer_3 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_3 == 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
}
}
        let enemy_behavior_state_22 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_22 == 1 {
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_O_LOWER, V::I(1))?;
r.set(object_6, field::enemy::WIDTH, V::I(55))?;
r.set(object_6, field::enemy::HEIGHT, V::I(90))?;
r.set(object_6, field::drawable::HALF_WIDTH, V::I(25))?;
r.set(object_6, field::drawable::HALF_HEIGHT, V::I(45))?;
let enemy_state_timer_20 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_20 == 60 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
}
}
        let enemy_behavior_state_37 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_37 == 2 {
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_O_LOWER, V::I(1))?;
let enemy_state_timer_41 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_41 == 70 {
let drawable_y_74 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_74.wrapping_add(37)))?;
r.set(object_6, field::enemy::WIDTH, V::I(68))?;
r.set(object_6, field::enemy::HEIGHT, V::I(60))?;
r.set(object_6, field::drawable::HALF_WIDTH, V::I(34))?;
r.set(object_6, field::drawable::HALF_HEIGHT, V::I(30))?;
}
let enemy_state_timer_68 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_68 == 75 {
let drawable_x_127 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_127.wrapping_add(20)))?;
r.set(object_6, field::enemy::WIDTH, V::I(68))?;
r.set(object_6, field::enemy::HEIGHT, V::I(80))?;
r.set(object_6, field::drawable::HALF_WIDTH, V::I(34))?;
r.set(object_6, field::drawable::HALF_HEIGHT, V::I(30))?;
}
let enemy_state_timer_90 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_90 == 80 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let player_state_a_3 = r.get(object_7, field::player::STATE_A)?.i();
        if player_state_a_3 == 0 { break 'check_enemy_state_o_lower; }
        } // check_enemy_state_z_34
        let enemy_behavior_state_59 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_59 == 0 { break 'check_enemy_state_o_lower; }
        } // check_enemy_state_o_lower_2
        r.set(object_6, field::enemy::STATE_O_LOWER, V::I(1))?;
        r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
        break 'check_enemy_state_f_12;
        } // check_enemy_state_o_lower
        r.set(object_6, field::enemy::STATE_O_LOWER, V::I(0))?;
        r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
        } // check_enemy_state_f_12
        let enemy_state_timer_89 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_89.wrapping_add(1)))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_5
        let enemy_behavior_state_5 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_5 != 0 { break 'check_enemy_state_z_21; }
        } // check_enemy_state_f_2
        let enemy_state_timer_4 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_4.wrapping_add(1)))?;
        let enemy_state_timer_5 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        let enemy_state_g_2 = r.get(object_6, field::enemy::STATE_G)?.i();
        if enemy_state_timer_5 > enemy_state_g_2 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::STATE_G, V::I(15))?;
let enemy_hit_count = r.get(object_6, field::enemy::HIT_COUNT)?.i();
r.set(object_6, field::enemy::HIT_COUNT, V::I(enemy_hit_count.wrapping_add(1)))?;
let enemy_hit_count_2 = r.get(object_6, field::enemy::HIT_COUNT)?.i();
if enemy_hit_count_2 == 4 {
r.set(object_6, field::enemy::HIT_COUNT, V::I(0))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(3))?;
}
}
        r.set(object_6, field::enemy::STATE_D_LOWER, V::I(10))?;
        r.set(object_6, field::enemy::DEAD, V::I(1))?;
        r.set(object_6, field::enemy::STATE_H, V::I(0))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_21
        let enemy_behavior_state_23 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_23 != 1 { break 'check_enemy_state_z_26; }
        } // check_enemy_state_f_4
        let enemy_state_timer_21 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_21.wrapping_add(1)))?;
        r.set(object_6, field::enemy::DEAD, V::I(0))?;
        r.set(object_6, field::enemy::STATE_H, V::I(1))?;
        let enemy_state_timer_22 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        let enemy_state_g_4 = r.get(object_6, field::enemy::STATE_G)?.i();
        if enemy_state_timer_22 > enemy_state_g_4 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::STATE_G, V::I(50))?;
}
        r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
        let drawable_x_61 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_62 = r.get(object_6, field::drawable::X)?.i();
        number_9 = drawable_x_61.wrapping_sub(drawable_x_62);
        number_10 = 0;
        if number_9 >= 0 { break 'check_enemy_facing_8; }
        } // check_enemy_facing_9
        r.set(object_6, field::enemy::FACING, V::I(3))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_26
        let enemy_behavior_state_38 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_38 != 2 { break 'check_enemy_state_z_30; }
        } // check_enemy_state_f_7
        let enemy_state_timer_42 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_42.wrapping_add(1)))?;
        r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
        r.set(object_6, field::enemy::DEAD, V::I(0))?;
        r.set(object_6, field::enemy::STATE_H, V::I(1))?;
        let enemy_state_timer_43 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        let enemy_state_g_6 = r.get(object_6, field::enemy::STATE_G)?.i();
        if enemy_state_timer_43 > enemy_state_g_6 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
let object_value_4 = r.alloc("java/util/Random", Native::Empty);
host::random_initialize(r, object_value_4)?;
object_9 = object_value_4;
let drawable_x_90 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_75 = r.get(object_6, field::drawable::Y)?.i();
let random_next_int_value_13 = host::random_next_int(r, object_9)?;
host::random_set_seed(r, object_value_4, ((drawable_x_90.wrapping_mul(drawable_y_75)).wrapping_add(random_next_int_value_13)) as i64)?;
let random_next_int_value_14 = host::random_next_int(r, object_9)?;
let remainder_value_12 = arithmetic::remainder_int(r, random_next_int_value_14, 100)?;
r.set(object_6, field::enemy::STATE_G, V::I(remainder_value_12.wrapping_add(50)))?;
let random_next_int_value_15 = host::random_next_int(r, object_9)?;
let remainder_value_13 = arithmetic::remainder_int(r, random_next_int_value_15, 4)?;
r.set(object_6, field::enemy::FACING, V::I((remainder_value_13) as i8 as i32))?;
}
        let enemy_state_timer_71 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_71 == 15 {
number_9 = 0;
number_10 = 0;
let enemy_facing_29 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_29 == 0 {
number_9 = -1;
}
let enemy_facing_34 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_34 == 1 {
number_10 = 1;
}
let enemy_facing_39 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_39 == 2 {
number_9 = 1;
}
let enemy_facing_45 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_45 == 3 {
number_10 = -1;
}
let projectile_value_11 = r.alloc("p", Native::Empty);
let world_resources_15 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_11 = super::resources::projectile_image(r, world_resources_15, 5)?;
super::projectile::initialize(r, projectile_value_11, projectile_image_value_11, 5, number_10, number_9)?;
object_11 = projectile_value_11;
let drawable_x_238 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_11, field::drawable::X, V::I(drawable_x_238.wrapping_add(number_10.wrapping_mul(16))))?;
let drawable_y_199 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I((drawable_y_199.wrapping_add(number_9.wrapping_mul(16))).wrapping_add(16)))?;
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_8 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_8, object_11)?;
}
        let drawable_x_130 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_131 = r.get(object_6, field::drawable::X)?.i();
        number_9 = drawable_x_130.wrapping_sub(drawable_x_131);
        number_10 = 0;
        if number_9 >= 0 { break 'check_enemy_facing_8; }
        } // check_enemy_facing_15
        r.set(object_6, field::enemy::FACING, V::I(3))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_30
        let enemy_behavior_state_50 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_50 == 3 {
let enemy_state_timer_69 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_69.wrapping_add(1)))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::drawable::X, V::I(128))?;
r.set(object_6, field::drawable::Y, V::I(90))?;
r.set(object_6, field::enemy::FACING, V::I(1))?;
let enemy_state_timer_70 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_70 == 20 {
let object_value_5 = r.alloc("java/util/Random", Native::Empty);
host::random_initialize(r, object_value_5)?;
object_9 = object_value_5;
let drawable_x_128 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_107 = r.get(object_6, field::drawable::Y)?.i();
let random_next_int_value_20 = host::random_next_int(r, object_9)?;
host::random_set_seed(r, object_value_5, ((drawable_x_128.wrapping_mul(drawable_y_107)).wrapping_add(random_next_int_value_20)) as i64)?;
let enemy_value = r.alloc("l", Native::Empty);
let world_resources_4 = r.get(object_8, field::world::RESOURCES)?.r();
let enemy_image_value = super::resources::enemy_image(r, world_resources_4, 19)?;
let drawable_x_129 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_108 = r.get(object_6, field::drawable::Y)?.i();
super::enemy::initialize(r, enemy_value, enemy_image_value, drawable_x_129.wrapping_add(16), drawable_y_108.wrapping_sub(16), 1, 19)?;
object_10 = enemy_value;
let world_enemies_5 = r.get(object_8, field::world::ENEMIES)?.r();
host::vector_add_element(r, world_enemies_5, object_10)?;
}
let enemy_state_timer_91 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_91 == 30 {
let object_value_7 = r.alloc("java/util/Random", Native::Empty);
host::random_initialize(r, object_value_7)?;
object_9 = object_value_7;
let drawable_x_165 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_139 = r.get(object_6, field::drawable::Y)?.i();
let random_next_int_value_21 = host::random_next_int(r, object_9)?;
host::random_set_seed(r, object_value_7, ((drawable_x_165.wrapping_mul(drawable_y_139)).wrapping_add(random_next_int_value_21)) as i64)?;
let enemy_value_2 = r.alloc("l", Native::Empty);
let world_resources_9 = r.get(object_8, field::world::RESOURCES)?.r();
let enemy_image_value_2 = super::resources::enemy_image(r, world_resources_9, 19)?;
let drawable_x_166 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_140 = r.get(object_6, field::drawable::Y)?.i();
super::enemy::initialize(r, enemy_value_2, enemy_image_value_2, drawable_x_166.wrapping_add(16), drawable_y_140, 1, 19)?;
object_10 = enemy_value_2;
let world_enemies_6 = r.get(object_8, field::world::ENEMIES)?.r();
host::vector_add_element(r, world_enemies_6, object_10)?;
}
let enemy_state_timer_108 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_108 == 40 {
let object_value_8 = r.alloc("java/util/Random", Native::Empty);
host::random_initialize(r, object_value_8)?;
object_9 = object_value_8;
let drawable_x_197 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_157 = r.get(object_6, field::drawable::Y)?.i();
let random_next_int_value_24 = host::random_next_int(r, object_9)?;
host::random_set_seed(r, object_value_8, ((drawable_x_197.wrapping_mul(drawable_y_157)).wrapping_add(random_next_int_value_24)) as i64)?;
let enemy_value_3 = r.alloc("l", Native::Empty);
let world_resources_12 = r.get(object_8, field::world::RESOURCES)?.r();
let enemy_image_value_3 = super::resources::enemy_image(r, world_resources_12, 19)?;
let drawable_x_198 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_158 = r.get(object_6, field::drawable::Y)?.i();
super::enemy::initialize(r, enemy_value_3, enemy_image_value_3, drawable_x_198.wrapping_add(16), drawable_y_158.wrapping_add(16), 1, 19)?;
object_10 = enemy_value_3;
let world_enemies_7 = r.get(object_8, field::world::ENEMIES)?.r();
host::vector_add_element(r, world_enemies_7, object_10)?;
}
let enemy_state_timer_121 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_121 == 60 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_6
        let enemy_behavior_state_6 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_6 == 10 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let drawable_x_22 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_22.wrapping_add(5)))?;
let drawable_x_23 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_23 > 200 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::FACING, V::I(3))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_24 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_24 == 11 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let drawable_x_35 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_35.wrapping_sub(5)))?;
let drawable_x_36 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_36 < 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::FACING, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_39 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_39 != 0 { break 'check_enemy_state_z_31; }
        } // check_enemy_active_2
        r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
        let enemy_state_timer_44 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_44.wrapping_add(1)))?;
        let enemy_facing_14 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_14 != 1 { break 'check_enemy_facing_10; }
        } // check_drawable_x_14
        let drawable_x_91 = r.get(object_7, field::drawable::X)?.i();
        if drawable_x_91 <= 60 { break 'check_drawable_y_10; }
        } // check_drawable_y_11
        let drawable_y_111 = r.get(object_7, field::drawable::Y)?.i();
        r.set(object_6, field::drawable::Y, V::I((176_i32).wrapping_sub(drawable_y_111)))?;
        break 'check_enemy_facing_10;
        } // check_drawable_y_10
        let drawable_y_109 = r.get(object_6, field::drawable::Y)?.i();
        let drawable_y_110 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_109.wrapping_add(50) <= drawable_y_110 { break 'check_enemy_state_z_40; }
        } // check_drawable_y_17
        let drawable_y_144 = r.get(object_6, field::drawable::Y)?.i();
        let drawable_y_145 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_144.wrapping_sub(50) >= drawable_y_145 { break 'check_enemy_state_z_40; }
        } // check_enemy_state_z_42
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
        r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
        break 'check_enemy_facing_10;
        } // check_enemy_state_z_40
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(10))?;
        } // check_enemy_facing_10
        let enemy_facing_21 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_21 != 3 { break 'check_enemy_state_f_13; }
        } // check_drawable_x_17
        let drawable_x_132 = r.get(object_7, field::drawable::X)?.i();
        if drawable_x_132 >= 170 { break 'check_drawable_y_15; }
        } // check_drawable_y_16
        let drawable_y_143 = r.get(object_7, field::drawable::Y)?.i();
        r.set(object_6, field::drawable::Y, V::I((176_i32).wrapping_sub(drawable_y_143)))?;
        break 'check_enemy_state_f_13;
        } // check_drawable_y_15
        let drawable_y_141 = r.get(object_6, field::drawable::Y)?.i();
        let drawable_y_142 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_141.wrapping_add(50) <= drawable_y_142 { break 'check_enemy_state_z_41; }
        } // check_drawable_y_20
        let drawable_y_159 = r.get(object_6, field::drawable::Y)?.i();
        let drawable_y_160 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_159.wrapping_sub(50) >= drawable_y_160 { break 'check_enemy_state_z_41; }
        } // check_enemy_state_z_44
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
        r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
        break 'check_enemy_state_f_13;
        } // check_enemy_state_z_41
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(11))?;
        } // check_enemy_state_f_13
        let enemy_state_timer_94 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_94 == 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(3))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
        } // check_enemy_state_z_31
        let enemy_behavior_state_51 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_51 == 1 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_72 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_72.wrapping_add(1)))?;
let enemy_state_timer_73 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_73 > 10 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
}
}
        let enemy_behavior_state_60 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_60 != 2 { break 'check_enemy_state_z_37; }
        } // check_enemy_active_4
        r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
        r.set(object_6, field::enemy::STATE_O, V::I(12))?;
        let enemy_state_timer_92 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_92.wrapping_add(1)))?;
        let enemy_state_timer_93 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_93 == 1 {
let enemy_facing_35 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_35 == 3 {
let drawable_x_199 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_199.wrapping_sub(30)))?;
}
}
        r.set(object_6, field::enemy::WIDTH, V::I(60))?;
        r.set(object_6, field::enemy::HEIGHT, V::I(64))?;
        let enemy_state_timer_111 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_111 < 10 { break 'check_enemy_state_z_37; }
        } // check_enemy_state_o
        r.set(object_6, field::enemy::STATE_O, V::I(4))?;
        r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
        let enemy_facing_40 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_40 == 3 {
let enemy_state_timer_136 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_136 == 10 {
let drawable_x_240 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_240.wrapping_add(30)))?;
}
}
        let drawable_x_228 = r.get(object_7, field::drawable::X)?.i();
        if drawable_x_228 > 100 { break 'check_drawable_x_21; }
        } // check_enemy_width
        r.set(object_6, field::enemy::WIDTH, V::I(35))?;
        r.set(object_6, field::enemy::HEIGHT, V::I(43))?;
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(10))?;
        break 'check_enemy_state_f_lower;
        } // check_drawable_x_21
        let drawable_x_239 = r.get(object_7, field::drawable::X)?.i();
        if drawable_x_239 < 120 { break 'check_enemy_width_2; }
        } // check_enemy_width_3
        r.set(object_6, field::enemy::WIDTH, V::I(35))?;
        r.set(object_6, field::enemy::HEIGHT, V::I(43))?;
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(11))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_width_2
        r.set(object_6, field::enemy::WIDTH, V::I(35))?;
        r.set(object_6, field::enemy::HEIGHT, V::I(43))?;
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        } // check_enemy_state_z_37
        let enemy_behavior_state_68 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_68 == 3 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_109 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_109.wrapping_add(1)))?;
let enemy_state_timer_110 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_110 > 10 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(4))?;
}
}
        let enemy_behavior_state_73 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_73 == 4 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_122 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_122.wrapping_add(1)))?;
let enemy_state_timer_123 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_123 > 10 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(5))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
let enemy_state_timer_135 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_135 == 2 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
number_9 = 0;
number_10 = 0;
let enemy_facing_51 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_51 == 1 {
number_9 = 1;
}
let enemy_facing_57 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_57 == 3 {
number_9 = -1;
}
let projectile_value_16 = r.alloc("p", Native::Empty);
let world_resources_20 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_16 = super::resources::projectile_image(r, world_resources_20, 7)?;
super::projectile::initialize(r, projectile_value_16, projectile_image_value_16, 7, number_9, 0)?;
object_11 = projectile_value_16;
let drawable_x_255 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_16, field::drawable::X, V::I(drawable_x_255.wrapping_add(number_9.wrapping_mul(16))))?;
let drawable_y_218 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I(drawable_y_218.wrapping_add(16)))?;
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_16 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_16, object_11)?;
}
}
        let enemy_behavior_state_78 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_78 != 5 { break 'check_enemy_state_z_43; }
        } // check_enemy_active_5
        r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
        let enemy_state_timer_133 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_133.wrapping_add(1)))?;
        let enemy_state_timer_134 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_134 > 70 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
let object_value_9 = r.alloc("java/util/Random", Native::Empty);
host::random_initialize(r, object_value_9)?;
object_9 = object_value_9;
let random_next_int_value_27 = host::random_next_int(r, object_value_9)?;
let math_abs_value_31 = host::math_abs(r, random_next_int_value_27)?;
let remainder_value_29 = arithmetic::remainder_int(r, math_abs_value_31, 3)?;
number_10 = remainder_value_29;
if remainder_value_29 == 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(4))?;
}

if number_10 == 1 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
}

if number_10 == 2 {
let drawable_x_259 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_260 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_259 <= drawable_x_260 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(11))?;
} else {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(10))?;
}
}
}
        let enemy_facing_50 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_50 != 1 { break 'check_enemy_facing_17; }
        } // check_drawable_x_22
        let drawable_x_250 = r.get(object_7, field::drawable::X)?.i();
        if drawable_x_250 > 60 { break 'check_enemy_facing_17; }
        } // check_drawable_y_22
        let drawable_y_216 = r.get(object_6, field::drawable::Y)?.i();
        let drawable_y_217 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_216.wrapping_add(50) <= drawable_y_217 { break 'check_enemy_state_z_45; }
        } // check_drawable_y_24
        let drawable_y_225 = r.get(object_6, field::drawable::Y)?.i();
        let drawable_y_226 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_225.wrapping_sub(50) >= drawable_y_226 { break 'check_enemy_state_z_45; }
        } // check_enemy_state_z_47
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
        r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
        break 'check_enemy_facing_17;
        } // check_enemy_state_z_45
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(10))?;
        } // check_enemy_facing_17
        let enemy_facing_56 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_56 != 3 { break 'check_enemy_state_z_43; }
        } // check_drawable_x_23
        let drawable_x_254 = r.get(object_7, field::drawable::X)?.i();
        if drawable_x_254 < 170 { break 'check_enemy_state_z_43; }
        } // check_drawable_y_23
        let drawable_y_223 = r.get(object_6, field::drawable::Y)?.i();
        let drawable_y_224 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_223.wrapping_add(50) <= drawable_y_224 { break 'check_enemy_state_z_46; }
        } // check_drawable_y_25
        let drawable_y_227 = r.get(object_6, field::drawable::Y)?.i();
        let drawable_y_228 = r.get(object_7, field::drawable::Y)?.i();
        if drawable_y_227.wrapping_sub(50) >= drawable_y_228 { break 'check_enemy_state_z_46; }
        } // check_enemy_state_z_48
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
        r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
        break 'check_enemy_state_z_43;
        } // check_enemy_state_z_46
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(11))?;
        } // check_enemy_state_z_43
        let enemy_behavior_state_83 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_83 == 6 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_143 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_143.wrapping_add(1)))?;
let enemy_state_timer_144 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_144 > 10 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(5))?;
}
}
        let enemy_behavior_state_86 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_86 == 7 {
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_151 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_151.wrapping_add(1)))?;
let enemy_state_timer_152 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_152 > 50 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(8))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
}
}
        let enemy_behavior_state_89 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_89 == 8 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_157 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_157.wrapping_add(1)))?;
let enemy_state_timer_158 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_158 > 10 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_7
        let enemy_behavior_state_7 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_7 == 0 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_6 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_6.wrapping_add(1)))?;
let enemy_state_timer_7 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_7 > 10 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_25 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_25 != 1 { break 'check_enemy_state_z_27; }
        } // check_enemy_active
        r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
        let enemy_state_timer_23 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_23.wrapping_add(1)))?;
        let drawable_y_24 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_24 <= 50 {
let drawable_x_63 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_64 = r.get(object_7, field::drawable::X)?.i();
if !(drawable_x_63 >= (256_i32).wrapping_sub(drawable_x_64)) {
let drawable_x_95 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_95.wrapping_add(4)))?;
}
let drawable_x_93 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_94 = r.get(object_7, field::drawable::X)?.i();
if !(drawable_x_93 <= (256_i32).wrapping_sub(drawable_x_94)) {
let drawable_x_136 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_136.wrapping_sub(4)))?;
}
} else {
let drawable_y_48 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_48.wrapping_sub(4)))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
        let enemy_state_timer_74 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_74 <= 20 { break 'check_enemy_state_z_27; }
        } // next_step_20
        let object_value_6 = r.alloc("java/util/Random", Native::Empty);
        host::random_initialize(r, object_value_6)?;
        object_9 = object_value_6;
        let system_current_time_millis_value = host::system_current_time_millis(r)?;
        let math_abs_value_13 = host::math_abs(r, (system_current_time_millis_value) as i32)?;
        let random_next_int_2_value = host::random_next_int_2(r, object_value_6, math_abs_value_13)?;
        let math_abs_value_14 = host::math_abs(r, random_next_int_2_value)?;
        let remainder_value_20 = arithmetic::remainder_int(r, math_abs_value_14, 3)?;
        number_10 = remainder_value_20;
        let enemy_health = r.get(object_6, field::enemy::HEALTH)?.i();
        if enemy_health <= 20 {

if number_10 == 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(5))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::STATE_W, V::F(f32::from_bits(1084227584)))?;
let drawable_x_204 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_205 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_164 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_165 = r.get(object_6, field::drawable::Y)?.i();
let enemy_state_w_2 = r.get(object_6, field::enemy::STATE_W)?.f();
r.set(object_6, field::enemy::STATE_V_LOWER, V::F((((drawable_x_204.wrapping_sub(drawable_x_205)).wrapping_add(1)) as f32 / (((drawable_y_164.wrapping_sub(drawable_y_165)).wrapping_add(1)) as f32 / enemy_state_w_2))))?;
let drawable_x_206 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::enemy::STATE_X, V::F((drawable_x_206) as f32))?;
let drawable_y_166 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::enemy::STATE_Y, V::F((drawable_y_166) as f32))?;
}

if number_10 == 1 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(6))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}

if number_10 == 2 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(7))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
} else {

if number_10 == 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::STATE_W, V::F(f32::from_bits(1082130432)))?;
let drawable_x_207 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_208 = r.get(object_6, field::drawable::X)?.i();
let drawable_y_167 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_168 = r.get(object_6, field::drawable::Y)?.i();
let enemy_state_w_3 = r.get(object_6, field::enemy::STATE_W)?.f();
r.set(object_6, field::enemy::STATE_V_LOWER, V::F((((drawable_x_207.wrapping_sub(drawable_x_208)).wrapping_add(1)) as f32 / (((drawable_y_167.wrapping_sub(drawable_y_168)).wrapping_add(1)) as f32 / enemy_state_w_3))))?;
let drawable_x_209 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::enemy::STATE_X, V::F((drawable_x_209) as f32))?;
let drawable_y_169 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::enemy::STATE_Y, V::F((drawable_y_169) as f32))?;
}

if number_10 == 1 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(3))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}

if number_10 == 2 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(4))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_27
        let enemy_behavior_state_40 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_40 == 2 { break 'check_enemy_active_3; }
        } // check_enemy_state_z_32
        let enemy_behavior_state_52 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_52 != 5 { break 'check_enemy_state_z_35; }
        } // check_enemy_active_3
        r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
        let enemy_state_x = r.get(object_6, field::enemy::STATE_X)?.f();
        let enemy_state_v_lower = r.get(object_6, field::enemy::STATE_V_LOWER)?.f();
        r.set(object_6, field::enemy::STATE_X, V::F((enemy_state_x + enemy_state_v_lower)))?;
        let enemy_state_y = r.get(object_6, field::enemy::STATE_Y)?.f();
        let enemy_state_w = r.get(object_6, field::enemy::STATE_W)?.f();
        r.set(object_6, field::enemy::STATE_Y, V::F((enemy_state_y + enemy_state_w)))?;
        let enemy_state_x_2 = r.get(object_6, field::enemy::STATE_X)?.f();
        r.set(object_6, field::drawable::X, V::I((enemy_state_x_2) as i32))?;
        let enemy_state_y_2 = r.get(object_6, field::enemy::STATE_Y)?.f();
        r.set(object_6, field::drawable::Y, V::I((enemy_state_y_2) as i32))?;
        let drawable_y_47 = r.get(object_6, field::drawable::Y)?.i();
        if drawable_y_47 > 150 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
        let drawable_x_92 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_92 > 220 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
        let drawable_x_133 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_133 < 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
        } // check_enemy_state_z_35
        let enemy_behavior_state_61 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_61 == 3 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let drawable_x_134 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_135 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_134 > drawable_x_135 {
let drawable_x_169 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_169.wrapping_add(1)))?;
}
let drawable_x_167 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_168 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_167 < drawable_x_168 {
let drawable_x_203 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_203.wrapping_sub(1)))?;
}
let drawable_y_162 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_163 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_162 > drawable_y_163 {
let drawable_y_187 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_187.wrapping_add(1)))?;
}
let drawable_y_185 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_186 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_185 < drawable_y_186 {
let drawable_y_205 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_205.wrapping_sub(1)))?;
}
let enemy_state_timer_147 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_147.wrapping_add(1)))?;
let enemy_state_timer_148 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_148 > 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_69 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_69 == 4 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_112 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_112.wrapping_add(1)))?;
let enemy_state_timer_113 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_113 == 5 {
number_9 = 0;
number_10 = 0;
let projectile_value_9 = r.alloc("p", Native::Empty);
let world_resources_13 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_9 = super::resources::projectile_image(r, world_resources_13, 7)?;
super::projectile::initialize(r, projectile_value_9, projectile_image_value_9, 7, 0, 1)?;
object_11 = projectile_value_9;
let drawable_x_202 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_9, field::drawable::X, V::I(drawable_x_202.wrapping_add(16)))?;
let drawable_y_161 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I((drawable_y_161.wrapping_add(16)).wrapping_add(16)))?;
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_6 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_6, object_11)?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_74 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_74 == 6 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let drawable_x_200 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_201 = r.get(object_6, field::drawable::X)?.i();
if !(drawable_x_200.wrapping_sub(16) <= drawable_x_201) {
let drawable_x_231 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_231.wrapping_add(4)))?;
}
let drawable_x_229 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_230 = r.get(object_6, field::drawable::X)?.i();
if !(drawable_x_229.wrapping_sub(16) >= drawable_x_230) {
let drawable_x_244 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_244.wrapping_sub(4)))?;
}
let drawable_y_203 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_204 = r.get(object_6, field::drawable::Y)?.i();
if !(drawable_y_203.wrapping_sub(45) <= drawable_y_204) {
let drawable_y_212 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_212.wrapping_add(4)))?;
}
let drawable_y_210 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_211 = r.get(object_6, field::drawable::Y)?.i();
if !(drawable_y_210.wrapping_sub(45) >= drawable_y_211) {
let drawable_y_219 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_219.wrapping_sub(4)))?;
}
let enemy_state_timer_161 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_161.wrapping_add(1)))?;
let enemy_state_timer_162 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_162 > 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(8))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_79 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_79 == 7 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_137 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_137.wrapping_add(1)))?;
let enemy_state_timer_138 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_138 == 5 {
number_9 = 0;
number_10 = 0;
let projectile_value_12 = r.alloc("p", Native::Empty);
let world_resources_16 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_12 = super::resources::projectile_image(r, world_resources_16, 7)?;
super::projectile::initialize(r, projectile_value_12, projectile_image_value_12, 7, 0, 1)?;
object_11 = projectile_value_12;
let drawable_x_241 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_12, field::drawable::X, V::I(drawable_x_241.wrapping_add(16)))?;
let drawable_y_200 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I((drawable_y_200.wrapping_add(16)).wrapping_add(16)))?;
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_9 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_9, object_11)?;
let projectile_value_13 = r.alloc("p", Native::Empty);
let world_resources_17 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_13 = super::resources::projectile_image(r, world_resources_17, 7)?;
super::projectile::initialize(r, projectile_value_13, projectile_image_value_13, 7, 1, 1)?;
object_12 = projectile_value_13;
let drawable_x_242 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_13, field::drawable::X, V::I((drawable_x_242.wrapping_add(16)).wrapping_add(16)))?;
let drawable_y_201 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_12, field::drawable::Y, V::I((drawable_y_201.wrapping_add(16)).wrapping_add(16)))?;
r.set(object_12, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_10 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_10, object_12)?;
let projectile_value_14 = r.alloc("p", Native::Empty);
let world_resources_18 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_14 = super::resources::projectile_image(r, world_resources_18, 7)?;
super::projectile::initialize(r, projectile_value_14, projectile_image_value_14, 7, -1, 1)?;
object_11 = projectile_value_14;
let drawable_x_243 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_14, field::drawable::X, V::I((drawable_x_243.wrapping_add(-16)).wrapping_add(16)))?;
let drawable_y_202 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I((drawable_y_202.wrapping_add(16)).wrapping_add(16)))?;
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_11 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_11, object_11)?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_84 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_84 == 8 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_145 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_145.wrapping_add(1)))?;
let enemy_state_timer_146 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_146 == 8 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(9))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_87 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_87 == 9 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_153 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_153.wrapping_add(1)))?;
r.set(object_6, field::enemy::HEIGHT, V::I(64))?;
let enemy_state_timer_154 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_154 == 3 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(10))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_90 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_90 == 10 {
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_159 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_159.wrapping_add(1)))?;
r.set(object_6, field::enemy::HEIGHT, V::I(64))?;
let enemy_state_timer_160 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_160 == 5 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::HEIGHT, V::I(48))?;
}
}
        let enemy_behavior_state_91 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_91 == 20 {
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_163 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_163.wrapping_add(1)))?;
let enemy_hit_count_3 = r.get(object_6, field::enemy::HIT_COUNT)?.i();
if enemy_hit_count_3 == 3 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(21))?;
}
let enemy_state_timer_168 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_168 == 70 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(21))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_92 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_92 != 21 { break 'check_enemy_state_f_lower; }
        } // check_enemy_active_6
        r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
        let enemy_state_timer_166 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_166.wrapping_add(1)))?;
        let enemy_state_timer_167 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_167 != 5 { break 'check_enemy_state_f_14; }
        } // next_step_26
        number_9 = 0;
        } // check_world_enemies_2
        'update_enemies_loop_137: loop {
            'check_world_enemies_3: {
            let world_enemies_8 = r.get(object_8, field::world::ENEMIES)?.r();
            let vector_size_value_15 = host::vector_size(r, world_enemies_8)?;
            if number_9 >= vector_size_value_15 { break 'check_enemy_random; }
            } // check_world_enemies_3
            let world_enemies_9 = r.get(object_8, field::world::ENEMIES)?.r();
            let vector_element_at_value_15 = host::vector_element_at(r, world_enemies_9, number_9)?;
            host::check_cast(r, vector_element_at_value_15, "l")?;
            object_10 = vector_element_at_value_15;
            let enemy_kind_34 = r.get(vector_element_at_value_15, field::enemy::KIND)?.i();
            if enemy_kind_34 == 134 {
let enemy_value_4 = r.alloc("l", Native::Empty);
let world_resources_24 = r.get(object_8, field::world::RESOURCES)?.r();
let enemy_image_value_4 = super::resources::enemy_image(r, world_resources_24, 36)?;
let drawable_x_264 = r.get(object_10, field::drawable::X)?.i();
let drawable_y_233 = r.get(object_10, field::drawable::Y)?.i();
super::enemy::initialize(r, enemy_value_4, enemy_image_value_4, drawable_x_264, drawable_y_233.wrapping_sub(24), 1, 36)?;
object_11 = enemy_value_4;
let world_enemies_11 = r.get(object_8, field::world::ENEMIES)?.r();
host::vector_add_element(r, world_enemies_11, object_11)?;
r.set(object_10, field::enemy::BEHAVIOR_STATE, V::I(1))?;
}
            number_9 = number_9.wrapping_add(1);
            continue 'update_enemies_loop_137;
        }
        } // check_enemy_random
        let enemy_random_18 = r.get(object_6, field::enemy::RANDOM)?.r();
        let random_next_int_value_34 = host::random_next_int(r, enemy_random_18)?;
        let remainder_value_50 = arithmetic::remainder_int(r, random_next_int_value_34, 10)?;
        let math_abs_value_41 = host::math_abs(r, remainder_value_50)?;
        number_9 = math_abs_value_41;
        number_10 = 0;
        } // next_step_29
        'update_enemies_loop_140: loop {
            'check_enemy_random_2: {
            if number_10 >= number_9 { break 'check_enemy_state_f_14; }
            } // check_enemy_random_2
            let enemy_random_20 = r.get(object_6, field::enemy::RANDOM)?.r();
            let random_next_int_value_36 = host::random_next_int(r, enemy_random_20)?;
            let remainder_value_56 = arithmetic::remainder_int(r, random_next_int_value_36, 200)?;
            let math_abs_value_43 = host::math_abs(r, remainder_value_56)?;
            number_11 = math_abs_value_43;
            let enemy_random_21 = r.get(object_6, field::enemy::RANDOM)?.r();
            let random_next_int_value_37 = host::random_next_int(r, enemy_random_21)?;
            let remainder_value_57 = arithmetic::remainder_int(r, random_next_int_value_37, 100)?;
            let math_abs_value_44 = host::math_abs(r, remainder_value_57)?;
            number_12 = math_abs_value_44;
            let enemy_value_5 = r.alloc("l", Native::Empty);
            let world_resources_26 = r.get(object_8, field::world::RESOURCES)?.r();
            let enemy_image_value_5 = super::resources::enemy_image(r, world_resources_26, 36)?;
            super::enemy::initialize(r, enemy_value_5, enemy_image_value_5, number_11.wrapping_add(50), number_12.wrapping_add(50), 1, 36)?;
            object_11 = enemy_value_5;
            let world_enemies_12 = r.get(object_8, field::world::ENEMIES)?.r();
            host::vector_add_element(r, world_enemies_12, object_11)?;
            number_10 = number_10.wrapping_add(1);
            continue 'update_enemies_loop_140;
        }
        } // check_enemy_state_f_14
        let enemy_state_timer_169 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_169 == 30 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::HIT_COUNT, V::I(0))?;
}
        break 'check_enemy_state_f_lower;
        } // next_step_6

        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_8
        let enemy_behavior_state_8 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_8 == 0 {
let world_random = r.get(object_8, field::world::RANDOM)?.r();
let random_next_int_value_4 = host::random_next_int(r, world_random)?;
let remainder_value_2 = arithmetic::remainder_int(r, random_next_int_value_4, 20)?;
let math_abs_value = host::math_abs(r, remainder_value_2)?;
number_9 = math_abs_value;
if math_abs_value == 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
}
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
}
        let enemy_behavior_state_26 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_26 == 1 {
let enemy_state_timer_24 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_24.wrapping_add(1)))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(2))?;
let enemy_state_timer_25 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_25 >= 30 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
}
let enemy_state_timer_47 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_47 >= 20 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
}
}
        let enemy_behavior_state_41 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_41 == 2 {
let enemy_state_timer_45 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_45.wrapping_add(1)))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(4))?;
let enemy_state_timer_46 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_46 > 10 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
let world_random_5 = r.get(object_8, field::world::RANDOM)?.r();
let random_next_int_value_16 = host::random_next_int(r, world_random_5)?;
let remainder_value_14 = arithmetic::remainder_int(r, random_next_int_value_16, 10)?;
let math_abs_value_6 = host::math_abs(r, remainder_value_14)?;
number_9 = math_abs_value_6;
if math_abs_value_6 > 3 {

if number_9 < 6 {
let world_random_8 = r.get(object_8, field::world::RANDOM)?.r();
let random_next_int_value_22 = host::random_next_int(r, world_random_8)?;
let remainder_value_21 = arithmetic::remainder_int(r, random_next_int_value_22, 4)?;
let math_abs_value_19 = host::math_abs(r, remainder_value_21)?;
number_9 = math_abs_value_19;
r.set(object_6, field::enemy::FACING, V::I((number_9) as i8 as i32))?;
}
} else {
let drawable_x_137 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_138 = r.get(object_6, field::drawable::X)?.i();
let math_abs_value_15 = host::math_abs(r, drawable_x_137.wrapping_sub(drawable_x_138))?;
number_10 = math_abs_value_15;
let drawable_y_112 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_113 = r.get(object_6, field::drawable::Y)?.i();
let math_abs_value_16 = host::math_abs(r, drawable_y_112.wrapping_sub(drawable_y_113))?;
number_11 = math_abs_value_16;
if number_10 < number_11 {
let drawable_y_146 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_147 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_146 > drawable_y_147 {
r.set(object_6, field::enemy::FACING, V::I(2))?;
}
let drawable_y_170 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_171 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_170 < drawable_y_171 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
}
} else {
let drawable_x_170 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_171 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_170 > drawable_x_171 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
let drawable_x_210 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_211 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_210 < drawable_x_211 {
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
}
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_d_lower
        r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
        let enemy_facing_2 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_2 == 0 {
let drawable_x_24 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_25 = r.get(object_7, field::drawable::X)?.i();
if drawable_x_24 > drawable_x_25 {
let drawable_x_37 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_38 = r.get(object_7, field::drawable::X)?.i();
if !(drawable_x_37 >= drawable_x_38.wrapping_add(32)) {
let drawable_y_51 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_52 = r.get(object_7, field::drawable::Y)?.i();
if !((drawable_y_51.wrapping_sub(drawable_y_52)).wrapping_sub(32) <= -50) {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(3))?;
}
}
}
}
        let enemy_facing_4 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_4 == 1 {
let drawable_y_25 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_26 = r.get(object_7, field::drawable::Y)?.i();
if !(drawable_y_25 <= drawable_y_26.wrapping_sub(32)) {
let drawable_y_49 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_50 = r.get(object_7, field::drawable::Y)?.i();
if drawable_y_49 < drawable_y_50 {
let drawable_x_98 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_99 = r.get(object_7, field::drawable::X)?.i();
if !(drawable_x_98.wrapping_sub(drawable_x_99) >= 50) {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(3))?;
}
}
}
}
        let enemy_facing_8 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_8 == 2 {
let drawable_x_65 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_66 = r.get(object_7, field::drawable::X)?.i();
if drawable_x_65 > drawable_x_66 {
let drawable_x_96 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_97 = r.get(object_7, field::drawable::X)?.i();
if !(drawable_x_96 >= drawable_x_97.wrapping_add(32)) {
let drawable_y_116 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_117 = r.get(object_7, field::drawable::Y)?.i();
if !((drawable_y_116.wrapping_sub(drawable_y_117)).wrapping_sub(32) >= 50) {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(3))?;
}
}
}
}
        let enemy_facing_15 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_15 == 3 {
let drawable_y_76 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_77 = r.get(object_7, field::drawable::Y)?.i();
if !(drawable_y_76 <= drawable_y_77.wrapping_sub(32)) {
let drawable_y_114 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_115 = r.get(object_7, field::drawable::Y)?.i();
if drawable_y_114 < drawable_y_115 {
let drawable_x_172 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_173 = r.get(object_7, field::drawable::X)?.i();
if !(drawable_x_172.wrapping_sub(drawable_x_173) <= -50) {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(3))?;
}
}
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_9
        let enemy_behavior_state_9 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_9 == 0 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(10))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(0))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_8 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_8.wrapping_add(1)))?;
let enemy_state_timer_9 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_9 > 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_27 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_27 == 1 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_26 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_26.wrapping_add(1)))?;
let enemy_state_timer_27 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_27 > 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_42 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_42 == 2 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_48 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_48.wrapping_add(1)))?;
let enemy_state_timer_49 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_49 > 100 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_53 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_53 == 3 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_75 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_75.wrapping_add(1)))?;
let enemy_state_timer_76 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_76 > 100 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_62 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_62 == 4 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_95 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_95.wrapping_add(1)))?;
let enemy_state_timer_96 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_96 > 24 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(5))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_70 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_70 == 5 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_114 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_114.wrapping_add(1)))?;
let enemy_state_timer_115 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_115 > 100 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(6))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(24))?;
}
let drawable_x_212 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_213 = r.get(object_6, field::drawable::X)?.i();
number_9 = drawable_x_212.wrapping_sub(drawable_x_213);
let drawable_y_172 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_173 = r.get(object_6, field::drawable::Y)?.i();
number_10 = drawable_y_172.wrapping_sub(drawable_y_173);
let math_abs_value_25 = host::math_abs(r, number_9)?;
let math_abs_value_26 = host::math_abs(r, number_10)?;
if math_abs_value_25 <= math_abs_value_26 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
} else {
let drawable_y_188 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_189 = r.get(object_7, field::drawable::Y)?.i();
if !(drawable_y_188 >= drawable_y_189.wrapping_sub(32)) {
let drawable_x_245 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_246 = r.get(object_7, field::drawable::X)?.i();
if drawable_x_245 < drawable_x_246 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
let drawable_x_251 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_252 = r.get(object_7, field::drawable::X)?.i();
if drawable_x_251 > drawable_x_252 {
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
}
}
let enemy_random_12 = r.get(object_6, field::enemy::RANDOM)?.r();
let random_next_int_value_28 = host::random_next_int(r, enemy_random_12)?;
let remainder_value_30 = arithmetic::remainder_int(r, random_next_int_value_28, 10)?;
if remainder_value_30 == 1 {
number_11 = 0;
number_12 = 0;
let math_abs_value_35 = host::math_abs(r, number_9)?;
if math_abs_value_35 > 30 {

if number_9 <= 0 {
number_12 = -1;
} else {
number_12 = 1;
}
}
let projectile_value_17 = r.alloc("p", Native::Empty);
let world_resources_21 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_17 = super::resources::projectile_image(r, world_resources_21, 0)?;
super::projectile::initialize(r, projectile_value_17, projectile_image_value_17, 0, number_12, 1)?;
object_11 = projectile_value_17;
let drawable_x_256 = r.get(object_6, field::drawable::X)?.i();
let enemy_state_v_7 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_7 = r.array_get(enemy_state_v_7, 0)?.i();
let enemy_state_v_8 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_8 = r.array_get(enemy_state_v_8, 1)?.i();
let enemy_state_v_9 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_9 = r.array_get(enemy_state_v_9, 2)?.i();
let enemy_state_v_10 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_10 = r.array_get(enemy_state_v_10, 3)?.i();
let enemy_state_v_11 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_11 = r.array_get(enemy_state_v_11, 4)?.i();
let enemy_state_v_12 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_12 = r.array_get(enemy_state_v_12, 5)?.i();
r.set(projectile_value_17, field::drawable::X, V::I(((((((((((((drawable_x_256.wrapping_add(number_12.wrapping_mul(16))).wrapping_add(5)).wrapping_sub(element_value_7)).wrapping_add(5)).wrapping_sub(element_value_8)).wrapping_add(5)).wrapping_sub(element_value_9)).wrapping_add(5)).wrapping_sub(element_value_10)).wrapping_add(5)).wrapping_sub(element_value_11)).wrapping_add(5)).wrapping_sub(element_value_12)))?;
let drawable_y_220 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I((drawable_y_220.wrapping_add(16)).wrapping_sub(60)))?;
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_17 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_17, object_11)?;
}
}
        let enemy_behavior_state_75 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_75 == 6 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_124 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_124.wrapping_sub(1)))?;
let enemy_state_timer_125 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_125 < 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_10
        let enemy_behavior_state_10 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_10 == 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
}
        let enemy_behavior_state_28 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_28 == 1 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_28 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_28.wrapping_add(1)))?;
let enemy_state_timer_29 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_29 > 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_43 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_43 == 2 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_50 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_50.wrapping_add(1)))?;
let enemy_state_timer_51 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_51 > 100 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(4))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(5))?;
}
}
        let enemy_behavior_state_54 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_54 == 4 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_77 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_77.wrapping_add(1)))?;
let enemy_state_timer_78 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_78 > 24 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(5))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_63 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_63 == 5 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_97 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_97.wrapping_add(1)))?;
let enemy_state_timer_98 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_98 > 100 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(7))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
let drawable_x_174 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_175 = r.get(object_6, field::drawable::X)?.i();
number_9 = drawable_x_174.wrapping_sub(drawable_x_175);
let drawable_y_148 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_149 = r.get(object_6, field::drawable::Y)?.i();
number_10 = drawable_y_148.wrapping_sub(drawable_y_149);
let math_abs_value_20 = host::math_abs(r, number_9)?;
let math_abs_value_21 = host::math_abs(r, number_10)?;
if math_abs_value_20 <= math_abs_value_21 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
} else {
let drawable_y_174 = r.get(object_6, field::drawable::Y)?.i();
let drawable_y_175 = r.get(object_7, field::drawable::Y)?.i();
if !(drawable_y_174 >= drawable_y_175.wrapping_sub(32)) {
let drawable_x_232 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_233 = r.get(object_7, field::drawable::X)?.i();
if drawable_x_232 < drawable_x_233 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
let drawable_x_247 = r.get(object_6, field::drawable::X)?.i();
let drawable_x_248 = r.get(object_7, field::drawable::X)?.i();
if drawable_x_247 > drawable_x_248 {
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
}
}
let enemy_random_10 = r.get(object_6, field::enemy::RANDOM)?.r();
let random_next_int_value_25 = host::random_next_int(r, enemy_random_10)?;
let remainder_value_25 = arithmetic::remainder_int(r, random_next_int_value_25, 15)?;
if remainder_value_25 == 1 {
number_11 = 0;
number_12 = 0;
let math_abs_value_32 = host::math_abs(r, number_9)?;
if math_abs_value_32 > 30 {

if number_9 <= 0 {
number_12 = -1;
} else {
number_12 = 1;
}
}
let projectile_value_15 = r.alloc("p", Native::Empty);
let world_resources_19 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_15 = super::resources::projectile_image(r, world_resources_19, 0)?;
super::projectile::initialize(r, projectile_value_15, projectile_image_value_15, 0, number_12, 1)?;
object_11 = projectile_value_15;
let drawable_x_253 = r.get(object_6, field::drawable::X)?.i();
let enemy_state_v = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value = r.array_get(enemy_state_v, 0)?.i();
let enemy_state_v_2 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_2 = r.array_get(enemy_state_v_2, 1)?.i();
let enemy_state_v_3 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_3 = r.array_get(enemy_state_v_3, 2)?.i();
let enemy_state_v_4 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_4 = r.array_get(enemy_state_v_4, 3)?.i();
let enemy_state_v_5 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_5 = r.array_get(enemy_state_v_5, 4)?.i();
let enemy_state_v_6 = r.get(object_6, field::enemy::STATE_V)?.r();
let element_value_6 = r.array_get(enemy_state_v_6, 5)?.i();
r.set(projectile_value_15, field::drawable::X, V::I(((((((((((((drawable_x_253.wrapping_add(number_12.wrapping_mul(16))).wrapping_add(5)).wrapping_sub(element_value)).wrapping_add(5)).wrapping_sub(element_value_2)).wrapping_add(5)).wrapping_sub(element_value_3)).wrapping_add(5)).wrapping_sub(element_value_4)).wrapping_add(5)).wrapping_sub(element_value_5)).wrapping_add(5)).wrapping_sub(element_value_6)))?;
let drawable_y_213 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I((drawable_y_213.wrapping_add(16)).wrapping_sub(60)))?;
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
r.set(object_11, field::projectile::DAMAGE, V::I(3))?;
let world_projectiles_14 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_14, object_11)?;
}
}
        let enemy_behavior_state_71 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_71 == 6 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_116 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_116.wrapping_sub(1)))?;
let enemy_state_timer_117 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_117 < 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_76 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_76 == 7 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_126 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_126.wrapping_add(1)))?;
let enemy_state_timer_127 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_127 > 100 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(6))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(24))?;
}
}
        let enemy_behavior_state_80 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_80 == 8 {
let enemy_state_timer_139 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_139.wrapping_add(1)))?;
let enemy_state_timer_140 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_140 > 160 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(6))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(24))?;
let player_state_ag = r.get(object_7, field::player::STATE_AG)?.i();
if player_state_ag == 1 {
r.set(object_7, field::player::HEALTH, V::I(0))?;
}
}
}
        let enemy_behavior_state_85 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_85 == 9 {
let enemy_state_timer_149 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_149.wrapping_add(1)))?;
let enemy_state_timer_150 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_150 > 32 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(10))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_7, field::drawable::X, V::I(110))?;
r.set(object_7, field::drawable::Y, V::I(130))?;
r.set(object_7, field::player::HURT_TIMER, V::I(10))?;
r.set(object_7, field::player::STATE_AG, V::I(0))?;
}
}
        let enemy_behavior_state_88 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_88 == 10 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_155 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_155.wrapping_add(1)))?;
let enemy_state_timer_156 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_156 > 100 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(6))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(24))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_11
        let enemy_behavior_state_11 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_11 == 0 {
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(4))?;
let drawable_x_26 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_27 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_26 >= drawable_x_27 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
} else {
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
let enemy_state_timer_54 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_54.wrapping_add(1)))?;
let drawable_x_69 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_70 = r.get(object_6, field::drawable::X)?.i();
number_9 = drawable_x_69.wrapping_sub(drawable_x_70);
let drawable_y_55 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_56 = r.get(object_6, field::drawable::Y)?.i();
number_10 = drawable_y_55.wrapping_sub(drawable_y_56);
let enemy_state_timer_55 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_55 == 25 {
let enemy_kind_10 = r.get(object_6, field::enemy::KIND)?.i();
if enemy_kind_10 == 30 {
let projectile_value_3 = r.alloc("p", Native::Empty);
let world_resources_5 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_3 = super::resources::projectile_image(r, world_resources_5, 9)?;
super::projectile::initialize(r, projectile_value_3, projectile_image_value_3, 1, -1, 0)?;
object_11 = projectile_value_3;
let drawable_x_139 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_3, field::drawable::X, V::I(drawable_x_139.wrapping_add(4)))?;
let drawable_y_118 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I(drawable_y_118.wrapping_add(4)))?;
r.set(object_11, field::projectile::KIND, V::I(9))?;
r.set(object_11, field::projectile::VELOCITY_Y, V::I(0))?;
r.set(object_11, field::projectile::VELOCITY_X, V::I(0))?;
let math_abs_value_17 = host::math_abs(r, number_9)?;
let math_abs_value_18 = host::math_abs(r, number_10)?;
if math_abs_value_17 >= math_abs_value_18 {

if number_9 <= 0 {
r.set(object_11, field::projectile::VELOCITY_X, V::I(-1))?;
} else {
r.set(object_11, field::projectile::VELOCITY_X, V::I(1))?;
}
let math_abs_value_29 = host::math_abs(r, number_10)?;
if math_abs_value_29 > 50 {

if number_10 <= 0 {
r.set(object_11, field::projectile::VELOCITY_Y, V::I(-1))?;
} else {
r.set(object_11, field::projectile::VELOCITY_Y, V::I(1))?;
}
}
} else {

if number_10 <= 0 {
r.set(object_11, field::projectile::VELOCITY_Y, V::I(-1))?;
} else {
r.set(object_11, field::projectile::VELOCITY_Y, V::I(1))?;
}
let math_abs_value_30 = host::math_abs(r, number_9)?;
if math_abs_value_30 > 50 {

if number_9 <= 0 {
r.set(object_11, field::projectile::VELOCITY_X, V::I(-1))?;
} else {
r.set(object_11, field::projectile::VELOCITY_X, V::I(1))?;
}
}
}
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_12 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_12, object_11)?;
}
let enemy_kind_11 = r.get(object_6, field::enemy::KIND)?.i();
if enemy_kind_11 == 31 {
let projectile_value_7 = r.alloc("p", Native::Empty);
let world_resources_10 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_7 = super::resources::projectile_image(r, world_resources_10, 10)?;
super::projectile::initialize(r, projectile_value_7, projectile_image_value_7, 1, -1, 0)?;
object_11 = projectile_value_7;
let drawable_x_176 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_7, field::drawable::X, V::I(drawable_x_176.wrapping_add(4)))?;
let drawable_y_150 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_11, field::drawable::Y, V::I(drawable_y_150.wrapping_add(4)))?;
r.set(object_11, field::projectile::KIND, V::I(10))?;
r.set(object_11, field::projectile::VELOCITY_Y, V::I(0))?;
r.set(object_11, field::projectile::VELOCITY_X, V::I(0))?;
let math_abs_value_22 = host::math_abs(r, number_9)?;
let math_abs_value_23 = host::math_abs(r, number_10)?;
if math_abs_value_22 >= math_abs_value_23 {

if number_9 <= 0 {
r.set(object_11, field::projectile::VELOCITY_X, V::I(-1))?;
} else {
r.set(object_11, field::projectile::VELOCITY_X, V::I(1))?;
}
let math_abs_value_33 = host::math_abs(r, number_10)?;
if math_abs_value_33 > 50 {

if number_10 <= 0 {
r.set(object_11, field::projectile::VELOCITY_Y, V::I(-1))?;
} else {
r.set(object_11, field::projectile::VELOCITY_Y, V::I(1))?;
}
}
} else {

if number_10 <= 0 {
r.set(object_11, field::projectile::VELOCITY_Y, V::I(-1))?;
} else {
r.set(object_11, field::projectile::VELOCITY_Y, V::I(1))?;
}
let math_abs_value_34 = host::math_abs(r, number_9)?;
if math_abs_value_34 > 50 {

if number_9 <= 0 {
r.set(object_11, field::projectile::VELOCITY_X, V::I(-1))?;
} else {
r.set(object_11, field::projectile::VELOCITY_X, V::I(1))?;
}
}
}
r.set(object_11, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_15 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_15, object_11)?;
}
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
let math_abs_value_7 = host::math_abs(r, number_9)?;
let math_abs_value_8 = host::math_abs(r, number_10)?;
if !(math_abs_value_7.wrapping_add(math_abs_value_8) >= 50) {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_29 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_29 == 1 {
let enemy_state_timer_30 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
let enemy_random_4 = r.get(object_6, field::enemy::RANDOM)?.r();
let random_next_int_value_8 = host::random_next_int(r, enemy_random_4)?;
let remainder_value_6 = arithmetic::remainder_int(r, random_next_int_value_8, 5)?;
let math_abs_value_3 = host::math_abs(r, remainder_value_6)?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_30.wrapping_add(math_abs_value_3)))?;
r.set(object_6, field::enemy::STATE_H, V::I(0))?;
let enemy_state_timer_31 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_31 > 200 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
let drawable_x_67 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_68 = r.get(object_6, field::drawable::X)?.i();
number_9 = drawable_x_67.wrapping_sub(drawable_x_68);
let drawable_y_53 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_54 = r.get(object_6, field::drawable::Y)?.i();
number_10 = drawable_y_53.wrapping_sub(drawable_y_54);
if !((number_9.wrapping_mul(number_9)).wrapping_add(number_10.wrapping_mul(number_10)) >= 1024) {
number_11 = 0;
let enemy_facing_22 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_22 == 0 {
r.set(object_6, field::enemy::FACING, V::I(2))?;
number_11 = number_11.wrapping_add(1);
}
let enemy_facing_30 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_30 == 1 {
r.set(object_6, field::enemy::FACING, V::I(3))?;
number_11 = number_11.wrapping_add(1);
}
let enemy_facing_36 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_36 == 2 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
number_11 = number_11.wrapping_add(1);
}
let enemy_facing_41 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_41 == 3 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
number_11 = number_11.wrapping_add(1);
}

if number_11 > 1 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
let enemy_state_m = r.get(object_6, field::enemy::STATE_M)?.i();
r.set(object_6, field::enemy::STATE_M, V::I(enemy_state_m.wrapping_neg()))?;
}
}
        let enemy_behavior_state_44 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_44 == 2 {
let enemy_state_timer_52 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_52.wrapping_add(1)))?;
let enemy_state_timer_53 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_53 > 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(3))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_55 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_55 == 3 {
let enemy_state_timer_79 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_79.wrapping_add(1)))?;
let enemy_state_timer_80 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_80 > 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(10))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(0))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
}
        let enemy_behavior_state_64 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_64 == 4 {
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
let enemy_state_timer_99 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_99.wrapping_add(1)))?;
let enemy_state_timer_100 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_100 > 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_12
        let enemy_behavior_state_12 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_12 == 0 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(10))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(0))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_10 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_10.wrapping_add(1)))?;
let enemy_state_timer_11 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_11 > 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_30 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_30 == 1 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_32 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_32.wrapping_add(1)))?;
let enemy_state_timer_33 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_33 > 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
}
        let enemy_behavior_state_45 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_45 == 2 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_56 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_56.wrapping_add(1)))?;
let enemy_state_timer_57 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_57 > 12 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(3))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_56 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_56 == 3 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(0))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_81 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
let enemy_random_6 = r.get(object_6, field::enemy::RANDOM)?.r();
let random_next_int_value_17 = host::random_next_int(r, enemy_random_6)?;
let remainder_value_15 = arithmetic::remainder_int(r, random_next_int_value_17, 3)?;
let math_abs_value_9 = host::math_abs(r, remainder_value_15)?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_81.wrapping_add(math_abs_value_9.wrapping_add(1))))?;
let enemy_state_timer_82 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_82 > 200 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(4))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
}
        let enemy_behavior_state_65 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_65 == 4 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_101 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_101.wrapping_add(1)))?;
let enemy_state_timer_102 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_102 > 12 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_13
        let enemy_behavior_state_13 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_13 == 0 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(10))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::STATE_H, V::I(0))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
let enemy_state_timer_12 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
let enemy_random_2 = r.get(object_6, field::enemy::RANDOM)?.r();
let random_next_int_value_5 = host::random_next_int(r, enemy_random_2)?;
let remainder_value_3 = arithmetic::remainder_int(r, random_next_int_value_5, 3)?;
let math_abs_value_2 = host::math_abs(r, remainder_value_3)?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_12.wrapping_add(math_abs_value_2.wrapping_add(1))))?;
let enemy_state_timer_13 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_13 > 150 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_31 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_31 == 1 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_34 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_34.wrapping_add(1)))?;
let enemy_state_timer_35 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_35 > 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
}
        let enemy_behavior_state_46 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_46 == 2 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_58 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_58.wrapping_add(1)))?;
let enemy_state_timer_59 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_59 > 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(3))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
}
        let enemy_behavior_state_57 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_57 == 3 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_83 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_83.wrapping_add(1)))?;
let enemy_state_timer_84 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_84 == 5 {
let projectile_value_4 = r.alloc("p", Native::Empty);
let world_resources_6 = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value_4 = super::resources::projectile_image(r, world_resources_6, 1)?;
super::projectile::initialize(r, projectile_value_4, projectile_image_value_4, 1, -1, 0)?;
object_9 = projectile_value_4;
let drawable_x_140 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value_4, field::drawable::X, V::I(drawable_x_140.wrapping_add(4)))?;
let drawable_y_119 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_9, field::drawable::Y, V::I(drawable_y_119.wrapping_add(4)))?;
let drawable_y_120 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_121 = r.get(object_6, field::drawable::Y)?.i();
if drawable_y_120 >= drawable_y_121 {
r.set(object_9, field::projectile::VELOCITY_Y, V::I(1))?;
} else {
r.set(object_9, field::projectile::VELOCITY_Y, V::I(-1))?;
}
let drawable_x_214 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_215 = r.get(object_6, field::drawable::X)?.i();
if drawable_x_214 >= drawable_x_215 {
r.set(object_9, field::projectile::VELOCITY_X, V::I(1))?;
} else {
r.set(object_9, field::projectile::VELOCITY_X, V::I(-1))?;
}
r.set(object_9, field::projectile::ENEMY_OWNED, V::I(1))?;
let world_projectiles_13 = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles_13, object_9)?;
}
let enemy_state_timer_105 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_105 > 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(4))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
}
        let enemy_behavior_state_66 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_66 == 4 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let enemy_state_timer_103 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_103.wrapping_add(1)))?;
let enemy_state_timer_104 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_104 > 10 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // next_step_7

        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_14
        let enemy_behavior_state_14 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_14 == 0 {
let drawable_y_12 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_12.wrapping_add(4)))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
let drawable_y_13 = r.get(object_6, field::drawable::Y)?.i();
let enemy_state_s_lower = r.get(object_6, field::enemy::STATE_S_LOWER)?.i();
if drawable_y_13 >= enemy_state_s_lower {
let enemy_state_s_lower_2 = r.get(object_6, field::enemy::STATE_S_LOWER)?.i();
r.set(object_6, field::drawable::Y, V::I(enemy_state_s_lower_2))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_drawable_x_3
        let drawable_x_10 = r.get(object_7, field::drawable::X)?.i();
        let drawable_x_11 = r.get(object_6, field::drawable::X)?.i();
        number_9 = (drawable_x_10.wrapping_add(8)).wrapping_sub(drawable_x_11.wrapping_add(16));
        let drawable_y_4 = r.get(object_7, field::drawable::Y)?.i();
        let drawable_y_5 = r.get(object_6, field::drawable::Y)?.i();
        number_10 = (drawable_y_4.wrapping_add(12)).wrapping_sub(drawable_y_5.wrapping_add(16));
        let enemy_behavior_state_15 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_15 != 0 { break 'check_enemy_state_z_22; }
        } // check_enemy_state_h_2
        r.set(object_6, field::enemy::STATE_H, V::I(1))?;
        let enemy_facing_5 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_5 == 2 { break 'check_enemy_facing_2; }
        } // check_enemy_facing_3
        let enemy_facing_9 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_9 != 3 { break 'check_enemy_facing_6; }
        } // check_enemy_facing_2
        r.set(object_6, field::enemy::FACING, V::I(0))?;
        } // check_enemy_facing_6
        let enemy_facing_16 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_16 != 0 {
let drawable_x_100 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_100.wrapping_sub(1)))?;
} else {
let drawable_x_101 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_101.wrapping_add(1)))?;
}
        let drawable_x_142 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_142 < 50 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
}
        let drawable_x_179 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_179 > 200 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
        let enemy_state_timer_130 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_130.wrapping_add(1)))?;
        r.set(object_6, field::enemy::DEAD, V::I(0))?;
        r.set(object_6, field::enemy::STATE_H, V::I(1))?;
        let enemy_state_timer_131 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_131 > 50 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
let enemy_random_11 = r.get(object_6, field::enemy::RANDOM)?.r();
let random_next_int_value_26 = host::random_next_int(r, enemy_random_11)?;
let remainder_value_26 = arithmetic::remainder_int(r, random_next_int_value_26, 2)?;
if remainder_value_26 != 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
} else {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
}
}
        } // check_enemy_state_z_22
        let enemy_behavior_state_32 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_32 != 1 { break 'check_enemy_state_z_28; }
        } // check_enemy_state_f_5
        let enemy_state_timer_36 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_36.wrapping_add(1)))?;
        r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
        let enemy_state_timer_37 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        let remainder_value_7 = arithmetic::remainder_int(r, enemy_state_timer_37, 9)?;
        if remainder_value_7 != 1 { break 'check_enemy_state_f_8; }
        } // next_step_13
        number_11 = 0;
        } // next_step_17
        'update_enemies_loop_161: loop {
            'next_step_21: {
            if number_11 >= 3 { break 'check_enemy_state_f_8; }
            } // next_step_21
            let projectile_value_5 = r.alloc("p", Native::Empty);
            let world_resources_7 = r.get(object_8, field::world::RESOURCES)?.r();
            let projectile_image_value_5 = super::resources::projectile_image(r, world_resources_7, 1)?;
            super::projectile::initialize(r, projectile_value_5, projectile_image_value_5, 1, -1, 0)?;
            object_12 = projectile_value_5;
            let drawable_x_141 = r.get(object_6, field::drawable::X)?.i();
            r.set(projectile_value_5, field::drawable::X, V::I(drawable_x_141.wrapping_add(14)))?;
            let drawable_y_122 = r.get(object_6, field::drawable::Y)?.i();
            r.set(object_12, field::drawable::Y, V::I(drawable_y_122.wrapping_add(16)))?;
            r.set(object_12, field::projectile::VELOCITY_Y, V::I(1))?;
            r.set(object_12, field::projectile::VELOCITY_X, V::I((1_i32).wrapping_sub(number_11)))?;
            r.set(object_12, field::projectile::ENEMY_OWNED, V::I(1))?;
            let world_projectiles_3 = r.get(object_8, field::world::PROJECTILES)?.r();
            host::vector_add_element(r, world_projectiles_3, object_12)?;
            number_11 = number_11.wrapping_add(1);
            continue 'update_enemies_loop_161;
        }
        } // check_enemy_state_f_8
        let enemy_state_timer_62 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_62 > 20 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
let enemy_random_7 = r.get(object_6, field::enemy::RANDOM)?.r();
let random_next_int_value_18 = host::random_next_int(r, enemy_random_7)?;
let remainder_value_16 = arithmetic::remainder_int(r, random_next_int_value_18, 2)?;
if remainder_value_16 != 0 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
} else {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
}
}
        } // check_enemy_state_z_28
        let enemy_behavior_state_47 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_47 == 2 {
let enemy_state_timer_60 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_60.wrapping_add(1)))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
let enemy_state_timer_61 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_61 > 10 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(4))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_58 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_58 == 4 {
let enemy_state_timer_85 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_85.wrapping_add(1)))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
let enemy_state_timer_86 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_86 > 10 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(5))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_67 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_67 == 5 {
let enemy_state_timer_106 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_106.wrapping_add(1)))?;
r.set(object_6, field::enemy::DEAD, V::I(1))?;
r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(2))?;
let enemy_state_timer_107 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_107 < 80 {
let drawable_x_177 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_178 = r.get(object_6, field::drawable::X)?.i();
if !(drawable_x_177 <= drawable_x_178.wrapping_sub(16)) {
let drawable_x_218 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_218.wrapping_add(1)))?;
}
let drawable_x_216 = r.get(object_7, field::drawable::X)?.i();
let drawable_x_217 = r.get(object_6, field::drawable::X)?.i();
if !(drawable_x_216 >= drawable_x_217.wrapping_add(16)) {
let drawable_x_235 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_235.wrapping_sub(1)))?;
}
let drawable_y_190 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_191 = r.get(object_6, field::drawable::Y)?.i();
if !(drawable_y_190 <= drawable_y_191.wrapping_sub(16)) {
let drawable_y_208 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_208.wrapping_add(1)))?;
}
let drawable_y_206 = r.get(object_7, field::drawable::Y)?.i();
let drawable_y_207 = r.get(object_6, field::drawable::Y)?.i();
if !(drawable_y_206 >= drawable_y_207.wrapping_add(16)) {
let drawable_y_215 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(drawable_y_215.wrapping_sub(1)))?;
}
}
let enemy_state_timer_120 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_120 > 100 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(6))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
let world_canvas = r.get(object_8, field::world::CANVAS)?.r();
r.set(world_canvas, field::canvas::STATE_F, V::I(10))?;
r.set(object_6, field::enemy::STATE_H, V::I(1))?;
r.set(object_6, field::enemy::DEAD, V::I(0))?;
r.set(object_6, field::enemy::ACTIVE, V::I(1))?;
let math_abs_value_27 = host::math_abs(r, number_9)?;
let math_abs_value_28 = host::math_abs(r, number_10)?;
if !(math_abs_value_27.wrapping_add(math_abs_value_28) >= 80) {
let player_health = r.get(object_7, field::player::HEALTH)?.i();
r.set(object_7, field::player::HEALTH, V::I(player_health.wrapping_add(4)))?;
r.set(object_7, field::player::HURT_TIMER, V::I(10))?;
}
}
}
        let enemy_behavior_state_72 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_72 == 6 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
let enemy_state_timer_118 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_118.wrapping_add(1)))?;
let enemy_state_timer_119 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_119 > 10 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(8))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_77 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_77 == 7 {
let enemy_state_timer_128 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_128.wrapping_add(1)))?;
let enemy_state_timer_129 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_129 > 10 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        let enemy_behavior_state_81 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_81 == 8 {
let drawable_x_234 = r.get(object_6, field::drawable::X)?.i();
if 128 > drawable_x_234 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
let drawable_x_249 = r.get(object_6, field::drawable::X)?.i();
if 128 < drawable_x_249 {
r.set(object_6, field::enemy::FACING, V::I(3))?;
}
let drawable_y_214 = r.get(object_6, field::drawable::Y)?.i();
if 50 > drawable_y_214 {
r.set(object_6, field::enemy::FACING, V::I(2))?;
}
let drawable_y_221 = r.get(object_6, field::drawable::Y)?.i();
if 50 < drawable_y_221 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
}
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(5))?;
r.set(object_6, field::enemy::STATE_H, V::I(0))?;
let enemy_state_timer_164 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_164.wrapping_add(1)))?;
let enemy_state_timer_165 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_165 > 20 {
r.set(object_6, field::enemy::STATE_D_LOWER, V::I(1))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_15
        let enemy_behavior_state_16 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_16 == 0 { break 'check_enemy_state_h_3; }
        } // check_enemy_state_z_23
        let enemy_behavior_state_33 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_33 != 1 { break 'check_enemy_state_z_29; }
        } // check_enemy_state_h_3
        r.set(object_6, field::enemy::STATE_H, V::I(1))?;
        r.set(object_6, field::enemy::ACTIVE, V::I(0))?;
        r.set(object_6, field::enemy::DEAD, V::I(0))?;
        let enemy_facing_6 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_6 == 2 { break 'check_enemy_facing_4; }
        } // check_enemy_facing_5
        let enemy_facing_10 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_10 != 3 { break 'check_enemy_facing_7; }
        } // check_enemy_facing_4
        r.set(object_6, field::enemy::FACING, V::I(0))?;
        } // check_enemy_facing_7
        let enemy_facing_17 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_17 != 0 {
let enemy_animation_2 = r.get(object_6, field::enemy::ANIMATION)?.i();
let remainder_value_17 = arithmetic::remainder_int(r, enemy_animation_2, 2)?;
if remainder_value_17 == 1 {
let drawable_x_144 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_144.wrapping_sub(2)))?;
}
} else {
let enemy_animation_3 = r.get(object_6, field::enemy::ANIMATION)?.i();
let remainder_value_18 = arithmetic::remainder_int(r, enemy_animation_3, 2)?;
if remainder_value_18 == 1 {
let drawable_x_145 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_145.wrapping_add(2)))?;
}
}
        let drawable_x_143 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_143 < 80 {
r.set(object_6, field::enemy::FACING, V::I(0))?;
}
        let drawable_x_180 = r.get(object_6, field::drawable::X)?.i();
        if drawable_x_180 > 130 {
r.set(object_6, field::enemy::FACING, V::I(1))?;
}
        let enemy_state_timer_132 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_132.wrapping_add(1)))?;
        r.set(object_6, field::enemy::DEAD, V::I(0))?;
        r.set(object_6, field::enemy::STATE_H, V::I(1))?;
        let enemy_behavior_state_82 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_82 != 0 {
let enemy_state_timer_141 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_141 > 50 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
}
} else {
let enemy_state_timer_142 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_142 > 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
}
        } // check_enemy_state_z_29
        let enemy_behavior_state_48 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_48 != 2 { break 'check_enemy_state_f_lower; }
        } // check_enemy_state_f_9
        let enemy_state_timer_63 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        let remainder_value_8 = arithmetic::remainder_int(r, enemy_state_timer_63, 9)?;
        if remainder_value_8 != 1 { break 'check_enemy_state_f_11; }
        } // next_step_18
        number_9 = 0;
        } // next_step_22
        'update_enemies_loop_174: loop {
            'next_step_25: {
            if number_9 >= 3 { break 'check_enemy_state_f_11; }
            } // next_step_25
            let projectile_value_8 = r.alloc("p", Native::Empty);
            let world_resources_11 = r.get(object_8, field::world::RESOURCES)?.r();
            let projectile_image_value_8 = super::resources::projectile_image(r, world_resources_11, 1)?;
            super::projectile::initialize(r, projectile_value_8, projectile_image_value_8, 1, -1, 0)?;
            object_10 = projectile_value_8;
            let drawable_x_181 = r.get(object_6, field::drawable::X)?.i();
            r.set(projectile_value_8, field::drawable::X, V::I(drawable_x_181.wrapping_add(24)))?;
            let drawable_y_151 = r.get(object_6, field::drawable::Y)?.i();
            r.set(object_10, field::drawable::Y, V::I(drawable_y_151.wrapping_add(12)))?;
            r.set(object_10, field::projectile::VELOCITY_Y, V::I(1))?;
            r.set(object_10, field::projectile::VELOCITY_X, V::I((1_i32).wrapping_sub(number_9)))?;
            r.set(object_10, field::projectile::ENEMY_OWNED, V::I(1))?;
            let world_projectiles_5 = r.get(object_8, field::world::PROJECTILES)?.r();
            host::vector_add_element(r, world_projectiles_5, object_10)?;
            number_9 = number_9.wrapping_add(1);
            continue 'update_enemies_loop_174;
        }
        } // check_enemy_state_f_11
        let enemy_state_timer_87 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_87.wrapping_add(1)))?;
        let enemy_state_timer_88 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_88 > 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_17
        let enemy_behavior_state_18 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_18 <= 0 {
let enemy_random_3 = r.get(object_6, field::enemy::RANDOM)?.r();
let random_next_int_value_6 = host::random_next_int(r, enemy_random_3)?;
let remainder_value_4 = arithmetic::remainder_int(r, random_next_int_value_6, 100)?;
if remainder_value_4 == 67 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(1))?;
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
}
} else {
let enemy_state_timer_16 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_16.wrapping_add(1)))?;
let enemy_state_timer_17 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_17 == 20 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
}
let enemy_state_timer_38 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_38 == 50 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(3))?;
}
let enemy_state_timer_66 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_66 == 70 {
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
}
}
        break 'check_enemy_state_f_lower;
        } // next_step_8
        let trail_point_value_2 = r.alloc("g", Native::Empty);
        super::trail_point::initialize(r, trail_point_value_2)?;
        object_9 = trail_point_value_2;
        let drawable_x_12 = r.get(object_7, field::drawable::X)?.i();
        r.set(trail_point_value_2, field::trail_point::X, V::I(drawable_x_12))?;
        let drawable_y_6 = r.get(object_7, field::drawable::Y)?.i();
        r.set(object_9, field::trail_point::Y, V::I(drawable_y_6))?;
        let player_facing_3 = r.get(object_7, field::player::FACING)?.i();
        r.set(object_9, field::trail_point::STATE_C, V::I(player_facing_3))?;
        let enemy_state_r_4 = r.get(object_6, field::enemy::STATE_R)?.r();
        host::vector_add_element(r, enemy_state_r_4, object_9)?;
        let enemy_state_r_5 = r.get(object_6, field::enemy::STATE_R)?.r();
        let vector_size_value_4 = host::vector_size(r, enemy_state_r_5)?;
        if vector_size_value_4 <= 30 { break 'check_enemy_state_f_lower; }
        } // check_enemy_state_r_2
        let enemy_state_r_7 = r.get(object_6, field::enemy::STATE_R)?.r();
        let vector_first_element_value = host::vector_first_element(r, enemy_state_r_7)?;
        host::check_cast(r, vector_first_element_value, "g")?;
        object_10 = vector_first_element_value;
        let drawable_x_28 = r.get(object_6, field::drawable::X)?.i();
        let trail_point_x_29 = r.get(object_10, field::trail_point::X)?.i();
        if drawable_x_28 == trail_point_x_29 {
let drawable_y_27 = r.get(object_6, field::drawable::Y)?.i();
let trail_point_y_28 = r.get(object_9, field::trail_point::Y)?.i();
if drawable_y_27 == trail_point_y_28 {
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
}
        let trail_point_x_39 = r.get(object_10, field::trail_point::X)?.i();
        let drawable_x_40 = r.get(object_7, field::drawable::X)?.i();
        if trail_point_x_39 != drawable_x_40 { break 'check_trail_point_x; }
        } // check_trail_point_y
        let trail_point_y_58 = r.get(object_10, field::trail_point::Y)?.i();
        let drawable_y_59 = r.get(object_7, field::drawable::Y)?.i();
        if trail_point_y_58 != drawable_y_59 { break 'check_trail_point_x; }
        } // check_enemy_facing_11
        let enemy_facing_23 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_23 == 0 {
let trail_point_x_146 = r.get(object_10, field::trail_point::X)?.i();
r.set(object_6, field::drawable::X, V::I(trail_point_x_146))?;
let trail_point_y_123 = r.get(object_10, field::trail_point::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(trail_point_y_123.wrapping_add(8)))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
        let enemy_facing_31 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_31 == 1 {
let trail_point_x_182 = r.get(object_10, field::trail_point::X)?.i();
r.set(object_6, field::drawable::X, V::I(trail_point_x_182.wrapping_sub(8)))?;
let trail_point_y_152 = r.get(object_10, field::trail_point::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(trail_point_y_152))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
        let enemy_facing_37 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_37 == 2 {
let trail_point_x_219 = r.get(object_10, field::trail_point::X)?.i();
r.set(object_6, field::drawable::X, V::I(trail_point_x_219))?;
let trail_point_y_176 = r.get(object_10, field::trail_point::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(trail_point_y_176.wrapping_sub(8)))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
        let enemy_facing_42 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_42 == 3 {
let trail_point_x_236 = r.get(object_10, field::trail_point::X)?.i();
r.set(object_6, field::drawable::X, V::I(trail_point_x_236.wrapping_add(8)))?;
let trail_point_y_192 = r.get(object_10, field::trail_point::Y)?.i();
r.set(object_6, field::drawable::Y, V::I(trail_point_y_192))?;
r.set(object_6, field::enemy::ANIMATION, V::I(0))?;
}
        break 'check_enemy_state_r_6;
        } // check_trail_point_x
        let trail_point_x_71 = r.get(object_10, field::trail_point::X)?.i();
        r.set(object_6, field::drawable::X, V::I(trail_point_x_71))?;
        let trail_point_y_57 = r.get(object_10, field::trail_point::Y)?.i();
        r.set(object_6, field::drawable::Y, V::I(trail_point_y_57))?;
        let trail_point_state_c = r.get(object_10, field::trail_point::STATE_C)?.i();
        r.set(object_6, field::enemy::FACING, V::I((trail_point_state_c) as i8 as i32))?;
        } // check_enemy_state_r_6
        let enemy_state_r_18 = r.get(object_6, field::enemy::STATE_R)?.r();
        host::vector_remove_element_at(r, enemy_state_r_18, 0)?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_16
        let enemy_behavior_state_17 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_17 == 0 {
let enemy_state_timer_14 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_14.wrapping_add(1)))?;
let enemy_state_timer_15 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_15 > 50 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
let projectile_value = r.alloc("p", Native::Empty);
let world_resources = r.get(object_8, field::world::RESOURCES)?.r();
let projectile_image_value = super::resources::projectile_image(r, world_resources, 11)?;
super::projectile::initialize(r, projectile_value, projectile_image_value, 11, -1, 0)?;
object_9 = projectile_value;
let drawable_x_41 = r.get(object_6, field::drawable::X)?.i();
r.set(projectile_value, field::drawable::X, V::I(drawable_x_41.wrapping_add(4)))?;
let drawable_y_29 = r.get(object_6, field::drawable::Y)?.i();
r.set(object_9, field::drawable::Y, V::I(drawable_y_29.wrapping_add(4)))?;
let enemy_facing_11 = r.get(object_6, field::enemy::FACING)?.i();
if enemy_facing_11 != 1 {
r.set(object_9, field::projectile::VELOCITY_Y, V::I(-1))?;
r.set(object_9, field::projectile::VELOCITY_X, V::I(1))?;
} else {
r.set(object_9, field::projectile::VELOCITY_Y, V::I(-1))?;
r.set(object_9, field::projectile::VELOCITY_X, V::I(-1))?;
}
r.set(object_9, field::projectile::FACING, V::I(0))?;
r.set(object_9, field::projectile::ENEMY_OWNED, V::I(0))?;
let world_projectiles = r.get(object_8, field::world::PROJECTILES)?.r();
host::vector_add_element(r, world_projectiles, object_9)?;
}
}
        let enemy_behavior_state_34 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_34 == 1 {
r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(2))?;
}
        let enemy_behavior_state_49 = r.get(object_6, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_49 != 2 { break 'check_enemy_state_f_lower; }
        } // check_enemy_state_f_10
        let enemy_state_timer_64 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        r.set(object_6, field::enemy::STATE_TIMER, V::I(enemy_state_timer_64.wrapping_add(1)))?;
        let enemy_state_timer_65 = r.get(object_6, field::enemy::STATE_TIMER)?.i();
        if enemy_state_timer_65 >= 75 { break 'check_enemy_state_z_36; }
        } // check_enemy_facing_12
        let enemy_facing_25 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_25 != 1 {
let drawable_x_147 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_147.wrapping_add(2)))?;
} else {
let drawable_x_148 = r.get(object_6, field::drawable::X)?.i();
r.set(object_6, field::drawable::X, V::I(drawable_x_148.wrapping_sub(2)))?;
}
        break 'check_enemy_state_f_lower;
        } // check_enemy_state_z_36
        r.set(object_6, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(object_6, field::enemy::STATE_TIMER, V::I(0))?;
        let enemy_facing_24 = r.get(object_6, field::enemy::FACING)?.i();
        if enemy_facing_24
            != 1
        {
            break 'check_enemy_facing_8;
        }
        } // check_enemy_facing_14
        r.set(object_6, field::enemy::FACING, V::I(3))?;
        break 'check_enemy_state_f_lower;
        } // check_enemy_facing_8
        r.set(
            object_6,
            field::enemy::FACING,
            V::I(1),
        )?;
        } // check_enemy_state_f_lower
        let state_f_lower_value = r
            .get(
                object_3,
                field::enemy::DYING,
            )?
            .i();
        if state_f_lower_value == 0 {
            break 'check_enemy_state_h;
        }
        } // check_enemy_state_e_lower
        let state_e_lower_value = r
            .get(
                object_3,
                field::enemy::DEATH_FRAME,
            )?
            .i();
        r.set(
            object_3,
            field::enemy::DEATH_FRAME,
            V::I(
                ((state_e_lower_value)
                    .wrapping_add(1))
                    as i8
                    as i32,
            ),
        )?;
        let state_e_lower_value_2 = r
            .get(
                object_3,
                field::enemy::DEATH_FRAME,
            )?
            .i();
        if state_e_lower_value_2 != 5 {
            break 'check_enemy_state_e_lower_2;
        }
        } // check_enemy_kind_2
        let kind_value_6 = r
            .get(object_3, field::enemy::KIND)?
            .i();
        if kind_value_6 == 16 {
            break 'check_enemy_kind_3;
        }
        } // check_enemy_kind_4
        let kind_value_8 = r
            .get(object_3, field::enemy::KIND)?
            .i();
        if kind_value_8 != 25 {
            break 'check_drawable_x_12;
        }
        } // check_enemy_kind_3
        let enemy_kind_7 =
            r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_7 == 25 {
            let progression_flags_value = r
                .get(
                    player,
                    field::player::PROGRESSION_FLAGS,
                )?
                .i();
            if (progression_flags_value & 2097152) > 0 {
                let x_value_109 = r
                    .get(object_3, field::drawable::X)?
                    .i();
                let half_width_value_3 = r
                    .get(
                        object_3,
                        field::drawable::HALF_WIDTH,
                    )?
                    .i();
                let y_value_91 = r
                    .get(object_3, field::drawable::Y)?
                    .i();
                let state_l_value_3 = r
                    .get(
                        object_3,
                        field::enemy::STATE_L,
                    )?
                    .i();
                super::world::spawn_drops(
                    r,
                    world,
                    x_value_109.wrapping_add(
                        (half_width_value_3)
                            .wrapping_shr((1) as u32),
                    ),
                    y_value_91.wrapping_add(16),
                    player,
                    state_l_value_3,
                )?;
            } else {
                let item_value =
                    r.alloc("c", Native::Empty);
                let resources_value_2 = r
                    .get(
                        world,
                        field::world::RESOURCES,
                    )?
                    .r();
                let item_image_value =
                    super::resources::item_image(
                        r,
                        resources_value_2,
                        154,
                    )?;
                let x_value_110 = r
                    .get(object_3, field::drawable::X)?
                    .i();
                let half_width_value_4 = r
                    .get(
                        object_3,
                        field::drawable::HALF_WIDTH,
                    )?
                    .i();
                let y_value_92 = r
                    .get(object_3, field::drawable::Y)?
                    .i();
                super::item::initialize(
                    r,
                    item_value,
                    item_image_value,
                    154,
                    x_value_110.wrapping_add(
                        (half_width_value_4)
                            .wrapping_shr((1) as u32),
                    ),
                    (y_value_92.wrapping_add(16))
                        .wrapping_sub(32),
                    0,
                    player,
                )?;
                object_6 = item_value;
                r.set(
                    item_value,
                    field::item::PERSISTENCE_ID,
                    V::I(21),
                )?;
                let items_value = r
                    .get(world, field::world::ITEMS)?
                    .r();
                host::vector_add_element(
                    r,
                    items_value,
                    object_6,
                )?;
            }
        }
        let enemy_kind_9 =
            r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_9 == 16 {
            let x_value_108 = r
                .get(object_3, field::drawable::X)?
                .i();
            let half_width_value_2 = r
                .get(
                    object_3,
                    field::drawable::HALF_WIDTH,
                )?
                .i();
            let y_value_90 = r
                .get(object_3, field::drawable::Y)?
                .i();
            let state_l_value_2 = r
                .get(object_3, field::enemy::STATE_L)?
                .i();
            super::world::spawn_drops(
                r,
                world,
                x_value_108.wrapping_add(
                    (half_width_value_2)
                        .wrapping_shr((1) as u32),
                ),
                y_value_90.wrapping_add(16),
                player,
                state_l_value_2,
            )?;
        }
        break 'check_enemy_state_e_lower_2;
        } // check_drawable_x_12
        let drawable_x_83 =
            r.get(object_3, field::drawable::X)?.i();
        let half_width_value = r
            .get(object_3, field::drawable::HALF_WIDTH)?
            .i();
        let drawable_y_70 =
            r.get(object_3, field::drawable::Y)?.i();
        let enemy_state_l =
            r.get(object_3, field::enemy::STATE_L)?.i();
        super::world::spawn_drops(
            r,
            world,
            drawable_x_83.wrapping_add(
                half_width_value.wrapping_shr((1) as u32),
            ),
            drawable_y_70,
            player,
            enemy_state_l,
        )?;
        } // check_enemy_state_e_lower_2
        let enemy_death_frame_3 =
            r.get(object_3, field::enemy::DEATH_FRAME)?.i();
        if enemy_death_frame_3 == 7 {
            let world_enemies_4 =
                r.get(world, field::world::ENEMIES)?.r();
            host::vector_remove_element_at(
                r,
                world_enemies_4,
                number_2,
            )?;
            return Ok(());
        }
        let world_enemies_3 =
            r.get(world, field::world::ENEMIES)?.r();
        host::vector_set_element_at(
            r,
            world_enemies_3,
            object_3,
            number_2,
        )?;
        break 'advance_index;
        } // check_enemy_state_h
        let enemy_state_h =
            r.get(object_3, field::enemy::STATE_H)?.i();
        if enemy_state_h != 0 {
            break 'advance_index;
        }
        } // check_enemy_facing
        let enemy_facing_7 = r.get(object_3, field::enemy::FACING)?.i();
        if enemy_facing_7 == 0 {
            number_4 = 0;
            let enemy_state_d_lower =
                r.get(object_3, field::enemy::STATE_D_LOWER)?.i();
            number_5 = enemy_state_d_lower.wrapping_neg();
        }
        let enemy_facing_13 =
            r.get(object_3, field::enemy::FACING)?.i();
        if enemy_facing_13 == 1 {
            let enemy_state_d_lower_2 =
                r.get(object_3, field::enemy::STATE_D_LOWER)?.i();
            number_4 = enemy_state_d_lower_2;
            number_5 = 0;
        }
        let enemy_facing_20 =
            r.get(object_3, field::enemy::FACING)?.i();
        if enemy_facing_20 == 2 {
            number_4 = 0;
            let enemy_state_d_lower_3 =
                r.get(object_3, field::enemy::STATE_D_LOWER)?.i();
            number_5 = enemy_state_d_lower_3;
        }
        let enemy_facing_27 =
            r.get(object_3, field::enemy::FACING)?.i();
        if enemy_facing_27 == 3 {
            let enemy_state_d_lower_4 =
                r.get(object_3, field::enemy::STATE_D_LOWER)?.i();
            number_4 = enemy_state_d_lower_4.wrapping_neg();
            number_5 = 0;
        }
        let drawable_x_156 = r.get(object_3, field::drawable::X)?.i();
        let drawable_y_132 = r.get(object_3, field::drawable::Y)?.i();
        let enemy_facing_32 =
            r.get(object_3, field::enemy::FACING)?.i();
        let enemy_kind_12 = r.get(object_3, field::enemy::KIND)?.i();
        let can_entity_move_value_13 = super::world::can_entity_move(
            r,
            world,
            drawable_x_156.wrapping_add(number_4),
            drawable_y_132.wrapping_add(number_5),
            enemy_facing_32,
            enemy_kind_12,
            0,
        )?;
        if can_entity_move_value_13 == 0 {
            break 'check_world_random;
        }
        } // check_drawable_x_19
        let drawable_x_189 = r.get(object_3, field::drawable::X)?.i();
        r.set(
            object_3,
            field::drawable::X,
            V::I(drawable_x_189.wrapping_add(number_4)),
        )?;
        let drawable_y_153 = r.get(object_3, field::drawable::Y)?.i();
        r.set(
            object_3,
            field::drawable::Y,
            V::I(drawable_y_153.wrapping_add(number_5)),
        )?;
        let drawable_x_190 = r.get(object_3, field::drawable::X)?.i();
        if drawable_x_190 > 240 {
            r.set(object_3, field::enemy::FACING, V::I(3))?;
        }
        let drawable_x_220 = r.get(object_3, field::drawable::X)?.i();
        if drawable_x_220 < 16 {
            r.set(object_3, field::enemy::FACING, V::I(1))?;
        }
        let drawable_y_195 = r.get(object_3, field::drawable::Y)?.i();
        if drawable_y_195 > 176 {
            r.set(object_3, field::enemy::FACING, V::I(0))?;
        }
        let drawable_y_209 = r.get(object_3, field::drawable::Y)?.i();
        if drawable_y_209 < 32 {
            r.set(object_3, field::enemy::FACING, V::I(2))?;
        }
        let world_random_13 = r.get(world, field::world::RANDOM)?.r();
        let random_next_int_value_29 =
            host::random_next_int(r, world_random_13)?;
        let remainder_value_35 =
            arithmetic::remainder_int(r, random_next_int_value_29, 30)?;
        let math_abs_value_36 = host::math_abs(r, remainder_value_35)?;
        if math_abs_value_36 == 0 {
            let world_random_14 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_30 =
                host::random_next_int(r, world_random_14)?;
            let remainder_value_36 =
                arithmetic::remainder_int(r, random_next_int_value_30, 4)?;
            let math_abs_value_37 = host::math_abs(r, remainder_value_36)?;
            number_6 = (math_abs_value_37) as i8 as i32;
            number_7 = 0;
            let enemy_kind_21 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_21 == 8 {
                number_7 = 1;
                let enemy_facing_68 =
                    r.get(object_3, field::enemy::FACING)?.i();
                let remainder_value_37 =
                    arithmetic::remainder_int(r, enemy_facing_68, 2)?;
                let remainder_value_38 =
                    arithmetic::remainder_int(r, number_6, 2)?;
                if remainder_value_37 != remainder_value_38 {
                    r.set(object_3, field::enemy::FACING, V::I(number_6))?;
                } else {
                    let enemy_facing_72 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    r.set(
                        object_3,
                        field::enemy::FACING,
                        V::I(
                            (enemy_facing_72.wrapping_add(1)) as i8
                                as i32,
                        ),
                    )?;
                    let enemy_facing_73 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    if enemy_facing_73 > 3 {
                        r.set(object_3, field::enemy::FACING, V::I(0))?;
                    }
                }
            }
            let enemy_kind_24 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_24 == 10 {
                number_7 = 1;
                let enemy_facing_71 =
                    r.get(object_3, field::enemy::FACING)?.i();
                let remainder_value_42 =
                    arithmetic::remainder_int(r, enemy_facing_71, 2)?;
                let remainder_value_43 =
                    arithmetic::remainder_int(r, number_6, 2)?;
                if remainder_value_42 != remainder_value_43 {
                    r.set(object_3, field::enemy::FACING, V::I(number_6))?;
                } else {
                    let enemy_facing_78 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    r.set(
                        object_3,
                        field::enemy::FACING,
                        V::I(
                            (enemy_facing_78.wrapping_add(1)) as i8
                                as i32,
                        ),
                    )?;
                    let enemy_facing_79 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    if enemy_facing_79 > 3 {
                        r.set(object_3, field::enemy::FACING, V::I(0))?;
                    }
                }
            }
            let enemy_kind_27 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_27 == 15 {
                number_7 = 1;
                let enemy_facing_77 =
                    r.get(object_3, field::enemy::FACING)?.i();
                let remainder_value_47 =
                    arithmetic::remainder_int(r, enemy_facing_77, 2)?;
                let remainder_value_48 =
                    arithmetic::remainder_int(r, number_6, 2)?;
                if remainder_value_47 != remainder_value_48 {
                    r.set(object_3, field::enemy::FACING, V::I(number_6))?;
                } else {
                    let enemy_facing_82 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    r.set(
                        object_3,
                        field::enemy::FACING,
                        V::I(
                            (enemy_facing_82.wrapping_add(1)) as i8
                                as i32,
                        ),
                    )?;
                    let enemy_facing_83 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    if enemy_facing_83 > 3 {
                        r.set(object_3, field::enemy::FACING, V::I(0))?;
                    }
                }
            }
            let enemy_kind_30 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_30 == 17 {
                number_7 = 1;
            }
            let enemy_kind_33 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_33 == 12 {
                number_7 = 1;
                let enemy_facing_88 =
                    r.get(object_3, field::enemy::FACING)?.i();
                let remainder_value_52 =
                    arithmetic::remainder_int(r, enemy_facing_88, 2)?;
                let remainder_value_53 =
                    arithmetic::remainder_int(r, number_6, 2)?;
                if remainder_value_52 != remainder_value_53 {
                    r.set(object_3, field::enemy::FACING, V::I(number_6))?;
                } else {
                    let enemy_facing_90 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    r.set(
                        object_3,
                        field::enemy::FACING,
                        V::I(
                            (enemy_facing_90.wrapping_add(1)) as i8
                                as i32,
                        ),
                    )?;
                    let enemy_facing_91 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    if enemy_facing_91 > 3 {
                        r.set(object_3, field::enemy::FACING, V::I(0))?;
                    }
                }
            }
            let enemy_kind_35 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_35 == 19 {
                number_7 = 1;
                let enemy_facing_89 =
                    r.get(object_3, field::enemy::FACING)?.i();
                let remainder_value_54 =
                    arithmetic::remainder_int(r, enemy_facing_89, 2)?;
                let remainder_value_55 =
                    arithmetic::remainder_int(r, number_6, 2)?;
                if remainder_value_54 != remainder_value_55 {
                    r.set(object_3, field::enemy::FACING, V::I(number_6))?;
                } else {
                    let enemy_facing_93 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    r.set(
                        object_3,
                        field::enemy::FACING,
                        V::I(
                            (enemy_facing_93.wrapping_add(1)) as i8
                                as i32,
                        ),
                    )?;
                    let enemy_facing_94 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    if enemy_facing_94 > 3 {
                        r.set(object_3, field::enemy::FACING, V::I(0))?;
                    }
                }
            }
            let enemy_kind_36 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_36 == 22 {
                number_7 = 1;
                let enemy_facing_92 =
                    r.get(object_3, field::enemy::FACING)?.i();
                let remainder_value_59 =
                    arithmetic::remainder_int(r, enemy_facing_92, 2)?;
                let remainder_value_60 =
                    arithmetic::remainder_int(r, number_6, 2)?;
                if remainder_value_59 != remainder_value_60 {
                    r.set(object_3, field::enemy::FACING, V::I(number_6))?;
                } else {
                    let enemy_facing_95 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    r.set(
                        object_3,
                        field::enemy::FACING,
                        V::I(
                            (enemy_facing_95.wrapping_add(1)) as i8
                                as i32,
                        ),
                    )?;
                    let enemy_facing_96 =
                        r.get(object_3, field::enemy::FACING)?.i();
                    if enemy_facing_96 > 3 {
                        r.set(object_3, field::enemy::FACING, V::I(0))?;
                    }
                }
            }
            let enemy_kind_37 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_37 == 30 {
                number_7 = 1;
            }
            let enemy_kind_38 = r.get(object_3, field::enemy::KIND)?.i();
            if enemy_kind_38 == 31 {
                number_7 = 1;
            }

            if number_7 == 0 {
                r.set(object_3, field::enemy::FACING, V::I(number_6))?;
            }
        }
        let enemy_kind_20 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_20 == 16 {
            let enemy_facing_67 =
                r.get(object_3, field::enemy::FACING)?.i();
            if enemy_facing_67 == 1 {
                r.set(object_3, field::enemy::FACING, V::I(0))?;
            }
            let enemy_facing_70 =
                r.get(object_3, field::enemy::FACING)?.i();
            if enemy_facing_70 == 3 {
                r.set(object_3, field::enemy::FACING, V::I(2))?;
            }
        }
        let enemy_kind_23 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_23 == 0 {
            let world_random_15 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_31 =
                host::random_next_int(r, world_random_15)?;
            let remainder_value_41 = arithmetic::remainder_int(
                r,
                random_next_int_value_31,
                100,
            )?;
            let math_abs_value_38 = host::math_abs(r, remainder_value_41)?;
            if math_abs_value_38 == 5 {
                let enemy_state_u =
                    r.get(object_3, field::enemy::STATE_U)?.i();
                if enemy_state_u <= 0 {
                    let projectile_value_18 = r.alloc("p", Native::Empty);
                    let world_resources_22 =
                        r.get(world, field::world::RESOURCES)?.r();
                    let projectile_image_value_18 =
                        super::resources::projectile_image(
                            r,
                            world_resources_22,
                            0,
                        )?;
                    super::projectile::initialize(
                        r,
                        projectile_value_18,
                        projectile_image_value_18,
                        0,
                        number_4,
                        number_5,
                    )?;
                    object_6 = projectile_value_18;
                    let drawable_x_262 =
                        r.get(object_3, field::drawable::X)?.i();
                    r.set(
                        projectile_value_18,
                        field::drawable::X,
                        V::I(drawable_x_262.wrapping_add(4)),
                    )?;
                    let drawable_y_231 =
                        r.get(object_3, field::drawable::Y)?.i();
                    r.set(
                        object_6,
                        field::drawable::Y,
                        V::I(drawable_y_231.wrapping_add(4)),
                    )?;
                    r.set(
                        object_6,
                        field::projectile::ENEMY_OWNED,
                        V::I(1),
                    )?;
                    let world_projectiles_18 =
                        r.get(world, field::world::PROJECTILES)?.r();
                    host::vector_add_element(
                        r,
                        world_projectiles_18,
                        object_6,
                    )?;
                    r.set(object_3, field::enemy::STATE_U, V::I(100))?;
                }
            }
        }
        let enemy_kind_26 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_26 == 6 {
            let world_random_16 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_32 =
                host::random_next_int(r, world_random_16)?;
            let remainder_value_46 =
                arithmetic::remainder_int(r, random_next_int_value_32, 50)?;
            let math_abs_value_39 = host::math_abs(r, remainder_value_46)?;
            if math_abs_value_39 == 5 {
                let enemy_has_projectile =
                    r.get(object_3, field::enemy::HAS_PROJECTILE)?.i();
                if enemy_has_projectile == 0 {
                    let projectile_value_19 = r.alloc("p", Native::Empty);
                    let world_resources_23 =
                        r.get(world, field::world::RESOURCES)?.r();
                    let projectile_image_value_19 =
                        super::resources::projectile_image(
                            r,
                            world_resources_23,
                            2,
                        )?;
                    super::projectile::initialize(
                        r,
                        projectile_value_19,
                        projectile_image_value_19,
                        2,
                        number_4,
                        number_5,
                    )?;
                    object_6 = projectile_value_19;
                    let drawable_x_263 =
                        r.get(object_3, field::drawable::X)?.i();
                    r.set(
                        projectile_value_19,
                        field::drawable::X,
                        V::I(
                            (drawable_x_263)
                                .wrapping_add(number_4.wrapping_mul(8)),
                        ),
                    )?;
                    let drawable_y_232 =
                        r.get(object_3, field::drawable::Y)?.i();
                    r.set(
                        object_6,
                        field::drawable::Y,
                        V::I(
                            (drawable_y_232)
                                .wrapping_add(number_5.wrapping_mul(8)),
                        ),
                    )?;
                    r.set(
                        object_6,
                        field::projectile::OWNER_INDEX,
                        V::I(number_2),
                    )?;
                    r.set(
                        object_6,
                        field::projectile::ENEMY_OWNED,
                        V::I(1),
                    )?;
                    let world_projectiles_19 =
                        r.get(world, field::world::PROJECTILES)?.r();
                    host::vector_add_element(
                        r,
                        world_projectiles_19,
                        object_6,
                    )?;
                    r.set(object_3, field::enemy::HAS_PROJECTILE, V::I(1))?;
                }
            }
        }
        let enemy_kind_29 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_29 == 12 {
            let world_random_17 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_33 =
                host::random_next_int(r, world_random_17)?;
            let remainder_value_49 = arithmetic::remainder_int(
                r,
                random_next_int_value_33,
                100,
            )?;
            let math_abs_value_40 = host::math_abs(r, remainder_value_49)?;
            if math_abs_value_40 == 5 {
                let enemy_state_u_2 =
                    r.get(object_3, field::enemy::STATE_U)?.i();
                if enemy_state_u_2 <= 0 {
                    let projectile_value_20 = r.alloc("p", Native::Empty);
                    let world_resources_25 =
                        r.get(world, field::world::RESOURCES)?.r();
                    let projectile_image_value_20 =
                        super::resources::projectile_image(
                            r,
                            world_resources_25,
                            3,
                        )?;
                    super::projectile::initialize(
                        r,
                        projectile_value_20,
                        projectile_image_value_20,
                        3,
                        number_4,
                        number_5,
                    )?;
                    object_6 = projectile_value_20;
                    let drawable_x_265 =
                        r.get(object_3, field::drawable::X)?.i();
                    r.set(
                        projectile_value_20,
                        field::drawable::X,
                        V::I(drawable_x_265.wrapping_add(4)),
                    )?;
                    let drawable_y_234 =
                        r.get(object_3, field::drawable::Y)?.i();
                    r.set(
                        object_6,
                        field::drawable::Y,
                        V::I(drawable_y_234.wrapping_add(4)),
                    )?;
                    r.set(
                        object_6,
                        field::projectile::ENEMY_OWNED,
                        V::I(1),
                    )?;
                    let world_projectiles_20 =
                        r.get(world, field::world::PROJECTILES)?.r();
                    host::vector_add_element(
                        r,
                        world_projectiles_20,
                        object_6,
                    )?;
                    r.set(object_3, field::enemy::STATE_U, V::I(100))?;
                }
            }
        }
        let enemy_kind_32 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_32 == 16 {
            let world_random_19 = r.get(world, field::world::RANDOM)?.r();
            let random_next_int_value_35 =
                host::random_next_int(r, world_random_19)?;
            let remainder_value_51 =
                arithmetic::remainder_int(r, random_next_int_value_35, 20)?;
            let math_abs_value_42 = host::math_abs(r, remainder_value_51)?;
            if math_abs_value_42 <= 3 {
                let enemy_state_u_3 =
                    r.get(object_3, field::enemy::STATE_U)?.i();
                if enemy_state_u_3 <= 0 {
                    let projectile_value_21 = r.alloc("p", Native::Empty);
                    let world_resources_27 =
                        r.get(world, field::world::RESOURCES)?.r();
                    let projectile_image_value_21 =
                        super::resources::projectile_image(
                            r,
                            world_resources_27,
                            1,
                        )?;
                    super::projectile::initialize(
                        r,
                        projectile_value_21,
                        projectile_image_value_21,
                        1,
                        -1,
                        0,
                    )?;
                    object_6 = projectile_value_21;
                    let drawable_x_266 =
                        r.get(object_3, field::drawable::X)?.i();
                    r.set(
                        projectile_value_21,
                        field::drawable::X,
                        V::I(drawable_x_266.wrapping_add(4)),
                    )?;
                    let drawable_y_235 =
                        r.get(object_3, field::drawable::Y)?.i();
                    r.set(
                        object_6,
                        field::drawable::Y,
                        V::I(drawable_y_235.wrapping_add(4)),
                    )?;
                    let world_random_22 =
                        r.get(world, field::world::RANDOM)?.r();
                    let random_next_int_value_38 =
                        host::random_next_int(r, world_random_22)?;
                    let remainder_value_58 = arithmetic::remainder_int(
                        r,
                        random_next_int_value_38,
                        10,
                    )?;
                    let math_abs_value_45 =
                        host::math_abs(r, remainder_value_58)?;
                    if math_abs_value_45 <= 4 {
                        let drawable_y_236 =
                            r.get(player, field::drawable::Y)?.i();
                        let drawable_y_237 =
                            r.get(object_3, field::drawable::Y)?.i();
                        if drawable_y_236 >= drawable_y_237 {
                            r.set(
                                object_6,
                                field::projectile::VELOCITY_Y,
                                V::I(1),
                            )?;
                        } else {
                            r.set(
                                object_6,
                                field::projectile::VELOCITY_Y,
                                V::I(-1),
                            )?;
                        }
                    }
                    r.set(
                        object_6,
                        field::projectile::ENEMY_OWNED,
                        V::I(1),
                    )?;
                    let world_projectiles_21 =
                        r.get(world, field::world::PROJECTILES)?.r();
                    host::vector_add_element(
                        r,
                        world_projectiles_21,
                        object_6,
                    )?;
                    r.set(object_3, field::enemy::STATE_U, V::I(10))?;
                }
            }
        }
        break 'check_world_enemies_4;
        } // check_world_random
        let world_random_9 = r.get(world, field::world::RANDOM)?.r();
        let random_next_int_value_23 =
            host::random_next_int(r, world_random_9)?;
        let remainder_value_22 =
            arithmetic::remainder_int(r, random_next_int_value_23, 4)?;
        let math_abs_value_24 = host::math_abs(r, remainder_value_22)?;
        number_6 = (math_abs_value_24) as i8 as i32;
        number_7 = 0;
        let enemy_kind_13 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_13 == 8 {
            number_7 = 1;
            let enemy_facing_43 = r.get(object_3, field::enemy::FACING)?.i();
            let remainder_value_23 =
                arithmetic::remainder_int(r, enemy_facing_43, 2)?;
            let remainder_value_24 = arithmetic::remainder_int(r, number_6, 2)?;
            if remainder_value_23 != remainder_value_24 {
                r.set(object_3, field::enemy::FACING, V::I(number_6))?;
            } else {
                let enemy_facing_47 =
                    r.get(object_3, field::enemy::FACING)?.i();
                r.set(
                    object_3,
                    field::enemy::FACING,
                    V::I((enemy_facing_47.wrapping_add(1)) as i8 as i32),
                )?;
                let enemy_facing_48 =
                    r.get(object_3, field::enemy::FACING)?.i();
                if enemy_facing_48 > 3 {
                    r.set(object_3, field::enemy::FACING, V::I(2))?;
                }
            }
        }
        let enemy_kind_15 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_15 == 10 {
            number_7 = 1;
            let enemy_facing_46 = r.get(object_3, field::enemy::FACING)?.i();
            let remainder_value_27 =
                arithmetic::remainder_int(r, enemy_facing_46, 2)?;
            let remainder_value_28 = arithmetic::remainder_int(r, number_6, 2)?;
            if remainder_value_27 != remainder_value_28 {
                r.set(object_3, field::enemy::FACING, V::I(number_6))?;
            } else {
                let enemy_facing_53 =
                    r.get(object_3, field::enemy::FACING)?.i();
                r.set(
                    object_3,
                    field::enemy::FACING,
                    V::I((enemy_facing_53.wrapping_add(1)) as i8 as i32),
                )?;
                let enemy_facing_54 =
                    r.get(object_3, field::enemy::FACING)?.i();
                if enemy_facing_54 > 3 {
                    r.set(object_3, field::enemy::FACING, V::I(2))?;
                }
            }
        }
        let enemy_kind_16 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_16 == 15 {
            number_7 = 1;
            let enemy_facing_52 = r.get(object_3, field::enemy::FACING)?.i();
            let remainder_value_31 =
                arithmetic::remainder_int(r, enemy_facing_52, 2)?;
            let remainder_value_32 = arithmetic::remainder_int(r, number_6, 2)?;
            if remainder_value_31 != remainder_value_32 {
                r.set(object_3, field::enemy::FACING, V::I(number_6))?;
            } else {
                let enemy_facing_59 =
                    r.get(object_3, field::enemy::FACING)?.i();
                r.set(
                    object_3,
                    field::enemy::FACING,
                    V::I((enemy_facing_59.wrapping_add(1)) as i8 as i32),
                )?;
                let enemy_facing_60 =
                    r.get(object_3, field::enemy::FACING)?.i();
                if enemy_facing_60 > 3 {
                    r.set(object_3, field::enemy::FACING, V::I(2))?;
                }
            }
        }
        let enemy_kind_17 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_17 == 12 {
            number_7 = 1;
            let enemy_facing_58 = r.get(object_3, field::enemy::FACING)?.i();
            let remainder_value_33 =
                arithmetic::remainder_int(r, enemy_facing_58, 2)?;
            let remainder_value_34 = arithmetic::remainder_int(r, number_6, 2)?;
            if remainder_value_33 != remainder_value_34 {
                r.set(object_3, field::enemy::FACING, V::I(number_6))?;
            } else {
                let enemy_facing_62 =
                    r.get(object_3, field::enemy::FACING)?.i();
                r.set(
                    object_3,
                    field::enemy::FACING,
                    V::I((enemy_facing_62.wrapping_add(1)) as i8 as i32),
                )?;
                let enemy_facing_63 =
                    r.get(object_3, field::enemy::FACING)?.i();
                if enemy_facing_63 > 3 {
                    r.set(object_3, field::enemy::FACING, V::I(2))?;
                }
            }
        }
        let enemy_kind_18 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_18 == 13 {
            let drawable_x_257 = r.get(object_3, field::drawable::X)?.i();
            r.set(
                object_3,
                field::drawable::X,
                V::I(drawable_x_257.wrapping_add(number_4)),
            )?;
            let drawable_y_222 = r.get(object_3, field::drawable::Y)?.i();
            r.set(
                object_3,
                field::drawable::Y,
                V::I(drawable_y_222.wrapping_add(number_5)),
            )?;
            let drawable_x_258 = r.get(object_3, field::drawable::X)?.i();
            if drawable_x_258 > 240 {
                r.set(object_3, field::enemy::FACING, V::I(3))?;
            }
            let drawable_x_261 = r.get(object_3, field::drawable::X)?.i();
            if drawable_x_261 < 16 {
                r.set(object_3, field::enemy::FACING, V::I(1))?;
            }
            let drawable_y_229 = r.get(object_3, field::drawable::Y)?.i();
            if drawable_y_229 > 176 {
                r.set(object_3, field::enemy::FACING, V::I(0))?;
            }
            let drawable_y_230 = r.get(object_3, field::drawable::Y)?.i();
            if drawable_y_230 < 32 {
                r.set(object_3, field::enemy::FACING, V::I(2))?;
            }
            number_7 = 1;
        }
        let enemy_kind_19 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_19 == 17 {
            number_7 = 1;
            let enemy_facing_65 = r.get(object_3, field::enemy::FACING)?.i();
            r.set(
                object_3,
                field::enemy::FACING,
                V::I((enemy_facing_65.wrapping_add(1)) as i8 as i32),
            )?;
            let enemy_facing_66 = r.get(object_3, field::enemy::FACING)?.i();
            if enemy_facing_66 > 3 {
                r.set(object_3, field::enemy::FACING, V::I(0))?;
            }
        }
        let enemy_kind_22 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_22 == 19 {
            number_7 = 1;
            let enemy_facing_69 = r.get(object_3, field::enemy::FACING)?.i();
            let remainder_value_39 =
                arithmetic::remainder_int(r, enemy_facing_69, 2)?;
            let remainder_value_40 = arithmetic::remainder_int(r, number_6, 2)?;
            if remainder_value_39 != remainder_value_40 {
                r.set(object_3, field::enemy::FACING, V::I(number_6))?;
            } else {
                let enemy_facing_75 =
                    r.get(object_3, field::enemy::FACING)?.i();
                r.set(
                    object_3,
                    field::enemy::FACING,
                    V::I((enemy_facing_75.wrapping_add(1)) as i8 as i32),
                )?;
                let enemy_facing_76 =
                    r.get(object_3, field::enemy::FACING)?.i();
                if enemy_facing_76 > 3 {
                    r.set(object_3, field::enemy::FACING, V::I(2))?;
                }
            }
        }
        let enemy_kind_25 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_25 == 22 {
            number_7 = 1;
            let enemy_facing_74 = r.get(object_3, field::enemy::FACING)?.i();
            let remainder_value_44 =
                arithmetic::remainder_int(r, enemy_facing_74, 2)?;
            let remainder_value_45 = arithmetic::remainder_int(r, number_6, 2)?;
            if remainder_value_44 != remainder_value_45 {
                r.set(object_3, field::enemy::FACING, V::I(number_6))?;
            } else {
                let enemy_facing_80 =
                    r.get(object_3, field::enemy::FACING)?.i();
                r.set(
                    object_3,
                    field::enemy::FACING,
                    V::I((enemy_facing_80.wrapping_add(1)) as i8 as i32),
                )?;
                let enemy_facing_81 =
                    r.get(object_3, field::enemy::FACING)?.i();
                if enemy_facing_81 > 3 {
                    r.set(object_3, field::enemy::FACING, V::I(2))?;
                }
            }
        }
        let enemy_kind_28 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_28 == 30 {
            break 'next_step_27;
        }
        } // check_enemy_kind_5
        let enemy_kind_31 = r.get(object_3, field::enemy::KIND)?.i();
        if enemy_kind_31 != 31 {
            break 'next_step_28;
        }
        } // next_step_27
        number_7 = 1;
        let enemy_state_m_2 = r.get(object_3, field::enemy::STATE_M)?.i();
        if enemy_state_m_2 <= 0 {
            number_7 = 1;
            let enemy_facing_84 = r.get(object_3, field::enemy::FACING)?.i();
            r.set(
                object_3,
                field::enemy::FACING,
                V::I((enemy_facing_84.wrapping_sub(1)) as i8 as i32),
            )?;
            let enemy_facing_85 = r.get(object_3, field::enemy::FACING)?.i();
            if enemy_facing_85 < 0 {
                r.set(object_3, field::enemy::FACING, V::I(3))?;
            }
        } else {
            let enemy_facing_86 = r.get(object_3, field::enemy::FACING)?.i();
            r.set(
                object_3,
                field::enemy::FACING,
                V::I((enemy_facing_86.wrapping_add(1)) as i8 as i32),
            )?;
            let enemy_facing_87 = r.get(object_3, field::enemy::FACING)?.i();
            if enemy_facing_87 > 3 {
                r.set(object_3, field::enemy::FACING, V::I(0))?;
            }
        }
        } // next_step_28

        if number_7 == 0 {
            r.set(object_3, field::enemy::FACING, V::I(number_6))?;
        }
        } // check_world_enemies_4
        let world_enemies_10 = r.get(world, field::world::ENEMIES)?.r();
        host::vector_set_element_at(r, world_enemies_10, object_3, number_2)?;
        } // advance_index
        number_2 = number_2.wrapping_add(1);
        continue 'update_enemies_loop_2;
    }
}
