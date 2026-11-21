//! Enemy and boss behavior and drawing.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `l.<init>(Ljavax/microedition/lcdui/Image;IIBI)V`.
pub(super) fn initialize(
    r: &mut Runtime,
    enemy: u32,
    mut image: u32,
    x: i32,
    y: i32,
    facing: i32,
    kind: i32,
) -> Result<(), u32> {
    let mut number_1: i32 = 0;
    let mut object_2: u32 = 0;
    let mut object_3: u32 = 0;
    let mut object_4: u32 = 0;
    let mut object_5: u32 = 0;
    super::drawable::initialize(r, enemy)?;
    r.set(enemy, field::drawable::X, V::I(x))?;
    r.set(enemy, field::drawable::Y, V::I(y.wrapping_add(32)))?;
    r.set(enemy, field::enemy::FACING, V::I(1))?;
    r.set(enemy, field::enemy::KIND, V::I(kind))?;
    r.set(enemy, field::enemy::IMAGE, V::R(image))?;
    r.set(enemy, field::drawable::HALF_WIDTH, V::I(8))?;
    r.set(enemy, field::drawable::HALF_HEIGHT, V::I(8))?;
    r.set(enemy, field::enemy::WIDTH, V::I(16))?;
    r.set(enemy, field::enemy::HEIGHT, V::I(16))?;
    r.set(enemy, field::drawable::DRAW_ORDER, V::I(0))?;
    r.set(enemy, field::enemy::STATE_D_LOWER, V::I(1))?;
    r.set(enemy, field::enemy::HEALTH, V::I(1))?;
    r.set(enemy, field::enemy::STATE_O, V::I(2))?;
    r.set(enemy, field::enemy::KNOCKBACK_TIMER, V::I(0))?;
    r.set(enemy, field::enemy::HURT_TIMER, V::I(0))?;
    r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
    r.set(enemy, field::enemy::DEAD, V::I(0))?;
    r.set(enemy, field::enemy::STATE_H, V::I(0))?;
    r.set(enemy, field::enemy::STATE_O_LOWER, V::I(1))?;
    r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
    let object_value = r.alloc("java/util/Random", Native::Empty);
    host::random_initialize(r, object_value)?;
    r.set(enemy, field::enemy::RANDOM, V::R(object_value))?;
    let enemy_random = r.get(enemy, field::enemy::RANDOM)?.r();
    let drawable_x = r.get(enemy, field::drawable::X)?.i();
    let drawable_y = r.get(enemy, field::drawable::Y)?.i();
    let enemy_random_2 = r.get(enemy, field::enemy::RANDOM)?.r();
    let random_next_int_value = host::random_next_int(r, enemy_random_2)?;
    host::random_set_seed(
        r,
        enemy_random,
        ((drawable_x.wrapping_mul(drawable_y)).wrapping_add(random_next_int_value)) as i64,
    )?;
    let enemy_random_3 = r.get(enemy, field::enemy::RANDOM)?.r();
    let random_next_int_value_2 = host::random_next_int(r, enemy_random_3)?;
    let remainder_value = arithmetic::remainder_int(r, random_next_int_value_2, 4)?;
    let math_abs_value = host::math_abs(r, remainder_value)?;
    r.set(
        enemy,
        field::enemy::FACING,
        V::I((math_abs_value) as i8 as i32),
    )?;
    r.set(enemy, field::enemy::STATE_W_UPPER, V::R(0))?;
    let enemy_kind = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind == 0 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(2))?;
        r.set(enemy, field::enemy::STATE_L, V::I(1))?;
    }
    let enemy_kind_2 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_2 == 1 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::STATE_L, V::I(1))?;
    }
    let enemy_kind_3 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_3 == 3 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(2))?;
        r.set(enemy, field::enemy::STATE_L, V::I(2))?;
    }
    let enemy_kind_4 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_4 == 4 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_L, V::I(0))?;
    }
    let enemy_kind_5 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_5 == 5 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(2))?;
        r.set(enemy, field::enemy::STATE_L, V::I(1))?;
    }
    let enemy_kind_6 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_6 == 6 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(3))?;
        r.set(enemy, field::enemy::STATE_L, V::I(2))?;
    }
    let enemy_kind_7 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_7 == 7 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(3))?;
        r.set(enemy, field::enemy::STATE_L, V::I(1))?;
    }
    let enemy_kind_8 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_8 == 8 {
        r.set(enemy, field::enemy::STATE_L, V::I(3))?;
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(2))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::HEALTH, V::I(4))?;
        let object_value_2 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_2)?;
        r.set(enemy, field::enemy::STATE_T, V::R(object_value_2))?;
        number_1 = 0;
        'initialize_loop_3: loop {
            if number_1 >= 22 {
                break 'initialize_loop_3;
            }
            let trail_point_value_6 = r.alloc("g", Native::Empty);
            super::trail_point::initialize(r, trail_point_value_6)?;
            object_2 = trail_point_value_6;
            let drawable_x_9 = r.get(enemy, field::drawable::X)?.i();
            r.set(
                trail_point_value_6,
                field::trail_point::X,
                V::I(drawable_x_9),
            )?;
            let drawable_y_9 = r.get(enemy, field::drawable::Y)?.i();
            r.set(object_2, field::trail_point::Y, V::I(drawable_y_9))?;
            r.set(object_2, field::trail_point::STATE_C, V::I(3))?;
            let enemy_state_t = r.get(enemy, field::enemy::STATE_T)?.r();
            host::vector_add_element(r, enemy_state_t, object_2)?;
            number_1 = number_1.wrapping_add(1);
            continue 'initialize_loop_3;
        }
        let object_value_3 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_3)?;
        r.set(enemy, field::enemy::STATE_R, V::R(object_value_3))?;
        let trail_point_value = r.alloc("g", Native::Empty);
        super::trail_point::initialize(r, trail_point_value)?;
        image = trail_point_value;
        let trail_point_value_2 = r.alloc("g", Native::Empty);
        super::trail_point::initialize(r, trail_point_value_2)?;
        object_2 = trail_point_value_2;
        let trail_point_value_3 = r.alloc("g", Native::Empty);
        super::trail_point::initialize(r, trail_point_value_3)?;
        object_3 = trail_point_value_3;
        let trail_point_value_4 = r.alloc("g", Native::Empty);
        super::trail_point::initialize(r, trail_point_value_4)?;
        object_4 = trail_point_value_4;
        let trail_point_value_5 = r.alloc("g", Native::Empty);
        super::trail_point::initialize(r, trail_point_value_5)?;
        object_5 = trail_point_value_5;
        let drawable_x_4 = r.get(enemy, field::drawable::X)?.i();
        r.set(image, field::trail_point::X, V::I(drawable_x_4))?;
        let drawable_y_4 = r.get(enemy, field::drawable::Y)?.i();
        r.set(image, field::trail_point::Y, V::I(drawable_y_4))?;
        r.set(image, field::trail_point::STATE_C, V::I(3))?;
        let enemy_state_r = r.get(enemy, field::enemy::STATE_R)?.r();
        host::vector_add_element(r, enemy_state_r, image)?;
        let trail_point_x_5 = r.get(image, field::trail_point::X)?.i();
        r.set(
            object_2,
            field::trail_point::X,
            V::I(trail_point_x_5.wrapping_sub(12)),
        )?;
        let trail_point_y_5 = r.get(image, field::trail_point::Y)?.i();
        r.set(object_2, field::trail_point::Y, V::I(trail_point_y_5))?;
        let trail_point_state_c = r.get(image, field::trail_point::STATE_C)?.i();
        r.set(
            object_2,
            field::trail_point::STATE_C,
            V::I(trail_point_state_c),
        )?;
        let enemy_state_r_2 = r.get(enemy, field::enemy::STATE_R)?.r();
        host::vector_add_element(r, enemy_state_r_2, object_2)?;
        let trail_point_x_6 = r.get(object_2, field::trail_point::X)?.i();
        r.set(
            object_3,
            field::trail_point::X,
            V::I(trail_point_x_6.wrapping_sub(12)),
        )?;
        let trail_point_y_6 = r.get(object_2, field::trail_point::Y)?.i();
        r.set(object_3, field::trail_point::Y, V::I(trail_point_y_6))?;
        let trail_point_state_c_2 = r.get(object_2, field::trail_point::STATE_C)?.i();
        r.set(
            object_3,
            field::trail_point::STATE_C,
            V::I(trail_point_state_c_2),
        )?;
        let enemy_state_r_3 = r.get(enemy, field::enemy::STATE_R)?.r();
        host::vector_add_element(r, enemy_state_r_3, object_3)?;
        let trail_point_x_7 = r.get(object_3, field::trail_point::X)?.i();
        r.set(
            object_4,
            field::trail_point::X,
            V::I(trail_point_x_7.wrapping_sub(12)),
        )?;
        let trail_point_y_7 = r.get(object_3, field::trail_point::Y)?.i();
        r.set(object_4, field::trail_point::Y, V::I(trail_point_y_7))?;
        let trail_point_state_c_3 = r.get(object_3, field::trail_point::STATE_C)?.i();
        r.set(
            object_4,
            field::trail_point::STATE_C,
            V::I(trail_point_state_c_3),
        )?;
        let enemy_state_r_4 = r.get(enemy, field::enemy::STATE_R)?.r();
        host::vector_add_element(r, enemy_state_r_4, object_4)?;
        let trail_point_x_8 = r.get(object_4, field::trail_point::X)?.i();
        r.set(
            object_5,
            field::trail_point::X,
            V::I(trail_point_x_8.wrapping_sub(12)),
        )?;
        let trail_point_y_8 = r.get(object_4, field::trail_point::Y)?.i();
        r.set(object_5, field::trail_point::Y, V::I(trail_point_y_8))?;
        let trail_point_state_c_4 = r.get(object_4, field::trail_point::STATE_C)?.i();
        r.set(
            object_5,
            field::trail_point::STATE_C,
            V::I(trail_point_state_c_4),
        )?;
        let enemy_state_r_5 = r.get(enemy, field::enemy::STATE_R)?.r();
        host::vector_add_element(r, enemy_state_r_5, object_5)?;
    }
    let enemy_kind_9 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_9 == 9 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        let drawable_x_2 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_2))?;
        let drawable_y_2 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_E, V::I(drawable_y_2))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
    }
    let enemy_kind_10 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_10 == 10 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        let drawable_x_3 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_3))?;
        let drawable_y_3 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_E, V::I(drawable_y_3))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::HEALTH, V::I(4))?;
        r.set(enemy, field::enemy::STATE_L, V::I(2))?;
    }
    let enemy_kind_11 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_11 == 11 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        let drawable_x_10 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_10))?;
        let drawable_y_10 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_E, V::I(drawable_y_10))?;
        r.set(enemy, field::enemy::STATE_O, V::I(1))?;
        r.set(enemy, field::enemy::HEALTH, V::I(1))?;
        r.set(enemy, field::enemy::STATE_L, V::I(1))?;
    }
    let enemy_kind_12 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_12 == 12 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        let drawable_x_11 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_11))?;
        let drawable_y_11 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_E, V::I(drawable_y_11))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::HEALTH, V::I(2))?;
        r.set(enemy, field::enemy::STATE_L, V::I(2))?;
    }
    let enemy_kind_13 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_13 == 13 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        let drawable_x_12 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_12))?;
        let drawable_y_12 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_E, V::I(drawable_y_12))?;
        r.set(enemy, field::enemy::STATE_O, V::I(1))?;
        r.set(enemy, field::enemy::HEALTH, V::I(3))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(0))?;
        r.set(enemy, field::enemy::STATE_A, V::I(16))?;
        r.set(enemy, field::enemy::STATE_L, V::I(1))?;
    }
    let enemy_kind_14 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_14 == 14 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        let drawable_x_13 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_13))?;
        let drawable_y_13 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_E, V::I(drawable_y_13))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::HEALTH, V::I(3))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(1))?;
        r.set(enemy, field::enemy::STATE_L, V::I(0))?;
    }
    let enemy_kind_15 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_15 == 15 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        let drawable_x_14 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_14))?;
        let drawable_y_14 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_E, V::I(drawable_y_14))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::HEALTH, V::I(8))?;
        r.set(enemy, field::enemy::WIDTH, V::I(16))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(26))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(8))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(12))?;
        r.set(enemy, field::enemy::STATE_L, V::I(2))?;
    }
    let enemy_kind_16 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_16 == 16 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(10))?;
        r.set(enemy, field::enemy::WIDTH, V::I(32))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(32))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(16))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(16))?;
        r.set(enemy, field::enemy::STATE_L, V::I(3))?;
    }
    let enemy_kind_17 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_17 == 17 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(10))?;
        r.set(enemy, field::enemy::WIDTH, V::I(16))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(16))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(20))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(20))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(4))?;
        r.set(enemy, field::enemy::WIDTH, V::I(20))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(20))?;
        r.set(enemy, field::enemy::STATE_L, V::I(3))?;
    }
    let enemy_kind_18 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_18 == 18 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(10))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(2))?;
        r.set(enemy, field::enemy::WIDTH, V::I(32))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(32))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(8))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(8))?;
        r.set(enemy, field::enemy::STATE_L, V::I(3))?;
    }
    let enemy_kind_19 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_19 == 20 {
        r.set(enemy, field::enemy::STATE_L, V::I(3))?;
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::HEALTH, V::I(10))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(2))?;
        r.set(enemy, field::enemy::WIDTH, V::I(48))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(64))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(24))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(32))?;
        let drawable_x_15 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_15))?;
        let drawable_y_15 = r.get(enemy, field::drawable::Y)?.i();
        r.set(
            enemy,
            field::enemy::STATE_E,
            V::I(drawable_y_15.wrapping_add(24)),
        )?;
        let enemy_state_d_lower = r.get(enemy, field::enemy::STATE_D_LOWER)?.i();
        r.set(enemy, field::enemy::VELOCITY_X, V::I(enemy_state_d_lower))?;
        let enemy_state_d_lower_2 = r.get(enemy, field::enemy::STATE_D_LOWER)?.i();
        r.set(
            enemy,
            field::enemy::VELOCITY_Y,
            V::I(enemy_state_d_lower_2.wrapping_neg()),
        )?;
        let object_value_4 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_4)?;
        r.set(enemy, field::enemy::STATE_R, V::R(object_value_4))?;
        let object_value_5 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_5)?;
        r.set(enemy, field::enemy::STATE_S, V::R(object_value_5))?;
        let object_value_6 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_6)?;
        r.set(enemy, field::enemy::STATE_Y_UPPER, V::R(object_value_6))?;
        let object_value_7 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_7)?;
        r.set(enemy, field::enemy::STATE_Z_UPPER, V::R(object_value_7))?;
        number_1 = 0;
        'initialize_loop_8: loop {
            if number_1 >= 5 {
                break 'initialize_loop_8;
            }
            let trail_point_value_7 = r.alloc("g", Native::Empty);
            super::trail_point::initialize(r, trail_point_value_7)?;
            object_2 = trail_point_value_7;
            let trail_point_value_8 = r.alloc("g", Native::Empty);
            super::trail_point::initialize(r, trail_point_value_8)?;
            object_3 = trail_point_value_8;
            let trail_point_value_9 = r.alloc("g", Native::Empty);
            super::trail_point::initialize(r, trail_point_value_9)?;
            object_4 = trail_point_value_9;
            let trail_point_value_10 = r.alloc("g", Native::Empty);
            super::trail_point::initialize(r, trail_point_value_10)?;
            object_5 = trail_point_value_10;
            let drawable_x_20 = r.get(enemy, field::drawable::X)?.i();
            r.set(
                object_2,
                field::trail_point::X,
                V::I(drawable_x_20.wrapping_sub(number_1.wrapping_mul(10))),
            )?;
            let drawable_y_20 = r.get(enemy, field::drawable::Y)?.i();
            r.set(
                object_2,
                field::trail_point::Y,
                V::I((drawable_y_20.wrapping_sub(number_1.wrapping_mul(6))).wrapping_add(24)),
            )?;
            r.set(object_2, field::trail_point::STATE_C, V::I(3))?;
            let enemy_state_r_6 = r.get(enemy, field::enemy::STATE_R)?.r();
            host::vector_add_element(r, enemy_state_r_6, object_2)?;
            let drawable_x_21 = r.get(enemy, field::drawable::X)?.i();
            r.set(
                object_3,
                field::trail_point::X,
                V::I(drawable_x_21.wrapping_sub(number_1.wrapping_shl((3) as u32))),
            )?;
            let drawable_y_21 = r.get(enemy, field::drawable::Y)?.i();
            r.set(
                object_3,
                field::trail_point::Y,
                V::I(
                    (drawable_y_21.wrapping_add(number_1.wrapping_shl((1) as u32)))
                        .wrapping_add(24),
                ),
            )?;
            r.set(object_3, field::trail_point::STATE_C, V::I(3))?;
            let enemy_state_s = r.get(enemy, field::enemy::STATE_S)?.r();
            host::vector_add_element(r, enemy_state_s, object_3)?;
            let drawable_x_22 = r.get(enemy, field::drawable::X)?.i();
            r.set(
                object_4,
                field::trail_point::X,
                V::I(drawable_x_22.wrapping_sub(number_1.wrapping_shl((2) as u32))),
            )?;
            let drawable_y_22 = r.get(enemy, field::drawable::Y)?.i();
            r.set(
                object_4,
                field::trail_point::Y,
                V::I(drawable_y_22.wrapping_add(number_1.wrapping_shl((1) as u32))),
            )?;
            r.set(object_4, field::trail_point::STATE_C, V::I(3))?;
            let enemy_state_y_upper = r.get(enemy, field::enemy::STATE_Y_UPPER)?.r();
            host::vector_add_element(r, enemy_state_y_upper, object_4)?;
            let drawable_x_23 = r.get(enemy, field::drawable::X)?.i();
            r.set(
                object_5,
                field::trail_point::X,
                V::I(drawable_x_23.wrapping_sub(number_1.wrapping_mul(6))),
            )?;
            let drawable_y_23 = r.get(enemy, field::drawable::Y)?.i();
            r.set(
                object_5,
                field::trail_point::Y,
                V::I(drawable_y_23.wrapping_add(number_1.wrapping_shl((3) as u32))),
            )?;
            r.set(object_5, field::trail_point::STATE_C, V::I(3))?;
            let enemy_state_z_upper = r.get(enemy, field::enemy::STATE_Z_UPPER)?.r();
            host::vector_add_element(r, enemy_state_z_upper, object_5)?;
            number_1 = number_1.wrapping_add(1);
            continue 'initialize_loop_8;
        }
    }
    let enemy_kind_20 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_20 == 19 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        let drawable_x_16 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_16))?;
        let drawable_y_16 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_E, V::I(drawable_y_16))?;
        r.set(enemy, field::enemy::STATE_O, V::I(1))?;
        r.set(enemy, field::enemy::HEALTH, V::I(3))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(0))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(0))?;
        let object_value_8 = r.alloc("java/util/Random", Native::Empty);
        host::random_initialize(r, object_value_8)?;
        image = object_value_8;
        let drawable_x_17 = r.get(enemy, field::drawable::X)?.i();
        let drawable_y_17 = r.get(enemy, field::drawable::Y)?.i();
        let random_next_int_value_3 = host::random_next_int(r, image)?;
        host::random_set_seed(
            r,
            object_value_8,
            ((drawable_x_17.wrapping_mul(drawable_y_17)).wrapping_add(random_next_int_value_3))
                as i64,
        )?;
        let random_next_int_value_4 = host::random_next_int(r, image)?;
        let remainder_value_2 = arithmetic::remainder_int(r, random_next_int_value_4, 100)?;
        r.set(enemy, field::enemy::STATE_G, V::I(remainder_value_2))?;
        r.set(enemy, field::enemy::STATE_L, V::I(2))?;
    }
    let enemy_kind_21 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_21 == 21 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::HEALTH, V::I(20))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(2))?;
        r.set(enemy, field::enemy::WIDTH, V::I(55))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(90))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(25))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(45))?;
        let drawable_y_18 = r.get(enemy, field::drawable::Y)?.i();
        r.set(
            enemy,
            field::drawable::Y,
            V::I(drawable_y_18.wrapping_sub(50)),
        )?;
        let drawable_x_18 = r.get(enemy, field::drawable::X)?.i();
        r.set(
            enemy,
            field::drawable::X,
            V::I(drawable_x_18.wrapping_sub(40)),
        )?;
        r.set(enemy, field::drawable::DRAW_ORDER, V::I(80))?;
        r.set(enemy, field::enemy::STATE_O, V::I(10))?;
        let drawable_x_19 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_19))?;
        let drawable_y_19 = r.get(enemy, field::drawable::Y)?.i();
        r.set(
            enemy,
            field::enemy::STATE_E,
            V::I(drawable_y_19.wrapping_add(100)),
        )?;
        r.set(enemy, field::enemy::STATE_O_LOWER, V::I(0))?;
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(0))?;
        r.set(enemy, field::enemy::STATE_L, V::I(3))?;
    }
    let enemy_kind_22 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_22 == 22 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::HEALTH, V::I(50))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(10))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(0))?;
        r.set(enemy, field::enemy::STATE_G, V::I(50))?;
        r.set(enemy, field::enemy::WIDTH, V::I(21))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(37))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(10))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(18))?;
        r.set(enemy, field::enemy::STATE_O, V::I(5))?;
        r.set(enemy, field::enemy::STATE_O_LOWER, V::I(1))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(0))?;
        r.set(enemy, field::enemy::STATE_L, V::I(3))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
    }
    let enemy_kind_23 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_23 == 23 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(50))?;
        r.set(enemy, field::enemy::WIDTH, V::I(35))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(43))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(17))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(21))?;
        r.set(enemy, field::enemy::FACING, V::I(1))?;
    }
    let enemy_kind_24 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_24 == 24 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(40))?;
        r.set(enemy, field::enemy::WIDTH, V::I(56))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(48))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(27))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(24))?;
        r.set(enemy, field::enemy::FACING, V::I(2))?;
        r.set(enemy, field::drawable::Y, V::I(50))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
    }
    let enemy_kind_25 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_25 == 25 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(40))?;
        r.set(enemy, field::enemy::WIDTH, V::I(72))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(72))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(4))?;
    }
    let enemy_kind_26 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_26 == 26 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(4))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
    }
    let enemy_kind_27 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_27 == 27 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(12))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(16))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(16))?;
        r.set(enemy, field::enemy::WIDTH, V::I(32))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(32))?;
    }
    let enemy_kind_28 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_28 == 28 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(24))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(16))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(16))?;
        r.set(enemy, field::enemy::WIDTH, V::I(32))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(32))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(1))?;
        r.set(enemy, field::enemy::STATE_G, V::I(0))?;
        let array_value = r.new_array(10, 6)?;
        r.set(enemy, field::enemy::STATE_V, V::R(array_value))?;
        let array_value_2 = r.new_array(10, 6)?;
        r.set(enemy, field::enemy::STATE_AB, V::R(array_value_2))?;
        let enemy_state_v = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v, 0, V::I(2))?;
        let enemy_state_v_2 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_2, 1, V::I(3))?;
        let enemy_state_v_3 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_3, 2, V::I(4))?;
        let enemy_state_v_4 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_4, 3, V::I(5))?;
        let enemy_state_v_5 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_5, 4, V::I(6))?;
        let enemy_state_v_6 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_6, 5, V::I(7))?;
        let enemy_state_ab = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab, 0, V::I(1))?;
        let enemy_state_ab_2 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_2, 1, V::I(1))?;
        let enemy_state_ab_3 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_3, 2, V::I(1))?;
        let enemy_state_ab_4 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_4, 3, V::I(1))?;
        let enemy_state_ab_5 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_5, 4, V::I(1))?;
        let enemy_state_ab_6 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_6, 5, V::I(1))?;
    }
    let enemy_kind_29 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_29 == 29 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(58))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(32))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(32))?;
        r.set(enemy, field::enemy::WIDTH, V::I(64))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(64))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(1))?;
        r.set(enemy, field::enemy::STATE_G, V::I(0))?;
        let array_value_3 = r.new_array(10, 6)?;
        r.set(enemy, field::enemy::STATE_V, V::R(array_value_3))?;
        let array_value_4 = r.new_array(10, 6)?;
        r.set(enemy, field::enemy::STATE_AB, V::R(array_value_4))?;
        let enemy_state_v_7 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_7, 0, V::I(2))?;
        let enemy_state_v_8 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_8, 1, V::I(3))?;
        let enemy_state_v_9 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_9, 2, V::I(4))?;
        let enemy_state_v_10 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_10, 3, V::I(5))?;
        let enemy_state_v_11 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_11, 4, V::I(6))?;
        let enemy_state_v_12 = r.get(enemy, field::enemy::STATE_V)?.r();
        r.array_set(enemy_state_v_12, 5, V::I(7))?;
        let enemy_state_ab_7 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_7, 0, V::I(1))?;
        let enemy_state_ab_8 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_8, 1, V::I(1))?;
        let enemy_state_ab_9 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_9, 2, V::I(1))?;
        let enemy_state_ab_10 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_10, 3, V::I(1))?;
        let enemy_state_ab_11 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_11, 4, V::I(1))?;
        let enemy_state_ab_12 = r.get(enemy, field::enemy::STATE_AB)?.r();
        r.array_set(enemy_state_ab_12, 5, V::I(1))?;
        let object_value_9 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_9)?;
        r.set(enemy, field::enemy::STATE_AA, V::R(object_value_9))?;
    }
    let enemy_kind_30 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_30 == 30 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(4))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(32))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(10))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(10))?;
        r.set(enemy, field::enemy::WIDTH, V::I(20))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(20))?;
        r.set(enemy, field::enemy::STATE_M, V::I(-1))?;
    }
    let enemy_kind_31 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_31 == 31 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(2))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(4))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(32))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(10))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(10))?;
        r.set(enemy, field::enemy::WIDTH, V::I(20))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(20))?;
        r.set(enemy, field::enemy::STATE_M, V::I(1))?;
    }
    let enemy_kind_32 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_32 == 32 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(4))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(8))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(8))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(9))?;
        r.set(enemy, field::enemy::WIDTH, V::I(16))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(18))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(30))?;
    }
    let enemy_kind_33 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_33 == 33 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(4))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(2))?;
        r.set(enemy, field::enemy::HIT_COUNT, V::I(0))?;
        r.set(enemy, field::enemy::FACING, V::I(0))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(8))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(8))?;
        r.set(enemy, field::enemy::WIDTH, V::I(16))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(16))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(100))?;
    }
    let enemy_kind_34 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_34 == 34 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(32))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(32))?;
        r.set(enemy, field::enemy::WIDTH, V::I(64))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(64))?;
    }
    let enemy_kind_35 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_35 == 35 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(1))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(16))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(22))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(22))?;
        r.set(enemy, field::enemy::WIDTH, V::I(44))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(40))?;
    }
    let enemy_kind_36 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_36 == 36 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::STATE_L, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::WIDTH, V::I(23))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(30))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(11))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(15))?;
        r.set(enemy, field::enemy::STATE_A, V::I(100))?;
        let drawable_x_24 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_X_UPPER, V::I(drawable_x_24))?;
        let drawable_y_24 = r.get(enemy, field::drawable::Y)?.i();
        r.set(enemy, field::enemy::STATE_S_LOWER, V::I(drawable_y_24))?;
        let object_value_10 = r.alloc("java/util/Random", Native::Empty);
        host::random_initialize(r, object_value_10)?;
        image = object_value_10;
        let drawable_x_25 = r.get(enemy, field::drawable::X)?.i();
        let drawable_y_25 = r.get(enemy, field::drawable::Y)?.i();
        let random_next_int_value_5 = host::random_next_int(r, image)?;
        host::random_set_seed(
            r,
            object_value_10,
            ((drawable_x_25.wrapping_mul(drawable_y_25)).wrapping_add(random_next_int_value_5))
                as i64,
        )?;
        let drawable_y_26 = r.get(enemy, field::drawable::Y)?.i();
        let random_next_int_value_6 = host::random_next_int(r, image)?;
        let remainder_value_3 = arithmetic::remainder_int(r, random_next_int_value_6, 30)?;
        let math_abs_value_2 = host::math_abs(r, remainder_value_3)?;
        r.set(
            enemy,
            field::drawable::Y,
            V::I(
                drawable_y_26.wrapping_sub(
                    (100_i32).wrapping_add(math_abs_value_2.wrapping_shl((2) as u32)),
                ),
            ),
        )?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
    }
    let enemy_kind_37 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_37 == 37 {
        r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(0))?;
        r.set(enemy, field::enemy::STATE_O, V::I(4))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(1))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(128))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(22))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(22))?;
        r.set(enemy, field::enemy::WIDTH, V::I(44))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(40))?;
        let drawable_x_26 = r.get(enemy, field::drawable::X)?.i();
        r.set(enemy, field::enemy::STATE_D, V::I(drawable_x_26))?;
        let drawable_y_27 = r.get(enemy, field::drawable::Y)?.i();
        r.set(
            enemy,
            field::enemy::STATE_E,
            V::I(drawable_y_27.wrapping_add(24)),
        )?;
    }
    let enemy_kind_38 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_38 == 38 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(0))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::enemy::STATE_O, V::I(10))?;
        r.set(enemy, field::enemy::STATE_D_LOWER, V::I(1))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(0))?;
        r.set(enemy, field::enemy::HEALTH, V::I(8))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(24))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(8))?;
        r.set(enemy, field::enemy::WIDTH, V::I(48))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(16))?;
    }
    let enemy_kind_39 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_39 == 128 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        let drawable_half_height = r.get(enemy, field::drawable::HALF_HEIGHT)?.i();
        r.set(
            enemy,
            field::drawable::HALF_HEIGHT,
            V::I(drawable_half_height.wrapping_add(4)),
        )?;
    }
    let enemy_kind_40 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_40 == 129 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        let drawable_half_height_2 = r.get(enemy, field::drawable::HALF_HEIGHT)?.i();
        r.set(
            enemy,
            field::drawable::HALF_HEIGHT,
            V::I(drawable_half_height_2.wrapping_add(4)),
        )?;
    }
    let enemy_kind_41 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_41 == 130 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        let drawable_half_height_3 = r.get(enemy, field::drawable::HALF_HEIGHT)?.i();
        r.set(
            enemy,
            field::drawable::HALF_HEIGHT,
            V::I(drawable_half_height_3.wrapping_add(4)),
        )?;
    }
    let enemy_kind_42 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_42 == 131 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        let drawable_half_height_4 = r.get(enemy, field::drawable::HALF_HEIGHT)?.i();
        r.set(
            enemy,
            field::drawable::HALF_HEIGHT,
            V::I(drawable_half_height_4.wrapping_add(4)),
        )?;
    }
    let enemy_kind_43 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_43 == 132 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(12))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(16))?;
        r.set(enemy, field::enemy::WIDTH, V::I(24))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(32))?;
        r.set(enemy, field::enemy::STATE_X_UPPER, V::I(200))?;
        r.set(enemy, field::enemy::STATE_S_LOWER, V::I(210))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(-1))?;
        r.set(enemy, field::enemy::STATE_G, V::I(1))?;
    }
    let enemy_kind_44 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_44 == 133 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(13))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(14))?;
        r.set(enemy, field::enemy::WIDTH, V::I(16))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(28))?;
        r.set(enemy, field::enemy::FACING, V::I(1))?;
        let object_value_11 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_11)?;
        r.set(enemy, field::enemy::STATE_R, V::R(object_value_11))?;
        r.set(enemy, field::drawable::DRAW_ORDER, V::I(8))?;
    }
    let enemy_kind_45 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_45 == 134 {
        r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
        r.set(enemy, field::enemy::DEAD, V::I(1))?;
        r.set(enemy, field::enemy::STATE_H, V::I(1))?;
        r.set(enemy, field::drawable::HALF_WIDTH, V::I(13))?;
        r.set(enemy, field::drawable::HALF_HEIGHT, V::I(14))?;
        r.set(enemy, field::enemy::WIDTH, V::I(16))?;
        r.set(enemy, field::enemy::HEIGHT, V::I(28))?;
        r.set(enemy, field::enemy::FACING, V::I(1))?;
        let object_value_12 = r.alloc("java/util/Vector", Native::Empty);
        host::vector_initialize(r, object_value_12)?;
        r.set(enemy, field::enemy::STATE_R, V::R(object_value_12))?;
        r.set(enemy, field::drawable::DRAW_ORDER, V::I(8))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(0))?;
    }
    let enemy_kind_46 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_46 != 135 {
        return Ok(());
    }
    r.set(enemy, field::enemy::ACTIVE, V::I(0))?;
    r.set(enemy, field::enemy::DEAD, V::I(1))?;
    r.set(enemy, field::enemy::STATE_H, V::I(1))?;
    return Ok(());
}

