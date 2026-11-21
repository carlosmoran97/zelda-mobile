//! Projectiles and visual effects.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `p.<init>(Ljavax/microedition/lcdui/Image;III)V`.
pub(super) fn initialize(
    r: &mut Runtime,
    projectile: u32,
    image: u32,
    kind: i32,
    velocity_x: i32,
    velocity_y: i32,
) -> Result<(), u32> {
    super::drawable::initialize(r, projectile)?;
    r.set(projectile, field::projectile::IMAGE, V::R(image))?;
    r.set(projectile, field::projectile::KIND, V::I(kind))?;
    r.set(projectile, field::projectile::VELOCITY_X, V::I(velocity_x))?;
    r.set(projectile, field::projectile::VELOCITY_Y, V::I(velocity_y))?;
    let projectile_kind = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind == 0 {
        r.set(projectile, field::projectile::STATE_U, V::I(4))?;
        r.set(projectile, field::projectile::STATE_V, V::I(4))?;
        r.set(projectile, field::projectile::WIDTH, V::I(8))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(8))?;
        r.set(projectile, field::projectile::STATE_N, V::I(4))?;
        r.set(projectile, field::projectile::DAMAGE, V::I(1))?;
        let projectile_velocity_y = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_3 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_3 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_3 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_3 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
    }
    let projectile_kind_2 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_2 == 1 {
        r.set(projectile, field::projectile::STATE_U, V::I(4))?;
        r.set(projectile, field::projectile::STATE_V, V::I(4))?;
        r.set(projectile, field::projectile::WIDTH, V::I(8))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(8))?;
        r.set(projectile, field::projectile::STATE_N, V::I(4))?;
        r.set(projectile, field::projectile::DAMAGE, V::I(3))?;
        let projectile_velocity_y_2 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_2 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_5 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_5 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_2 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_2 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_5 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_5 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
    }
    let projectile_kind_3 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_3 == 2 {
        r.set(projectile, field::projectile::STATE_U, V::I(5))?;
        r.set(projectile, field::projectile::STATE_V, V::I(5))?;
        r.set(projectile, field::projectile::WIDTH, V::I(10))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(10))?;
        r.set(projectile, field::projectile::STATE_N, V::I(5))?;
        r.set(projectile, field::projectile::DAMAGE, V::I(1))?;
        let projectile_velocity_y_4 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_4 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_7 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_7 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_4 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_4 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_7 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_7 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
    }
    let projectile_kind_4 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_4 == 3 {
        r.set(projectile, field::projectile::STATE_U, V::I(0))?;
        r.set(projectile, field::projectile::STATE_V, V::I(0))?;
        r.set(projectile, field::projectile::STATE_N, V::I(5))?;
        r.set(projectile, field::projectile::WIDTH, V::I(16))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(5))?;
        let projectile_velocity_y_6 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_6 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_9 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_9 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_6 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_6 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_9 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_9 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
        r.set(projectile, field::projectile::DAMAGE, V::I(3))?;
    }
    let projectile_kind_5 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_5 == 5 {
        r.set(projectile, field::projectile::STATE_U, V::I(8))?;
        r.set(projectile, field::projectile::STATE_V, V::I(8))?;
        r.set(projectile, field::projectile::WIDTH, V::I(15))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(15))?;
        r.set(projectile, field::projectile::STATE_N, V::I(5))?;
        let projectile_velocity_y_8 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_8 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_11 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_11 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_8 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_8 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_11 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_11 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
        r.set(projectile, field::projectile::DAMAGE, V::I(5))?;
    }
    let projectile_kind_6 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_6 == 6 {
        r.set(projectile, field::projectile::STATE_U, V::I(8))?;
        r.set(projectile, field::projectile::STATE_V, V::I(8))?;
        r.set(projectile, field::projectile::STATE_N, V::I(5))?;
        r.set(projectile, field::projectile::DAMAGE, V::I(10))?;
        r.set(projectile, field::projectile::WIDTH, V::I(16))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(16))?;
        r.set(projectile, field::projectile::STATE_T, V::I(0))?;
        let projectile_velocity_y_10 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_10 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_13 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_13 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_10 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_10 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_13 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_13 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
    }
    let projectile_kind_7 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_7 == 7 {
        r.set(projectile, field::projectile::STATE_U, V::I(12))?;
        r.set(projectile, field::projectile::STATE_V, V::I(12))?;
        r.set(projectile, field::projectile::WIDTH, V::I(25))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(25))?;
        r.set(projectile, field::projectile::STATE_N, V::I(5))?;
        r.set(projectile, field::projectile::DAMAGE, V::I(10))?;
        r.set(projectile, field::projectile::STATE_T, V::I(0))?;
        let projectile_velocity_y_12 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_12 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_15 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_15 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_12 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_12 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_15 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_15 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
    }
    let projectile_kind_8 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_8 == 8 {
        r.set(projectile, field::projectile::STATE_U, V::I(5))?;
        r.set(projectile, field::projectile::STATE_V, V::I(5))?;
        r.set(projectile, field::projectile::WIDTH, V::I(10))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(10))?;
        r.set(projectile, field::projectile::STATE_N, V::I(4))?;
        r.set(projectile, field::projectile::DAMAGE, V::I(1))?;
        let projectile_velocity_y_14 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_14 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_17 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_17 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_14 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_14 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_17 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_17 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
    }
    let projectile_kind_9 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_9 == 9 {
        r.set(projectile, field::projectile::STATE_U, V::I(5))?;
        r.set(projectile, field::projectile::STATE_V, V::I(5))?;
        r.set(projectile, field::projectile::WIDTH, V::I(10))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(10))?;
        r.set(projectile, field::projectile::STATE_N, V::I(4))?;
        r.set(projectile, field::projectile::DAMAGE, V::I(1))?;
        let projectile_velocity_y_16 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_16 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_19 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_19 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_16 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_16 > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_19 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_19 < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
    }
    let projectile_kind_10 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_10 != 10 {
        let projectile_kind_11_read = r.get(projectile, field::projectile::KIND)?.i();
        if projectile_kind_11_read != 11 {
            return Ok(());
        }
        r.set(projectile, field::projectile::STATE_U, V::I(0))?;
        r.set(projectile, field::projectile::STATE_V, V::I(0))?;
        r.set(projectile, field::projectile::STATE_N, V::I(5))?;
        r.set(projectile, field::projectile::WIDTH, V::I(17))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(7))?;
        let projectile_velocity_y_20_read = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_20_read > 0 {
            r.set(projectile, field::projectile::FACING, V::I(2))?;
        }
        let projectile_velocity_y_22_read = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
        if projectile_velocity_y_22_read < 0 {
            r.set(projectile, field::projectile::FACING, V::I(0))?;
        }
        let projectile_velocity_x_20_read = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_20_read > 0 {
            r.set(projectile, field::projectile::FACING, V::I(1))?;
        }
        let projectile_velocity_x_22_read = r.get(projectile, field::projectile::VELOCITY_X)?.i();
        if projectile_velocity_x_22_read < 0 {
            r.set(projectile, field::projectile::FACING, V::I(3))?;
        }
        r.set(projectile, field::projectile::DAMAGE, V::I(3))?;
        return Ok(());
    }
    r.set(projectile, field::projectile::STATE_U, V::I(5))?;
    r.set(projectile, field::projectile::STATE_V, V::I(5))?;
    r.set(projectile, field::projectile::WIDTH, V::I(10))?;
    r.set(projectile, field::projectile::HEIGHT, V::I(10))?;
    r.set(projectile, field::projectile::STATE_N, V::I(4))?;
    r.set(projectile, field::projectile::DAMAGE, V::I(1))?;
    let projectile_velocity_y_18 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
    if projectile_velocity_y_18 > 0 {
        r.set(projectile, field::projectile::FACING, V::I(2))?;
    }
    let projectile_velocity_y_21 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
    if projectile_velocity_y_21 < 0 {
        r.set(projectile, field::projectile::FACING, V::I(0))?;
    }
    let projectile_velocity_x_18 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
    if projectile_velocity_x_18 > 0 {
        r.set(projectile, field::projectile::FACING, V::I(1))?;
    }
    let projectile_velocity_x_21 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
    if projectile_velocity_x_21 < 0 {
        r.set(projectile, field::projectile::FACING, V::I(3))?;
    }
    let projectile_kind_11_read = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_11_read != 11 {
        return Ok(());
    }
    r.set(projectile, field::projectile::STATE_U, V::I(0))?;
    r.set(projectile, field::projectile::STATE_V, V::I(0))?;
    r.set(projectile, field::projectile::STATE_N, V::I(5))?;
    r.set(projectile, field::projectile::WIDTH, V::I(17))?;
    r.set(projectile, field::projectile::HEIGHT, V::I(7))?;
    let projectile_velocity_y_20_read = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
    if projectile_velocity_y_20_read > 0 {
        r.set(projectile, field::projectile::FACING, V::I(2))?;
    }
    let projectile_velocity_y_22_read = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
    if projectile_velocity_y_22_read < 0 {
        r.set(projectile, field::projectile::FACING, V::I(0))?;
    }
    let projectile_velocity_x_20_read = r.get(projectile, field::projectile::VELOCITY_X)?.i();
    if projectile_velocity_x_20_read > 0 {
        r.set(projectile, field::projectile::FACING, V::I(1))?;
    }
    let projectile_velocity_x_22_read = r.get(projectile, field::projectile::VELOCITY_X)?.i();
    if projectile_velocity_x_22_read < 0 {
        r.set(projectile, field::projectile::FACING, V::I(3))?;
    }
    r.set(projectile, field::projectile::DAMAGE, V::I(3))?;
    return Ok(());
}

