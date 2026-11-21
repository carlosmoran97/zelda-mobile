//! World rendering.
use super::*;

/// Draw. Original reference: `j.a(Ljavax/microedition/lcdui/Graphics;IIIILh;)V`.
pub(in crate::game) fn draw(
    r: &mut Runtime,
    world: u32,
    graphics: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    player: u32,
) -> Result<(), u32> {
    let mut number_7: i32 = 0;
    let mut number_8: i32 = 0;
    let mut object_8: u32 = 0;
    let mut object_9: u32 = 0;
    let mut object_10: u32 = 0;
    let mut number_11: i32 = 0;
    let mut number_12: i32 = 0;
    let mut number_13: i32 = 0;
    let mut object_13: u32 = 0;
    let mut number_14: i32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    number_7 = 0;
    loop {
        let world_objects = r.get(world, field::world::OBJECTS)?.r();
        let vector_size_value = host::vector_size(r, world_objects)?;
        if number_7 >= vector_size_value {
            let player_state_a = r.get(player, field::player::STATE_A)?.i();
            if player_state_a != 0 {
                let player_overlay_sprite = r.get(player, field::player::OVERLAY_SPRITE)?.r();
                if player_overlay_sprite == 0 {
                    let world_state_c = r.get(world, field::world::STATE_C)?.r();
                    if world_state_c == 0 {
                        let world_canvas_3 = r.get(world, field::world::CANVAS)?.r();
                        let canvas_visual_effects_3 =
                            r.get(world_canvas_3, field::canvas::VISUAL_EFFECTS)?.i();
                        if canvas_visual_effects_3 == 0 {
                            let text_value = r.intern("/Images/Glow2.png");
                            outcome = (|| -> JResult {
                                Ok(V::R(host::image_create_image(r, text_value)?))
                            })();
                            (carry_0_r,) = (world,);
                            if outcome.is_err() {
                                carry_0_r = outcome.unwrap_err();
                                if r.is_instance(carry_0_r, "java/io/IOException") {
                                    return Ok(());
                                }
                                return Err(outcome.unwrap_err());
                            }
                            let image_create_image_value = outcome.unwrap().r();
                            r.set(
                                carry_0_r,
                                field::world::STATE_C,
                                V::R(image_create_image_value),
                            )?;
                        } else {
                            let text_value_2 = r.intern("/Images/Glow.png");
                            outcome = (|| -> JResult {
                                Ok(V::R(host::image_create_image(r, text_value_2)?))
                            })();
                            (carry_0_r,) = (world,);
                            if outcome.is_err() {
                                carry_0_r = outcome.unwrap_err();
                                if r.is_instance(carry_0_r, "java/io/IOException") {
                                    return Ok(());
                                }
                                return Err(outcome.unwrap_err());
                            }
                            let image_create_image_value_2 = outcome.unwrap().r();
                            r.set(
                                carry_0_r,
                                field::world::STATE_C,
                                V::R(image_create_image_value_2),
                            )?;
                        }
                    } else {
                        let world_state_c_3 = r.get(world, field::world::STATE_C)?.r();
                        r.set(player, field::player::OVERLAY_SPRITE, V::R(world_state_c_3))?;
                    }
                } else {
                    let world_canvas = r.get(world, field::world::CANVAS)?.r();
                    let canvas_visual_effects =
                        r.get(world_canvas, field::canvas::VISUAL_EFFECTS)?.i();
                    number_8 = canvas_visual_effects;
                    number_12 = y;
                    number_11 = x;
                    object_10 = graphics;
                    object_9 = player;
                    let graphics_get_translate_x_value =
                        host::graphics_get_translate_x(r, object_10)?;
                    let graphics_get_translate_y_value =
                        host::graphics_get_translate_y(r, object_10)?;
                    host::graphics_translate(
                        r,
                        object_10,
                        graphics_get_translate_x_value.wrapping_neg(),
                        graphics_get_translate_y_value.wrapping_neg(),
                    )?;
                    let drawable_x_2 = r.get(object_9, field::drawable::X)?.i();
                    let drawable_y = r.get(object_9, field::drawable::Y)?.i();
                    host::graphics_translate(
                        r,
                        object_10,
                        drawable_x_2.wrapping_sub(number_11),
                        drawable_y.wrapping_sub(number_12),
                    )?;
                    let player_animation = r.get(object_9, field::player::ANIMATION)?.i();
                    let player_animation_2 = r.get(object_9, field::player::ANIMATION)?.i();
                    host::graphics_translate(
                        r,
                        object_10,
                        (-32_i32).wrapping_add(player_animation.wrapping_shr((3) as u32)),
                        (-38_i32).wrapping_add(player_animation_2.wrapping_shr((3) as u32)),
                    )?;
                    host::graphics_set_clip(r, object_10, 0, 0, 96, 96)?;
                    host::graphics_clip_rect(r, object_10, 0, 0, 96, 96)?;
                    if number_8 == 0 {
                        let object_value = r.alloc("java/util/Random", Native::Empty);
                        host::random_initialize(r, object_value)?;
                        object_13 = object_value;
                        let player_overlay_sprite_2 =
                            r.get(object_9, field::player::OVERLAY_SPRITE)?.r();
                        let random_next_int_value = host::random_next_int(r, object_13)?;
                        let remainder_value =
                            arithmetic::remainder_int(r, random_next_int_value, 6)?;
                        let math_abs_value = host::math_abs(r, remainder_value)?;
                        host::graphics_draw_region(
                            r,
                            object_10,
                            player_overlay_sprite_2,
                            0,
                            0,
                            96,
                            96,
                            math_abs_value,
                            0,
                            0,
                            0,
                        )?;
                    } else {
                        let player_overlay_sprite_3 =
                            r.get(object_9, field::player::OVERLAY_SPRITE)?.r();
                        host::graphics_draw_image(r, object_10, player_overlay_sprite_3, 0, 0, 0)?;
                    }
                }
            }
            number_7 = 0;
            loop {
                let world_enemies = r.get(world, field::world::ENEMIES)?.r();
                let vector_size_value_2 = host::vector_size(r, world_enemies)?;
                if number_7 >= vector_size_value_2 {
                    number_7 = 0;
                    loop {
                        let world_projectiles = r.get(world, field::world::PROJECTILES)?.r();
                        let vector_size_value_3 = host::vector_size(r, world_projectiles)?;
                        if number_7 >= vector_size_value_3 {
                            return Ok(());
                        }
                        let world_projectiles_2 = r.get(world, field::world::PROJECTILES)?.r();
                        let vector_element_at_value_3 =
                            host::vector_element_at(r, world_projectiles_2, number_7)?;
                        host::check_cast(r, vector_element_at_value_3, "p")?;
                        object_8 = vector_element_at_value_3;
                        let kind_value_4 = r
                            .get(vector_element_at_value_3, field::projectile::KIND)?
                            .i();
                        if kind_value_4 == 11 {
                            let world_state_z_3 = r.get(world, field::world::STATE_Z)?.r();
                            if world_state_z_3 != 0 {
                                let world_state_z_4 = r.get(world, field::world::STATE_Z)?.r();
                                r.set(
                                    object_8,
                                    field::projectile::OVERLAY_IMAGE,
                                    V::R(world_state_z_4),
                                )?;
                                number_12 = y;
                                number_11 = x;
                                object_10 = graphics;
                                object_9 = object_8;
                                let graphics_get_translate_x_value_3 =
                                    host::graphics_get_translate_x(r, object_10)?;
                                let graphics_get_translate_y_value_3 =
                                    host::graphics_get_translate_y(r, object_10)?;
                                host::graphics_translate(
                                    r,
                                    object_10,
                                    graphics_get_translate_x_value_3.wrapping_neg(),
                                    graphics_get_translate_y_value_3.wrapping_neg(),
                                )?;
                                let drawable_x_6 = r.get(object_9, field::drawable::X)?.i();
                                let projectile_velocity_x =
                                    r.get(object_9, field::projectile::VELOCITY_X)?.i();
                                let projectile_state_n =
                                    r.get(object_9, field::projectile::STATE_N)?.i();
                                let drawable_y_5 = r.get(object_9, field::drawable::Y)?.i();
                                let projectile_velocity_y =
                                    r.get(object_9, field::projectile::VELOCITY_Y)?.i();
                                let projectile_state_n_2 =
                                    r.get(object_9, field::projectile::STATE_N)?.i();
                                host::graphics_translate(
                                    r,
                                    object_10,
                                    (drawable_x_6.wrapping_sub(number_11)).wrapping_sub(
                                        projectile_velocity_x.wrapping_mul(projectile_state_n),
                                    ),
                                    (drawable_y_5.wrapping_sub(number_12)).wrapping_sub(
                                        projectile_velocity_y.wrapping_mul(projectile_state_n_2),
                                    ),
                                )?;
                                host::graphics_translate(r, object_10, -2, 0)?;
                                host::graphics_set_clip(r, object_10, 0, 0, 13, 20)?;
                                host::graphics_clip_rect(r, object_10, 0, 0, 13, 20)?;
                                let projectile_animation_3 =
                                    r.get(object_9, field::projectile::ANIMATION)?.i();
                                let remainder_value_2 =
                                    arithmetic::remainder_int(r, projectile_animation_3, 2)?;
                                if remainder_value_2 != 0 {
                                    let projectile_overlay_image =
                                        r.get(object_9, field::projectile::OVERLAY_IMAGE)?.r();
                                    host::graphics_draw_region(
                                        r,
                                        object_10,
                                        projectile_overlay_image,
                                        0,
                                        0,
                                        13,
                                        20,
                                        2,
                                        0,
                                        0,
                                        0,
                                    )?;
                                } else {
                                    let projectile_overlay_image_2 =
                                        r.get(object_9, field::projectile::OVERLAY_IMAGE)?.r();
                                    host::graphics_draw_region(
                                        r,
                                        object_10,
                                        projectile_overlay_image_2,
                                        0,
                                        0,
                                        13,
                                        20,
                                        0,
                                        0,
                                        0,
                                        0,
                                    )?;
                                }
                            } else {
                                let text_value_6 = r.intern("/Images/ArrowLightEffect.png");
                                outcome = (|| -> JResult {
                                    Ok(V::R(host::image_create_image(r, text_value_6)?))
                                })();
                                (carry_0_r,) = (world,);
                                if outcome.is_err() {
                                    carry_0_r = outcome.unwrap_err();
                                    if r.is_instance(carry_0_r, "java/io/IOException") {
                                        return Ok(());
                                    }
                                    return Err(outcome.unwrap_err());
                                }
                                let image_create_image_value_6 = outcome.unwrap().r();
                                r.set(
                                    carry_0_r,
                                    field::world::STATE_Z,
                                    V::R(image_create_image_value_6),
                                )?;
                            }
                        }
                        number_7 = number_7.wrapping_add(1);
                    }
                }
                let world_enemies_2 = r.get(world, field::world::ENEMIES)?.r();
                let vector_element_at_value_2 =
                    host::vector_element_at(r, world_enemies_2, number_7)?;
                host::check_cast(r, vector_element_at_value_2, "l")?;
                object_8 = vector_element_at_value_2;
                let enemy_kind_2 = r.get(vector_element_at_value_2, field::enemy::KIND)?.i();
                if enemy_kind_2 == 23 {
                    let enemy_behavior_state = r.get(object_8, field::enemy::BEHAVIOR_STATE)?.i();
                    if enemy_behavior_state == 2 {
                        let world_state_y = r.get(world, field::world::STATE_Y)?.r();
                        if world_state_y != 0 {
                            let world_state_y_2 = r.get(world, field::world::STATE_Y)?.r();
                            r.set(object_8, field::enemy::OVERLAY_IMAGE, V::R(world_state_y_2))?;
                            number_12 = y;
                            number_11 = x;
                            object_10 = graphics;
                            object_9 = object_8;
                            let graphics_get_translate_x_value_2 =
                                host::graphics_get_translate_x(r, object_10)?;
                            let graphics_get_translate_y_value_2 =
                                host::graphics_get_translate_y(r, object_10)?;
                            host::graphics_translate(
                                r,
                                object_10,
                                graphics_get_translate_x_value_2.wrapping_neg(),
                                graphics_get_translate_y_value_2.wrapping_neg(),
                            )?;
                            let drawable_x_5 = r.get(object_9, field::drawable::X)?.i();
                            let drawable_y_3 = r.get(object_9, field::drawable::Y)?.i();
                            host::graphics_translate(
                                r,
                                object_10,
                                drawable_x_5.wrapping_sub(number_11),
                                drawable_y_3.wrapping_sub(number_12),
                            )?;
                            let enemy_facing = r.get(object_9, field::enemy::FACING)?.i();
                            if enemy_facing == 1 {
                                host::graphics_translate(r, object_10, -30, -12)?;
                                host::graphics_set_clip(r, object_10, 0, 0, 89, 72)?;
                                host::graphics_clip_rect(r, object_10, 0, 0, 89, 72)?;
                                let enemy_overlay_image_3 =
                                    r.get(object_9, field::enemy::OVERLAY_IMAGE)?.r();
                                host::graphics_draw_image(
                                    r,
                                    object_10,
                                    enemy_overlay_image_3,
                                    0,
                                    0,
                                    0,
                                )?;
                            }
                            let enemy_facing_2 = r.get(object_9, field::enemy::FACING)?.i();
                            if enemy_facing_2 == 3 {
                                let enemy_behavior_state_5 =
                                    r.get(object_9, field::enemy::BEHAVIOR_STATE)?.i();
                                if enemy_behavior_state_5 == 2 {
                                    host::graphics_translate(r, object_10, 30, 0)?;
                                }
                                host::graphics_translate(r, object_10, -30, -12)?;
                                host::graphics_set_clip(r, object_10, 0, 0, 89, 72)?;
                                host::graphics_clip_rect(r, object_10, 0, 0, 89, 72)?;
                                let enemy_overlay_image_4 =
                                    r.get(object_9, field::enemy::OVERLAY_IMAGE)?.r();
                                host::graphics_draw_image(
                                    r,
                                    object_10,
                                    enemy_overlay_image_4,
                                    -89,
                                    0,
                                    0,
                                )?;
                            }
                        } else {
                            let text_value_5 = r.intern("/Images/GanonSlash.png");
                            outcome = (|| -> JResult {
                                Ok(V::R(host::image_create_image(r, text_value_5)?))
                            })();
                            (carry_0_r,) = (world,);
                            if outcome.is_err() {
                                carry_0_r = outcome.unwrap_err();
                                if r.is_instance(carry_0_r, "java/io/IOException") {
                                    return Ok(());
                                }
                                return Err(outcome.unwrap_err());
                            }
                            let image_create_image_value_5 = outcome.unwrap().r();
                            r.set(
                                carry_0_r,
                                field::world::STATE_Y,
                                V::R(image_create_image_value_5),
                            )?;
                        }
                    }
                }
                let enemy_kind_3 = r.get(object_8, field::enemy::KIND)?.i();
                if enemy_kind_3 == 29 {
                    let enemy_behavior_state_2 = r.get(object_8, field::enemy::BEHAVIOR_STATE)?.i();
                    if enemy_behavior_state_2 == 7 {
                        super::enemy::draw_boss_overlay(r, object_8, graphics, x, y, player)?;
                    }
                }
                number_7 = number_7.wrapping_add(1);
            }
        }
        let world_objects_2 = r.get(world, field::world::OBJECTS)?.r();
        let vector_element_at_value = host::vector_element_at(r, world_objects_2, number_7)?;
        host::check_cast(r, vector_element_at_value, "n")?;
        object_8 = vector_element_at_value;
        let world_alternate_map = r.get(world, field::world::ALTERNATE_MAP)?.i();
        r.set(
            vector_element_at_value,
            field::map_object::STATE_C,
            V::I(world_alternate_map),
        )?;
        let drawable_x = r.get(player, field::drawable::X)?.i();
        r.set(object_8, field::map_object::STATE_H_UPPER, V::I(drawable_x))?;
        let map_object_kind = r.get(object_8, field::map_object::KIND)?.i();
        if map_object_kind == 8 {
            let map_object_overlay_image = r.get(object_8, field::map_object::OVERLAY_IMAGE)?.r();
            if map_object_overlay_image == 0 {
                let world_state_c_2 = r.get(world, field::world::STATE_C)?.r();
                if world_state_c_2 == 0 {
                    let world_canvas_4 = r.get(world, field::world::CANVAS)?.r();
                    let canvas_visual_effects_4 =
                        r.get(world_canvas_4, field::canvas::VISUAL_EFFECTS)?.i();
                    if canvas_visual_effects_4 == 0 {
                        let text_value_3 = r.intern("/Images/Glow2.png");
                        outcome = (|| -> JResult {
                            Ok(V::R(host::image_create_image(r, text_value_3)?))
                        })();
                        (carry_0_r,) = (world,);
                        if outcome.is_err() {
                            carry_0_r = outcome.unwrap_err();
                            if r.is_instance(carry_0_r, "java/io/IOException") {
                                return Ok(());
                            }
                            return Err(outcome.unwrap_err());
                        }
                        let image_create_image_value_3 = outcome.unwrap().r();
                        r.set(
                            carry_0_r,
                            field::world::STATE_C,
                            V::R(image_create_image_value_3),
                        )?;
                    } else {
                        let text_value_4 = r.intern("/Images/Glow.png");
                        outcome = (|| -> JResult {
                            Ok(V::R(host::image_create_image(r, text_value_4)?))
                        })();
                        (carry_0_r,) = (world,);
                        if outcome.is_err() {
                            carry_0_r = outcome.unwrap_err();
                            if r.is_instance(carry_0_r, "java/io/IOException") {
                                return Ok(());
                            }
                            return Err(outcome.unwrap_err());
                        }
                        let image_create_image_value_4 = outcome.unwrap().r();
                        r.set(
                            carry_0_r,
                            field::world::STATE_C,
                            V::R(image_create_image_value_4),
                        )?;
                    }
                } else {
                    let world_state_c_4 = r.get(world, field::world::STATE_C)?.r();
                    r.set(
                        object_8,
                        field::map_object::OVERLAY_IMAGE,
                        V::R(world_state_c_4),
                    )?;
                }
            } else {
                let world_canvas_2 = r.get(world, field::world::CANVAS)?.r();
                let canvas_visual_effects_2 =
                    r.get(world_canvas_2, field::canvas::VISUAL_EFFECTS)?.i();
                number_14 = canvas_visual_effects_2;
                number_13 = height;
                number_8 = width;
                number_12 = y;
                number_11 = x;
                object_10 = graphics;
                object_9 = object_8;
                let map_object_overlay_image_2 =
                    r.get(object_8, field::map_object::OVERLAY_IMAGE)?.r();
                if map_object_overlay_image_2 != 0 {
                    let drawable_x_3 = r.get(object_9, field::drawable::X)?.i();
                    if !(number_8.wrapping_add(number_11) < drawable_x_3.wrapping_sub(48)) {
                        let drawable_x_4 = r.get(object_9, field::drawable::X)?.i();
                        if !((drawable_x_4.wrapping_add(16)).wrapping_add(48) < number_11) {
                            let drawable_y_2 = r.get(object_9, field::drawable::Y)?.i();
                            if !(number_13.wrapping_add(number_12) < drawable_y_2.wrapping_sub(48))
                            {
                                let drawable_y_4 = r.get(object_9, field::drawable::Y)?.i();
                                if !(((drawable_y_4.wrapping_add(16)).wrapping_sub(32))
                                    .wrapping_add(48)
                                    < number_12)
                                {
                                    let map_object_solid =
                                        r.get(object_9, field::map_object::SOLID)?.i();
                                    if map_object_solid != 0 {
                                        let graphics_get_translate_x_value_4 =
                                            host::graphics_get_translate_x(r, object_10)?;
                                        let graphics_get_translate_y_value_4 =
                                            host::graphics_get_translate_y(r, object_10)?;
                                        host::graphics_translate(
                                            r,
                                            object_10,
                                            graphics_get_translate_x_value_4.wrapping_neg(),
                                            graphics_get_translate_y_value_4.wrapping_neg(),
                                        )?;
                                        let drawable_x_7 = r.get(object_9, field::drawable::X)?.i();
                                        let drawable_y_6 = r.get(object_9, field::drawable::Y)?.i();
                                        host::graphics_translate(
                                            r,
                                            object_10,
                                            drawable_x_7.wrapping_sub(number_11),
                                            drawable_y_6.wrapping_sub(number_12),
                                        )?;
                                        let map_object_kind_5 =
                                            r.get(object_9, field::map_object::KIND)?.i();
                                        if map_object_kind_5 == 8 {
                                            let map_object_state_b =
                                                r.get(object_9, field::map_object::STATE_B)?.i();
                                            let map_object_state_b_2 =
                                                r.get(object_9, field::map_object::STATE_B)?.i();
                                            host::graphics_translate(
                                                r,
                                                object_10,
                                                (-40_i32).wrapping_add(
                                                    map_object_state_b.wrapping_shr((3) as u32),
                                                ),
                                                (-40_i32).wrapping_add(
                                                    map_object_state_b_2.wrapping_shr((3) as u32),
                                                ),
                                            )?;
                                            host::graphics_set_clip(r, object_10, 0, 0, 96, 96)?;
                                            host::graphics_clip_rect(r, object_10, 0, 0, 96, 96)?;
                                            if number_14 != 0 {
                                                let overlay_image_value_5 = r
                                                    .get(
                                                        object_9,
                                                        field::map_object::OVERLAY_IMAGE,
                                                    )?
                                                    .r();
                                                host::graphics_draw_image(
                                                    r,
                                                    object_10,
                                                    overlay_image_value_5,
                                                    0,
                                                    0,
                                                    0,
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
        number_7 = number_7.wrapping_add(1);
    }
}

/// Draw dialogue. Original reference: `j.a(ILjavax/microedition/lcdui/Graphics;II)Z`.
pub(in crate::game) fn draw_dialogue(
    r: &mut Runtime,
    world: u32,
    mut object_index: i32,
    graphics: u32,
    mut width: i32,
    mut height: i32,
) -> Result<i32, u32> {
    let mut object_1: u32 = 0;
    let mut object_5: u32 = 0;
    let mut object_6: u32 = 0;
    let mut number_7: i32 = 0;
    let mut object_8: u32 = 0;
    let mut number_9: i32 = 0;
    let world_objects = r.get(world, field::world::OBJECTS)?.r();
    let vector_element_at_value = host::vector_element_at(r, world_objects, object_index)?;
    host::check_cast(r, vector_element_at_value, "n")?;
    object_1 = vector_element_at_value;

    if width > 256 {
        width = 256;
    }

    if height > 208 {
        height = 208;
    }
    let map_object_has_dialogue = r.get(object_1, field::map_object::HAS_DIALOGUE)?.i();
    if map_object_has_dialogue <= 0 {
        return Ok(1);
    }
    let world_dialogue = r.get(world, field::world::DIALOGUE)?.r();
    let map_object_text_index = r.get(object_1, field::map_object::TEXT_INDEX)?.i();
    let vector_element_at_value_2 =
        host::vector_element_at(r, world_dialogue, map_object_text_index)?;
    host::check_cast(r, vector_element_at_value_2, "java/lang/String")?;
    object_1 = vector_element_at_value_2;
    let world_text_frame = r.get(world, field::world::TEXT_FRAME)?.i();
    r.set(
        world,
        field::world::TEXT_FRAME,
        V::I(world_text_frame.wrapping_add(1)),
    )?;
    let object_value = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value)?;
    object_5 = object_value;
    object_6 = object_1;
    number_7 = 0;
    number_7 = 0;
    let graphics_get_font_value = host::graphics_get_font(r, graphics)?;
    object_8 = graphics_get_font_value;
    number_9 = 0;
    loop {
        let string_length_value = host::string_length(r, object_1)?;
        if number_9 >= string_length_value {
            let object_value_2 = r.alloc("java/lang/StringBuffer", Native::Empty);
            host::string_buffer_initialize(r, object_value_2)?;
            let string_substring_2_value = host::string_substring_2(r, object_6, 0, number_7)?;
            let string_buffer_append_2_value =
                host::string_buffer_append_2(r, object_value_2, string_substring_2_value)?;
            let text_value = r.intern(" ");
            let string_buffer_append_2_value_2 =
                host::string_buffer_append_2(r, string_buffer_append_2_value, text_value)?;
            let string_buffer_to_string_value =
                host::string_buffer_to_string(r, string_buffer_append_2_value_2)?;
            host::vector_add_element(r, object_5, string_buffer_to_string_value)?;
            let vector_size_value = host::vector_size(r, object_5)?;
            number_9 = vector_size_value;
            if vector_size_value > 7 {
                let world_text_scroll_2 = r.get(world, field::world::TEXT_SCROLL)?.i();
                r.set(
                    world,
                    field::world::TEXT_SCROLL,
                    V::I(world_text_scroll_2.wrapping_add(1)),
                )?;
                number_9 = 7;
                let vector_size_value_2 = host::vector_size(r, object_5)?;
                let world_text_scroll_3 = r.get(world, field::world::TEXT_SCROLL)?.i();
                let font_get_height_value_2 = host::font_get_height(r, object_8)?;
                let quotient_value_9 =
                    arithmetic::divide_int(r, world_text_scroll_3, font_get_height_value_2)?;
                if !(vector_size_value_2.wrapping_sub(quotient_value_9) >= 7) {
                    let world_text_scroll_6 = r.get(world, field::world::TEXT_SCROLL)?.i();
                    r.set(
                        world,
                        field::world::TEXT_SCROLL,
                        V::I(world_text_scroll_6.wrapping_sub(1)),
                    )?;
                }
            }
            let font_get_height_value = host::font_get_height(r, object_8)?;
            object_index = font_get_height_value.wrapping_mul(number_9);
            let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value.wrapping_neg(),
                graphics_get_translate_y_value.wrapping_neg(),
            )?;
            host::graphics_set_color(r, graphics, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, width, height)?;
            let quotient_value = arithmetic::divide_int(r, height, 3)?;
            let quotient_value_2 = arithmetic::divide_int(r, object_index, 2)?;
            host::graphics_fill_rect(
                r,
                graphics,
                10,
                (((quotient_value.wrapping_shl((1) as u32)).wrapping_sub(48)).wrapping_add(32))
                    .wrapping_sub(quotient_value_2),
                width.wrapping_sub(20),
                object_index.wrapping_add(10),
            )?;
            host::graphics_set_color(r, graphics, 16777215)?;
            let quotient_value_3 = arithmetic::divide_int(r, height, 3)?;
            let quotient_value_4 = arithmetic::divide_int(r, object_index, 2)?;
            host::graphics_draw_rect(
                r,
                graphics,
                10,
                (((quotient_value_3.wrapping_shl((1) as u32)).wrapping_sub(48)).wrapping_add(32))
                    .wrapping_sub(quotient_value_4),
                width.wrapping_sub(20),
                object_index.wrapping_add(10),
            )?;
            host::graphics_set_color(r, graphics, 11184810)?;
            let quotient_value_5 = arithmetic::divide_int(r, height, 3)?;
            let quotient_value_6 = arithmetic::divide_int(r, object_index, 2)?;
            host::graphics_draw_rect(
                r,
                graphics,
                11,
                (((quotient_value_5.wrapping_shl((1) as u32)).wrapping_sub(47)).wrapping_add(32))
                    .wrapping_sub(quotient_value_6),
                width.wrapping_sub(22),
                object_index.wrapping_add(8),
            )?;
            host::graphics_set_color(r, graphics, 16777215)?;
            let quotient_value_7 = arithmetic::divide_int(r, height, 3)?;
            let quotient_value_8 = arithmetic::divide_int(r, object_index, 2)?;
            host::graphics_translate(
                r,
                graphics,
                15,
                (((quotient_value_7.wrapping_shl((1) as u32)).wrapping_sub(44)).wrapping_add(32))
                    .wrapping_sub(quotient_value_8),
            )?;
            let world_text_scroll = r.get(world, field::world::TEXT_SCROLL)?.i();
            if world_text_scroll > 0 {
                let font_get_height_value_4 = host::font_get_height(r, object_8)?;
                host::graphics_translate(r, graphics, 0, font_get_height_value_4)?;
                let world_text_scroll_5 = r.get(world, field::world::TEXT_SCROLL)?.i();
                let font_get_height_value_5 = host::font_get_height(r, object_8)?;
                let remainder_value =
                    arithmetic::remainder_int(r, world_text_scroll_5, font_get_height_value_5)?;
                host::graphics_translate(r, graphics, 0, remainder_value.wrapping_neg())?;
            }
            number_7 = 0;
            object_index = 1;
            let world_text_scroll_4 = r.get(world, field::world::TEXT_SCROLL)?.i();
            let font_get_height_value_3 = host::font_get_height(r, object_8)?;
            let quotient_value_10 =
                arithmetic::divide_int(r, world_text_scroll_4, font_get_height_value_3)?;
            height = quotient_value_10;
            loop {
                let vector_size_value_3 = host::vector_size(r, object_5)?;
                if height >= vector_size_value_3 {
                    let world_text_frame_2_read = r.get(world, field::world::TEXT_FRAME)?.i();
                    let remainder_value_2 =
                        arithmetic::remainder_int(r, world_text_frame_2_read, 8)?;
                    if remainder_value_2 < 4 {
                        return Ok(1);
                    }

                    if object_index == 0 {
                        return Ok(1);
                    }
                    let font_get_height_value_7 = host::font_get_height(r, object_8)?;
                    host::graphics_translate(
                        r,
                        graphics,
                        width.wrapping_sub(38),
                        (font_get_height_value_7.wrapping_neg()).wrapping_add(8),
                    )?;
                    host::graphics_set_color(r, graphics, 33274)?;
                    host::graphics_fill_rect(r, graphics, 0, 0, 10, 10)?;
                    host::graphics_set_color(r, graphics, 16777215)?;
                    return Ok(1);
                }
                let vector_element_at_value_3 = host::vector_element_at(r, object_5, height)?;
                host::check_cast(r, vector_element_at_value_3, "java/lang/String")?;
                host::graphics_draw_string(r, graphics, vector_element_at_value_3, 0, 0, 20)?;
                let font_get_height_value_6 = host::font_get_height(r, object_8)?;
                host::graphics_translate(r, graphics, 0, font_get_height_value_6)?;
                number_7 = number_7.wrapping_add(1);
                if number_7 > 7 {
                    object_index = 0;
                    let world_text_frame_2_read = r.get(world, field::world::TEXT_FRAME)?.i();
                    let remainder_value_2 =
                        arithmetic::remainder_int(r, world_text_frame_2_read, 8)?;
                    if remainder_value_2 < 4 {
                        return Ok(1);
                    }

                    if object_index == 0 {
                        return Ok(1);
                    }
                    let font_get_height_value_7 = host::font_get_height(r, object_8)?;
                    host::graphics_translate(
                        r,
                        graphics,
                        width.wrapping_sub(38),
                        (font_get_height_value_7.wrapping_neg()).wrapping_add(8),
                    )?;
                    host::graphics_set_color(r, graphics, 33274)?;
                    host::graphics_fill_rect(r, graphics, 0, 0, 10, 10)?;
                    host::graphics_set_color(r, graphics, 16777215)?;
                    return Ok(1);
                }
                height = height.wrapping_add(1);
            }
        }
        let font_substring_width_value =
            host::font_substring_width(r, object_8, object_6, 0, number_7)?;
        if !(font_substring_width_value <= width.wrapping_sub(32)) {
            let string_substring_2_value_2 = host::string_substring_2(r, object_6, 0, number_7)?;
            host::vector_add_element(r, object_5, string_substring_2_value_2)?;

            let string_substring_value = host::string_substring(r, object_6, number_7)?;
            object_6 = string_substring_value;
            number_7 = 0;
        }
        number_7 = number_7.wrapping_add(1);
        number_9 = number_9.wrapping_add(1);
    }
}

/// Draw item message. Original reference: `j.a(Ljavax/microedition/lcdui/Graphics;III)Z`.
pub(in crate::game) fn draw_item_message(
    r: &mut Runtime,
    world: u32,
    graphics: u32,
    mut width: i32,
    mut height: i32,
    mut message_index: i32,
) -> Result<i32, u32> {
    let mut object_4: u32 = 0;
    let mut object_5: u32 = 0;
    let mut object_6: u32 = 0;
    let mut number_7: i32 = 0;
    let mut object_8: u32 = 0;
    let mut number_9: i32 = 0;
    let world_canvas = r.get(world, field::world::CANVAS)?.r();
    let canvas_item_messages = r.get(world_canvas, field::canvas::ITEM_MESSAGES)?.r();
    let element_value = r.array_get(canvas_item_messages, message_index)?.r();
    object_4 = element_value;

    if width > 256 {
        width = 256;
    }

    if height > 208 {
        height = 208;
    }
    let world_text_frame = r.get(world, field::world::TEXT_FRAME)?.i();
    r.set(
        world,
        field::world::TEXT_FRAME,
        V::I(world_text_frame.wrapping_add(1)),
    )?;
    let object_value = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value)?;
    object_5 = object_value;
    object_6 = object_4;
    number_7 = 0;
    number_7 = 0;
    let graphics_get_font_value = host::graphics_get_font(r, graphics)?;
    object_8 = graphics_get_font_value;
    number_9 = 0;
    loop {
        let string_length_value = host::string_length(r, object_4)?;
        if number_9 >= string_length_value {
            let object_value_2 = r.alloc("java/lang/StringBuffer", Native::Empty);
            host::string_buffer_initialize(r, object_value_2)?;
            let string_substring_2_value = host::string_substring_2(r, object_6, 0, number_7)?;
            let string_buffer_append_2_value =
                host::string_buffer_append_2(r, object_value_2, string_substring_2_value)?;
            let text_value = r.intern(" ");
            let string_buffer_append_2_value_2 =
                host::string_buffer_append_2(r, string_buffer_append_2_value, text_value)?;
            let string_buffer_to_string_value =
                host::string_buffer_to_string(r, string_buffer_append_2_value_2)?;
            host::vector_add_element(r, object_5, string_buffer_to_string_value)?;
            let vector_size_value = host::vector_size(r, object_5)?;
            number_9 = vector_size_value;
            if vector_size_value > 7 {
                let world_text_scroll_2 = r.get(world, field::world::TEXT_SCROLL)?.i();
                r.set(
                    world,
                    field::world::TEXT_SCROLL,
                    V::I(world_text_scroll_2.wrapping_add(1)),
                )?;
                number_9 = 7;
                let vector_size_value_2 = host::vector_size(r, object_5)?;
                let world_text_scroll_3 = r.get(world, field::world::TEXT_SCROLL)?.i();
                let font_get_height_value_2 = host::font_get_height(r, object_8)?;
                let quotient_value_9 =
                    arithmetic::divide_int(r, world_text_scroll_3, font_get_height_value_2)?;
                if !(vector_size_value_2.wrapping_sub(quotient_value_9) >= 7) {
                    let world_text_scroll_6 = r.get(world, field::world::TEXT_SCROLL)?.i();
                    r.set(
                        world,
                        field::world::TEXT_SCROLL,
                        V::I(world_text_scroll_6.wrapping_sub(1)),
                    )?;
                }
            }
            let font_get_height_value = host::font_get_height(r, object_8)?;
            message_index = font_get_height_value.wrapping_mul(number_9);
            let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value.wrapping_neg(),
                graphics_get_translate_y_value.wrapping_neg(),
            )?;
            host::graphics_set_color(r, graphics, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, width, height)?;
            let quotient_value = arithmetic::divide_int(r, height, 3)?;
            let quotient_value_2 = arithmetic::divide_int(r, message_index, 2)?;
            host::graphics_fill_rect(
                r,
                graphics,
                10,
                (((quotient_value.wrapping_shl((1) as u32)).wrapping_sub(48)).wrapping_add(32))
                    .wrapping_sub(quotient_value_2),
                width.wrapping_sub(20),
                message_index.wrapping_add(10),
            )?;
            host::graphics_set_color(r, graphics, 16777215)?;
            let quotient_value_3 = arithmetic::divide_int(r, height, 3)?;
            let quotient_value_4 = arithmetic::divide_int(r, message_index, 2)?;
            host::graphics_draw_rect(
                r,
                graphics,
                10,
                (((quotient_value_3.wrapping_shl((1) as u32)).wrapping_sub(48)).wrapping_add(32))
                    .wrapping_sub(quotient_value_4),
                width.wrapping_sub(20),
                message_index.wrapping_add(10),
            )?;
            host::graphics_set_color(r, graphics, 11184810)?;
            let quotient_value_5 = arithmetic::divide_int(r, height, 3)?;
            let quotient_value_6 = arithmetic::divide_int(r, message_index, 2)?;
            host::graphics_draw_rect(
                r,
                graphics,
                11,
                (((quotient_value_5.wrapping_shl((1) as u32)).wrapping_sub(47)).wrapping_add(32))
                    .wrapping_sub(quotient_value_6),
                width.wrapping_sub(22),
                message_index.wrapping_add(8),
            )?;
            host::graphics_set_color(r, graphics, 16777215)?;
            let quotient_value_7 = arithmetic::divide_int(r, height, 3)?;
            let quotient_value_8 = arithmetic::divide_int(r, message_index, 2)?;
            host::graphics_translate(
                r,
                graphics,
                15,
                (((quotient_value_7.wrapping_shl((1) as u32)).wrapping_sub(44)).wrapping_add(32))
                    .wrapping_sub(quotient_value_8),
            )?;
            let world_text_scroll = r.get(world, field::world::TEXT_SCROLL)?.i();
            if world_text_scroll > 0 {
                let font_get_height_value_4 = host::font_get_height(r, object_8)?;
                host::graphics_translate(r, graphics, 0, font_get_height_value_4)?;
                let world_text_scroll_5 = r.get(world, field::world::TEXT_SCROLL)?.i();
                let font_get_height_value_5 = host::font_get_height(r, object_8)?;
                let remainder_value =
                    arithmetic::remainder_int(r, world_text_scroll_5, font_get_height_value_5)?;
                host::graphics_translate(r, graphics, 0, remainder_value.wrapping_neg())?;
            }
            number_7 = 0;
            height = 1;
            let world_text_scroll_4 = r.get(world, field::world::TEXT_SCROLL)?.i();
            let font_get_height_value_3 = host::font_get_height(r, object_8)?;
            let quotient_value_10 =
                arithmetic::divide_int(r, world_text_scroll_4, font_get_height_value_3)?;
            message_index = quotient_value_10;
            loop {
                let vector_size_value_3 = host::vector_size(r, object_5)?;
                if message_index >= vector_size_value_3 {
                    let world_text_frame_2_read = r.get(world, field::world::TEXT_FRAME)?.i();
                    let remainder_value_2 =
                        arithmetic::remainder_int(r, world_text_frame_2_read, 8)?;
                    if remainder_value_2 < 4 {
                        return Ok(1);
                    }

                    if height == 0 {
                        return Ok(1);
                    }
                    let font_get_height_value_7 = host::font_get_height(r, object_8)?;
                    host::graphics_translate(
                        r,
                        graphics,
                        width.wrapping_sub(38),
                        (font_get_height_value_7.wrapping_neg()).wrapping_add(8),
                    )?;
                    host::graphics_set_color(r, graphics, 33274)?;
                    host::graphics_fill_rect(r, graphics, 0, 0, 10, 10)?;
                    host::graphics_set_color(r, graphics, 16777215)?;
                    return Ok(1);
                }
                let vector_element_at_value = host::vector_element_at(r, object_5, message_index)?;
                host::check_cast(r, vector_element_at_value, "java/lang/String")?;
                host::graphics_draw_string(r, graphics, vector_element_at_value, 0, 0, 20)?;
                let font_get_height_value_6 = host::font_get_height(r, object_8)?;
                host::graphics_translate(r, graphics, 0, font_get_height_value_6)?;
                number_7 = number_7.wrapping_add(1);
                if number_7 > 7 {
                    height = 0;
                    let world_text_frame_2_read = r.get(world, field::world::TEXT_FRAME)?.i();
                    let remainder_value_2 =
                        arithmetic::remainder_int(r, world_text_frame_2_read, 8)?;
                    if remainder_value_2 < 4 {
                        return Ok(1);
                    }

                    if height == 0 {
                        return Ok(1);
                    }
                    let font_get_height_value_7 = host::font_get_height(r, object_8)?;
                    host::graphics_translate(
                        r,
                        graphics,
                        width.wrapping_sub(38),
                        (font_get_height_value_7.wrapping_neg()).wrapping_add(8),
                    )?;
                    host::graphics_set_color(r, graphics, 33274)?;
                    host::graphics_fill_rect(r, graphics, 0, 0, 10, 10)?;
                    host::graphics_set_color(r, graphics, 16777215)?;
                    return Ok(1);
                }
                message_index = message_index.wrapping_add(1);
            }
        }
        let font_substring_width_value =
            host::font_substring_width(r, object_8, object_6, 0, number_7)?;
        if !(font_substring_width_value <= width.wrapping_sub(32)) {
            let string_substring_2_value_2 = host::string_substring_2(r, object_6, 0, number_7)?;
            host::vector_add_element(r, object_5, string_substring_2_value_2)?;

            let string_substring_value = host::string_substring(r, object_6, number_7)?;
            object_6 = string_substring_value;
            number_7 = 0;
        }
        number_7 = number_7.wrapping_add(1);
        number_9 = number_9.wrapping_add(1);
    }
}