/// Draw. Original reference: `l.a(Ljavax/microedition/lcdui/Graphics;IIII)V`.
#[rustfmt::skip]
pub(super) fn draw(
    r: &mut Runtime,
    enemy: u32,
    graphics: u32,
    argument_2: i32,
    argument_3: i32,
    mut argument_4: i32,
    mut argument_5: i32,
) -> Result<(), u32> {
    let mut object_4: u32 = 0;
    let mut object_5: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    'check_enemy_kind: {
    'check_enemy_state_x_upper: {
    'graphics_get_translate_x_16: {
    'handle_result: {
    'next_step_14: {
    'handle_result_3: {
    'handle_result_2: {
    'graphics_translate_11: {
    'check_enemy_kind_11: {
    'check_enemy_state_z_38: {
    'graphics_set_clip_9: {
    'check_enemy_state_z_36: {
    'check_enemy_state_z_33: {
    'check_enemy_kind_10: {
    'check_enemy_state_z_37: {
    'graphics_translate_6: {
    'check_enemy_state_z_34: {
    'check_enemy_state_z_31: {
    'check_enemy_kind_9: {
    'check_enemy_state_z_25: {
    'check_enemy_state_z_19: {
    'check_enemy_state_z_23: {
    'check_enemy_state_z_20: {
    'check_enemy_state_z_16: {
    'check_enemy_kind_8: {
    'check_enemy_kind_7: {
    'check_enemy_state_t_lower: {
    'next_step_13: {
    'graphics_get_translate_x_15: {
    'check_enemy_state_z_35: {
    'graphics_translate_10: {
    'next_step_12: {
    'graphics_get_translate_x_14: {
    'check_enemy_state_z_32: {
    'graphics_translate_9: {
    'next_step_11: {
    'graphics_get_translate_x_13: {
    'check_enemy_state_z_30: {
    'graphics_translate_7: {
    'next_step_9: {
    'graphics_get_translate_x_12: {
    'check_enemy_state_z_29: {
    'graphics_translate_5: {
    'next_step_7: {
    'graphics_get_translate_x_11: {
    'check_enemy_state_z_28: {
    'graphics_translate_8: {
    'next_step_10: {
    'graphics_get_translate_x_9: {
    'check_enemy_state_z_26: {
    'graphics_translate_3: {
    'next_step_5: {
    'graphics_get_translate_x_7: {
    'graphics_get_translate_x_5: {
    'check_enemy_kind_6: {
    'graphics_translate_4: {
    'next_step_6: {
    'graphics_get_translate_x_10: {
    'check_enemy_state_z_27: {
    'check_enemy_facing_3: {
    'next_step_8: {
    'graphics_get_translate_x_8: {
    'check_enemy_state_z_24: {
    'graphics_translate_2: {
    'next_step_4: {
    'graphics_get_translate_x_6: {
    'graphics_get_translate_x_4: {
    'check_enemy_kind_5: {
    'check_enemy_state_z_21: {
    'graphics_set_clip: {
    'check_enemy_state_z_17: {
    'check_enemy_state_z_13: {
    'check_enemy_animation_2: {
    'check_enemy_state_z_9: {
    'graphics_translate: {
    'check_enemy_state_z_14: {
    'check_enemy_state_z_10: {
    'check_enemy_state_z_7: {
    'check_enemy_state_z_5: {
    'graphics_get_translate_x_2: {
    'check_enemy_kind_4: {
    'check_enemy_state_z_22: {
    'check_enemy_facing_2: {
    'check_enemy_state_z_18: {
    'check_enemy_state_z_15: {
    'check_enemy_state_z_11: {
    'check_enemy_facing: {
    'check_enemy_animation: {
    'check_enemy_state_z_3: {
    'check_enemy_state_z: {
    'check_enemy_state_z_12: {
    'check_enemy_state_z_8: {
    'check_enemy_state_z_6: {
    'check_enemy_state_z_4: {
    'check_enemy_state_z_2: {
    'graphics_get_translate_x: {
    'check_enemy_kind_3: {
    'graphics_get_translate_x_3: {
    'check_enemy_state_s: {
    'next_step_3: {
    'check_enemy_state_r_3: {
    'next_step_2: {
    'check_enemy_kind_2: {
    'check_enemy_state_r: {
    'next_step: {
    let drawable_x = r.get(enemy, field::drawable::X)?.i();
    if !(argument_4.wrapping_add(argument_2) >= drawable_x) {
let enemy_kind = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind != 8 {
let enemy_kind_3 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_3 != 20 {
let enemy_kind_6 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_6 != 21 {
return Ok(());
}
}
}
}
    let drawable_x_2 = r.get(enemy, field::drawable::X)?.i();
    let drawable_half_width = r.get(enemy, field::drawable::HALF_WIDTH)?.i();
    let drawable_half_width_2 = r.get(enemy, field::drawable::HALF_WIDTH)?.i();
    if !((drawable_x_2.wrapping_add(drawable_half_width)).wrapping_add(drawable_half_width_2) >= argument_2) {
let enemy_kind_2 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_2 != 8 {
let enemy_kind_5 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_5 != 20 {
let enemy_kind_9 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_9 != 21 {
return Ok(());
}
}
}
}
    let drawable_y = r.get(enemy, field::drawable::Y)?.i();
    if !(argument_5.wrapping_add(argument_3) >= drawable_y) {
let enemy_kind_4 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_4 != 8 {
let enemy_kind_8 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_8 != 20 {
let enemy_kind_11 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_11 != 21 {
return Ok(());
}
}
}
}
    let drawable_y_2 = r.get(enemy, field::drawable::Y)?.i();
    let drawable_half_height = r.get(enemy, field::drawable::HALF_HEIGHT)?.i();
    let drawable_half_height_2 = r.get(enemy, field::drawable::HALF_HEIGHT)?.i();
    if !(((drawable_y_2.wrapping_add(drawable_half_height)).wrapping_add(drawable_half_height_2)).wrapping_sub(32) >= argument_3) {
let enemy_kind_7 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_7 != 8 {
let enemy_kind_10 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_10 != 20 {
let enemy_kind_16 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_16 != 21 {
return Ok(());
}
}
}
}
    let enemy_hurt_timer = r.get(enemy, field::enemy::HURT_TIMER)?.i();
    let remainder_value_3 = arithmetic::remainder_int(r, enemy_hurt_timer, 3)?;
    if remainder_value_3 == 1 {
let enemy_behavior_state = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state != 20 {
let enemy_kind_15 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_15 != 29 {
super::enemy::reset_animation(r, enemy)?;
return Ok(());
}
}
}
    super::enemy::reset_animation(r, enemy)?;
    let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value.wrapping_neg(), graphics_get_translate_y_value.wrapping_neg())?;
    let drawable_x_3 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_3 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_3.wrapping_sub(argument_2), drawable_y_3.wrapping_sub(argument_3))?;
    host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
    let enemy_dying = r.get(enemy, field::enemy::DYING)?.i();
    if enemy_dying != 0 {
let enemy_kind_13 = r.get(enemy, field::enemy::KIND)?.i();
if enemy_kind_13 == 16 {
host::graphics_translate(r, graphics, 16, 16)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
}
let enemy_image_2 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_death_frame = r.get(enemy, field::enemy::DEATH_FRAME)?.i();
host::graphics_draw_image(r, graphics, enemy_image_2, (-16_i32).wrapping_mul(enemy_death_frame.wrapping_shr((1) as u32)), 0, 0)?;
return Ok(());
}
    let enemy_kind_14 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_14 == 0 {
let enemy_image_3 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_3 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_2 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_3, (-16_i32).wrapping_mul(enemy_animation_3.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_2), 0)?;
}
    let enemy_kind_18 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_18 == 1 {
let enemy_image_5 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_6 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_5, (-16_i32).wrapping_mul(enemy_animation_6.wrapping_shr((2) as u32)), 0, 0)?;
}
    let enemy_kind_20 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_20 == 2 {
let enemy_image_6 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_7 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_6, (-16_i32).wrapping_mul(enemy_animation_7.wrapping_shr((3) as u32)), 0, 0)?;
}
    let enemy_kind_21 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_21 == 3 {
let enemy_image_7 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_8 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_7, (-16_i32).wrapping_mul(enemy_animation_8.wrapping_shr((3) as u32)), 0, 0)?;
}
    let enemy_kind_22 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_22 == 4 {
host::graphics_translate(r, graphics, 0, 4)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 8)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 8)?;
let enemy_image_9 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_9 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_9, 0, (-8_i32).wrapping_mul(enemy_animation_9.wrapping_shr((3) as u32)), 0)?;
}
    let enemy_kind_23 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_23 == 5 {
let enemy_image_10 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_10 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_3 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_10, (-16_i32).wrapping_mul(enemy_animation_10.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_3), 0)?;
}
    let enemy_kind_24 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_24 == 6 {
let enemy_image_11 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_11 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_4 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_11, (-16_i32).wrapping_mul(enemy_animation_11.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_4), 0)?;
}
    let enemy_kind_25 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_25 == 7 {
let enemy_image_12 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_12 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_5 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_12, (-16_i32).wrapping_mul(enemy_animation_12.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_5), 0)?;
}
    let enemy_kind_26 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_26 != 8 { break 'check_enemy_kind_2; }
    } // next_step
    argument_4 = 0;
    } // check_enemy_state_r
    'draw_loop_6: loop {
        'check_enemy_state_r_2: {
        let enemy_state_r = r.get(enemy, field::enemy::STATE_R)?.r();
        let vector_size_value = host::vector_size(r, enemy_state_r)?;
        if argument_4 >= vector_size_value { break 'check_enemy_kind_2; }
        } // check_enemy_state_r_2
        let enemy_state_r_2 = r.get(enemy, field::enemy::STATE_R)?.r();
        let vector_element_at_value = host::vector_element_at(r, enemy_state_r_2, argument_4)?;
        host::check_cast(r, vector_element_at_value, "g")?;
        object_5 = vector_element_at_value;
        let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, graphics)?;
        let graphics_get_translate_y_value_2 = host::graphics_get_translate_y(r, graphics)?;
        host::graphics_translate(r, graphics, graphics_get_translate_x_value_2.wrapping_neg(), graphics_get_translate_y_value_2.wrapping_neg())?;
        let trail_point_x_4 = r.get(object_5, field::trail_point::X)?.i();
        let trail_point_y_4 = r.get(object_5, field::trail_point::Y)?.i();
        host::graphics_translate(r, graphics, trail_point_x_4.wrapping_sub(argument_2), trail_point_y_4.wrapping_sub(argument_3))?;
        let enemy_state_r_3 = r.get(enemy, field::enemy::STATE_R)?.r();
        let vector_size_value_2 = host::vector_size(r, enemy_state_r_3)?;
        if argument_4 != vector_size_value_2.wrapping_sub(1) {

if argument_4 != 0 {
host::graphics_translate(r, graphics, 2, 2)?;
host::graphics_set_clip(r, graphics, 0, 0, 12, 12)?;
host::graphics_clip_rect(r, graphics, 0, 0, 12, 12)?;
let enemy_image_17 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_17, 0, -32, 0)?;
} else {
host::graphics_translate(r, graphics, 7, 7)?;
host::graphics_set_clip(r, graphics, 0, 0, 6, 6)?;
host::graphics_clip_rect(r, graphics, 0, 0, 6, 6)?;
let enemy_image_18 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_18, -12, -32, 0)?;
}
} else {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_facing_7 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_7 == 0 {
let enemy_image_19 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_19, 0, 0, 0)?;
}
let enemy_facing_9 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_9 == 1 {
let enemy_image_23 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_23, -16, 0, 0)?;
}
let enemy_facing_11 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_11 == 2 {
let enemy_image_26 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_26, 0, -16, 0)?;
}
let enemy_facing_14 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_14 == 3 {
let enemy_image_29 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_29, -16, -16, 0)?;
}
}
        argument_4 = argument_4.wrapping_add(1);
        continue 'draw_loop_6;
    }
    } // check_enemy_kind_2
    let enemy_kind_27 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_27 == 9 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_13 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_13, 0, 0, 0)?;
}
    let enemy_kind_28 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_28 == 10 {
let enemy_image_14 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_13 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_6 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_14, (-16_i32).wrapping_mul(enemy_animation_13.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_6), 0)?;
}
    let enemy_kind_29 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_29 == 11 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_behavior_state_5 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_5 == 1 {
let enemy_image_16 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_16, 0, -32, 0)?;
}
let enemy_behavior_state_6 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_6 == 2 {
let enemy_image_21 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_21, -16, -32, 0)?;
let enemy_state_a_2 = r.get(enemy, field::enemy::STATE_A)?.i();
host::graphics_translate(r, graphics, 0, enemy_state_a_2.wrapping_neg())?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_22 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_facing_10 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_22, 0, ((enemy_facing_10.wrapping_shr((1) as u32)).wrapping_neg()).wrapping_shl((4) as u32), 0)?;
let enemy_state_a_3 = r.get(enemy, field::enemy::STATE_A)?.i();
host::graphics_translate(r, graphics, 0, enemy_state_a_3)?;
}
let enemy_behavior_state_7 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_7 == 3 {
let enemy_image_25 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_15 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_13 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_25, ((enemy_animation_15.wrapping_shr((3) as u32)).wrapping_neg()).wrapping_shl((4) as u32), ((enemy_facing_13.wrapping_shr((1) as u32)).wrapping_neg()).wrapping_shl((4) as u32), 0)?;
}
}
    let enemy_kind_30 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_30 == 12 {
let enemy_image_15 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_14 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_8 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_15, (-16_i32).wrapping_mul(enemy_animation_14.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_8), 0)?;
}
    let enemy_kind_31 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_31 == 13 {
let enemy_image_20 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_20, 0, -64, 0)?;
let enemy_state_a = r.get(enemy, field::enemy::STATE_A)?.i();
if enemy_state_a <= 0 {
let enemy_state_a_4 = r.get(enemy, field::enemy::STATE_A)?.i();
host::graphics_translate(r, graphics, 0, (-16_i32).wrapping_sub(enemy_state_a_4))?;
} else {
let enemy_state_a_5 = r.get(enemy, field::enemy::STATE_A)?.i();
host::graphics_translate(r, graphics, 0, (-16_i32).wrapping_add(enemy_state_a_5))?;
}
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_28 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_17 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_16 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_28, (-16_i32).wrapping_mul(enemy_animation_17.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_16), 0)?;
}
    let enemy_kind_32 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_32 == 14 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_24 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_facing_12 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_24, (-16_i32).wrapping_mul(enemy_facing_12), 0, 0)?;
}
    let enemy_kind_33 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_33 == 15 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 26)?;