/// Draw. Original reference: `p.a(Ljavax/microedition/lcdui/Graphics;IIII)V`.
pub(super) fn draw(
    r: &mut Runtime,
    projectile: u32,
    graphics: u32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
    argument_5: i32,
) -> Result<(), u32> {
    let mut object_4: u32 = 0;
    let drawable_x = r.get(projectile, field::drawable::X)?.i();
    if !(argument_4.wrapping_add(argument_2) >= drawable_x) {
        super::projectile::reset_animation(r, projectile)?;
        return Ok(());
    }
    let drawable_x_2 = r.get(projectile, field::drawable::X)?.i();
    let projectile_state_u = r.get(projectile, field::projectile::STATE_U)?.i();
    let projectile_state_u_2 = r.get(projectile, field::projectile::STATE_U)?.i();
    if !((drawable_x_2.wrapping_add(projectile_state_u)).wrapping_add(projectile_state_u_2)
        >= argument_2)
    {
        super::projectile::reset_animation(r, projectile)?;
        return Ok(());
    }
    let drawable_y = r.get(projectile, field::drawable::Y)?.i();
    if !(argument_5.wrapping_add(argument_3) >= drawable_y) {
        super::projectile::reset_animation(r, projectile)?;
        return Ok(());
    }
    let drawable_y_2 = r.get(projectile, field::drawable::Y)?.i();
    let projectile_state_v = r.get(projectile, field::projectile::STATE_V)?.i();
    let projectile_state_v_2 = r.get(projectile, field::projectile::STATE_V)?.i();
    if !(((drawable_y_2.wrapping_add(projectile_state_v)).wrapping_add(projectile_state_v_2))
        .wrapping_sub(32)
        >= argument_3)
    {
        super::projectile::reset_animation(r, projectile)?;
        return Ok(());
    }
    let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value.wrapping_neg(),
        graphics_get_translate_y_value.wrapping_neg(),
    )?;
    let drawable_x_3 = r.get(projectile, field::drawable::X)?.i();
    let drawable_y_3 = r.get(projectile, field::drawable::Y)?.i();
    host::graphics_translate(
        r,
        graphics,
        drawable_x_3.wrapping_sub(argument_2),
        drawable_y_3.wrapping_sub(argument_3),
    )?;
    let projectile_kind = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind == 0 {
        let drawable_x_4 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_4.wrapping_sub(argument_2)).wrapping_add(projectile_width) <= 256) {
            return Ok(());
        }
        let drawable_y_4 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_4.wrapping_sub(argument_3)).wrapping_add(projectile_height) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
        let projectile_image = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation = r.get(projectile, field::projectile::ANIMATION)?.i();
        let remainder_value = arithmetic::remainder_int(r, projectile_animation, 2)?;
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image,
            (-8_i32).wrapping_mul(remainder_value),
            0,
            0,
        )?;
    }
    let projectile_kind_2 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_2 == 1 {
        let drawable_x_5 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_2 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_5.wrapping_sub(argument_2)).wrapping_add(projectile_width_2) <= 256) {
            return Ok(());
        }
        let drawable_y_5 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_2 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_5.wrapping_sub(argument_3)).wrapping_add(projectile_height_2) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
        let projectile_image_2 = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation_2 = r.get(projectile, field::projectile::ANIMATION)?.i();
        let remainder_value_2 = arithmetic::remainder_int(r, projectile_animation_2, 3)?;
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image_2,
            (-8_i32).wrapping_mul(remainder_value_2),
            0,
            0,
        )?;
    }
    let projectile_kind_3 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_3 == 2 {
        let drawable_x_6 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_3 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_6.wrapping_sub(argument_2)).wrapping_add(projectile_width_3) <= 256) {
            return Ok(());
        }
        let drawable_y_6 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_3 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_6.wrapping_sub(argument_3)).wrapping_add(projectile_height_3) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 10, 10)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 10, 10)?;
        let projectile_image_3 = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation_3 = r.get(projectile, field::projectile::ANIMATION)?.i();
        let quotient_value = arithmetic::divide_int(r, projectile_animation_3, 2)?;
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image_3,
            (-10_i32).wrapping_mul(quotient_value),
            0,
            0,
        )?;
    }
    let projectile_kind_4 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_4 == 3 {
        let projectile_facing = r.get(projectile, field::projectile::FACING)?.i();
        if projectile_facing == 0 {
            r.set(projectile, field::projectile::WIDTH, V::I(5))?;
            r.set(projectile, field::projectile::HEIGHT, V::I(16))?;
            let drawable_x_8 = r.get(projectile, field::drawable::X)?.i();
            let projectile_width_5 = r.get(projectile, field::projectile::WIDTH)?.i();
            if !((drawable_x_8.wrapping_sub(argument_2)).wrapping_add(projectile_width_5) <= 256) {
                return Ok(());
            }
            let drawable_y_8 = r.get(projectile, field::drawable::Y)?.i();
            let projectile_height_5 = r.get(projectile, field::projectile::HEIGHT)?.i();
            if !((drawable_y_8.wrapping_sub(argument_3)).wrapping_add(projectile_height_5) <= 208) {
                return Ok(());
            }
            host::graphics_set_clip(r, graphics, 0, 0, 5, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 5, 16)?;
            let projectile_image_5 = r.get(projectile, field::projectile::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, projectile_image_5, -21, 0, 0)?;
        }
        let projectile_facing_2 = r.get(projectile, field::projectile::FACING)?.i();
        if projectile_facing_2 == 1 {
            r.set(projectile, field::projectile::WIDTH, V::I(16))?;
            r.set(projectile, field::projectile::HEIGHT, V::I(5))?;
            let drawable_x_10 = r.get(projectile, field::drawable::X)?.i();
            let projectile_width_7 = r.get(projectile, field::projectile::WIDTH)?.i();
            if !((drawable_x_10.wrapping_sub(argument_2)).wrapping_add(projectile_width_7) <= 256) {
                return Ok(());
            }
            let drawable_y_10 = r.get(projectile, field::drawable::Y)?.i();
            let projectile_height_7 = r.get(projectile, field::projectile::HEIGHT)?.i();
            if !((drawable_y_10.wrapping_sub(argument_3)).wrapping_add(projectile_height_7) <= 208)
            {
                return Ok(());
            }
            host::graphics_set_clip(r, graphics, 0, 0, 16, 5)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 5)?;
            let projectile_image_7 = r.get(projectile, field::projectile::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, projectile_image_7, 0, 0, 0)?;
        }
        let projectile_facing_3 = r.get(projectile, field::projectile::FACING)?.i();
        if projectile_facing_3 == 2 {
            r.set(projectile, field::projectile::WIDTH, V::I(5))?;
            r.set(projectile, field::projectile::HEIGHT, V::I(16))?;
            let drawable_x_12 = r.get(projectile, field::drawable::X)?.i();
            let projectile_width_9 = r.get(projectile, field::projectile::WIDTH)?.i();
            if !((drawable_x_12.wrapping_sub(argument_2)).wrapping_add(projectile_width_9) <= 256) {
                return Ok(());
            }
            let drawable_y_13 = r.get(projectile, field::drawable::Y)?.i();
            let projectile_height_9 = r.get(projectile, field::projectile::HEIGHT)?.i();
            if !((drawable_y_13.wrapping_sub(argument_3)).wrapping_add(projectile_height_9) <= 208)
            {
                return Ok(());
            }
            host::graphics_set_clip(r, graphics, 0, 0, 5, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 5, 16)?;
            let projectile_image_10 = r.get(projectile, field::projectile::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, projectile_image_10, -16, 0, 0)?;
        }
        let projectile_facing_5 = r.get(projectile, field::projectile::FACING)?.i();
        if projectile_facing_5 == 3 {
            r.set(projectile, field::projectile::WIDTH, V::I(16))?;
            r.set(projectile, field::projectile::HEIGHT, V::I(5))?;
            let drawable_x_15 = r.get(projectile, field::drawable::X)?.i();
            let projectile_width_11 = r.get(projectile, field::projectile::WIDTH)?.i();
            if !((drawable_x_15.wrapping_sub(argument_2)).wrapping_add(projectile_width_11) <= 256)
            {
                return Ok(());
            }
            let drawable_y_15 = r.get(projectile, field::drawable::Y)?.i();
            let projectile_height_11 = r.get(projectile, field::projectile::HEIGHT)?.i();
            if !((drawable_y_15.wrapping_sub(argument_3)).wrapping_add(projectile_height_11) <= 208)
            {
                return Ok(());
            }
            host::graphics_set_clip(r, graphics, 0, 0, 16, 5)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 5)?;
            let projectile_image_13 = r.get(projectile, field::projectile::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, projectile_image_13, 0, -5, 0)?;
        }
    }
    let projectile_kind_5 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_5 == 5 {
        let drawable_x_7 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_4 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_7.wrapping_sub(argument_2)).wrapping_add(projectile_width_4) <= 256) {
            return Ok(());
        }
        let drawable_y_7 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_4 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_7.wrapping_sub(argument_3)).wrapping_add(projectile_height_4) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let projectile_image_4 = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation_4 = r.get(projectile, field::projectile::ANIMATION)?.i();
        let remainder_value_3 = arithmetic::remainder_int(r, projectile_animation_4, 3)?;
        let projectile_facing_4 = r.get(projectile, field::projectile::FACING)?.i();
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image_4,
            (-16_i32).wrapping_mul(remainder_value_3),
            (-16_i32).wrapping_mul(projectile_facing_4),
            0,
        )?;
    }
    let projectile_kind_6 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_6 == 6 {
        let drawable_x_9 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_6 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_9.wrapping_sub(argument_2)).wrapping_add(projectile_width_6) <= 256) {
            return Ok(());
        }
        let drawable_y_9 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_6 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_9.wrapping_sub(argument_3)).wrapping_add(projectile_height_6) <= 208) {
            return Ok(());
        }
        let object_value = r.alloc("java/util/Random", Native::Empty);
        host::random_initialize(r, object_value)?;
        object_4 = object_value;
        let drawable_x_14 = r.get(projectile, field::drawable::X)?.i();
        let drawable_y_12 = r.get(projectile, field::drawable::Y)?.i();
        let random_next_int_value = host::random_next_int(r, object_4)?;
        host::random_set_seed(
            r,
            object_value,
            ((drawable_x_14.wrapping_mul(drawable_y_12)).wrapping_add(random_next_int_value))
                as i64,
        )?;
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let projectile_image_6 = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation_5 = r.get(projectile, field::projectile::ANIMATION)?.i();
        let remainder_value_4 = arithmetic::remainder_int(r, projectile_animation_5, 2)?;
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image_6,
            (-16_i32).wrapping_mul(remainder_value_4),
            0,
            0,
        )?;
        let projectile_state_t = r.get(projectile, field::projectile::STATE_T)?.i();
        if projectile_state_t > 12 {
            host::graphics_translate(r, graphics, 12, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let projectile_image_9 = r.get(projectile, field::projectile::IMAGE)?.r();
            let projectile_animation_7 = r.get(projectile, field::projectile::ANIMATION)?.i();
            let remainder_value_6 = arithmetic::remainder_int(r, projectile_animation_7, 3)?;
            host::graphics_draw_image(
                r,
                graphics,
                projectile_image_9,
                (-16_i32).wrapping_mul(remainder_value_6),
                0,
                0,
            )?;
        }
        let projectile_state_t_2 = r.get(projectile, field::projectile::STATE_T)?.i();
        if projectile_state_t_2 > 24 {
            host::graphics_translate(r, graphics, 12, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let projectile_image_12 = r.get(projectile, field::projectile::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, projectile_image_12, -32, 0, 0)?;
        }
        let projectile_state_t_3 = r.get(projectile, field::projectile::STATE_T)?.i();
        if projectile_state_t_3 > 36 {
            host::graphics_translate(r, graphics, 12, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let projectile_image_15 = r.get(projectile, field::projectile::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, projectile_image_15, -48, 0, 0)?;
        }
        let projectile_state_t_4 = r.get(projectile, field::projectile::STATE_T)?.i();
        if projectile_state_t_4 > 48 {
            host::graphics_translate(r, graphics, 12, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let projectile_image_17 = r.get(projectile, field::projectile::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, projectile_image_17, -64, 0, 0)?;
        }
    }
    let projectile_kind_7 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_7 == 7 {
        let drawable_x_11 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_8 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_11.wrapping_sub(argument_2)).wrapping_add(projectile_width_8) <= 256) {
            return Ok(());
        }
        let drawable_y_11 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_8 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_11.wrapping_sub(argument_3)).wrapping_add(projectile_height_8) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 25, 25)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 25, 25)?;
        let projectile_image_8 = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation_6 = r.get(projectile, field::projectile::ANIMATION)?.i();
        let remainder_value_5 = arithmetic::remainder_int(r, projectile_animation_6, 2)?;
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image_8,
            (-25_i32).wrapping_mul(remainder_value_5),
            0,
            0,
        )?;
    }
    let projectile_kind_8 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_8 == 8 {
        let drawable_x_13 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_10 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_13.wrapping_sub(argument_2)).wrapping_add(projectile_width_10) <= 256) {
            return Ok(());
        }
        let drawable_y_14 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_10 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_14.wrapping_sub(argument_3)).wrapping_add(projectile_height_10) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let projectile_image_11 = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation_8 = r.get(projectile, field::projectile::ANIMATION)?.i();
        let remainder_value_7 = arithmetic::remainder_int(r, projectile_animation_8, 4)?;
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image_11,
            (-16_i32).wrapping_mul(remainder_value_7),
            0,
            0,
        )?;
    }
    let projectile_kind_9 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_9 == 9 {
        let drawable_x_16 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_12 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_16.wrapping_sub(argument_2)).wrapping_add(projectile_width_12) <= 256) {
            return Ok(());
        }
        let drawable_y_16 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_12 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_16.wrapping_sub(argument_3)).wrapping_add(projectile_height_12) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
        let projectile_image_14 = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation_9 = r.get(projectile, field::projectile::ANIMATION)?.i();
        let remainder_value_8 =
            arithmetic::remainder_int(r, projectile_animation_9.wrapping_shr((1) as u32), 3)?;
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image_14,
            (-24_i32).wrapping_mul(remainder_value_8),
            0,
            0,
        )?;
    }
    let projectile_kind_10 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_10 == 10 {
        let drawable_x_17 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_13 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_17.wrapping_sub(argument_2)).wrapping_add(projectile_width_13) <= 256) {
            return Ok(());
        }
        let drawable_y_17 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_13 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_17.wrapping_sub(argument_3)).wrapping_add(projectile_height_13) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
        let projectile_image_16 = r.get(projectile, field::projectile::IMAGE)?.r();
        let projectile_animation_10 = r.get(projectile, field::projectile::ANIMATION)?.i();
        let remainder_value_9 =
            arithmetic::remainder_int(r, projectile_animation_10.wrapping_shr((1) as u32), 3)?;
        host::graphics_draw_image(
            r,
            graphics,
            projectile_image_16,
            (-24_i32).wrapping_mul(remainder_value_9),
            0,
            0,
        )?;
    }
    let projectile_kind_11 = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind_11 != 11 {
        super::projectile::reset_animation(r, projectile)?;
        return Ok(());
    }
    let projectile_facing_6 = r.get(projectile, field::projectile::FACING)?.i();
    if projectile_facing_6 == 0 {
        r.set(projectile, field::projectile::WIDTH, V::I(7))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(17))?;
        let drawable_x_18 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_14 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_18.wrapping_sub(argument_2)).wrapping_add(projectile_width_14) <= 256) {
            return Ok(());
        }
        let drawable_y_18 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_14 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_18.wrapping_sub(argument_3)).wrapping_add(projectile_height_14) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 7, 17)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 7, 17)?;
        let projectile_image_18 = r.get(projectile, field::projectile::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, projectile_image_18, -24, 0, 0)?;
    }
    let projectile_facing_7 = r.get(projectile, field::projectile::FACING)?.i();
    if projectile_facing_7 == 1 {
        r.set(projectile, field::projectile::WIDTH, V::I(17))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(7))?;
        let drawable_x_19 = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_15 = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_19.wrapping_sub(argument_2)).wrapping_add(projectile_width_15) <= 256) {
            return Ok(());
        }
        let drawable_y_19 = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_15 = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_19.wrapping_sub(argument_3)).wrapping_add(projectile_height_15) <= 208) {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 17, 7)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 17, 7)?;
        let projectile_image_19 = r.get(projectile, field::projectile::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, projectile_image_19, 0, 0, 0)?;
    }
    let projectile_facing_8 = r.get(projectile, field::projectile::FACING)?.i();
    if projectile_facing_8 != 2 {
        let projectile_facing_9_read = r.get(projectile, field::projectile::FACING)?.i();
        if projectile_facing_9_read != 3 {
            super::projectile::reset_animation(r, projectile)?;
            return Ok(());
        }
        r.set(projectile, field::projectile::WIDTH, V::I(17))?;
        r.set(projectile, field::projectile::HEIGHT, V::I(7))?;
        let drawable_x_21_read = r.get(projectile, field::drawable::X)?.i();
        let projectile_width_17_read = r.get(projectile, field::projectile::WIDTH)?.i();
        if !((drawable_x_21_read.wrapping_sub(argument_2)).wrapping_add(projectile_width_17_read)
            <= 256)
        {
            return Ok(());
        }
        let drawable_y_21_read = r.get(projectile, field::drawable::Y)?.i();
        let projectile_height_17_read = r.get(projectile, field::projectile::HEIGHT)?.i();
        if !((drawable_y_21_read.wrapping_sub(argument_3)).wrapping_add(projectile_height_17_read)
            <= 208)
        {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 17, 7)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 17, 7)?;
        let projectile_image_21_read = r.get(projectile, field::projectile::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, projectile_image_21_read, 0, -7, 0)?;
        super::projectile::reset_animation(r, projectile)?;
        return Ok(());
    }
    r.set(projectile, field::projectile::WIDTH, V::I(7))?;
    r.set(projectile, field::projectile::HEIGHT, V::I(17))?;
    let drawable_x_20 = r.get(projectile, field::drawable::X)?.i();
    let projectile_width_16 = r.get(projectile, field::projectile::WIDTH)?.i();
    if !((drawable_x_20.wrapping_sub(argument_2)).wrapping_add(projectile_width_16) <= 256) {
        return Ok(());
    }
    let drawable_y_20 = r.get(projectile, field::drawable::Y)?.i();
    let projectile_height_16 = r.get(projectile, field::projectile::HEIGHT)?.i();
    if !((drawable_y_20.wrapping_sub(argument_3)).wrapping_add(projectile_height_16) <= 208) {
        return Ok(());
    }
    host::graphics_set_clip(r, graphics, 0, 0, 7, 17)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 7, 17)?;
    let projectile_image_20 = r.get(projectile, field::projectile::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, projectile_image_20, -17, 0, 0)?;
    let projectile_facing_9_read = r.get(projectile, field::projectile::FACING)?.i();
    if projectile_facing_9_read != 3 {
        super::projectile::reset_animation(r, projectile)?;
        return Ok(());
    }
    r.set(projectile, field::projectile::WIDTH, V::I(17))?;
    r.set(projectile, field::projectile::HEIGHT, V::I(7))?;
    let drawable_x_21_read = r.get(projectile, field::drawable::X)?.i();
    let projectile_width_17_read = r.get(projectile, field::projectile::WIDTH)?.i();
    if !((drawable_x_21_read.wrapping_sub(argument_2)).wrapping_add(projectile_width_17_read)
        <= 256)
    {
        return Ok(());
    }
    let drawable_y_21_read = r.get(projectile, field::drawable::Y)?.i();
    let projectile_height_17_read = r.get(projectile, field::projectile::HEIGHT)?.i();
    if !((drawable_y_21_read.wrapping_sub(argument_3)).wrapping_add(projectile_height_17_read)
        <= 208)
    {
        return Ok(());
    }
    host::graphics_set_clip(r, graphics, 0, 0, 17, 7)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 17, 7)?;
    let projectile_image_21_read = r.get(projectile, field::projectile::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, projectile_image_21_read, 0, -7, 0)?;
    super::projectile::reset_animation(r, projectile)?;
    return Ok(());
}