let enemy_image_27 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_16 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_15 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_27, (-16_i32).wrapping_mul(enemy_animation_16.wrapping_shr((3) as u32)), (-26_i32).wrapping_mul(enemy_facing_15), 0)?;
}
    let enemy_kind_34 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_34 == 16 {
host::graphics_set_clip(r, graphics, 0, 0, 32, 32)?;
host::graphics_clip_rect(r, graphics, 0, 0, 32, 32)?;
let enemy_image_30 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_18 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_30, (-32_i32).wrapping_mul(enemy_animation_18.wrapping_shr((3) as u32)), 0, 0)?;
}
    let enemy_kind_35 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_35 == 17 {
let object_value = r.alloc("java/util/Random", Native::Empty);
host::random_initialize(r, object_value)?;
object_4 = object_value;
let enemy_animation_19 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::random_set_seed(r, object_value, (enemy_animation_19.wrapping_mul(3456)) as i64)?;
let random_next_int_value_3 = host::random_next_int(r, object_4)?;
let remainder_value_4 = arithmetic::remainder_int(r, random_next_int_value_3, 2)?;
argument_5 = remainder_value_4;
let random_next_int_value_4 = host::random_next_int(r, object_4)?;
let remainder_value_5 = arithmetic::remainder_int(r, random_next_int_value_4, 2)?;
argument_4 = remainder_value_5;
let enemy_facing_17 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_17 == 0 {
r.set(enemy, field::drawable::HALF_WIDTH, V::I(20))?;
r.set(enemy, field::drawable::HALF_HEIGHT, V::I(20))?;
r.set(enemy, field::enemy::WIDTH, V::I(20))?;
r.set(enemy, field::enemy::HEIGHT, V::I(20))?;
host::graphics_translate(r, graphics, -8, 0)?;
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_32 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_32, argument_5.wrapping_mul(-15), -85, 0)?;
host::graphics_translate(r, graphics, 0, 8)?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_33 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_33, (-15_i32).wrapping_sub(argument_4.wrapping_mul(15)), -85, 0)?;
host::graphics_translate(r, graphics, (0_i32).wrapping_sub(argument_5.wrapping_shl((1) as u32)), (8_i32).wrapping_sub(argument_4.wrapping_shl((1) as u32)))?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_34 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_34, -30, -85, 0)?;
let graphics_get_translate_x_value_3 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_3 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_3.wrapping_neg(), graphics_get_translate_y_value_3.wrapping_neg())?;
let drawable_x_5 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_5 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_5.wrapping_sub(argument_2), drawable_y_5.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, -8, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 23, 45)?;
host::graphics_clip_rect(r, graphics, 0, 0, 23, 45)?;
let enemy_image_35 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_20 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_35, (-23_i32).wrapping_mul(enemy_animation_20.wrapping_shr((3) as u32)), 0, 0)?;
r.set(enemy, field::drawable::HALF_WIDTH, V::I(12))?;
r.set(enemy, field::drawable::HALF_HEIGHT, V::I(22))?;
r.set(enemy, field::enemy::WIDTH, V::I(23))?;
r.set(enemy, field::enemy::HEIGHT, V::I(40))?;
}
let enemy_facing_18 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_18 == 1 {
host::graphics_translate(r, graphics, 0, -8)?;
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_37 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_37, argument_5.wrapping_mul(-15), -85, 0)?;
host::graphics_translate(r, graphics, -8, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_38 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_38, (-15_i32).wrapping_sub(argument_4.wrapping_mul(15)), -85, 0)?;
host::graphics_translate(r, graphics, (-8_i32).wrapping_sub(argument_5.wrapping_mul(argument_5)), (argument_4.wrapping_neg()).wrapping_shl((1) as u32))?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_39 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_39, -30, -85, 0)?;
let graphics_get_translate_x_value_4 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_4 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_4.wrapping_neg(), graphics_get_translate_y_value_4.wrapping_neg())?;
let drawable_x_6 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_6 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_6.wrapping_sub(argument_2), drawable_y_6.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, 0, -8)?;
host::graphics_set_clip(r, graphics, 0, 0, 44, 20)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 20)?;
let enemy_image_40 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_22 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_40, -44, (-45_i32).wrapping_sub((20_i32).wrapping_mul(enemy_animation_22.wrapping_shr((3) as u32))), 0)?;
r.set(enemy, field::drawable::HALF_WIDTH, V::I(22))?;
r.set(enemy, field::drawable::HALF_HEIGHT, V::I(10))?;
r.set(enemy, field::enemy::WIDTH, V::I(44))?;
r.set(enemy, field::enemy::HEIGHT, V::I(20))?;
}
let enemy_facing_19 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_19 == 2 {
host::graphics_translate(r, graphics, -8, 0)?;
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_42 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_42, argument_5.wrapping_mul(-15), -85, 0)?;
host::graphics_translate(r, graphics, 0, -8)?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_43 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_43, (-15_i32).wrapping_sub(argument_4.wrapping_mul(15)), -85, 0)?;
host::graphics_translate(r, graphics, (0_i32).wrapping_sub(argument_5.wrapping_mul(argument_5)), (-8_i32).wrapping_sub(argument_4.wrapping_shl((1) as u32)))?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_44 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_44, -30, -85, 0)?;
let graphics_get_translate_x_value_5 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_5 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_5.wrapping_neg(), graphics_get_translate_y_value_5.wrapping_neg())?;
let drawable_x_7 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_7 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_7.wrapping_sub(argument_2), drawable_y_7.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, -8, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 23, 45)?;
host::graphics_clip_rect(r, graphics, 0, 0, 23, 45)?;
let enemy_image_45 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_24 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_45, ((-23_i32).wrapping_mul(enemy_animation_24.wrapping_shr((3) as u32))).wrapping_sub(46), 0, 0)?;
r.set(enemy, field::drawable::HALF_WIDTH, V::I(12))?;
r.set(enemy, field::drawable::HALF_HEIGHT, V::I(22))?;
r.set(enemy, field::enemy::WIDTH, V::I(23))?;
r.set(enemy, field::enemy::HEIGHT, V::I(40))?;
}
let enemy_facing_20 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_20 == 3 {
host::graphics_translate(r, graphics, 0, -8)?;
host::graphics_translate(r, graphics, 35, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_47 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_47, argument_5.wrapping_mul(-15), -85, 0)?;
host::graphics_translate(r, graphics, 8, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_48 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_48, (-15_i32).wrapping_sub(argument_4.wrapping_mul(15)), -85, 0)?;
host::graphics_translate(r, graphics, (8_i32).wrapping_sub(argument_5.wrapping_mul(argument_5)), (0_i32).wrapping_sub(argument_4.wrapping_shl((1) as u32)))?;
host::graphics_set_clip(r, graphics, 0, 0, 15, 15)?;
host::graphics_clip_rect(r, graphics, 0, 0, 15, 15)?;
let enemy_image_49 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_49, -30, -85, 0)?;
let graphics_get_translate_x_value_6 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_6 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_6.wrapping_neg(), graphics_get_translate_y_value_6.wrapping_neg())?;
let drawable_x_8 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_8 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_8.wrapping_sub(argument_2), drawable_y_8.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, 0, -8)?;
host::graphics_set_clip(r, graphics, 0, 0, 44, 20)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 20)?;
let enemy_image_50 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_28 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_50, 0, (-45_i32).wrapping_sub((20_i32).wrapping_mul(enemy_animation_28.wrapping_shr((3) as u32))), 0)?;
r.set(enemy, field::drawable::HALF_WIDTH, V::I(22))?;
r.set(enemy, field::drawable::HALF_HEIGHT, V::I(10))?;
r.set(enemy, field::enemy::WIDTH, V::I(44))?;
r.set(enemy, field::enemy::HEIGHT, V::I(20))?;
}
let enemy_animation_27 = r.get(enemy, field::enemy::ANIMATION)?.i();
r.set(enemy, field::enemy::ANIMATION, V::I((enemy_animation_27.wrapping_add(1)) as i8 as i32))?;
}
    let enemy_kind_36 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_36 == 18 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_31 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_31, 0, 0, 0)?;
let enemy_health = r.get(enemy, field::enemy::HEALTH)?.i();
if enemy_health > 1 {
host::graphics_translate(r, graphics, 12, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_36 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_21 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_36, (-16_i32).wrapping_add((16_i32).wrapping_mul(enemy_animation_21.wrapping_shr((3) as u32))), -32, 0)?;
host::graphics_translate(r, graphics, -12, 0)?;
}
let enemy_health_2 = r.get(enemy, field::enemy::HEALTH)?.i();
if enemy_health_2 > 3 {
host::graphics_translate(r, graphics, -12, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_41 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_23 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_41, (-16_i32).wrapping_mul(enemy_animation_23.wrapping_shr((3) as u32)), -16, 0)?;
host::graphics_translate(r, graphics, 12, 0)?;
}
let enemy_health_3 = r.get(enemy, field::enemy::HEALTH)?.i();
if enemy_health_3 > 5 {
host::graphics_translate(r, graphics, 0, -12)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_46 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_26 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_46, -32, (-32_i32).wrapping_add((16_i32).wrapping_mul(enemy_animation_26.wrapping_shr((3) as u32))), 0)?;
host::graphics_translate(r, graphics, 0, 12)?;
}
let enemy_health_4 = r.get(enemy, field::enemy::HEALTH)?.i();
if enemy_health_4 > 7 {
host::graphics_translate(r, graphics, 0, 12)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_53 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_31 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_draw_image(r, graphics, enemy_image_53, (-16_i32).wrapping_sub((16_i32).wrapping_mul(enemy_animation_31.wrapping_shr((3) as u32))), 0, 0)?;
}
}
    let enemy_kind_37 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_37 == 19 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_behavior_state_8 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_8 != 0 {
let enemy_behavior_state_9 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_9 == 1 {
let enemy_animation_25 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_6 = arithmetic::remainder_int(r, enemy_animation_25, 2)?;
if remainder_value_6 == 0 {
let enemy_image_52 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_30 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_22 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_52, (-16_i32).wrapping_mul(enemy_animation_30.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_22), 0)?;
}
}
let enemy_behavior_state_10 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_10 == 2 {
let enemy_image_51 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_29 = r.get(enemy, field::enemy::ANIMATION)?.i();
let enemy_facing_21 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_51, (-16_i32).wrapping_mul(enemy_animation_29.wrapping_shr((3) as u32)), (-16_i32).wrapping_mul(enemy_facing_21), 0)?;
}
}
}
    let enemy_kind_38 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_38 != 20 { break 'check_enemy_kind_3; }
    } // next_step_2
    argument_4 = 0;
    } // check_enemy_state_r_3
    'draw_loop_10: loop {
        'check_enemy_state_r_4: {
        let enemy_state_r_4 = r.get(enemy, field::enemy::STATE_R)?.r();
        let vector_size_value_3 = host::vector_size(r, enemy_state_r_4)?;
        if argument_4 >= vector_size_value_3 { break 'next_step_3; }
        } // check_enemy_state_r_4
        let enemy_state_r_5 = r.get(enemy, field::enemy::STATE_R)?.r();
        let vector_element_at_value_2 = host::vector_element_at(r, enemy_state_r_5, argument_4)?;
        host::check_cast(r, vector_element_at_value_2, "g")?;
        object_5 = vector_element_at_value_2;
        let graphics_get_translate_x_value_7 = host::graphics_get_translate_x(r, graphics)?;
        let graphics_get_translate_y_value_7 = host::graphics_get_translate_y(r, graphics)?;
        host::graphics_translate(r, graphics, graphics_get_translate_x_value_7.wrapping_neg(), graphics_get_translate_y_value_7.wrapping_neg())?;
        let trail_point_x_9 = r.get(object_5, field::trail_point::X)?.i();
        let trail_point_y_9 = r.get(object_5, field::trail_point::Y)?.i();
        host::graphics_translate(r, graphics, trail_point_x_9.wrapping_sub(argument_2), trail_point_y_9.wrapping_sub(argument_3))?;
        let enemy_state_r_6 = r.get(enemy, field::enemy::STATE_R)?.r();
        let vector_size_value_4 = host::vector_size(r, enemy_state_r_6)?;
        if argument_4 >= vector_size_value_4.wrapping_sub(1) {
let trail_point_state_c = r.get(object_5, field::trail_point::STATE_C)?.i();
if trail_point_state_c >= 0 {
host::graphics_set_clip(r, graphics, 0, 0, 33, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 33, 24)?;
let enemy_image_57 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_57, -47, -26, 0)?;
} else {
let trail_point_state_c_2 = r.get(object_5, field::trail_point::STATE_C)?.i();
r.set(object_5, field::trail_point::STATE_C, V::I(trail_point_state_c_2.wrapping_add(1)))?;
host::graphics_set_clip(r, graphics, 0, 0, 33, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 33, 26)?;
let enemy_image_58 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_58, -47, 0, 0)?;
}
} else {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_54 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_54, -48, -50, 0)?;
}
        argument_4 = argument_4.wrapping_add(1);
        continue 'draw_loop_10;
    }
    } // next_step_3
    argument_4 = 0;
    } // check_enemy_state_s
    'draw_loop_13: loop {
        'check_enemy_state_s_2: {
        let enemy_state_s = r.get(enemy, field::enemy::STATE_S)?.r();
        let vector_size_value_5 = host::vector_size(r, enemy_state_s)?;
        if argument_4 >= vector_size_value_5 { break 'graphics_get_translate_x_3; }
        } // check_enemy_state_s_2
        let enemy_state_s_2 = r.get(enemy, field::enemy::STATE_S)?.r();
        let vector_element_at_value_3 = host::vector_element_at(r, enemy_state_s_2, argument_4)?;
        host::check_cast(r, vector_element_at_value_3, "g")?;
        object_5 = vector_element_at_value_3;
        let graphics_get_translate_x_value_11 = host::graphics_get_translate_x(r, graphics)?;
        let graphics_get_translate_y_value_11 = host::graphics_get_translate_y(r, graphics)?;
        host::graphics_translate(r, graphics, graphics_get_translate_x_value_11.wrapping_neg(), graphics_get_translate_y_value_11.wrapping_neg())?;
        let trail_point_x_13 = r.get(object_5, field::trail_point::X)?.i();
        let trail_point_y_13 = r.get(object_5, field::trail_point::Y)?.i();
        host::graphics_translate(r, graphics, trail_point_x_13.wrapping_sub(argument_2), trail_point_y_13.wrapping_sub(argument_3))?;
        let enemy_state_s_3 = r.get(enemy, field::enemy::STATE_S)?.r();
        let vector_size_value_6 = host::vector_size(r, enemy_state_s_3)?;
        if argument_4 >= vector_size_value_6.wrapping_sub(1) {
let trail_point_state_c_3 = r.get(object_5, field::trail_point::STATE_C)?.i();
if trail_point_state_c_3 >= 0 {
host::graphics_set_clip(r, graphics, 0, 0, 33, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 33, 24)?;
let enemy_image_66 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_66, -47, -26, 0)?;
} else {
let trail_point_state_c_4 = r.get(object_5, field::trail_point::STATE_C)?.i();
r.set(object_5, field::trail_point::STATE_C, V::I(trail_point_state_c_4.wrapping_add(1)))?;
host::graphics_set_clip(r, graphics, 0, 0, 33, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 33, 26)?;
let enemy_image_67 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_67, -47, 0, 0)?;
}
} else {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_image_62 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_62, -48, -50, 0)?;
}
        argument_4 = argument_4.wrapping_add(1);
        continue 'draw_loop_13;
    }
    } // graphics_get_translate_x_3
    let graphics_get_translate_x_value_10 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_10 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_10.wrapping_neg(), graphics_get_translate_y_value_10.wrapping_neg())?;
    let drawable_x_12 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_12 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_12.wrapping_sub(argument_2), drawable_y_12.wrapping_sub(argument_3))?;
    host::graphics_set_clip(r, graphics, 0, 0, 47, 64)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 47, 64)?;
    let enemy_image_56 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_56, 0, 0, 0)?;
    } // check_enemy_kind_3
    let enemy_kind_39 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_39 == 21 {
let enemy_state_o_lower = r.get(enemy, field::enemy::STATE_O_LOWER)?.i();
if enemy_state_o_lower != 0 {
let enemy_behavior_state_12 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_12 == 1 {
let enemy_state_timer_2 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
let remainder_value_7 = arithmetic::remainder_int(r, enemy_state_timer_2, 2)?;
if remainder_value_7 == 0 {
host::graphics_set_clip(r, graphics, 0, 0, 56, 90)?;
host::graphics_clip_rect(r, graphics, 0, 0, 56, 90)?;
let enemy_image_55 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_55, 0, 0, 0)?;
}
}
let enemy_behavior_state_15 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_15 == 2 {
let enemy_state_timer_3 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_3 < 70 {
host::graphics_set_clip(r, graphics, 0, 0, 56, 90)?;
host::graphics_clip_rect(r, graphics, 0, 0, 56, 90)?;
let enemy_image_61 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_61, 0, 0, 0)?;
}
let enemy_state_timer_4 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_4 >= 70 {
let enemy_state_timer_6 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_6 < 75 {
host::graphics_set_clip(r, graphics, 0, 0, 49, 60)?;
host::graphics_clip_rect(r, graphics, 0, 0, 49, 60)?;
let enemy_image_74 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_74, -56, -58, 0)?;
}
}
let enemy_state_timer_5 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
if enemy_state_timer_5 >= 75 {
host::graphics_set_clip(r, graphics, 0, 0, 68, 59)?;
host::graphics_clip_rect(r, graphics, 0, 0, 68, 59)?;
let enemy_image_73 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_73, -56, 0, 0)?;
}
}
}
}
    let enemy_kind_40 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_40 == 22 {
let enemy_behavior_state_11 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_11 == 0 {
return Ok(());
}
let enemy_behavior_state_14 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_14 == 1 {
let enemy_animation_32 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_8 = arithmetic::remainder_int(r, enemy_animation_32, 2)?;
if remainder_value_8 == 0 {
let enemy_facing_25 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_25 == 1 {
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 22, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 36)?;
let enemy_image_65 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_65, -80, 0, 0)?;
}
let enemy_facing_30 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_30 == 3 {
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 21, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 21, 36)?;
let enemy_image_72 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_72, 0, 0, 0)?;
}
}
}
let enemy_behavior_state_19 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_19 == 2 {
let enemy_facing_24 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_24 == 1 {
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 28, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 28, 36)?;
let enemy_image_64 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_64, -50, 0, 0)?;
}
let enemy_facing_29 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_29 == 3 {
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 21, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 21, 36)?;
let enemy_image_71 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_71, -21, 0, 0)?;
}
}
let enemy_behavior_state_23 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_23 == 3 {
let enemy_facing_28 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_28 == 1 {
let enemy_animation_37 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_37.wrapping_shr((3) as u32) >= 1 {
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 28, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 28, 36)?;
let enemy_image_79 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_79, -50, 0, 0)?;
} else {
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 22, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 36)?;
let enemy_image_80 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_80, -80, 0, 0)?;
}
}
let enemy_facing_34 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_34 == 3 {
let enemy_animation_40 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_40.wrapping_shr((3) as u32) >= 1 {
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 21, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 21, 36)?;
let enemy_image_83 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_83, -21, 0, 0)?;
} else {
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 21, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 21, 36)?;
let enemy_image_84 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_84, 0, 0, 0)?;
}
}
}
}
    let enemy_kind_41 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_41 != 23 { break 'check_enemy_kind_4; }
    } // graphics_get_translate_x
    let graphics_get_translate_x_value_8 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_8 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_8.wrapping_neg(), graphics_get_translate_y_value_8.wrapping_neg())?;
    let drawable_x_10 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_10 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_10.wrapping_sub(argument_2), drawable_y_10.wrapping_sub(argument_3))?;
    let enemy_behavior_state_13 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_13 == 0 { break 'check_enemy_state_z; }
    } // check_enemy_state_z_2
    let enemy_behavior_state_18 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_18 == 11 { break 'check_enemy_state_z; }
    } // check_enemy_state_z_4
    let enemy_behavior_state_22 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_22 == 10 { break 'check_enemy_state_z; }
    } // check_enemy_state_z_6
    let enemy_behavior_state_27 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_27 == 1 { break 'check_enemy_state_z; }
    } // check_enemy_state_z_8
    let enemy_behavior_state_31 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_31 == 3 { break 'check_enemy_state_z; }
    } // check_enemy_state_z_12
    let enemy_behavior_state_37 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_37 != 5 { break 'check_enemy_state_z_11; }
    } // check_enemy_state_z
    let enemy_behavior_state_17 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_17 == 1 { break 'check_enemy_animation; }
    } // check_enemy_state_z_3
    let enemy_behavior_state_21 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_21 != 3 { break 'check_enemy_facing; }
    } // check_enemy_animation
    let enemy_animation_33 = r.get(enemy, field::enemy::ANIMATION)?.i();
    let remainder_value_9 = arithmetic::remainder_int(r, enemy_animation_33, 2)?;
    if remainder_value_9 == 1 {
return Ok(());
}
    } // check_enemy_facing
    let enemy_facing_27 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_27 == 1 {
host::graphics_translate(r, graphics, 7, 47)?;
host::graphics_set_clip(r, graphics, 0, 0, 27, 7)?;
host::graphics_clip_rect(r, graphics, 0, 0, 27, 7)?;
let enemy_image_69 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_69, 0, -107, 0)?;
let enemy_animation_36 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_translate(r, graphics, -7, (-47_i32).wrapping_add(enemy_animation_36.wrapping_shr((3) as u32)))?;
host::graphics_set_clip(r, graphics, 0, 0, 35, 42)?;
host::graphics_clip_rect(r, graphics, 0, 0, 35, 42)?;
let enemy_image_70 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_70, 0, 0, 0)?;
}
    let enemy_facing_33 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_33 == 3 {
host::graphics_translate(r, graphics, 7, 47)?;
host::graphics_set_clip(r, graphics, 0, 0, 27, 7)?;
host::graphics_clip_rect(r, graphics, 0, 0, 27, 7)?;
let enemy_image_77 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_77, 0, -107, 0)?;
let enemy_animation_39 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_translate(r, graphics, -7, (-47_i32).wrapping_add(enemy_animation_39.wrapping_shr((3) as u32)))?;
host::graphics_set_clip(r, graphics, 0, 0, 35, 42)?;
host::graphics_clip_rect(r, graphics, 0, 0, 35, 42)?;
let enemy_image_78 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_78, -35, 0, 0)?;
}
    } // check_enemy_state_z_11
    let enemy_behavior_state_36 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_36 == 2 { break 'check_enemy_facing_2; }
    } // check_enemy_state_z_15
    let enemy_behavior_state_42 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_42 == 4 { break 'check_enemy_facing_2; }
    } // check_enemy_state_z_18
    let enemy_behavior_state_47 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_47 != 6 { break 'check_enemy_state_z_22; }
    } // check_enemy_facing_2
    let enemy_facing_43 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_43 == 1 {
host::graphics_translate(r, graphics, 7, 47)?;
host::graphics_set_clip(r, graphics, 0, 0, 27, 7)?;
host::graphics_clip_rect(r, graphics, 0, 0, 27, 7)?;
let enemy_image_93 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_93, 0, -107, 0)?;
let enemy_animation_51 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_translate(r, graphics, -7, (-69_i32).wrapping_add(enemy_animation_51.wrapping_shr((3) as u32)))?;
host::graphics_set_clip(r, graphics, 0, 0, 34, 64)?;
host::graphics_clip_rect(r, graphics, 0, 0, 34, 64)?;
let enemy_image_94 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_94, 0, -43, 0)?;
}
    let enemy_facing_48 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_48 == 3 {
let enemy_behavior_state_54 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_54 == 2 {
host::graphics_translate(r, graphics, 30, 0)?;
}
host::graphics_translate(r, graphics, 7, 47)?;
host::graphics_set_clip(r, graphics, 0, 0, 27, 7)?;
host::graphics_clip_rect(r, graphics, 0, 0, 27, 7)?;
let enemy_image_118 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_118, 0, -107, 0)?;
let enemy_animation_68 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_translate(r, graphics, -7, (-69_i32).wrapping_add(enemy_animation_68.wrapping_shr((3) as u32)))?;
host::graphics_set_clip(r, graphics, 0, 0, 34, 64)?;
host::graphics_clip_rect(r, graphics, 0, 0, 34, 64)?;
let enemy_image_119 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_119, -34, -43, 0)?;
}
    } // check_enemy_state_z_22
    let enemy_behavior_state_53 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_53 == 7 {
host::graphics_set_clip(r, graphics, 0, 0, 38, 40)?;
host::graphics_clip_rect(r, graphics, 0, 0, 38, 40)?;
let enemy_image_117 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_67 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_19 = arithmetic::remainder_int(r, enemy_animation_67.wrapping_shr((2) as u32), 2)?;
host::graphics_draw_image(r, graphics, enemy_image_117, -70, (-40_i32).wrapping_mul(remainder_value_19), 0)?;
}
    let enemy_behavior_state_60 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_60 == 8 {
let enemy_animation_75 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_23 = arithmetic::remainder_int(r, enemy_animation_75, 2)?;
if remainder_value_23 == 1 {
return Ok(());
}
host::graphics_set_clip(r, graphics, 0, 0, 38, 40)?;
host::graphics_clip_rect(r, graphics, 0, 0, 38, 40)?;
let enemy_image_146 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_146, -70, -80, 0)?;
}
    } // check_enemy_kind_4
    let enemy_kind_42 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_42 != 24 { break 'check_enemy_kind_5; }
    } // graphics_get_translate_x_2
    let graphics_get_translate_x_value_9 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_9 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_9.wrapping_neg(), graphics_get_translate_y_value_9.wrapping_neg())?;
    let drawable_x_11 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_11 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_11.wrapping_sub(argument_2), drawable_y_11.wrapping_sub(argument_3))?;
    let enemy_behavior_state_16 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_16 == 0 {
host::graphics_translate(r, graphics, 9, 38)?;
host::graphics_set_clip(r, graphics, 0, 0, 36, 14)?;
host::graphics_clip_rect(r, graphics, 0, 0, 36, 14)?;
let enemy_image_59 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_59, -1, -152, 0)?;
host::graphics_translate(r, graphics, -9, -38)?;
host::graphics_set_clip(r, graphics, 0, 0, 56, 48)?;
host::graphics_clip_rect(r, graphics, 0, 0, 56, 48)?;
let enemy_image_60 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_60, 0, 0, 0)?;
}
    let enemy_behavior_state_20 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_20 == 1 { break 'graphics_translate; }
    } // check_enemy_state_z_5
    let enemy_behavior_state_26 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_26 == 3 { break 'graphics_translate; }
    } // check_enemy_state_z_7
    let enemy_behavior_state_30 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_30 == 4 { break 'graphics_translate; }
    } // check_enemy_state_z_10
    let enemy_behavior_state_35 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_35 == 7 { break 'graphics_translate; }
    } // check_enemy_state_z_14
    let enemy_behavior_state_41 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_41 != 6 { break 'check_enemy_state_z_13; }
    } // graphics_translate
    host::graphics_translate(r, graphics, 9, 40)?;
    host::graphics_set_clip(r, graphics, 0, 0, 36, 14)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 36, 14)?;
    let enemy_image_63 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_63, -1, -152, 0)?;
    let enemy_behavior_state_25 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_25 == 1 {
let enemy_animation_35 = r.get(enemy, field::enemy::ANIMATION)?.i();
host::graphics_translate(r, graphics, -9, (-40_i32).wrapping_sub(enemy_animation_35.wrapping_shr((3) as u32)))?;
host::graphics_set_clip(r, graphics, 0, 0, 47, 46)?;
host::graphics_clip_rect(r, graphics, 0, 0, 47, 46)?;
let enemy_image_68 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_68, -57, -1, 0)?;
}
    let enemy_behavior_state_29 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_29 == 4 { break 'check_enemy_animation_2; }
    } // check_enemy_state_z_9
    let enemy_behavior_state_34 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_34 != 7 { break 'check_enemy_state_z_13; }
    } // check_enemy_animation_2
    let enemy_animation_38 = r.get(enemy, field::enemy::ANIMATION)?.i();
    host::graphics_translate(r, graphics, -9, (-40_i32).wrapping_sub(enemy_animation_38.wrapping_shr((3) as u32)))?;
    host::graphics_set_clip(r, graphics, 0, 0, 51, 48)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 51, 48)?;
    let enemy_image_76 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_76, -105, 0, 0)?;
    } // check_enemy_state_z_13
    let enemy_behavior_state_40 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_40 == 2 { break 'graphics_set_clip; }
    } // check_enemy_state_z_17
    let enemy_behavior_state_46 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_46 != 5 { break 'check_enemy_state_z_21; }
    } // graphics_set_clip
    host::graphics_set_clip(r, graphics, 0, 0, 43, 48)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 43, 48)?;
    let enemy_image_92 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_92, -157, 0, 0)?;
    } // check_enemy_state_z_21
    let enemy_behavior_state_52 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_52 == 8 {
let enemy_animation_66 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_18 = arithmetic::remainder_int(r, enemy_animation_66, 2)?;
if remainder_value_18 == 1 {
return Ok(());
}
host::graphics_translate(r, graphics, 9, 50)?;
host::graphics_set_clip(r, graphics, 0, 0, 36, 14)?;
host::graphics_clip_rect(r, graphics, 0, 0, 36, 14)?;
let enemy_image_129 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_129, -1, -152, 0)?;
host::graphics_translate(r, graphics, 0, -10)?;
host::graphics_translate(r, graphics, -9, -50)?;
host::graphics_set_clip(r, graphics, 0, 0, 51, 64)?;
host::graphics_clip_rect(r, graphics, 0, 0, 51, 64)?;
let enemy_image_130 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_130, -114, -49, 0)?;
}
    let enemy_behavior_state_59 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_59 == 9 {
host::graphics_translate(r, graphics, 9, 50)?;
host::graphics_set_clip(r, graphics, 0, 0, 36, 14)?;
host::graphics_clip_rect(r, graphics, 0, 0, 36, 14)?;
let enemy_image_127 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_127, -1, -152, 0)?;
host::graphics_translate(r, graphics, -9, -50)?;
host::graphics_set_clip(r, graphics, 0, 0, 54, 57)?;
host::graphics_clip_rect(r, graphics, 0, 0, 54, 57)?;
let enemy_image_128 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_128, -112, -114, 0)?;
}
    let enemy_behavior_state_66 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_66 == 10 {
host::graphics_translate(r, graphics, 9, 50)?;
host::graphics_set_clip(r, graphics, 0, 0, 36, 14)?;
host::graphics_clip_rect(r, graphics, 0, 0, 36, 14)?;
let enemy_image_143 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_143, -1, -152, 0)?;
host::graphics_translate(r, graphics, -9, -50)?;
host::graphics_set_clip(r, graphics, 0, 0, 48, 69)?;
host::graphics_clip_rect(r, graphics, 0, 0, 48, 69)?;
let enemy_image_144 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_144, -167, -70, 0)?;
host::graphics_translate(r, graphics, 48, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 28, 69)?;
host::graphics_clip_rect(r, graphics, 0, 0, 28, 69)?;
let enemy_image_145 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_145, -201, 0, 0)?;
}
    let enemy_behavior_state_70 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_70 == 20 {
let enemy_hurt_timer_2 = r.get(enemy, field::enemy::HURT_TIMER)?.i();
let remainder_value_30 = arithmetic::remainder_int(r, enemy_hurt_timer_2, 2)?;
if remainder_value_30 != 1 {
host::graphics_set_clip(r, graphics, 0, 0, 56, 53)?;
host::graphics_clip_rect(r, graphics, 0, 0, 56, 53)?;
let enemy_image_165 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_165, 0, -49, 0)?;
} else {
host::graphics_set_clip(r, graphics, 0, 0, 56, 53)?;
host::graphics_clip_rect(r, graphics, 0, 0, 56, 53)?;
let enemy_image_166 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_166, -57, -49, 0)?;
}
}
    let enemy_behavior_state_74 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_74 == 21 {
let enemy_animation_94 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_32 = arithmetic::remainder_int(r, enemy_animation_94, 3)?;
if remainder_value_32 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 55, 48)?;
host::graphics_clip_rect(r, graphics, 0, 0, 55, 48)?;
let enemy_image_175 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_175, 0, -103, 0)?;
} else {
host::graphics_set_clip(r, graphics, 0, 0, 55, 48)?;
host::graphics_clip_rect(r, graphics, 0, 0, 55, 48)?;
let enemy_image_176 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_176, -56, -103, 0)?;
}
}
    } // check_enemy_kind_5
    let enemy_kind_43 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_43 == 25 {
let graphics_get_translate_x_value_12 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_12 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_12.wrapping_neg(), graphics_get_translate_y_value_12.wrapping_neg())?;
let drawable_x_14 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_14 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_14.wrapping_sub(argument_2), drawable_y_14.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 38, 33)?;
host::graphics_clip_rect(r, graphics, 0, 0, 38, 33)?;
argument_4 = 0;
argument_5 = 0;
let enemy_facing_23 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_23 == 0 {
argument_4 = 0;
argument_5 = -16;
}
let enemy_facing_26 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_26 == 1 {
argument_4 = -16;
argument_5 = 0;
}
let enemy_facing_32 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_32 == 2 {
argument_4 = 0;
argument_5 = 16;
}
let enemy_facing_38 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_38 == 3 {
argument_4 = 16;
argument_5 = 0;
}
host::graphics_translate(r, graphics, 32, 32)?;
let enemy_animation_43 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_43 > 3 {
let enemy_animation_50 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_50 < 10 {
host::graphics_translate(r, graphics, argument_4, argument_5)?;
}
}
host::graphics_set_clip(r, graphics, 0, 0, 38, 33)?;
host::graphics_clip_rect(r, graphics, 0, 0, 38, 33)?;
let enemy_image_91 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_91, 64, 0, 38, 33, 0, 0, 0, 0)?;
let graphics_get_translate_x_value_15 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_15 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_15.wrapping_neg(), graphics_get_translate_y_value_15.wrapping_neg())?;
let drawable_x_17 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_17 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_17.wrapping_sub(argument_2), drawable_y_17.wrapping_sub(argument_3))?;
let enemy_animation_49 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_49 > 1 {
let enemy_animation_58 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_58 < 5 {
host::graphics_translate(r, graphics, argument_4, argument_5)?;
}
}
host::graphics_translate(r, graphics, 0, 32)?;
host::graphics_set_clip(r, graphics, 0, 0, 33, 38)?;
host::graphics_clip_rect(r, graphics, 0, 0, 33, 38)?;
let enemy_image_106 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_106, 64, 0, 38, 33, 5, 0, 0, 0)?;
let graphics_get_translate_x_value_17 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_17 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_17.wrapping_neg(), graphics_get_translate_y_value_17.wrapping_neg())?;
let drawable_x_19 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_19 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_19.wrapping_sub(argument_2), drawable_y_19.wrapping_sub(argument_3))?;
let enemy_animation_57 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_57 > 7 {
let enemy_animation_65 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_65 < 12 {
host::graphics_translate(r, graphics, argument_4, argument_5)?;
}
}
host::graphics_translate(r, graphics, 32, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 38, 33)?;
host::graphics_clip_rect(r, graphics, 0, 0, 38, 33)?;
let enemy_image_116 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_116, 64, 0, 38, 33, 6, 0, 0, 0)?;
let graphics_get_translate_x_value_20 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_20 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_20.wrapping_neg(), graphics_get_translate_y_value_20.wrapping_neg())?;
let drawable_x_22 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_22 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_22.wrapping_sub(argument_2), drawable_y_22.wrapping_sub(argument_3))?;
let enemy_animation_64 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_64 > 8 {
let enemy_animation_74 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_74 < 15 {
host::graphics_translate(r, graphics, argument_4, argument_5)?;
}
}
host::graphics_translate(r, graphics, 0, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 33, 38)?;
host::graphics_clip_rect(r, graphics, 0, 0, 33, 38)?;
let enemy_image_126 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_126, 64, 0, 38, 33, 3, 0, 0, 0)?;
let graphics_get_translate_x_value_23 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_23 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_23.wrapping_neg(), graphics_get_translate_y_value_23.wrapping_neg())?;
let drawable_x_25 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_25 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_25.wrapping_sub(argument_2), drawable_y_25.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, 0, 0)?;
let enemy_facing_55 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_55 == 0 {
let enemy_animation_86 = r.get(enemy, field::enemy::ANIMATION)?.i();
let math_abs_value_3 = host::math_abs(r, enemy_animation_86.wrapping_sub(7))?;
host::graphics_translate(r, graphics, 12, (-16_i32).wrapping_sub(math_abs_value_3))?;
host::graphics_set_clip(r, graphics, 0, 0, 42, 46)?;
host::graphics_clip_rect(r, graphics, 0, 0, 42, 46)?;
let enemy_image_142 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_142, 64, 33, 42, 46, 3, 0, 0, 0)?;
}
let enemy_facing_57 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_57 == 1 {
let enemy_animation_90 = r.get(enemy, field::enemy::ANIMATION)?.i();
let math_abs_value_4 = host::math_abs(r, enemy_animation_90.wrapping_sub(7))?;
host::graphics_translate(r, graphics, (32_i32).wrapping_add(math_abs_value_4), 12)?;
host::graphics_set_clip(r, graphics, 0, 0, 46, 42)?;
host::graphics_clip_rect(r, graphics, 0, 0, 46, 42)?;
let enemy_image_155 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_155, 64, 33, 42, 46, 6, 0, 0, 0)?;
}
let enemy_facing_59 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_59 == 2 {
let enemy_animation_93 = r.get(enemy, field::enemy::ANIMATION)?.i();
let math_abs_value_5 = host::math_abs(r, enemy_animation_93.wrapping_sub(7))?;
host::graphics_translate(r, graphics, 12, (32_i32).wrapping_add(math_abs_value_5))?;
host::graphics_set_clip(r, graphics, 0, 0, 42, 46)?;
host::graphics_clip_rect(r, graphics, 0, 0, 42, 46)?;
let enemy_image_164 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_164, 64, 33, 42, 46, 0, 0, 0, 0)?;
}
let enemy_facing_61 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_61 == 3 {
let enemy_animation_98 = r.get(enemy, field::enemy::ANIMATION)?.i();
let math_abs_value_6 = host::math_abs(r, enemy_animation_98.wrapping_sub(7))?;
host::graphics_translate(r, graphics, (-32_i32).wrapping_sub(math_abs_value_6), 12)?;
host::graphics_set_clip(r, graphics, 0, 0, 46, 42)?;
host::graphics_clip_rect(r, graphics, 0, 0, 46, 42)?;
let enemy_image_174 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_174, 64, 33, 42, 46, 5, 0, 0, 0)?;
}
let graphics_get_translate_x_value_33 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_33 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_33.wrapping_neg(), graphics_get_translate_y_value_33.wrapping_neg())?;
let drawable_x_35 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_35 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_35.wrapping_sub(argument_2), drawable_y_35.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, 0, 3)?;
host::graphics_set_clip(r, graphics, 0, 0, 64, 64)?;
host::graphics_clip_rect(r, graphics, 0, 0, 64, 64)?;
let enemy_facing_64 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_64 == 0 {
let enemy_image_189 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_189, 0, 0, 64, 64, 3, 0, 0, 0)?;
}
let enemy_facing_68 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_68 == 1 {
let enemy_image_199 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_199, 0, 0, 64, 64, 6, 0, 0, 0)?;
}
let enemy_facing_72 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_72 == 2 {
let enemy_image_210 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_210, 0, 0, 64, 64, 0, 0, 0, 0)?;
}
let enemy_facing_77 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_77 == 3 {
let enemy_image_221 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_221, 0, 0, 64, 64, 5, 0, 0, 0)?;
}
}
    let enemy_kind_44 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_44 == 26 {
let enemy_behavior_state_24 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_24 == 0 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_facing_31 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_31 == 0 {
let enemy_image_75 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_75, -16, 0, 0)?;
}
let enemy_facing_37 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_37 == 1 {
let enemy_image_82 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_82, -16, -16, 0)?;
}
let enemy_facing_42 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_42 == 2 {
let enemy_image_90 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_90, -16, -16, 0)?;
}
let enemy_facing_47 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_47 == 3 {
let enemy_image_105 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_105, -16, 0, 0)?;
}
}
let enemy_behavior_state_28 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_28 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_facing_36 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_36 == 0 {
let enemy_image_81 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_81, -16, 0, 0)?;
}
let enemy_facing_41 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_41 == 1 {
let enemy_image_89 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_89, -16, -16, 0)?;
}
let enemy_facing_46 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_46 == 2 {
let enemy_image_104 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_104, -16, -16, 0)?;
}
let enemy_facing_51 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_51 == 3 {
let enemy_image_115 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_115, -16, 0, 0)?;
}
}
let enemy_behavior_state_33 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
if enemy_behavior_state_33 == 2 {
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let enemy_facing_40 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_40 == 0 {
let enemy_image_88 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_48 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_13 = arithmetic::remainder_int(r, enemy_animation_48, 2)?;
host::graphics_draw_image(r, graphics, enemy_image_88, (-16_i32).wrapping_mul(remainder_value_13), 0, 0)?;
}
let enemy_facing_45 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_45 == 1 {
let enemy_image_103 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_56 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_15 = arithmetic::remainder_int(r, enemy_animation_56, 2)?;
host::graphics_draw_image(r, graphics, enemy_image_103, (-16_i32).wrapping_mul(remainder_value_15), -16, 0)?;
}
let enemy_facing_50 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_50 == 2 {
let enemy_image_114 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_63 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_17 = arithmetic::remainder_int(r, enemy_animation_63, 2)?;
host::graphics_draw_image(r, graphics, enemy_image_114, (-16_i32).wrapping_mul(remainder_value_17), -16, 0)?;
}
let enemy_facing_53 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_53 == 3 {
let enemy_image_125 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_73 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_22 = arithmetic::remainder_int(r, enemy_animation_73, 2)?;
host::graphics_draw_image(r, graphics, enemy_image_125, (-16_i32).wrapping_mul(remainder_value_22), 0, 0)?;
}
}
}
    let enemy_kind_45 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_45 == 27 {
let enemy_animation_34 = r.get(enemy, field::enemy::ANIMATION)?.i();
if enemy_animation_34 >= 15 {
r.set(enemy, field::enemy::ANIMATION, V::I(0))?;
}
let enemy_facing_35 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_35 == 0 {
let enemy_animation_42 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value = arithmetic::divide_int(r, enemy_animation_42, 5)?;
if quotient_value == 0 {
host::graphics_set_clip(r, graphics, 0, 0, 22, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 26)?;
let enemy_image_87 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_87, -55, 0, 0)?;
}
let enemy_animation_47 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_3 = arithmetic::divide_int(r, enemy_animation_47, 5)?;
if quotient_value_3 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 22, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 26)?;
let enemy_image_102 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_102, -55, -34, 0)?;
}
let enemy_animation_55 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_6 = arithmetic::divide_int(r, enemy_animation_55, 5)?;
if quotient_value_6 == 2 {
host::graphics_set_clip(r, graphics, 0, 0, 22, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 26)?;
let enemy_image_113 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_113, -55, -67, 0)?;
}
}
let enemy_facing_39 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_39 == 1 {
let enemy_animation_46 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_2 = arithmetic::divide_int(r, enemy_animation_46, 5)?;
if quotient_value_2 == 0 {
host::graphics_set_clip(r, graphics, 0, 0, 29, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 29, 26)?;
let enemy_image_101 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_101, 25, 0, 29, 26, 2, 0, 0, 0)?;
}
let enemy_animation_54 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_5 = arithmetic::divide_int(r, enemy_animation_54, 5)?;
if quotient_value_5 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 29, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 29, 27)?;
let enemy_image_112 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_112, 25, 32, 29, 27, 2, 0, 0, 0)?;
}
let enemy_animation_62 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_10 = arithmetic::divide_int(r, enemy_animation_62, 5)?;
if quotient_value_10 == 2 {
host::graphics_set_clip(r, graphics, 0, 0, 32, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 32, 27)?;
let enemy_image_124 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_124, 22, 65, 32, 27, 2, 0, 0, 0)?;
}
}
let enemy_facing_44 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_44 == 2 {
let enemy_animation_53 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_4 = arithmetic::divide_int(r, enemy_animation_53, 5)?;
if quotient_value_4 == 0 {
host::graphics_set_clip(r, graphics, 0, 0, 22, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 36)?;
let enemy_image_111 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_111, 0, 0, 0)?;
}
let enemy_animation_61 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_9 = arithmetic::divide_int(r, enemy_animation_61, 5)?;
if quotient_value_9 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 22, 33)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 33)?;
let enemy_image_123 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_123, 0, -36, 0)?;
}
let enemy_animation_72 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_13 = arithmetic::divide_int(r, enemy_animation_72, 5)?;
if quotient_value_13 == 2 {
host::graphics_set_clip(r, graphics, 0, 0, 22, 34)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 34)?;
let enemy_image_141 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_141, 0, -69, 0)?;
}
}
let enemy_facing_49 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_49 == 3 {
let enemy_animation_60 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_8 = arithmetic::divide_int(r, enemy_animation_60, 5)?;
if quotient_value_8 == 0 {
host::graphics_set_clip(r, graphics, 0, 0, 29, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 29, 26)?;
let enemy_image_122 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_122, 25, 0, 29, 26, 0, 0, 0, 0)?;
}
let enemy_animation_71 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_12 = arithmetic::divide_int(r, enemy_animation_71, 5)?;
if quotient_value_12 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 29, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 29, 27)?;
let enemy_image_140 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_140, 25, 32, 29, 27, 0, 0, 0, 0)?;
}
let enemy_animation_85 = r.get(enemy, field::enemy::ANIMATION)?.i();
let quotient_value_16 = arithmetic::divide_int(r, enemy_animation_85, 5)?;
if quotient_value_16 == 2 {
host::graphics_set_clip(r, graphics, 0, 0, 32, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 32, 27)?;
let enemy_image_154 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_154, 22, 65, 32, 27, 0, 0, 0, 0)?;
}
}
}
    let enemy_kind_46 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_46 != 28 { break 'check_enemy_kind_6; }
    } // graphics_get_translate_x_4
    let graphics_get_translate_x_value_13 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_13 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_13.wrapping_neg(), graphics_get_translate_y_value_13.wrapping_neg())?;
    let drawable_x_15 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_15 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_15.wrapping_sub(argument_2), drawable_y_15.wrapping_sub(argument_3))?;
    let enemy_behavior_state_32 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_32 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 32, 25)?;