/// Reset animation. Original reference: `p.a()V`.
pub(super) fn reset_animation(r: &mut Runtime, projectile: u32) -> Result<(), u32> {
    let drawable_x = r.get(projectile, field::drawable::X)?.i();
    let projectile_velocity_x = r.get(projectile, field::projectile::VELOCITY_X)?.i();
    let projectile_state_n = r.get(projectile, field::projectile::STATE_N)?.i();
    r.set(
        projectile,
        field::drawable::X,
        V::I(drawable_x.wrapping_add(projectile_velocity_x.wrapping_mul(projectile_state_n))),
    )?;
    let drawable_y = r.get(projectile, field::drawable::Y)?.i();
    let projectile_velocity_y = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
    let projectile_state_n_2 = r.get(projectile, field::projectile::STATE_N)?.i();
    r.set(
        projectile,
        field::drawable::Y,
        V::I(drawable_y.wrapping_add(projectile_velocity_y.wrapping_mul(projectile_state_n_2))),
    )?;
    let projectile_animation = r.get(projectile, field::projectile::ANIMATION)?.i();
    r.set(
        projectile,
        field::projectile::ANIMATION,
        V::I((projectile_animation.wrapping_add(1)) as i8 as i32),
    )?;
    let projectile_state_t = r.get(projectile, field::projectile::STATE_T)?.i();
    let projectile_state_n_3 = r.get(projectile, field::projectile::STATE_N)?.i();
    r.set(
        projectile,
        field::projectile::STATE_T,
        V::I(projectile_state_t.wrapping_add(projectile_state_n_3)),
    )?;
    let projectile_animation_2 = r.get(projectile, field::projectile::ANIMATION)?.i();
    if projectile_animation_2 >= 7 {
        r.set(projectile, field::projectile::ANIMATION, V::I(0))?;
    }
    let projectile_velocity_y_2 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
    if projectile_velocity_y_2 > 0 {
        r.set(projectile, field::projectile::FACING, V::I(2))?;
    }
    let projectile_velocity_y_3 = r.get(projectile, field::projectile::VELOCITY_Y)?.i();
    if projectile_velocity_y_3 < 0 {
        r.set(projectile, field::projectile::FACING, V::I(0))?;
    }
    let projectile_velocity_x_2 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
    if projectile_velocity_x_2 > 0 {
        r.set(projectile, field::projectile::FACING, V::I(1))?;
    }
    let projectile_velocity_x_3 = r.get(projectile, field::projectile::VELOCITY_X)?.i();
    if projectile_velocity_x_3 < 0 {
        r.set(projectile, field::projectile::FACING, V::I(3))?;
    }
    let projectile_kind = r.get(projectile, field::projectile::KIND)?.i();
    if projectile_kind != 11 {
        return Ok(());
    }
    r.set(projectile, field::projectile::FACING, V::I(0))?;
    return Ok(());
}