host::graphics_clip_rect(r, graphics, 0, 0, 32, 25)?;
let enemy_animation_41 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_10 = arithmetic::remainder_int(r, enemy_animation_41.wrapping_shr((1) as u32), 2)?;
if remainder_value_10 != 0 {
let enemy_image_85 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_85, 0, 0, 32, 25, 2, 0, 0, 0)?;
} else {
let enemy_image_86 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_86, 0, 0, 32, 25, 0, 0, 0, 0)?;
}
}
    let enemy_behavior_state_39 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_39 == 2 {
let enemy_animation_45 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_12 = arithmetic::remainder_int(r, enemy_animation_45.wrapping_shr((1) as u32), 2)?;
if remainder_value_12 != 0 {
host::graphics_set_clip(r, graphics, 0, 0, 39, 25)?;
host::graphics_clip_rect(r, graphics, 0, 0, 39, 25)?;
let enemy_image_99 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_99, -68, 0, 0)?;
} else {
host::graphics_set_clip(r, graphics, 0, 0, 36, 25)?;
host::graphics_clip_rect(r, graphics, 0, 0, 36, 25)?;
let enemy_image_100 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_100, -32, 0, 0)?;
}
}
    let enemy_behavior_state_45 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_45 == 3 {
host::graphics_set_clip(r, graphics, 0, 0, 18, 23)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 23)?;
let enemy_image_97 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_97, -41, -27, 0)?;
let graphics_get_translate_x_value_16 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_16 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_16.wrapping_neg(), graphics_get_translate_y_value_16.wrapping_neg())?;
let drawable_x_18 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_18 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_18.wrapping_sub(argument_2), drawable_y_18.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, -10, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
let enemy_image_98 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_98, -59, -27, 0)?;
}
    let enemy_behavior_state_51 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_51 != 4 { break 'check_enemy_state_z_24; }
    } // graphics_get_translate_x_6
    let graphics_get_translate_x_value_19 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_19 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_19.wrapping_neg(), graphics_get_translate_y_value_19.wrapping_neg())?;
    let drawable_x_21 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_21 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_21.wrapping_sub(argument_2), drawable_y_21.wrapping_sub(argument_3))?;
    let enemy_state_timer_7 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
    let quotient_value_7 = arithmetic::divide_int(r, enemy_state_timer_7, 5)?;
    argument_4 = quotient_value_7;
    argument_5 = 0;
    } // next_step_4
    'draw_loop_48: loop {
        'graphics_set_clip_2: {
        if argument_5 >= argument_4 { break 'graphics_translate_2; }
        } // graphics_set_clip_2
        host::graphics_set_clip(r, graphics, 0, 0, 18, 18)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 18, 18)?;
        let enemy_image_139 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_139, -20, -33, 0)?;
        if argument_5 == 0 {
host::graphics_translate(r, graphics, -10, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
let enemy_image_153 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_153, -59, -27, 0)?;
host::graphics_translate(r, graphics, 10, 0)?;
let enemy_state_timer_11 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
let remainder_value_29 = arithmetic::remainder_int(r, enemy_state_timer_11, 5)?;
host::graphics_translate(r, graphics, 0, (10_i32).wrapping_sub(remainder_value_29.wrapping_shl((1) as u32)))?;
}
        host::graphics_translate(r, graphics, 0, -10)?;
        argument_5 = argument_5.wrapping_add(1);
        continue 'draw_loop_48;
    }
    } // graphics_translate_2
    host::graphics_translate(r, graphics, 0, -5)?;
    host::graphics_set_clip(r, graphics, 0, 0, 20, 21)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 20, 21)?;
    let enemy_image_137 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_137, 0, -29, 0)?;
    let graphics_get_translate_x_value_26 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_26 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_26.wrapping_neg(), graphics_get_translate_y_value_26.wrapping_neg())?;
    let drawable_x_28 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_28 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_28.wrapping_sub(argument_2), drawable_y_28.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -10, 0)?;
    host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
    let enemy_image_138 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_138, -59, -27, 0)?;
    } // check_enemy_state_z_24
    let enemy_behavior_state_58 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_58 != 5 { break 'check_enemy_state_z_27; }
    } // graphics_get_translate_x_8
    let graphics_get_translate_x_value_22 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_22 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_22.wrapping_neg(), graphics_get_translate_y_value_22.wrapping_neg())?;
    let drawable_x_24 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_24 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_24.wrapping_sub(argument_2), drawable_y_24.wrapping_sub(argument_3))?;
    argument_4 = 0;
    let enemy_state_g_2 = r.get(enemy, field::enemy::STATE_G)?.i();
    let enemy_hit_count = r.get(enemy, field::enemy::HIT_COUNT)?.i();
    r.set(enemy, field::enemy::STATE_G, V::I(enemy_state_g_2.wrapping_add(enemy_hit_count)))?;
    let enemy_state_g_3 = r.get(enemy, field::enemy::STATE_G)?.i();
    if enemy_state_g_3 > 10 {
r.set(enemy, field::enemy::HIT_COUNT, V::I(-1))?;
r.set(enemy, field::enemy::STATE_G, V::I(10))?;
}
    let enemy_state_g_6 = r.get(enemy, field::enemy::STATE_G)?.i();
    if enemy_state_g_6 < 0 {
r.set(enemy, field::enemy::HIT_COUNT, V::I(1))?;
r.set(enemy, field::enemy::STATE_G, V::I(0))?;
}
    argument_5 = 0;
    argument_4 = 0;
    } // next_step_8
    'draw_loop_53: loop {
        'graphics_set_clip_6: {
        if argument_4 >= 5 { break 'check_enemy_facing_3; }
        } // graphics_set_clip_6
        host::graphics_set_clip(r, graphics, 0, 0, 18, 18)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 18, 18)?;
        let enemy_image_173 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_173, -20, -33, 0)?;
        if argument_4 == 0 {
host::graphics_translate(r, graphics, -10, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
let enemy_image_188 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_188, -59, -27, 0)?;
host::graphics_translate(r, graphics, 10, 0)?;
}
        let enemy_state_v_3 = r.get(enemy, field::enemy::STATE_V)?.r();
        let element_value_4 = r.array_get(enemy_state_v_3, argument_4)?.i();
        let enemy_state_ab_2 = r.get(enemy, field::enemy::STATE_AB)?.r();
        let element_value_5 = r.array_get(enemy_state_ab_2, argument_4)?.i();
        r.array_set(enemy_state_v_3, argument_4, V::I(element_value_4.wrapping_add(element_value_5)))?;
        let enemy_state_v_4 = r.get(enemy, field::enemy::STATE_V)?.r();
        let element_value_6 = r.array_get(enemy_state_v_4, argument_4)?.i();
        if element_value_6 > 10 {
let enemy_state_v_8 = r.get(enemy, field::enemy::STATE_V)?.r();
r.array_set(enemy_state_v_8, argument_4, V::I(10))?;
let enemy_state_ab_4 = r.get(enemy, field::enemy::STATE_AB)?.r();
r.array_set(enemy_state_ab_4, argument_4, V::I(-1))?;
}
        let enemy_state_v_7 = r.get(enemy, field::enemy::STATE_V)?.r();
        let element_value_8 = r.array_get(enemy_state_v_7, argument_4)?.i();
        if element_value_8 < 0 {
let enemy_state_v_12 = r.get(enemy, field::enemy::STATE_V)?.r();
r.array_set(enemy_state_v_12, argument_4, V::I(0))?;
let enemy_state_ab_6 = r.get(enemy, field::enemy::STATE_AB)?.r();
r.array_set(enemy_state_ab_6, argument_4, V::I(1))?;
}
        let enemy_state_v_11 = r.get(enemy, field::enemy::STATE_V)?.r();
        let element_value_10 = r.array_get(enemy_state_v_11, argument_4)?.i();
        host::graphics_translate(r, graphics, (5_i32).wrapping_sub(element_value_10), -10)?;
        argument_4 = argument_4.wrapping_add(1);
        continue 'draw_loop_53;
    }
    } // check_enemy_facing_3
    let enemy_facing_63 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_63 == 0 {
host::graphics_translate(r, graphics, 0, -5)?;
host::graphics_set_clip(r, graphics, 0, 0, 20, 21)?;
host::graphics_clip_rect(r, graphics, 0, 0, 20, 21)?;
let enemy_image_187 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_187, 0, 29, 20, 21, 0, 0, 0, 0)?;
}
    let enemy_facing_67 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_67 == 1 {
host::graphics_translate(r, graphics, 0, -5)?;
host::graphics_set_clip(r, graphics, 0, 0, 20, 21)?;
host::graphics_clip_rect(r, graphics, 0, 0, 20, 21)?;
let enemy_image_198 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_198, 0, 29, 20, 21, 6, 0, 0, 0)?;
}
    let enemy_facing_71 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_71 == 2 {
host::graphics_translate(r, graphics, 0, -5)?;
host::graphics_set_clip(r, graphics, 0, 0, 20, 21)?;
host::graphics_clip_rect(r, graphics, 0, 0, 20, 21)?;
let enemy_image_209 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_209, 0, 29, 20, 21, 0, 0, 0, 0)?;
}
    let enemy_facing_76 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_76 == 3 {
host::graphics_translate(r, graphics, 0, -5)?;
host::graphics_set_clip(r, graphics, 0, 0, 20, 21)?;
host::graphics_clip_rect(r, graphics, 0, 0, 20, 21)?;
let enemy_image_220 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_220, 0, 29, 20, 21, 5, 0, 0, 0)?;
}
    } // check_enemy_state_z_27
    let enemy_behavior_state_65 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_65 != 6 { break 'check_enemy_kind_6; }
    } // graphics_get_translate_x_10
    let graphics_get_translate_x_value_25 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_25 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_25.wrapping_neg(), graphics_get_translate_y_value_25.wrapping_neg())?;
    let drawable_x_27 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_27 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_27.wrapping_sub(argument_2), drawable_y_27.wrapping_sub(argument_3))?;
    let enemy_state_timer_9 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
    let quotient_value_15 = arithmetic::divide_int(r, enemy_state_timer_9, 5)?;
    argument_4 = quotient_value_15;
    argument_5 = 0;
    } // next_step_6
    'draw_loop_58: loop {
        'graphics_set_clip_4: {
        if argument_5 >= argument_4 { break 'graphics_translate_4; }
        } // graphics_set_clip_4
        host::graphics_set_clip(r, graphics, 0, 0, 18, 18)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 18, 18)?;
        let enemy_image_163 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_163, -20, -33, 0)?;
        if argument_5 == 0 {
host::graphics_translate(r, graphics, -10, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
let enemy_image_172 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_172, -59, -27, 0)?;
host::graphics_translate(r, graphics, 10, 0)?;
let enemy_state_timer_13 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
let remainder_value_35 = arithmetic::remainder_int(r, enemy_state_timer_13, 5)?;
host::graphics_translate(r, graphics, 0, (10_i32).wrapping_sub(remainder_value_35.wrapping_shl((1) as u32)))?;
}
        host::graphics_translate(r, graphics, 0, -10)?;
        argument_5 = argument_5.wrapping_add(1);
        continue 'draw_loop_58;
    }
    } // graphics_translate_4
    host::graphics_translate(r, graphics, 0, -5)?;
    host::graphics_set_clip(r, graphics, 0, 0, 20, 21)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 20, 21)?;
    let enemy_image_161 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_161, 0, -29, 0)?;
    let graphics_get_translate_x_value_30 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_30 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_30.wrapping_neg(), graphics_get_translate_y_value_30.wrapping_neg())?;
    let drawable_x_32 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_32 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_32.wrapping_sub(argument_2), drawable_y_32.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -10, 0)?;
    host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
    let enemy_image_162 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_162, -59, -27, 0)?;
    } // check_enemy_kind_6
    let enemy_kind_47 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_47 != 29 { break 'check_enemy_kind_7; }
    } // graphics_get_translate_x_5
    let graphics_get_translate_x_value_14 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_14 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_14.wrapping_neg(), graphics_get_translate_y_value_14.wrapping_neg())?;
    let drawable_x_16 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_16 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_16.wrapping_sub(argument_2), drawable_y_16.wrapping_sub(argument_3))?;
    let enemy_behavior_state_38 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_38 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 43, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 43, 26)?;
let enemy_animation_44 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_11 = arithmetic::remainder_int(r, enemy_animation_44.wrapping_shr((1) as u32), 2)?;
if remainder_value_11 != 0 {
let enemy_image_95 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_95, 0, 30, 43, 26, 2, 0, 0, 0)?;
} else {
let enemy_image_96 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_96, 0, 30, 43, 26, 0, 0, 0, 0)?;
}
}
    let enemy_behavior_state_44 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_44 == 2 {
let enemy_animation_52 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_14 = arithmetic::remainder_int(r, enemy_animation_52.wrapping_shr((1) as u32), 2)?;
if remainder_value_14 != 0 {
host::graphics_set_clip(r, graphics, 0, 0, 50, 30)?;
host::graphics_clip_rect(r, graphics, 0, 0, 50, 30)?;
let enemy_image_109 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_109, -50, -27, 0)?;
} else {
host::graphics_set_clip(r, graphics, 0, 0, 50, 30)?;
host::graphics_clip_rect(r, graphics, 0, 0, 50, 30)?;
let enemy_image_110 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_110, 0, -27, 0)?;
}
}
    let enemy_behavior_state_50 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_50 == 3 {
host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
let enemy_image_107 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_107, -33, -58, 0)?;
let graphics_get_translate_x_value_18 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_18 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_18.wrapping_neg(), graphics_get_translate_y_value_18.wrapping_neg())?;
let drawable_x_20 = r.get(enemy, field::drawable::X)?.i();
let drawable_y_20 = r.get(enemy, field::drawable::Y)?.i();
host::graphics_translate(r, graphics, drawable_x_20.wrapping_sub(argument_2), drawable_y_20.wrapping_sub(argument_3))?;
host::graphics_translate(r, graphics, -6, 20)?;
host::graphics_set_clip(r, graphics, 0, 0, 53, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 53, 26)?;
let enemy_image_108 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_108, 0, 0, 0)?;
}
    let enemy_behavior_state_57 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_57 != 4 { break 'check_enemy_state_z_26; }
    } // graphics_get_translate_x_7
    let graphics_get_translate_x_value_21 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_21 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_21.wrapping_neg(), graphics_get_translate_y_value_21.wrapping_neg())?;
    let drawable_x_23 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_23 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_23.wrapping_sub(argument_2), drawable_y_23.wrapping_sub(argument_3))?;
    let enemy_state_timer_8 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
    let quotient_value_11 = arithmetic::divide_int(r, enemy_state_timer_8, 5)?;
    argument_4 = quotient_value_11;
    argument_5 = 0;
    } // next_step_5
    'draw_loop_64: loop {
        'graphics_set_clip_3: {
        if argument_5 >= argument_4 { break 'graphics_translate_3; }
        } // graphics_set_clip_3
        host::graphics_set_clip(r, graphics, 0, 0, 32, 35)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 35)?;
        let enemy_image_152 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_152, 0, -66, 0)?;
        if argument_5 == 0 {
let enemy_state_timer_12 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
let remainder_value_31 = arithmetic::remainder_int(r, enemy_state_timer_12, 5)?;
host::graphics_translate(r, graphics, 0, (10_i32).wrapping_sub(remainder_value_31.wrapping_shl((1) as u32)))?;
}
        host::graphics_translate(r, graphics, 0, -10)?;
        argument_5 = argument_5.wrapping_add(1);
        continue 'draw_loop_64;
    }
    } // graphics_translate_3
    host::graphics_translate(r, graphics, -6, -5)?;
    host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
    let enemy_image_150 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_150, -33, -58, 0)?;
    host::graphics_translate(r, graphics, 6, 0)?;
    let graphics_get_translate_x_value_28 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_28 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_28.wrapping_neg(), graphics_get_translate_y_value_28.wrapping_neg())?;
    let drawable_x_30 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_30 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_30.wrapping_sub(argument_2), drawable_y_30.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -12, 16)?;
    host::graphics_set_clip(r, graphics, 0, 0, 53, 26)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 53, 26)?;
    let enemy_image_151 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_151, 0, 0, 0)?;
    } // check_enemy_state_z_26
    let enemy_behavior_state_64 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_64 != 5 { break 'check_enemy_state_z_28; }
    } // graphics_get_translate_x_9
    let graphics_get_translate_x_value_24 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_24 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_24.wrapping_neg(), graphics_get_translate_y_value_24.wrapping_neg())?;
    let drawable_x_26 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_26 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_26.wrapping_sub(argument_2), drawable_y_26.wrapping_sub(argument_3))?;
    argument_4 = 0;
    let enemy_state_g_4 = r.get(enemy, field::enemy::STATE_G)?.i();
    let enemy_hit_count_2 = r.get(enemy, field::enemy::HIT_COUNT)?.i();
    r.set(enemy, field::enemy::STATE_G, V::I(enemy_state_g_4.wrapping_add(enemy_hit_count_2)))?;
    let enemy_state_g_5 = r.get(enemy, field::enemy::STATE_G)?.i();
    if enemy_state_g_5 > 10 {
r.set(enemy, field::enemy::HIT_COUNT, V::I(-1))?;
r.set(enemy, field::enemy::STATE_G, V::I(10))?;
}
    let enemy_state_g_7 = r.get(enemy, field::enemy::STATE_G)?.i();
    if enemy_state_g_7 < 0 {
r.set(enemy, field::enemy::HIT_COUNT, V::I(1))?;
r.set(enemy, field::enemy::STATE_G, V::I(0))?;
}
    argument_5 = 0;
    argument_4 = 0;
    } // next_step_10
    'draw_loop_69: loop {
        'graphics_set_clip_8: {
        if argument_4 >= 5 { break 'graphics_translate_8; }
        } // graphics_set_clip_8
        host::graphics_set_clip(r, graphics, 0, 0, 32, 35)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 35)?;
        let enemy_image_186 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_186, 0, -66, 0)?;
        let enemy_state_v = r.get(enemy, field::enemy::STATE_V)?.r();
        let element_value = r.array_get(enemy_state_v, argument_4)?.i();
        let enemy_state_ab = r.get(enemy, field::enemy::STATE_AB)?.r();
        let element_value_2 = r.array_get(enemy_state_ab, argument_4)?.i();
        r.array_set(enemy_state_v, argument_4, V::I(element_value.wrapping_add(element_value_2)))?;
        let enemy_state_v_2 = r.get(enemy, field::enemy::STATE_V)?.r();
        let element_value_3 = r.array_get(enemy_state_v_2, argument_4)?.i();
        if element_value_3 > 10 {
let enemy_state_v_6 = r.get(enemy, field::enemy::STATE_V)?.r();
r.array_set(enemy_state_v_6, argument_4, V::I(10))?;
let enemy_state_ab_3 = r.get(enemy, field::enemy::STATE_AB)?.r();
r.array_set(enemy_state_ab_3, argument_4, V::I(-1))?;
}
        let enemy_state_v_5 = r.get(enemy, field::enemy::STATE_V)?.r();
        let element_value_7 = r.array_get(enemy_state_v_5, argument_4)?.i();
        if element_value_7 < 0 {
let enemy_state_v_10 = r.get(enemy, field::enemy::STATE_V)?.r();
r.array_set(enemy_state_v_10, argument_4, V::I(0))?;
let enemy_state_ab_5 = r.get(enemy, field::enemy::STATE_AB)?.r();
r.array_set(enemy_state_ab_5, argument_4, V::I(1))?;
}
        let enemy_state_v_9 = r.get(enemy, field::enemy::STATE_V)?.r();
        let element_value_9 = r.array_get(enemy_state_v_9, argument_4)?.i();
        host::graphics_translate(r, graphics, (5_i32).wrapping_sub(element_value_9), -10)?;
        argument_4 = argument_4.wrapping_add(1);
        continue 'draw_loop_69;
    }
    } // graphics_translate_8
    host::graphics_translate(r, graphics, -6, -5)?;
    host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
    let enemy_image_185 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_185, -33, -58, 0)?;
    host::graphics_translate(r, graphics, 6, 0)?;
    let graphics_get_translate_x_value_39 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_39 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_39.wrapping_neg(), graphics_get_translate_y_value_39.wrapping_neg())?;
    let drawable_x_40 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_40 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_40.wrapping_sub(argument_2), drawable_y_40.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -12, 16)?;
    host::graphics_set_clip(r, graphics, 0, 0, 53, 26)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 53, 26)?;
    let enemy_animation_102 = r.get(enemy, field::enemy::ANIMATION)?.i();
    if !(enemy_animation_102.wrapping_shr((3) as u32) != 0) {
let enemy_image_197 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_197, 0, 0, 0)?;
}
    let enemy_animation_105 = r.get(enemy, field::enemy::ANIMATION)?.i();
    if !(enemy_animation_105.wrapping_shr((3) as u32) != 1) {
let enemy_image_208 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_208, -53, 0, 0)?;
}
    let enemy_animation_107 = r.get(enemy, field::enemy::ANIMATION)?.i();
    if !(enemy_animation_107.wrapping_shr((3) as u32) != 2) {
let enemy_image_219 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_219, -106, 0, 0)?;
}
    } // check_enemy_state_z_28
    let enemy_behavior_state_69 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_69 != 6 { break 'check_enemy_state_z_29; }
    } // graphics_get_translate_x_11
    let graphics_get_translate_x_value_27 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_27 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_27.wrapping_neg(), graphics_get_translate_y_value_27.wrapping_neg())?;
    let drawable_x_29 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_29 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_29.wrapping_sub(argument_2), drawable_y_29.wrapping_sub(argument_3))?;
    let enemy_state_timer_10 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
    let quotient_value_17 = arithmetic::divide_int(r, enemy_state_timer_10, 5)?;
    argument_4 = quotient_value_17;
    argument_5 = 0;
    } // next_step_7
    'draw_loop_74: loop {
        'graphics_set_clip_5: {
        if argument_5 >= argument_4 { break 'graphics_translate_5; }
        } // graphics_set_clip_5
        host::graphics_set_clip(r, graphics, 0, 0, 32, 35)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 35)?;
        let enemy_image_171 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_171, 0, -66, 0)?;
        if argument_5 == 0 {
let enemy_state_timer_14 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
let remainder_value_37 = arithmetic::remainder_int(r, enemy_state_timer_14, 5)?;
host::graphics_translate(r, graphics, 0, (10_i32).wrapping_sub(remainder_value_37.wrapping_shl((1) as u32)))?;
}
        host::graphics_translate(r, graphics, 0, -10)?;
        argument_5 = argument_5.wrapping_add(1);
        continue 'draw_loop_74;
    }
    } // graphics_translate_5
    host::graphics_translate(r, graphics, -6, -5)?;
    host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
    let enemy_image_169 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_169, -33, -58, 0)?;
    host::graphics_translate(r, graphics, 6, 0)?;
    let graphics_get_translate_x_value_32 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_32 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_32.wrapping_neg(), graphics_get_translate_y_value_32.wrapping_neg())?;
    let drawable_x_34 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_34 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_34.wrapping_sub(argument_2), drawable_y_34.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -6, 16)?;
    host::graphics_set_clip(r, graphics, 0, 0, 53, 26)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 53, 26)?;
    let enemy_image_170 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_170, 0, 0, 0)?;
    } // check_enemy_state_z_29
    let enemy_behavior_state_73 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_73 != 7 { break 'check_enemy_state_z_30; }
    } // graphics_get_translate_x_12
    let graphics_get_translate_x_value_29 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_29 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_29.wrapping_neg(), graphics_get_translate_y_value_29.wrapping_neg())?;
    let drawable_x_31 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_31 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_31.wrapping_sub(argument_2), drawable_y_31.wrapping_sub(argument_3))?;
    argument_4 = 0;
    argument_5 = 0;
    } // next_step_9
    'draw_loop_79: loop {
        'graphics_set_clip_7: {
        if argument_5 >= 4 { break 'graphics_translate_7; }
        } // graphics_set_clip_7
        host::graphics_set_clip(r, graphics, 0, 0, 32, 35)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 35)?;
        let enemy_image_184 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_184, 0, -66, 0)?;
        if argument_5 == 0 {
host::graphics_translate(r, graphics, 0, 2)?;
}
        host::graphics_translate(r, graphics, 0, -10)?;
        argument_5 = argument_5.wrapping_add(1);
        continue 'draw_loop_79;
    }
    } // graphics_translate_7
    host::graphics_translate(r, graphics, -6, -5)?;
    host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
    let enemy_image_182 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_182, -33, -58, 0)?;
    host::graphics_translate(r, graphics, 6, 0)?;
    let graphics_get_translate_x_value_38 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_38 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_38.wrapping_neg(), graphics_get_translate_y_value_38.wrapping_neg())?;
    let drawable_x_39 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_39 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_39.wrapping_sub(argument_2), drawable_y_39.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -6, 16)?;
    host::graphics_set_clip(r, graphics, 0, 0, 53, 26)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 53, 26)?;
    let enemy_image_183 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_183, 0, 0, 0)?;
    } // check_enemy_state_z_30
    let enemy_behavior_state_77 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_77 != 8 { break 'check_enemy_state_z_32; }
    } // graphics_get_translate_x_13
    let graphics_get_translate_x_value_31 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_31 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_31.wrapping_neg(), graphics_get_translate_y_value_31.wrapping_neg())?;
    let drawable_x_33 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_33 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_33.wrapping_sub(argument_2), drawable_y_33.wrapping_sub(argument_3))?;
    argument_4 = 0;
    argument_5 = 0;
    } // next_step_11
    'draw_loop_84: loop {
        'graphics_set_clip_10: {
        if argument_5 >= 4 { break 'graphics_translate_9; }
        } // graphics_set_clip_10
        host::graphics_set_clip(r, graphics, 0, 0, 32, 35)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 35)?;
        let enemy_image_196 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_196, 0, -66, 0)?;
        if argument_5 == 0 {
host::graphics_translate(r, graphics, 0, 2)?;
}
        host::graphics_translate(r, graphics, 0, -10)?;
        argument_5 = argument_5.wrapping_add(1);
        continue 'draw_loop_84;
    }
    } // graphics_translate_9
    host::graphics_translate(r, graphics, -6, -5)?;
    host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
    let enemy_image_194 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_194, -33, -58, 0)?;
    host::graphics_translate(r, graphics, 6, 0)?;
    let graphics_get_translate_x_value_42 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_42 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_42.wrapping_neg(), graphics_get_translate_y_value_42.wrapping_neg())?;
    let drawable_x_43 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_43 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_43.wrapping_sub(argument_2), drawable_y_43.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -6, 16)?;
    host::graphics_set_clip(r, graphics, 0, 0, 53, 26)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 53, 26)?;
    let enemy_image_195 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_195, 0, 0, 0)?;
    let graphics_get_translate_x_value_43 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_43 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_43.wrapping_neg(), graphics_get_translate_y_value_43.wrapping_neg())?;
    let drawable_x_44 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_44 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_44.wrapping_sub(argument_2), drawable_y_44.wrapping_sub(argument_3))?;
    let enemy_animation_104 = r.get(enemy, field::enemy::ANIMATION)?.i();
    let remainder_value_39 = arithmetic::remainder_int(r, enemy_animation_104, 8)?;
    if remainder_value_39 <= 4 {
let enemy_state_timer_15 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
let quotient_value_18 = arithmetic::divide_int(r, enemy_state_timer_15, 6)?;
host::graphics_translate(r, graphics, 32, quotient_value_18.wrapping_sub(32))?;
host::graphics_set_clip(r, graphics, 0, 0, 22, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 22, 24)?;
let enemy_image_206 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_206, -80, -82, 0)?;
} else {
let enemy_state_timer_16 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
let quotient_value_19 = arithmetic::divide_int(r, enemy_state_timer_16, 6)?;
host::graphics_translate(r, graphics, 32, quotient_value_19.wrapping_sub(32))?;
host::graphics_set_clip(r, graphics, 0, 0, 20, 20)?;
host::graphics_clip_rect(r, graphics, 0, 0, 20, 20)?;
let enemy_image_207 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_207, -80, -62, 0)?;
}
    let enemy_animation_110 = r.get(enemy, field::enemy::ANIMATION)?.i();
    let remainder_value_42 = arithmetic::remainder_int(r, enemy_animation_110, 2)?;
    if remainder_value_42 == 0 {
let graphics_get_translate_x_value_44 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_44 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_44.wrapping_neg(), graphics_get_translate_y_value_44.wrapping_neg())?;
host::graphics_translate(r, graphics, 10, 10)?;
host::graphics_set_clip(r, graphics, 0, 0, 44, 68)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 68)?;
let enemy_image_233 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_233, -159, 0, 0)?;
}
    } // check_enemy_state_z_32
    let enemy_behavior_state_80 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_80 != 9 { break 'check_enemy_state_z_35; }
    } // graphics_get_translate_x_14
    let graphics_get_translate_x_value_36 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_36 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_36.wrapping_neg(), graphics_get_translate_y_value_36.wrapping_neg())?;
    let drawable_x_37 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_37 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_37.wrapping_sub(argument_2), drawable_y_37.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -6, 16)?;
    host::graphics_set_clip(r, graphics, 0, 0, 53, 26)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 53, 26)?;
    let enemy_image_181 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_181, 0, 0, 0)?;
    let graphics_get_translate_x_value_37 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_37 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_37.wrapping_neg(), graphics_get_translate_y_value_37.wrapping_neg())?;
    let drawable_x_38 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_38 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_38.wrapping_sub(argument_2), drawable_y_38.wrapping_sub(argument_3))?;
    argument_4 = 0;
    argument_5 = 0;
    } // next_step_12
    'draw_loop_89: loop {
        'graphics_set_clip_11: {
        if argument_5 >= 4 { break 'graphics_translate_10; }
        } // graphics_set_clip_11
        host::graphics_set_clip(r, graphics, 0, 0, 32, 35)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 35)?;
        let enemy_image_205 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_205, 0, -66, 0)?;
        if argument_5 == 0 {
host::graphics_translate(r, graphics, 0, 2)?;
}
        let enemy_state_timer_17 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
        let quotient_value_20 = arithmetic::divide_int(r, enemy_state_timer_17, 4)?;
        let enemy_state_timer_18 = r.get(enemy, field::enemy::STATE_TIMER)?.i();
        let quotient_value_21 = arithmetic::divide_int(r, enemy_state_timer_18, 2)?;
        host::graphics_translate(r, graphics, (0_i32).wrapping_sub(quotient_value_20), (-10_i32).wrapping_add(quotient_value_21))?;
        argument_5 = argument_5.wrapping_add(1);
        continue 'draw_loop_89;
    }
    } // graphics_translate_10
    host::graphics_translate(r, graphics, -6, -5)?;
    host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
    let enemy_image_204 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_204, -33, -58, 0)?;
    host::graphics_translate(r, graphics, 6, 0)?;
    } // check_enemy_state_z_35
    let enemy_behavior_state_83 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_83 != 10 { break 'check_enemy_kind_7; }
    } // graphics_get_translate_x_15
    let graphics_get_translate_x_value_40 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_40 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_40.wrapping_neg(), graphics_get_translate_y_value_40.wrapping_neg())?;
    let drawable_x_41 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_41 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_41.wrapping_sub(argument_2), drawable_y_41.wrapping_sub(argument_3))?;
    host::graphics_translate(r, graphics, -6, 16)?;
    host::graphics_set_clip(r, graphics, 0, 0, 53, 26)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 53, 26)?;
    let enemy_image_193 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_193, 0, 0, 0)?;
    let graphics_get_translate_x_value_41 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_41 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_41.wrapping_neg(), graphics_get_translate_y_value_41.wrapping_neg())?;
    let drawable_x_42 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_42 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_42.wrapping_sub(argument_2), drawable_y_42.wrapping_sub(argument_3))?;
    argument_4 = 0;
    argument_5 = 0;
    } // next_step_13
    'draw_loop_94: loop {
        'graphics_set_clip_12: {
        if argument_5 >= 4 { break 'check_enemy_state_t_lower; }
        } // graphics_set_clip_12
        host::graphics_set_clip(r, graphics, 0, 0, 32, 35)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 35)?;
        let enemy_image_218 = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_image_218, 0, -66, 0)?;
        if argument_5 == 0 {
host::graphics_translate(r, graphics, 0, 2)?;
}
        host::graphics_translate(r, graphics, -8, 6)?;
        argument_5 = argument_5.wrapping_add(1);
        continue 'draw_loop_94;
    }
    } // check_enemy_state_t_lower
    let enemy_hurt_timer_3 = r.get(enemy, field::enemy::HURT_TIMER)?.i();
    let remainder_value_41 = arithmetic::remainder_int(r, enemy_hurt_timer_3, 3)?;
    if remainder_value_41 != 1 {
host::graphics_translate(r, graphics, -6, -5)?;
host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
let enemy_image_231 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_231, -33, -58, 0)?;
host::graphics_translate(r, graphics, 6, 0)?;
} else {
host::graphics_translate(r, graphics, -6, -5)?;
host::graphics_set_clip(r, graphics, 0, 0, 44, 48)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 48)?;
let enemy_image_232 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_image(r, graphics, enemy_image_232, -104, -59, 0)?;
host::graphics_translate(r, graphics, 6, 0)?;
}
    } // check_enemy_kind_7
    let enemy_kind_48 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_48 == 30 { break 'check_enemy_state_z_16; }
    } // check_enemy_kind_8
    let enemy_kind_49 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_49 != 31 { break 'check_enemy_kind_9; }
    } // check_enemy_state_z_16
    let enemy_behavior_state_43 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_43 == 0 { break 'check_enemy_state_z_19; }
    } // check_enemy_state_z_20
    let enemy_behavior_state_49 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_49 == 1 { break 'check_enemy_state_z_19; }
    } // check_enemy_state_z_23
    let enemy_behavior_state_55 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_55 != 2 { break 'check_enemy_state_z_25; }
    } // check_enemy_state_z_19
    let enemy_behavior_state_48 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_48 == 2 {
let enemy_animation_59 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_16 = arithmetic::remainder_int(r, enemy_animation_59, 2)?;
if remainder_value_16 == 1 {
return Ok(());
}
}
    let enemy_facing_52 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_52 == 0 {
host::graphics_set_clip(r, graphics, 0, 0, 18, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 27)?;
let enemy_image_120 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_69 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_20 = arithmetic::remainder_int(r, enemy_animation_69.wrapping_shr((3) as u32), 2)?;
host::graphics_draw_image(r, graphics, enemy_image_120, (-18_i32).wrapping_mul(remainder_value_20), -54, 0)?;
}
    let enemy_facing_54 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_54 == 1 {
host::graphics_set_clip(r, graphics, 0, 0, 18, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 27)?;
let enemy_image_131 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_76 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_24 = arithmetic::remainder_int(r, enemy_animation_76.wrapping_shr((3) as u32), 2)?;
host::graphics_draw_image(r, graphics, enemy_image_131, (-18_i32).wrapping_mul(remainder_value_24), -27, 0)?;
}
    let enemy_facing_56 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_56 == 2 {
host::graphics_set_clip(r, graphics, 0, 0, 18, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 27)?;
let enemy_image_147 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_animation_87 = r.get(enemy, field::enemy::ANIMATION)?.i();
let remainder_value_28 = arithmetic::remainder_int(r, enemy_animation_87.wrapping_shr((3) as u32), 2)?;
host::graphics_draw_image(r, graphics, enemy_image_147, (-18_i32).wrapping_mul(remainder_value_28), 0, 0)?;
}
    let enemy_facing_58 = r.get(enemy, field::enemy::FACING)?.i();
    if enemy_facing_58 == 3 {
host::graphics_set_clip(r, graphics, 0, 0, 18, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 27)?;
let enemy_image_157 = r.get(enemy, field::enemy::IMAGE)?.r();
host::graphics_draw_region(r, graphics, enemy_image_157, 18, 27, 18, 27, 2, 0, 0, 0)?;
}
    host::graphics_translate(r, graphics, 4, 28)?;
    host::graphics_set_clip(r, graphics, 0, 0, 12, 8)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 12, 8)?;
    let enemy_image_156 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_156, 0, -148, 0)?;
    } // check_enemy_state_z_25
    let enemy_behavior_state_61 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_61
        == 4
    {
        host::graphics_set_clip(r, graphics, 0, 0, 22, 25)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 22, 25)?;
        let enemy_image_132 = r.get(enemy, field::enemy::IMAGE)?.r();
        let enemy_animation_77 = r.get(enemy, field::enemy::ANIMATION)?.i();
        let remainder_value_25 = arithmetic::remainder_int(r, enemy_animation_77.wrapping_shr((3) as u32), 2)?;
        host::graphics_draw_image(r, graphics, enemy_image_132, (-22_i32).wrapping_mul(remainder_value_25), -105, 0)?;
        host::graphics_set_clip(r, graphics, 0, 0, 22, 13)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 22, 13)?;
        let enemy_image_133 = r.get(enemy, field::enemy::IMAGE)?.r();
        let enemy_animation_78 = r.get(enemy, field::enemy::ANIMATION)?.i();
        let remainder_value_26 = arithmetic::remainder_int(r, enemy_animation_78.wrapping_shr((2) as u32), 2)?;
        host::graphics_draw_image(r, graphics, enemy_image_133, (-22_i32).wrapping_mul(remainder_value_26), -130, 0)?;
        let enemy_animation_79 = r.get(enemy, field::enemy::ANIMATION)?.i();
        r.set(enemy, field::enemy::ANIMATION, V::I((enemy_animation_79.wrapping_add(1)) as i8 as i32))?;
        let enemy_animation_80 = r.get(enemy, field::enemy::ANIMATION)?.i();
        if enemy_animation_80 > 15 {
r.set(enemy, field::enemy::ANIMATION, V::I(0))?;
}
    }
    } // check_enemy_kind_9
    let enemy_kind_50 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_50 == 32 {
        let enemy_behavior_state_56 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_56
            == 1
        {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 19)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 19)?;
            let enemy_image_121 = r.get(enemy, field::enemy::IMAGE)?.r();
            let enemy_animation_70 = r.get(enemy, field::enemy::ANIMATION)?.i();
            let remainder_value_21 = arithmetic::remainder_int(r, enemy_animation_70.wrapping_shr((1) as u32), 2)?;
            host::graphics_draw_image(r, graphics, enemy_image_121, (-16_i32).wrapping_mul(remainder_value_21), 0, 0)?;
        }
        let enemy_behavior_state_63 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_63
            == 2
        {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 19)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 19)?;
            let enemy_image_136 = r.get(enemy, field::enemy::IMAGE)?.r();
            let enemy_animation_84 = r.get(enemy, field::enemy::ANIMATION)?.i();
            host::graphics_draw_image(r, graphics, enemy_image_136, ((-16_i32).wrapping_mul(enemy_animation_84.wrapping_shr((2) as u32))).wrapping_sub(32), 0, 0)?;
            return Ok(());
        }
        let enemy_behavior_state_68 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_68
            == 3
        {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 19)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 19)?;
            let enemy_image_149 = r.get(enemy, field::enemy::IMAGE)?.r();
            let enemy_animation_89 = r.get(enemy, field::enemy::ANIMATION)?.i();
            host::graphics_draw_image(r, graphics, enemy_image_149, ((-16_i32).wrapping_mul(enemy_animation_89.wrapping_shr((2) as u32))).wrapping_sub(80), 0, 0)?;
            return Ok(());
        }
        let enemy_behavior_state_72 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_72
            == 4
        {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 19)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 19)?;
            let enemy_image_160 = r.get(enemy, field::enemy::IMAGE)?.r();
            let enemy_animation_92 = r.get(enemy, field::enemy::ANIMATION)?.i();
            host::graphics_draw_image(r, graphics, enemy_image_160, ((16_i32).wrapping_mul(enemy_animation_92.wrapping_shr((2) as u32))).wrapping_sub(80), 0, 0)?;
            return Ok(());
        }
    }
    let enemy_kind_51 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_51 == 33 {
        let enemy_behavior_state_62 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_62
            == 1
        {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let enemy_image_135 = r.get(enemy, field::enemy::IMAGE)?.r();
            let enemy_animation_83 = r.get(enemy, field::enemy::ANIMATION)?.i();
            let remainder_value_27 = arithmetic::remainder_int(r, enemy_animation_83.wrapping_shr((1) as u32), 2)?;
            host::graphics_draw_image(r, graphics, enemy_image_135, (-16_i32).wrapping_mul(remainder_value_27), 0, 0)?;
        }
        let enemy_behavior_state_67 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_67
            == 2
        {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let enemy_image_148 = r.get(enemy, field::enemy::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, enemy_image_148, -32, 0, 0)?;
            return Ok(());
        }
        let enemy_behavior_state_71 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_71
            == 3
        {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let enemy_image_159 = r.get(enemy, field::enemy::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, enemy_image_159, -48, 0, 0)?;
            return Ok(());
        }
        let enemy_behavior_state_76 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_76
            == 4
        {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let enemy_image_168 = r.get(enemy, field::enemy::IMAGE)?.r();
            let enemy_animation_97 = r.get(enemy, field::enemy::ANIMATION)?.i();
            let remainder_value_34 = arithmetic::remainder_int(r, enemy_animation_97.wrapping_shr((1) as u32), 2)?;
            host::graphics_draw_image(r, graphics, enemy_image_168, (-16_i32).wrapping_mul(remainder_value_34), 0, 0)?;
            return Ok(());
        }
    }
    let enemy_kind_52 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_52 == 34 {
        host::graphics_translate(r, graphics, 0, 0)?;
        host::graphics_set_clip(r, graphics, 0, 0, 24, 32)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 24, 32)?;
        let enemy_image_134 = r.get(enemy, field::enemy::IMAGE)?.r();
        let enemy_animation_81 = r.get(enemy, field::enemy::ANIMATION)?.i();
        let quotient_value_14 = arithmetic::divide_int(r, enemy_animation_81, 3)?;
        host::graphics_draw_image(r, graphics, enemy_image_134, (-24_i32).wrapping_mul(quotient_value_14), 0, 0)?;
        let enemy_animation_82 = r.get(enemy, field::enemy::ANIMATION)?.i();
        if enemy_animation_82 > 7 {
r.set(enemy, field::enemy::ANIMATION, V::I(-1))?;
}
    }
    let enemy_kind_53 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_53 == 35 {
        let enemy_animation_88 = r.get(enemy, field::enemy::ANIMATION)?.i();
        if enemy_animation_88 < 4 {
host::graphics_set_clip(r, graphics, 0, 0, 44, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 36)?;
let enemy_image_158 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_facing_60 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_158, 0, enemy_facing_60.wrapping_mul(-36), 0)?;
return Ok(());
}
        let enemy_animation_91 = r.get(enemy, field::enemy::ANIMATION)?.i();
        if enemy_animation_91 < 8 {
let enemy_facing_62 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_62 == 1 {
host::graphics_translate(r, graphics, -4, 0)?;
}
let enemy_facing_66 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_66 == 2 {
host::graphics_translate(r, graphics, 0, -1)?;
}
let enemy_facing_70 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_70 == 3 {
host::graphics_translate(r, graphics, 4, 0)?;
}
host::graphics_set_clip(r, graphics, 0, 0, 44, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 36)?;
let enemy_image_203 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_facing_75 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_203, -44, enemy_facing_75.wrapping_mul(-36), 0)?;
return Ok(());
}
        let enemy_animation_96 = r.get(enemy, field::enemy::ANIMATION)?.i();
        if enemy_animation_96 < 12 {
host::graphics_set_clip(r, graphics, 0, 0, 44, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 36)?;
let enemy_image_180 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_facing_65 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_180, 0, enemy_facing_65.wrapping_mul(-36), 0)?;
return Ok(());
}
        let enemy_animation_101 = r.get(enemy, field::enemy::ANIMATION)?.i();
        if enemy_animation_101 <= 16 {
let enemy_facing_69 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_69 == 1 {
host::graphics_translate(r, graphics, 6, 0)?;
}
let enemy_facing_74 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_74 == 2 {
host::graphics_translate(r, graphics, 0, 2)?;
}
let enemy_facing_78 = r.get(enemy, field::enemy::FACING)?.i();
if enemy_facing_78 == 3 {
host::graphics_translate(r, graphics, -6, 0)?;
}
host::graphics_set_clip(r, graphics, 0, 0, 44, 36)?;
host::graphics_clip_rect(r, graphics, 0, 0, 44, 36)?;
let enemy_image_230 = r.get(enemy, field::enemy::IMAGE)?.r();
let enemy_facing_79 = r.get(enemy, field::enemy::FACING)?.i();
host::graphics_draw_image(r, graphics, enemy_image_230, -88, enemy_facing_79.wrapping_mul(-36), 0)?;
return Ok(());
}
    }
    let enemy_kind_54 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_54 == 36 {
        let enemy_behavior_state_75 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_75
            == 1
        {
            host::graphics_set_clip(r, graphics, 0, 0, 23, 30)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 23, 30)?;
            let enemy_image_167 = r.get(enemy, field::enemy::IMAGE)?.r();
            let enemy_animation_95 = r.get(enemy, field::enemy::ANIMATION)?.i();
            let remainder_value_33 = arithmetic::remainder_int(r, enemy_animation_95, 3)?;
            host::graphics_draw_image(r, graphics, enemy_image_167, (-23_i32).wrapping_mul(remainder_value_33), 0, 0)?;
            let enemy_state_a_6 = r.get(enemy, field::enemy::STATE_A)?.i();
            r.set(enemy, field::enemy::STATE_A, V::I(enemy_state_a_6.wrapping_sub(1)))?;
            let enemy_state_a_7 = r.get(enemy, field::enemy::STATE_A)?.i();
            if enemy_state_a_7 < 0 {
r.set(enemy, field::enemy::HEALTH, V::I(0))?;
r.set(enemy, field::enemy::DYING, V::I(1))?;
r.set(enemy, field::enemy::ACTIVE, V::I(1))?;
}
        }
        let enemy_behavior_state_79 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_79
            == 0
        {
            let graphics_get_translate_x_value_34 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_34 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(r, graphics, graphics_get_translate_x_value_34.wrapping_neg(), graphics_get_translate_y_value_34.wrapping_neg())?;
            let enemy_state_x_upper_4 = r.get(enemy, field::enemy::STATE_X_UPPER)?.i();
            let enemy_state_s_lower_4 = r.get(enemy, field::enemy::STATE_S_LOWER)?.i();
            host::graphics_translate(r, graphics, enemy_state_x_upper_4.wrapping_sub(argument_2), enemy_state_s_lower_4.wrapping_sub(argument_3))?;
            host::graphics_set_clip(r, graphics, 0, 0, 23, 30)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 23, 30)?;
            let enemy_image_178 = r.get(enemy, field::enemy::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, enemy_image_178, -69, 0, 0)?;
            let graphics_get_translate_x_value_35 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_35 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(r, graphics, graphics_get_translate_x_value_35.wrapping_neg(), graphics_get_translate_y_value_35.wrapping_neg())?;
            let drawable_x_36 = r.get(enemy, field::drawable::X)?.i();
            let drawable_y_36 = r.get(enemy, field::drawable::Y)?.i();
            host::graphics_translate(r, graphics, drawable_x_36.wrapping_sub(argument_2), drawable_y_36.wrapping_sub(argument_3))?;
            host::graphics_set_clip(r, graphics, 0, 0, 23, 30)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 23, 30)?;
            let enemy_image_179 = r.get(enemy, field::enemy::IMAGE)?.r();
            let enemy_animation_100 = r.get(enemy, field::enemy::ANIMATION)?.i();
            let remainder_value_36 = arithmetic::remainder_int(r, enemy_animation_100, 3)?;
            host::graphics_draw_image(r, graphics, enemy_image_179, (-23_i32).wrapping_mul(remainder_value_36), 0, 0)?;
        }
    }
    let enemy_kind_55 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_55 != 37 {
        break 'check_enemy_kind_10;
    }
    } // check_enemy_state_z_31
    let enemy_behavior_state_78 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_78 == 0 {
        break 'graphics_translate_6;
    }
    } // check_enemy_state_z_34
    let enemy_behavior_state_82 = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_82 != 8 {
        break 'check_enemy_state_z_37;
    }
    } // graphics_translate_6
    host::graphics_translate(
        r, graphics, 3, 32,
    )?;
    host::graphics_set_clip(
        r, graphics, 0, 0, 32, 9,
    )?;
    host::graphics_clip_rect(
        r, graphics, 0, 0, 32, 9,
    )?;
    let image_value_177 = r
        .get(
            enemy,
            field::enemy::IMAGE,
        )?
        .r();
    host::graphics_draw_image(
        r,
        graphics,
        image_value_177,
        0,
        -45,
        0,
    )?;
    let animation_value_99 = r
        .get(
            enemy,
            field::enemy::ANIMATION,
        )?
        .i();
    if animation_value_99 <= 6 {
        host::graphics_translate(
            r, graphics, -3, -32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 38, 38,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 38, 38,
        )?;
        let image_value_191 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_191,
            -38,
            0,
            0,
        )?;
    } else {
        host::graphics_translate(
            r, graphics, -3, -32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 38, 38,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 38, 38,
        )?;
        let image_value_192 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_192,
            0,
            0,
            0,
        )?;
    }
    } // check_enemy_state_z_37
    let state_z_value_85 = r
        .get(
            enemy,
            field::enemy::BEHAVIOR_STATE,
        )?
        .i();
    if state_z_value_85 == 1 {
        host::graphics_translate(
            r, graphics, 3, 32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 9,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 9,
        )?;
        let image_value_202 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_202,
            0,
            -45,
            0,
        )?;
        let animation_value_106 = r
            .get(
                enemy,
                field::enemy::ANIMATION,
            )?
            .i();
        if animation_value_106 >= 5 {
            let animation_value_109 = r
                .get(
                    enemy,
                    field::enemy::ANIMATION,
                )?
                .i();
            if animation_value_109 >= 7 {
                host::graphics_translate(
                    r, graphics, -9, -23,
                )?;
                host::graphics_set_clip(
                    r, graphics, 0, 0, 44,
                    29,
                )?;
                host::graphics_clip_rect(
                    r, graphics, 0, 0, 44,
                    29,
                )?;
                let image_value_228 = r
                    .get(
                        enemy,
                        field::enemy::IMAGE,
                    )?
                    .r();
                host::graphics_draw_image(
                    r,
                    graphics,
                    image_value_228,
                    0,
                    -57,
                    0,
                )?;
            } else {
                host::graphics_translate(
                    r, graphics, -3, -32,
                )?;
                host::graphics_set_clip(
                    r, graphics, 0, 0, 38,
                    38,
                )?;
                host::graphics_clip_rect(
                    r, graphics, 0, 0, 38,
                    38,
                )?;
                let image_value_229 = r
                    .get(
                        enemy,
                        field::enemy::IMAGE,
                    )?
                    .r();
                host::graphics_draw_image(
                    r,
                    graphics,
                    image_value_229,
                    -38,
                    0,
                    0,
                )?;
            }
        } else {
            host::graphics_translate(
                r, graphics, -3, -32,
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 38, 38,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 38, 38,
            )?;
            let image_value_217 = r
                .get(
                    enemy,
                    field::enemy::IMAGE,
                )?
                .r();
            host::graphics_draw_image(
                r,
                graphics,
                image_value_217,
                0,
                0,
                0,
            )?;
        }
    }
    let state_z_value_87 = r
        .get(
            enemy,
            field::enemy::BEHAVIOR_STATE,
        )?
        .i();
    if state_z_value_87 == 2 {
        host::graphics_translate(
            r, graphics, 3, 32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 9,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 9,
        )?;
        let image_value_215 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_215,
            0,
            -45,
            0,
        )?;
        host::graphics_translate(
            r, graphics, -9, -23,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 44, 29,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 44, 29,
        )?;
        let image_value_216 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_216,
            0,
            -57,
            0,
        )?;
    }
    let state_z_value_89 = r
        .get(
            enemy,
            field::enemy::BEHAVIOR_STATE,
        )?
        .i();
    if state_z_value_89 == 3 {
        host::graphics_translate(
            r, graphics, 3, 32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 9,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 9,
        )?;
        let image_value_226 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_226,
            0,
            -45,
            0,
        )?;
        host::graphics_translate(
            r, graphics, 0, -39,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 45,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 45,
        )?;
        let image_value_227 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_227,
            -44,
            -40,
            0,
        )?;
    }
    let state_z_value_90 = r
        .get(
            enemy,
            field::enemy::BEHAVIOR_STATE,
        )?
        .i();
    if state_z_value_90 == 4 {
        host::graphics_translate(
            r, graphics, 3, 32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 9,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 9,
        )?;
        let image_value_235 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_235,
            0,
            -45,
            0,
        )?;
        let state_f_value_19 = r
            .get(
                enemy,
                field::enemy::STATE_TIMER,
            )?
            .i();
        host::graphics_translate(
            r,
            graphics,
            0,
            (-39_i32).wrapping_sub(
                (state_f_value_19)
                    .wrapping_mul(25),
            ),
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 45,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 45,
        )?;
        let image_value_236 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_236,
            -44,
            -40,
            0,
        )?;
    }
    let state_z_value_91 = r
        .get(
            enemy,
            field::enemy::BEHAVIOR_STATE,
        )?
        .i();
    if state_z_value_91 == 5 {
        host::graphics_translate(
            r, graphics, 3, 32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 9,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 9,
        )?;
        let image_value_237 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_237,
            0,
            -45,
            0,
        )?;
    }
    let state_z_value_92 = r
        .get(
            enemy,
            field::enemy::BEHAVIOR_STATE,
        )?
        .i();
    if state_z_value_92 == 6 {
        host::graphics_translate(
            r, graphics, 3, 32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 9,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 9,
        )?;
        let image_value_238 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_238,
            0,
            -45,
            0,
        )?;
        let animation_value_112 = r
            .get(
                enemy,
                field::enemy::ANIMATION,
            )?
            .i();
        let remainder_value_44 =
            arithmetic::remainder_int(
                r,
                animation_value_112,
                4,
            )?;
        if remainder_value_44 <= 1 {
            host::graphics_translate(
                r, graphics, -3, -32,
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 38, 38,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 38, 38,
            )?;
            let image_value_240 = r
                .get(
                    enemy,
                    field::enemy::IMAGE,
                )?
                .r();
            host::graphics_draw_image(
                r,
                graphics,
                image_value_240,
                0,
                0,
                0,
            )?;
        } else {
            host::graphics_translate(
                r, graphics, -7, -23,
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 44, 29,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 44, 29,
            )?;
            let image_value_241 = r
                .get(
                    enemy,
                    field::enemy::IMAGE,
                )?
                .r();
            host::graphics_draw_image(
                r,
                graphics,
                image_value_241,
                0,
                -57,
                0,
            )?;
        }
    }
    let state_z_value_93 = r
        .get(
            enemy,
            field::enemy::BEHAVIOR_STATE,
        )?
        .i();
    if state_z_value_93 == 7 {
        host::graphics_translate(
            r, graphics, 3, 32,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 32, 9,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 32, 9,
        )?;
        let image_value_239 = r
            .get(
                enemy,
                field::enemy::IMAGE,
            )?
            .r();
        host::graphics_draw_image(
            r,
            graphics,
            image_value_239,
            0,
            -45,
            0,
        )?;
        let animation_value_113 = r
            .get(
                enemy,
                field::enemy::ANIMATION,
            )?
            .i();
        let remainder_value_45 =
            arithmetic::remainder_int(
                r,
                animation_value_113,
                4,
            )?;
        if remainder_value_45 <= 1 {
            host::graphics_translate(
                r, graphics, -7, -23,
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 46, 35,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 46, 35,
            )?;
            let image_value_242 = r
                .get(
                    enemy,
                    field::enemy::IMAGE,
                )?
                .r();
            host::graphics_draw_image(
                r,
                graphics,
                image_value_242,
                -45,
                -87,
                0,
            )?;
        } else {
            host::graphics_translate(
                r, graphics, -7, -23,
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 46, 35,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 46, 35,
            )?;
            let image_value_243 = r
                .get(
                    enemy,
                    field::enemy::IMAGE,
                )?
                .r();
            host::graphics_draw_image(
                r,
                graphics,
                image_value_243,
                0,
                -87,
                0,
            )?;
        }
    }
    } // check_enemy_kind_10
    let enemy_kind_56 =
        r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_56 != 38 {
        break 'check_enemy_kind_11;
    }
    } // check_enemy_state_z_33
    let state_z_value_81 = r
        .get(enemy, field::enemy::BEHAVIOR_STATE)?
        .i();
    if state_z_value_81 == 0 {
        break 'graphics_set_clip_9;
    }
    } // check_enemy_state_z_36
    let enemy_behavior_state_84 =
        r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_84 != 1 {
        break 'check_enemy_state_z_38;
    }
    } // graphics_set_clip_9
    host::graphics_set_clip(r, graphics, 0, 0, 48, 16)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 48, 16)?;
    let enemy_animation_103 =
        r.get(enemy, field::enemy::ANIMATION)?.i();
    let remainder_value_38 = arithmetic::remainder_int(
        r,
        enemy_animation_103,
        5,
    )?;
    if remainder_value_38 < 3 {
        let enemy_image_201 =
            r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image_201,
            0,
            -32,
            0,
        )?;
    } else {
        let enemy_facing_73 =
            r.get(enemy, field::enemy::FACING)?.i();
        if enemy_facing_73 != 0 {
            let enemy_image_212 =
                r.get(enemy, field::enemy::IMAGE)?.r();
            host::graphics_draw_image(
                r,
                graphics,
                enemy_image_212,
                0,
                -16,
                0,
            )?;
        } else {
            let enemy_image_213 =
                r.get(enemy, field::enemy::IMAGE)?.r();
            host::graphics_draw_image(
                r,
                graphics,
                enemy_image_213,
                0,
                0,
                0,
            )?;
        }
    }
    let enemy_behavior_state_88 =
        r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_88 == 1 {
        host::graphics_translate(r, graphics, 16, 0)?;
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 16, 16,
        )?;
        let enemy_image_223 =
            r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image_223,
            0,
            -48,
            0,
        )?;
    }
    } // check_enemy_state_z_38
    let enemy_behavior_state_86 =
        r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_86 == 2 {
        host::graphics_set_clip(r, graphics, 0, 0, 48, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 48, 16)?;
        let enemy_image_214 =
            r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image_214,
            0,
            -32,
            0,
        )?;
        let enemy_animation_108 =
            r.get(enemy, field::enemy::ANIMATION)?.i();
        let remainder_value_40 = arithmetic::remainder_int(
            r,
            enemy_animation_108,
            5,
        )?;
        if remainder_value_40 <= 2 {
            host::graphics_translate(r, graphics, 16, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 16, 16,
            )?;
            let enemy_image_224 =
                r.get(enemy, field::enemy::IMAGE)?.r();
            host::graphics_draw_image(
                r,
                graphics,
                enemy_image_224,
                -16,
                -48,
                0,
            )?;
        } else {
            host::graphics_translate(r, graphics, 16, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 16, 16,
            )?;
            let enemy_image_225 =
                r.get(enemy, field::enemy::IMAGE)?.r();
            host::graphics_draw_image(
                r,
                graphics,
                enemy_image_225,
                0,
                -48,
                0,
            )?;
        }
    }
    } // check_enemy_kind_11
    let enemy_kind_57 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_57 == 128 {
        host::graphics_translate(r, graphics, 0, -6)?;
        host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
        let enemy_image_190 =
            r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image_190,
            0,
            0,
            0,
        )?;
    }
    let enemy_kind_58 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_58 == 129 {
        host::graphics_translate(r, graphics, 0, -6)?;
        host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
        let enemy_image_200 =
            r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image_200,
            -16,
            0,
            0,
        )?;
    }
    let enemy_kind_59 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_59 == 130 {
        host::graphics_translate(r, graphics, 0, -6)?;
        host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
        let enemy_image_211 =
            r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image_211,
            -32,
            0,
            0,
        )?;
    }
    let enemy_kind_60 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_60 == 131 {
        let enemy_image_222 =
            r.get(enemy, field::enemy::IMAGE)?.r();
        let enemy_animation_111 =
            r.get(enemy, field::enemy::ANIMATION)?.i();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image_222,
            (-16_i32).wrapping_mul(
                enemy_animation_111.wrapping_shr((3) as u32),
            ),
            -32,
            0,
        )?;
    }
    let enemy_kind_61 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_61 != 132 {
        break 'check_enemy_kind;
    }
    } // graphics_translate_11
    host::graphics_translate(r, graphics, 0, -12)?;
    host::graphics_set_clip(r, graphics, 0, 0, 46, 32)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 46, 32)?;
    let enemy_image_234 = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, enemy_image_234, 0, 0, 0)?;
    let graphics_get_translate_x_value_45 =
        host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_45 =
        host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_45.wrapping_neg(),
        graphics_get_translate_y_value_45.wrapping_neg(),
    )?;
    host::graphics_translate(r, graphics, 0, -12)?;
    host::graphics_set_clip(r, graphics, 0, 0, 256, 176)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 256, 176)?;
    host::graphics_set_color_2(r, graphics, 0, 0, 0)?;
    let enemy_state_x_upper_5 =
        r.get(enemy, field::enemy::STATE_X_UPPER)?.i();
    let enemy_state_s_lower_5 =
        r.get(enemy, field::enemy::STATE_S_LOWER)?.i();
    let drawable_x_45 = r.get(enemy, field::drawable::X)?.i();
    let drawable_y_45 = r.get(enemy, field::drawable::Y)?.i();
    host::graphics_draw_line(
        r,
        graphics,
        enemy_state_x_upper_5.wrapping_sub(argument_2),
        enemy_state_s_lower_5.wrapping_sub(argument_3),
        (drawable_x_45.wrapping_sub(argument_2)).wrapping_add(45),
        (drawable_y_45.wrapping_sub(argument_3)).wrapping_add(6),
    )?;
    let enemy_overlay_image =
        r.get(enemy, field::enemy::OVERLAY_IMAGE)?.r();
    if enemy_overlay_image != 0 {
        break 'graphics_get_translate_x_16;
    }
    } // handle_result_2
    let text_value_2 = r.intern("/Images/RingsOnWater.png");
    outcome = (|| -> JResult {
        Ok(V::R(host::image_create_image(r, text_value_2)?))
    })();
    (carry_0_r,) = (enemy,);
    if outcome.is_err() {
        break 'handle_result;
    }
    } // handle_result_3
    let image_create_image_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        r.set(
            carry_0_r,
            field::enemy::OVERLAY_IMAGE,
            V::R(image_create_image_value),
        )?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        break 'handle_result;
    }
    } // next_step_14

    break 'check_enemy_state_x_upper;
    } // handle_result
    carry_0_r = outcome.unwrap_err();
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let text_value = r.intern("error?!?");
    host::print_stream_println(r, state_out_value, text_value)?;
    break 'check_enemy_state_x_upper;
    } // graphics_get_translate_x_16
    let graphics_get_translate_x_value_46 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_46 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_46.wrapping_neg(),
        graphics_get_translate_y_value_46.wrapping_neg(),
    )?;
    let enemy_state_x_upper_6 = r.get(enemy, field::enemy::STATE_X_UPPER)?.i();
    let enemy_state_s_lower_6 = r.get(enemy, field::enemy::STATE_S_LOWER)?.i();
    host::graphics_translate(
        r,
        graphics,
        enemy_state_x_upper_6.wrapping_sub(argument_2),
        enemy_state_s_lower_6.wrapping_sub(argument_3),
    )?;
    host::graphics_translate(r, graphics, 0, -16)?;
    let enemy_random_3 = r.get(enemy, field::enemy::RANDOM)?.r();
    let random_next_int_value_5 = host::random_next_int(r, enemy_random_3)?;
    let remainder_value_43 = arithmetic::remainder_int(r, random_next_int_value_5, 2)?;
    let math_abs_value_7 = host::math_abs(r, remainder_value_43)?;
    if math_abs_value_7 != 1 {
        host::graphics_translate(r, graphics, -6, -4)?;
        host::graphics_set_clip(r, graphics, 0, 0, 12, 8)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 12, 8)?;
        let enemy_overlay_image_2 = r.get(enemy, field::enemy::OVERLAY_IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_overlay_image_2, -14, 0, 0)?;
    } else {
        host::graphics_translate(r, graphics, -7, -4)?;
        host::graphics_set_clip(r, graphics, 0, 0, 14, 8)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 14, 8)?;
        let enemy_overlay_image_3 = r.get(enemy, field::enemy::OVERLAY_IMAGE)?.r();
        host::graphics_draw_image(r, graphics, enemy_overlay_image_3, 0, 0, 0)?;
    }
    } // check_enemy_state_x_upper
    let enemy_state_x_upper = r.get(enemy, field::enemy::STATE_X_UPPER)?.i();
    let enemy_state_timer = r.get(enemy, field::enemy::STATE_TIMER)?.i();
    let enemy_random = r.get(enemy, field::enemy::RANDOM)?.r();
    let random_next_int_value = host::random_next_int(r, enemy_random)?;
    let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 3)?;
    let math_abs_value = host::math_abs(r, remainder_value)?;
    r.set(
        enemy,
        field::enemy::STATE_X_UPPER,
        V::I(enemy_state_x_upper.wrapping_add(enemy_state_timer.wrapping_mul(math_abs_value))),
    )?;
    let enemy_state_x_upper_2 = r.get(enemy, field::enemy::STATE_X_UPPER)?.i();
    if enemy_state_x_upper_2 < 150 {
        r.set(enemy, field::enemy::STATE_TIMER, V::I(1))?;
    }
    let enemy_state_x_upper_3 = r.get(enemy, field::enemy::STATE_X_UPPER)?.i();
    if enemy_state_x_upper_3 > 250 {
        r.set(enemy, field::enemy::STATE_TIMER, V::I(-1))?;
    }
    let enemy_state_s_lower = r.get(enemy, field::enemy::STATE_S_LOWER)?.i();
    let enemy_state_g = r.get(enemy, field::enemy::STATE_G)?.i();
    let enemy_random_2 = r.get(enemy, field::enemy::RANDOM)?.r();
    let random_next_int_value_2 = host::random_next_int(r, enemy_random_2)?;
    let remainder_value_2 = arithmetic::remainder_int(r, random_next_int_value_2, 2)?;
    let math_abs_value_2 = host::math_abs(r, remainder_value_2)?;
    r.set(
        enemy,
        field::enemy::STATE_S_LOWER,
        V::I(
            enemy_state_s_lower.wrapping_add(enemy_state_g.wrapping_mul(math_abs_value_2)),
        ),
    )?;
    let enemy_state_s_lower_2 = r.get(enemy, field::enemy::STATE_S_LOWER)?.i();
    if enemy_state_s_lower_2 < 200 {
        r.set(enemy, field::enemy::STATE_G, V::I(1))?;
    }
    let enemy_state_s_lower_3 = r.get(enemy, field::enemy::STATE_S_LOWER)?.i();
    if enemy_state_s_lower_3 > 230 {
        r.set(enemy, field::enemy::STATE_G, V::I(-1))?;
    }
    } // check_enemy_kind
    let enemy_kind_12 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_12 == 133 {
        host::graphics_set_clip(r, graphics, 0, 0, 26, 28)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 26, 28)?;
        let enemy_image = r.get(enemy, field::enemy::IMAGE)?.r();
        let enemy_animation = r.get(enemy, field::enemy::ANIMATION)?.i();
        let enemy_facing = r.get(enemy, field::enemy::FACING)?.i();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image,
            (-26_i32).wrapping_mul(enemy_animation.wrapping_shr((1) as u32)),
            (-28_i32).wrapping_mul(enemy_facing),
            0,
        )?;
        let enemy_animation_2 = r.get(enemy, field::enemy::ANIMATION)?.i();
        if enemy_animation_2 > 10 {
            r.set(enemy, field::enemy::ANIMATION, V::I(0))?;
        }
    }
    let enemy_kind_17 = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_17 != 134 {
        let enemy_kind_19_read = r.get(enemy, field::enemy::KIND)?.i();
        if enemy_kind_19_read != 135 {
            return Ok(());
        }
        argument_4 = 0;
        host::graphics_set_clip(r, graphics, 0, 0, 22, 26)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 22, 26)?;
        let enemy_behavior_state_2_read = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        argument_4 = enemy_behavior_state_2_read;
        let enemy_behavior_state_3_read = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_3_read == 2 {
            argument_4 = 0;
        }
        let enemy_behavior_state_4_read = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
        if enemy_behavior_state_4_read == 3 {
            argument_4 = 2;
        }
        let enemy_image_8_read = r.get(enemy, field::enemy::IMAGE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            enemy_image_8_read,
            argument_4.wrapping_mul(-22),
            0,
            0,
        )?;
        return Ok(());
    }
    host::graphics_set_clip(r, graphics, 0, 0, 25, 33)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 25, 33)?;
    let enemy_image_4 = r.get(enemy, field::enemy::IMAGE)?.r();
    let enemy_animation_4 = r.get(enemy, field::enemy::ANIMATION)?.i();
    host::graphics_draw_image(
        r,
        graphics,
        enemy_image_4,
        (-25_i32).wrapping_mul(enemy_animation_4.wrapping_shr((1) as u32)),
        0,
        0,
    )?;
    let enemy_animation_5 = r.get(enemy, field::enemy::ANIMATION)?.i();
    if enemy_animation_5 > 10 {
        r.set(enemy, field::enemy::ANIMATION, V::I(0))?;
    }
    let enemy_kind_19_read = r.get(enemy, field::enemy::KIND)?.i();
    if enemy_kind_19_read != 135 {
        return Ok(());
    }
    argument_4 = 0;
    host::graphics_set_clip(r, graphics, 0, 0, 22, 26)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 22, 26)?;
    let enemy_behavior_state_2_read = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    argument_4 = enemy_behavior_state_2_read;
    let enemy_behavior_state_3_read = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_3_read == 2 {
        argument_4 = 0;
    }
    let enemy_behavior_state_4_read = r.get(enemy, field::enemy::BEHAVIOR_STATE)?.i();
    if enemy_behavior_state_4_read == 3 {
        argument_4 = 2;
    }
    let enemy_image_8_read = r.get(enemy, field::enemy::IMAGE)?.r();
    host::graphics_draw_image(
        r,
        graphics,
        enemy_image_8_read,
        argument_4.wrapping_mul(-22),
        0,
        0,
    )?;
    return Ok(());
}

/// Reset animation. Original reference: `l.a()V`.
pub(super) fn reset_animation(r: &mut Runtime, enemy: u32) -> Result<(), u32> {
    let enemy_animation = r.get(enemy, field::enemy::ANIMATION)?.i();
    r.set(
        enemy,
        field::enemy::ANIMATION,
        V::I((enemy_animation.wrapping_add(1)) as i8 as i32),
    )?;
    let enemy_animation_2 = r.get(enemy, field::enemy::ANIMATION)?.i();
    if enemy_animation_2 > 15 {
        r.set(enemy, field::enemy::ANIMATION, V::I(0))?;
    }
    let enemy_hurt_timer = r.get(enemy, field::enemy::HURT_TIMER)?.i();
    if enemy_hurt_timer > 0 {
        let enemy_hurt_timer_2 = r.get(enemy, field::enemy::HURT_TIMER)?.i();
        r.set(
            enemy,
            field::enemy::HURT_TIMER,
            V::I(enemy_hurt_timer_2.wrapping_sub(1)),
        )?;
    }
    let enemy_state_u = r.get(enemy, field::enemy::STATE_U)?.i();
    if enemy_state_u <= 0 {
        return Ok(());
    }
    let enemy_state_u_2 = r.get(enemy, field::enemy::STATE_U)?.i();
    r.set(
        enemy,
        field::enemy::STATE_U,
        V::I(enemy_state_u_2.wrapping_sub(1)),
    )?;
    return Ok(());
}

/// Draw boss overlay. Original reference: `l.a(Ljavax/microedition/lcdui/Graphics;IILh;)V`.
#[rustfmt::skip]
pub(super) fn draw_boss_overlay(
    r: &mut Runtime,
    enemy: u32,
    graphics: u32,
    argument_2: i32,
    argument_3: i32,
    player: u32,
) -> Result<(), u32> {
    let mut number_5: i32 = 0;
    let mut number_6: i32 = 0;
    let mut number_7: i32 = 0;
    let mut object_8: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut carry_1_i: i32 = 0;
    let mut carry_2_i: i32 = 0;
    let mut carry_3_i: i32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    'check_drawable_y_2: {
    'next_step_2: {
    'next_step: {
    'handle_result: {
    'next_step_4: {
    'check_system_state_out_2: {
    'next_step_3: {
    'handle_result_3: {
    'handle_result_2: {
    'check_system_state_out: {
    let drawable_y = r.get(enemy, field::drawable::Y)?.i();
    number_5 = drawable_y.wrapping_sub(32);
    let drawable_x = r.get(enemy, field::drawable::X)?.i();
    number_6 = drawable_x.wrapping_add(8);
    let enemy_state_w_upper =
        r.get(enemy, field::enemy::STATE_W_UPPER)?.r();
    if enemy_state_w_upper != 0 {
        break 'next_step;
    }
    } // check_system_state_out
    let state_out_value_2 =
        r.static_get(field::system::STATE_OUT).r();
    let text_value_2 = r.intern("Loading effects");
    outcome = (|| -> JResult {
        host::print_stream_println(
            r,
            state_out_value_2,
            text_value_2,
        )?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        break 'handle_result;
    }
    } // handle_result_2
    let text_value_3 = r.intern("/Images/WindEffect.png");
    outcome = (|| -> JResult {
        Ok(V::R(host::image_create_image(r, text_value_3)?))
    })();
    (carry_0_r,) = (enemy,);
    if outcome.is_err() {
        break 'handle_result;
    }
    } // handle_result_3
    let image_create_image_value = outcome.unwrap().r();
    r.set(
        carry_0_r,
        field::enemy::STATE_W_UPPER,
        V::R(image_create_image_value),
    )?;
    let enemy_state_aa = r.get(enemy, field::enemy::STATE_AA)?.r();
    host::vector_remove_all_elements(r, enemy_state_aa)?;
    number_7 = 0;
    } // next_step_3
    'draw_boss_overlay_loop_11: loop {
        'handle_result_8: {
        'handle_result_7: {
        'handle_result_6: {
        'handle_result_5: {
        'handle_result_4: {
        if number_7 >= 100 {
            break 'check_system_state_out_2;
        }
        } // handle_result_4
        let motion_value = r.alloc("d", Native::Empty);
        outcome = (|| -> JResult {
            super::motion::initialize(r, motion_value)?;
            Ok(V::I(0))
        })(
        );
        (carry_0_r,) = (motion_value,);
        if outcome.is_err() {
            break 'handle_result;
        }
        } // handle_result_5
        object_8 = carry_0_r;
        let drawable_x_2 = r.get(enemy, field::drawable::X)?.i();
        let enemy_random =
            r.get(enemy, field::enemy::RANDOM)?.r();
        outcome = (|| -> JResult {
            Ok(V::I(host::random_next_int(r, enemy_random)?))
        })(
        );
        (carry_0_r, carry_1_i) = (carry_0_r, drawable_x_2);
        if outcome.is_err() {
            break 'handle_result;
        }
        } // handle_result_6
        let random_next_int_value = outcome.unwrap().i();
        let remainder_value = arithmetic::remainder_int(
            r,
            random_next_int_value,
            50,
        )?;
        let enemy_random_2 =
            r.get(enemy, field::enemy::RANDOM)?.r();
        outcome = (|| -> JResult {
            Ok(V::I(host::random_next_int(r, enemy_random_2)?))
        })();
        (carry_0_r, carry_1_i, carry_2_i, carry_3_i) =
            (carry_0_r, carry_1_i, remainder_value, 1);
        if outcome.is_err() {
            break 'handle_result;
        }
        } // handle_result_7
        let random_next_int_value_2 = outcome.unwrap().i();
        let remainder_value_2 =
            arithmetic::remainder_int(r, random_next_int_value_2, 3)?;
        r.set(
            carry_0_r,
            field::motion::X,
            V::I(carry_1_i.wrapping_add(carry_2_i.wrapping_mul(
                carry_3_i.wrapping_sub(remainder_value_2),
            ))),
        )?;
        let drawable_y_6 = r.get(enemy, field::drawable::Y)?.i();
        let enemy_random_3 = r.get(enemy, field::enemy::RANDOM)?.r();
        outcome = (|| -> JResult {
            Ok(V::I(host::random_next_int(r, enemy_random_3)?))
        })();
        (carry_0_r, carry_1_i) = (object_8, drawable_y_6);
        if outcome.is_err() {
            break 'handle_result;
        }
        } // handle_result_8
        let random_next_int_value_3 = outcome.unwrap().i();
        let remainder_value_3 =
            arithmetic::remainder_int(r, random_next_int_value_3, 100)?;
        r.set(
            carry_0_r,
            field::motion::Y,
            V::I(carry_1_i.wrapping_add(remainder_value_3)),
        )?;
        r.set(object_8, field::motion::STATE_C, V::I(5))?;
        let motion_x_3 = r.get(object_8, field::motion::X)?.i();
        let motion_y_7 = r.get(object_8, field::motion::Y)?.i();
        r.set(
            object_8,
            field::motion::STATE_F,
            V::F(
                (((number_6) as f32 - (motion_x_3) as f32)
                    / ((motion_y_7) as f32 - (number_5) as f32)),
            ),
        )?;
        let motion_x_4 = r.get(object_8, field::motion::X)?.i();
        r.set(
            object_8,
            field::motion::VELOCITY_X,
            V::F((motion_x_4) as f32),
        )?;
        let motion_y_8 = r.get(object_8, field::motion::Y)?.i();
        r.set(
            object_8,
            field::motion::VELOCITY_Y,
            V::F((motion_y_8) as f32),
        )?;
        let enemy_state_aa_3 = r.get(enemy, field::enemy::STATE_AA)?.r();
        host::vector_add_element(r, enemy_state_aa_3, object_8)?;
        number_7 = number_7.wrapping_add(1);
        continue 'draw_boss_overlay_loop_11;
    }
    } // check_system_state_out_2
    let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
    let text_value_4 = r.intern("setting up pointers");
    outcome = (|| -> JResult {
        host::print_stream_println(r, state_out_value_3, text_value_4)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        break 'handle_result;
    }
    } // next_step_4

    break 'check_drawable_y_2;
    } // handle_result
    carry_0_r = outcome.unwrap_err();
    if !(r.is_instance(carry_0_r, "java/io/IOException")) {
        return Err(outcome.unwrap_err());
    }
    r.set(enemy, field::enemy::STATE_W_UPPER, V::R(0))?;
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let text_value = r.intern("Loading failed");
    host::print_stream_println(r, state_out_value, text_value)?;
    break 'check_drawable_y_2;
    } // next_step
    number_7 = 0;
    } // next_step_2
    'draw_boss_overlay_loop_15: loop {
        'check_enemy_state_aa: {
        if number_7 >= 100 {
            break 'check_drawable_y_2;
        }
        } // check_enemy_state_aa
        let enemy_state_aa_2 = r.get(enemy, field::enemy::STATE_AA)?.r();
        let vector_element_at_value = host::vector_element_at(r, enemy_state_aa_2, number_7)?;
        host::check_cast(r, vector_element_at_value, "d")?;
        object_8 = vector_element_at_value;
        let motion_y_5 = r.get(vector_element_at_value, field::motion::Y)?.i();
        if motion_y_5 <= number_5 {
            let motion_state_c = r.get(object_8, field::motion::STATE_C)?.i();
            if motion_state_c <= 0 {
                let drawable_x_9 = r.get(enemy, field::drawable::X)?.i();
                let enemy_random_4 = r.get(enemy, field::enemy::RANDOM)?.r();
                let random_next_int_value_4 = host::random_next_int(r, enemy_random_4)?;
                let remainder_value_4 =
                    arithmetic::remainder_int(r, random_next_int_value_4, 50)?;
                let enemy_random_5 = r.get(enemy, field::enemy::RANDOM)?.r();
                let random_next_int_value_5 = host::random_next_int(r, enemy_random_5)?;
                let remainder_value_5 =
                    arithmetic::remainder_int(r, random_next_int_value_5, 3)?;
                r.set(
                    object_8,
                    field::motion::X,
                    V::I(
                        drawable_x_9.wrapping_add(
                            (remainder_value_4)
                                .wrapping_mul((1_i32).wrapping_sub(remainder_value_5)),
                        ),
                    ),
                )?;
                let drawable_y_12 = r.get(enemy, field::drawable::Y)?.i();
                r.set(
                    object_8,
                    field::motion::Y,
                    V::I(drawable_y_12.wrapping_add(100)),
                )?;
                let motion_x_10 = r.get(object_8, field::motion::X)?.i();
                let motion_y_13 = r.get(object_8, field::motion::Y)?.i();
                r.set(
                    object_8,
                    field::motion::STATE_F,
                    V::F(
                        (((number_6) as f32 - (motion_x_10) as f32)
                            / ((motion_y_13) as f32 - (number_5) as f32)),
                    ),
                )?;
                let motion_x_11 = r.get(object_8, field::motion::X)?.i();
                r.set(
                    object_8,
                    field::motion::VELOCITY_X,
                    V::F((motion_x_11) as f32),
                )?;
                let motion_y_14 = r.get(object_8, field::motion::Y)?.i();
                r.set(
                    object_8,
                    field::motion::VELOCITY_Y,
                    V::F((motion_y_14) as f32),
                )?;
                r.set(object_8, field::motion::STATE_C, V::I(6))?;
            }
        }
        let motion_velocity_x = r.get(object_8, field::motion::VELOCITY_X)?.f();
        r.set(object_8, field::motion::X, V::I((motion_velocity_x) as i32))?;
        let motion_velocity_y = r.get(object_8, field::motion::VELOCITY_Y)?.f();
        r.set(object_8, field::motion::Y, V::I((motion_velocity_y) as i32))?;
        let motion_y_11 = r.get(object_8, field::motion::Y)?.i();
        if motion_y_11 <= number_5 {
            r.set(object_8, field::motion::STATE_C, V::I(0))?;
        } else {
            let motion_velocity_x_2 = r.get(object_8, field::motion::VELOCITY_X)?.f();
            let motion_state_f = r.get(object_8, field::motion::STATE_F)?.f();
            r.set(
                object_8,
                field::motion::VELOCITY_X,
                V::F((motion_velocity_x_2 + (motion_state_f * f32::from_bits(1088421888)))),
            )?;
            let motion_velocity_y_2 = r.get(object_8, field::motion::VELOCITY_Y)?.f();
            r.set(
                object_8,
                field::motion::VELOCITY_Y,
                V::F((motion_velocity_y_2 - f32::from_bits(1090519040))),
            )?;
        }
        let motion_y_15 = r.get(object_8, field::motion::Y)?.i();
        if !(((motion_y_15.wrapping_sub(argument_3)).wrapping_add(32)).wrapping_add(8) > 208) {
            let motion_y_16 = r.get(object_8, field::motion::Y)?.i();
            if !(((motion_y_16.wrapping_sub(argument_3)).wrapping_add(32)).wrapping_add(8) < 0)
            {
                let motion_x_14 = r.get(object_8, field::motion::X)?.i();
                if !((motion_x_14.wrapping_sub(argument_2)).wrapping_add(8) > 256) {
                    let motion_x_15 = r.get(object_8, field::motion::X)?.i();
                    if !((motion_x_15.wrapping_sub(argument_2)).wrapping_add(8) < 0) {
                        let graphics_get_translate_x_value =
                            host::graphics_get_translate_x(r, graphics)?;
                        let graphics_get_translate_y_value =
                            host::graphics_get_translate_y(r, graphics)?;
                        host::graphics_translate(
                            r,
                            graphics,
                            graphics_get_translate_x_value.wrapping_neg(),
                            graphics_get_translate_y_value.wrapping_neg(),
                        )?;
                        let motion_x_16 = r.get(object_8, field::motion::X)?.i();
                        let motion_y_17 = r.get(object_8, field::motion::Y)?.i();
                        host::graphics_translate(
                            r,
                            graphics,
                            motion_x_16.wrapping_sub(argument_2),
                            (motion_y_17.wrapping_sub(argument_3)).wrapping_add(32),
                        )?;
                        host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
                        host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
                        let motion_state_f_2 = r.get(object_8, field::motion::STATE_F)?.f();
                        if !((if motion_state_f_2 > 0.0 {
                            1
                        } else if motion_state_f_2 == 0.0 {
                            0
                        } else if motion_state_f_2 < 0.0 {
                            -1
                        } else {
                            -1
                        }) <= 0)
                        {
                            let enemy_state_w_upper_2 =
                                r.get(enemy, field::enemy::STATE_W_UPPER)?.r();
                            host::graphics_draw_image(
                                r,
                                graphics,
                                enemy_state_w_upper_2,
                                0,
                                0,
                                0,
                            )?;
                        }
                        let motion_state_f_3 = r.get(object_8, field::motion::STATE_F)?.f();
                        if !((if motion_state_f_3 > 0.0 {
                            1
                        } else if motion_state_f_3 == 0.0 {
                            0
                        } else if motion_state_f_3 < 0.0 {
                            -1
                        } else {
                            1
                        }) >= 0)
                        {
                            let enemy_state_w_upper_3 =
                                r.get(enemy, field::enemy::STATE_W_UPPER)?.r();
                            host::graphics_draw_image(
                                r,
                                graphics,
                                enemy_state_w_upper_3,
                                -16,
                                0,
                                0,
                            )?;
                        }
                        let motion_state_f_4 = r.get(object_8, field::motion::STATE_F)?.f();
                        if !((if motion_state_f_4 > 0.0 {
                            1
                        } else if motion_state_f_4 == 0.0 {
                            0
                        } else if motion_state_f_4 < 0.0 {
                            -1
                        } else {
                            -1
                        }) != 0)
                        {
                            let enemy_state_w_upper_4 =
                                r.get(enemy, field::enemy::STATE_W_UPPER)?.r();
                            host::graphics_draw_image(
                                r,
                                graphics,
                                enemy_state_w_upper_4,
                                -8,
                                0,
                                0,
                            )?;
                        }
                    }
                }
            }
        }
        number_7 = number_7.wrapping_add(1);
        continue 'draw_boss_overlay_loop_15;
    }
    } // check_drawable_y_2
    let drawable_y_2 = r.get(player, field::drawable::Y)?.i();
    if drawable_y_2 < number_5.wrapping_add(32) {
        let drawable_y_3_read = r.get(player, field::drawable::Y)?.i();
        if drawable_y_3_read >= (number_5.wrapping_add(32)).wrapping_add(4) {
            return Ok(());
        }
        let drawable_y_9_read = r.get(player, field::drawable::Y)?.i();
        if drawable_y_9_read <= (number_5.wrapping_add(32)).wrapping_sub(4) {
            return Ok(());
        }
        let drawable_x_6_read = r.get(player, field::drawable::X)?.i();
        if drawable_x_6_read >= number_6.wrapping_add(10) {
            return Ok(());
        }
        let drawable_x_12_read = r.get(player, field::drawable::X)?.i();
        if drawable_x_12_read <= number_6.wrapping_sub(10) {
            return Ok(());
        }
        r.set(player, field::player::STATE_AG, V::I(1))?;
        r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(8))?;
        r.set(enemy, field::enemy::STATE_TIMER, V::I(0))?;
        return Ok(());
    }
    let drawable_y_4 = r.get(player, field::drawable::Y)?.i();
    if !(drawable_y_4 <= number_5.wrapping_add(32)) {
        let drawable_y_10 = r.get(player, field::drawable::Y)?.i();
        r.set(
            player,
            field::drawable::Y,
            V::I(drawable_y_10.wrapping_sub(2)),
        )?;
    }
    let drawable_x_5 = r.get(player, field::drawable::X)?.i();
    if drawable_x_5 > number_6 {
        let drawable_x_8 = r.get(player, field::drawable::X)?.i();
        r.set(
            player,
            field::drawable::X,
            V::I(drawable_x_8.wrapping_sub(1)),
        )?;
    }
    let drawable_x_7 = r.get(player, field::drawable::X)?.i();
    if drawable_x_7 < number_6 {
        let drawable_x_13 = r.get(player, field::drawable::X)?.i();
        r.set(
            player,
            field::drawable::X,
            V::I(drawable_x_13.wrapping_add(1)),
        )?;
    }
    let drawable_y_3_read = r.get(player, field::drawable::Y)?.i();
    if drawable_y_3_read >= (number_5.wrapping_add(32)).wrapping_add(4) {
        return Ok(());
    }
    let drawable_y_9_read = r.get(player, field::drawable::Y)?.i();
    if drawable_y_9_read <= (number_5.wrapping_add(32)).wrapping_sub(4) {
        return Ok(());
    }
    let drawable_x_6_read = r.get(player, field::drawable::X)?.i();
    if drawable_x_6_read >= number_6.wrapping_add(10) {
        return Ok(());
    }
    let drawable_x_12_read = r.get(player, field::drawable::X)?.i();
    if drawable_x_12_read <= number_6.wrapping_sub(10) {
        return Ok(());
    }
    r.set(player, field::player::STATE_AG, V::I(1))?;
    r.set(enemy, field::enemy::BEHAVIOR_STATE, V::I(8))?;
    r.set(enemy, field::enemy::STATE_TIMER, V::I(0))?;
    return Ok(());
}
