//! Canvas settings.
use super::*;

/// Draw settings. Original reference: `k.b(Ljavax/microedition/lcdui/Graphics;)V`.
pub(in crate::game) fn draw_settings(
    r: &mut Runtime,
    canvas: u32,
    graphics: u32,
) -> Result<(), u32> {
    let mut number_2: i32 = 0;
    let mut number_3: i32 = 0;
    let mut number_4: i32 = 0;
    number_2 = 0_i32;
    let canvas_get_height_value = host::canvas_get_height(r, canvas)?;
    number_3 = canvas_get_height_value;
    if !(canvas_get_height_value <= 208_i32) {
        number_3 = 208_i32;
    }
    let canvas_zone = r.get(canvas, field::canvas::ZONE)?.i();
    match canvas_zone {
        0 => {
            let canvas_get_width_value = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_2 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value,
                canvas_get_height_value_2,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 65280_i32)?;
            let canvas_map = r.get(canvas, field::canvas::MAP)?.i();
            let remainder_value =
                arithmetic::remainder_int(r, canvas_map.wrapping_sub(1_i32), 16_i32)?;
            let canvas_map_2 = r.get(canvas, field::canvas::MAP)?.i();
            let quotient_value =
                arithmetic::divide_int(r, canvas_map_2.wrapping_sub(1_i32), 16_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                (2_i32).wrapping_add(remainder_value.wrapping_shl((1_i32) as u32)),
                (number_3.wrapping_sub(18_i32))
                    .wrapping_add(quotient_value.wrapping_shl((1_i32) as u32)),
                2_i32,
                2_i32,
            )?;
            host::graphics_set_color(r, graphics, 16711680_i32)?;
            let canvas_player = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces = r.get(canvas_player, field::player::TRIFORCE_PIECES)?.r();
            let element_value = r.array_get(player_triforce_pieces, 0_i32)?.i();
            if !(element_value != 0) {
                number_2 = 56_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_11 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_11 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_11.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_11.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_3 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_3 =
                r.get(canvas_player_3, field::player::TRIFORCE_PIECES)?.r();
            let element_value_3 = r.array_get(player_triforce_pieces_3, 1_i32)?.i();
            if !(element_value_3 != 0) {
                number_2 = 61_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_11 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_11 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_11.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_11.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_5 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_5 =
                r.get(canvas_player_5, field::player::TRIFORCE_PIECES)?.r();
            let element_value_5 = r.array_get(player_triforce_pieces_5, 2_i32)?.i();
            if !(element_value_5 != 0) {
                number_2 = 117_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_11 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_11 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_11.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_11.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_15 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_15 =
                r.get(canvas_player_15, field::player::TRIFORCE_PIECES)?.r();
            let element_value_15 = r.array_get(player_triforce_pieces_15, 3_i32)?.i();
            if !(element_value_15 != 0) {
                number_2 = 70_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_11 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_11 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_11.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_11.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_17 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_17 =
                r.get(canvas_player_17, field::player::TRIFORCE_PIECES)?.r();
            let element_value_17 = r.array_get(player_triforce_pieces_17, 4_i32)?.i();
            if !(element_value_17 != 0) {
                number_2 = 12_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_11 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_11 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_11.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_11.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_19 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_19 =
                r.get(canvas_player_19, field::player::TRIFORCE_PIECES)?.r();
            let element_value_35 = r.array_get(player_triforce_pieces_19, 5_i32)?.i();
            if !(element_value_35 != 0) {
                number_2 = 35_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_11 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_11 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_11.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_11.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_21 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_21 =
                r.get(canvas_player_21, field::player::TRIFORCE_PIECES)?.r();
            let element_value_37 = r.array_get(player_triforce_pieces_21, 6_i32)?.i();
            if element_value_37 != 0 {
                let canvas_player_23 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_triforce_pieces_23 =
                    r.get(canvas_player_23, field::player::TRIFORCE_PIECES)?.r();
                let element_value_39 = r.array_get(player_triforce_pieces_23, 7_i32)?.i();
                if element_value_39 != 0 {
                    number_2 = 6_i32;
                } else {
                    number_2 = 110_i32;
                }
            } else {
                number_2 = 67_i32;
            }

            if number_2 == 0 {
                return Ok(());
            }
            let remainder_value_11 =
                arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
            let quotient_value_11 =
                arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                (2_i32).wrapping_add(remainder_value_11.wrapping_shl((1_i32) as u32)),
                (number_3.wrapping_sub(18_i32))
                    .wrapping_add(quotient_value_11.wrapping_shl((1_i32) as u32)),
                2_i32,
                2_i32,
            )?;
            return Ok(());
        }
        1 => {
            let canvas_get_width_value_2 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_3 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_2,
                canvas_get_height_value_3,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 65280_i32)?;
            let canvas_map_3 = r.get(canvas, field::canvas::MAP)?.i();
            let remainder_value_2 =
                arithmetic::remainder_int(r, canvas_map_3.wrapping_sub(1_i32), 16_i32)?;
            let canvas_map_4 = r.get(canvas, field::canvas::MAP)?.i();
            let quotient_value_2 =
                arithmetic::divide_int(r, canvas_map_4.wrapping_sub(1_i32), 16_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                (2_i32).wrapping_add(remainder_value_2.wrapping_shl((1_i32) as u32)),
                (number_3.wrapping_sub(18_i32))
                    .wrapping_add(quotient_value_2.wrapping_shl((1_i32) as u32)),
                2_i32,
                2_i32,
            )?;
            host::graphics_set_color(r, graphics, 16711680_i32)?;
            let canvas_player_2 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_2 =
                r.get(canvas_player_2, field::player::TRIFORCE_PIECES)?.r();
            let element_value_2 = r.array_get(player_triforce_pieces_2, 0_i32)?.i();
            if !(element_value_2 != 0) {
                number_2 = 56_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_12 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_12 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_12.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_12.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_4 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_4 =
                r.get(canvas_player_4, field::player::TRIFORCE_PIECES)?.r();
            let element_value_4 = r.array_get(player_triforce_pieces_4, 1_i32)?.i();
            if !(element_value_4 != 0) {
                number_2 = 61_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_12 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_12 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_12.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_12.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_6 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_6 =
                r.get(canvas_player_6, field::player::TRIFORCE_PIECES)?.r();
            let element_value_6 = r.array_get(player_triforce_pieces_6, 2_i32)?.i();
            if !(element_value_6 != 0) {
                number_2 = 117_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_12 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_12 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_12.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_12.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_16 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_16 =
                r.get(canvas_player_16, field::player::TRIFORCE_PIECES)?.r();
            let element_value_16 = r.array_get(player_triforce_pieces_16, 3_i32)?.i();
            if !(element_value_16 != 0) {
                number_2 = 70_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_12 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_12 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_12.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_12.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_18 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_18 =
                r.get(canvas_player_18, field::player::TRIFORCE_PIECES)?.r();
            let element_value_18 = r.array_get(player_triforce_pieces_18, 4_i32)?.i();
            if !(element_value_18 != 0) {
                number_2 = 12_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_12 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_12 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_12.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_12.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_20 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_20 =
                r.get(canvas_player_20, field::player::TRIFORCE_PIECES)?.r();
            let element_value_36 = r.array_get(player_triforce_pieces_20, 5_i32)?.i();
            if !(element_value_36 != 0) {
                number_2 = 35_i32;

                if number_2 == 0 {
                    return Ok(());
                }
                let remainder_value_12 =
                    arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                let quotient_value_12 =
                    arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
                host::graphics_fill_rect(
                    r,
                    graphics,
                    (2_i32).wrapping_add(remainder_value_12.wrapping_shl((1_i32) as u32)),
                    (number_3.wrapping_sub(18_i32))
                        .wrapping_add(quotient_value_12.wrapping_shl((1_i32) as u32)),
                    2_i32,
                    2_i32,
                )?;
                return Ok(());
            }
            let canvas_player_22 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_triforce_pieces_22 =
                r.get(canvas_player_22, field::player::TRIFORCE_PIECES)?.r();
            let element_value_38 = r.array_get(player_triforce_pieces_22, 6_i32)?.i();
            if element_value_38 != 0 {
                let canvas_player_24 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_triforce_pieces_24 =
                    r.get(canvas_player_24, field::player::TRIFORCE_PIECES)?.r();
                let element_value_40 = r.array_get(player_triforce_pieces_24, 7_i32)?.i();
                if element_value_40 != 0 {
                    number_2 = 6_i32;
                } else {
                    number_2 = 110_i32;
                }
            } else {
                number_2 = 67_i32;
            }

            if number_2 == 0 {
                return Ok(());
            }
            let remainder_value_12 =
                arithmetic::remainder_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
            let quotient_value_12 =
                arithmetic::divide_int(r, number_2.wrapping_sub(1_i32), 16_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                (2_i32).wrapping_add(remainder_value_12.wrapping_shl((1_i32) as u32)),
                (number_3.wrapping_sub(18_i32))
                    .wrapping_add(quotient_value_12.wrapping_shl((1_i32) as u32)),
                2_i32,
                2_i32,
            )?;
            return Ok(());
        }
        3 => {
            let canvas_get_width_value_3 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_4 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_3,
                canvas_get_height_value_4,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 153_i32)?;
            number_2 = 0_i32;
            'draw_settings_loop_5: loop {
                if number_2 >= 6_i32 {
                    host::graphics_set_color(r, graphics, 65280_i32)?;
                    let canvas_map_5 = r.get(canvas, field::canvas::MAP)?.i();
                    let remainder_value_3 =
                        arithmetic::remainder_int(r, canvas_map_5.wrapping_sub(1_i32), 6_i32)?;
                    let canvas_map_6 = r.get(canvas, field::canvas::MAP)?.i();
                    let quotient_value_3 =
                        arithmetic::divide_int(r, canvas_map_6.wrapping_sub(1_i32), 6_i32)?;
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        (10_i32).wrapping_add(remainder_value_3.wrapping_shl((1_i32) as u32)),
                        (number_3.wrapping_sub(16_i32))
                            .wrapping_add(quotient_value_3.wrapping_shl((1_i32) as u32)),
                        2_i32,
                        2_i32,
                    )?;
                    host::graphics_set_color(r, graphics, 16711680_i32)?;
                    let canvas_player_7 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_triforce_pieces_7 =
                        r.get(canvas_player_7, field::player::TRIFORCE_PIECES)?.r();
                    let element_value_7 = r.array_get(player_triforce_pieces_7, 0_i32)?.i();
                    if element_value_7 != 0 {
                        return Ok(());
                    }
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        20_i32,
                        (number_3.wrapping_sub(16_i32)).wrapping_add(2_i32),
                        2_i32,
                        2_i32,
                    )?;
                    return Ok(());
                }
                number_4 = 0_i32;
                'draw_settings_loop_6: loop {
                    if number_4 >= 6_i32 {
                        break 'draw_settings_loop_6;
                    }
                    let canvas_state_az = r.get(canvas, field::canvas::STATE_AZ)?.r();
                    let element_value_19 = r.array_get(canvas_state_az, number_2)?.r();
                    let element_value_20 = r.array_get(element_value_19, number_4)?.i();
                    if !(element_value_20 != 1_i32) {
                        host::graphics_fill_rect(
                            r,
                            graphics,
                            (10_i32).wrapping_add(number_4.wrapping_shl((1_i32) as u32)),
                            ((number_2.wrapping_shl((1_i32) as u32)).wrapping_add(number_3))
                                .wrapping_sub(16_i32),
                            2_i32,
                            2_i32,
                        )?;
                    }
                    number_4 = number_4.wrapping_add(1);
                    continue 'draw_settings_loop_6;
                }
                number_2 = number_2.wrapping_add(1);
                continue 'draw_settings_loop_5;
            }
        }
        4 => {
            let canvas_get_width_value_4 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_5 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_4,
                canvas_get_height_value_5,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 153_i32)?;
            number_2 = 0_i32;
            'draw_settings_loop_10: loop {
                if number_2 >= 8_i32 {
                    host::graphics_set_color(r, graphics, 65280_i32)?;
                    let canvas_map_7 = r.get(canvas, field::canvas::MAP)?.i();
                    let remainder_value_4 =
                        arithmetic::remainder_int(r, canvas_map_7.wrapping_sub(1_i32), 4_i32)?;
                    let canvas_map_8 = r.get(canvas, field::canvas::MAP)?.i();
                    let quotient_value_4 =
                        arithmetic::divide_int(r, canvas_map_8.wrapping_sub(1_i32), 4_i32)?;
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        (10_i32).wrapping_add(remainder_value_4.wrapping_shl((1_i32) as u32)),
                        (number_3.wrapping_sub(18_i32))
                            .wrapping_add(quotient_value_4.wrapping_shl((1_i32) as u32)),
                        2_i32,
                        2_i32,
                    )?;
                    host::graphics_set_color(r, graphics, 16711680_i32)?;
                    let canvas_player_8 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_triforce_pieces_8 =
                        r.get(canvas_player_8, field::player::TRIFORCE_PIECES)?.r();
                    let element_value_8 = r.array_get(player_triforce_pieces_8, 1_i32)?.i();
                    if element_value_8 != 0 {
                        return Ok(());
                    }
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        12_i32,
                        number_3.wrapping_sub(18_i32),
                        2_i32,
                        2_i32,
                    )?;
                    return Ok(());
                }
                number_4 = 0_i32;
                'draw_settings_loop_11: loop {
                    if number_4 >= 4_i32 {
                        break 'draw_settings_loop_11;
                    }
                    let canvas_state_ba = r.get(canvas, field::canvas::STATE_BA)?.r();
                    let element_value_21 = r.array_get(canvas_state_ba, number_2)?.r();
                    let element_value_22 = r.array_get(element_value_21, number_4)?.i();
                    if !(element_value_22 != 1_i32) {
                        host::graphics_fill_rect(
                            r,
                            graphics,
                            (10_i32).wrapping_add(number_4.wrapping_shl((1_i32) as u32)),
                            ((number_2.wrapping_shl((1_i32) as u32)).wrapping_add(number_3))
                                .wrapping_sub(18_i32),
                            2_i32,
                            2_i32,
                        )?;
                    }
                    number_4 = number_4.wrapping_add(1);
                    continue 'draw_settings_loop_11;
                }
                number_2 = number_2.wrapping_add(1);
                continue 'draw_settings_loop_10;
            }
        }
        5 => {
            let canvas_get_width_value_5 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_6 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_5,
                canvas_get_height_value_6,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 153_i32)?;
            number_2 = 0_i32;
            'draw_settings_loop_15: loop {
                if number_2 >= 6_i32 {
                    host::graphics_set_color(r, graphics, 65280_i32)?;
                    let canvas_map_9 = r.get(canvas, field::canvas::MAP)?.i();
                    let remainder_value_5 =
                        arithmetic::remainder_int(r, canvas_map_9.wrapping_sub(1_i32), 5_i32)?;
                    let canvas_map_10 = r.get(canvas, field::canvas::MAP)?.i();
                    let quotient_value_5 =
                        arithmetic::divide_int(r, canvas_map_10.wrapping_sub(1_i32), 5_i32)?;
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        (10_i32).wrapping_add(remainder_value_5.wrapping_shl((1_i32) as u32)),
                        (number_3.wrapping_sub(16_i32))
                            .wrapping_add(quotient_value_5.wrapping_shl((1_i32) as u32)),
                        2_i32,
                        2_i32,
                    )?;
                    host::graphics_set_color(r, graphics, 16711680_i32)?;
                    let canvas_player_9 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_triforce_pieces_9 =
                        r.get(canvas_player_9, field::player::TRIFORCE_PIECES)?.r();
                    let element_value_9 = r.array_get(player_triforce_pieces_9, 2_i32)?.i();
                    if element_value_9 != 0 {
                        return Ok(());
                    }
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        18_i32,
                        (number_3.wrapping_sub(16_i32)).wrapping_add(2_i32),
                        2_i32,
                        2_i32,
                    )?;
                    return Ok(());
                }
                number_4 = 0_i32;
                'draw_settings_loop_16: loop {
                    if number_4 >= 5_i32 {
                        break 'draw_settings_loop_16;
                    }
                    let canvas_state_bb = r.get(canvas, field::canvas::STATE_BB)?.r();
                    let element_value_23 = r.array_get(canvas_state_bb, number_2)?.r();
                    let element_value_24 = r.array_get(element_value_23, number_4)?.i();
                    if !(element_value_24 != 1_i32) {
                        host::graphics_fill_rect(
                            r,
                            graphics,
                            (10_i32).wrapping_add(number_4.wrapping_shl((1_i32) as u32)),
                            ((number_2.wrapping_shl((1_i32) as u32)).wrapping_add(number_3))
                                .wrapping_sub(16_i32),
                            2_i32,
                            2_i32,
                        )?;
                    }
                    number_4 = number_4.wrapping_add(1);
                    continue 'draw_settings_loop_16;
                }
                number_2 = number_2.wrapping_add(1);
                continue 'draw_settings_loop_15;
            }
        }
        6 => {
            let canvas_get_width_value_6 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_7 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_6,
                canvas_get_height_value_7,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 153_i32)?;
            number_2 = 0_i32;
            'draw_settings_loop_20: loop {
                if number_2 >= 8_i32 {
                    host::graphics_set_color(r, graphics, 65280_i32)?;
                    let canvas_map_11 = r.get(canvas, field::canvas::MAP)?.i();
                    let remainder_value_6 =
                        arithmetic::remainder_int(r, canvas_map_11.wrapping_sub(1_i32), 4_i32)?;
                    let canvas_map_12 = r.get(canvas, field::canvas::MAP)?.i();
                    let quotient_value_6 =
                        arithmetic::divide_int(r, canvas_map_12.wrapping_sub(1_i32), 4_i32)?;
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        (10_i32).wrapping_add(remainder_value_6.wrapping_shl((1_i32) as u32)),
                        (number_3.wrapping_sub(16_i32))
                            .wrapping_add(quotient_value_6.wrapping_shl((1_i32) as u32)),
                        2_i32,
                        2_i32,
                    )?;
                    host::graphics_set_color(r, graphics, 16711680_i32)?;
                    let canvas_player_10 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_triforce_pieces_10 =
                        r.get(canvas_player_10, field::player::TRIFORCE_PIECES)?.r();
                    let element_value_10 = r.array_get(player_triforce_pieces_10, 3_i32)?.i();
                    if element_value_10 != 0 {
                        return Ok(());
                    }
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        16_i32,
                        number_3.wrapping_sub(16_i32),
                        2_i32,
                        2_i32,
                    )?;
                    return Ok(());
                }
                number_4 = 0_i32;
                'draw_settings_loop_21: loop {
                    if number_4 >= 4_i32 {
                        break 'draw_settings_loop_21;
                    }
                    let canvas_state_bc = r.get(canvas, field::canvas::STATE_BC)?.r();
                    let element_value_25 = r.array_get(canvas_state_bc, number_2)?.r();
                    let element_value_26 = r.array_get(element_value_25, number_4)?.i();
                    if !(element_value_26 != 1_i32) {
                        host::graphics_fill_rect(
                            r,
                            graphics,
                            (10_i32).wrapping_add(number_4.wrapping_shl((1_i32) as u32)),
                            ((number_2.wrapping_shl((1_i32) as u32)).wrapping_add(number_3))
                                .wrapping_sub(16_i32),
                            2_i32,
                            2_i32,
                        )?;
                    }
                    number_4 = number_4.wrapping_add(1);
                    continue 'draw_settings_loop_21;
                }
                number_2 = number_2.wrapping_add(1);
                continue 'draw_settings_loop_20;
            }
        }
        7 => {
            let canvas_get_width_value_7 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_8 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_7,
                canvas_get_height_value_8,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 153_i32)?;
            number_2 = 0_i32;
            'draw_settings_loop_25: loop {
                if number_2 >= 8_i32 {
                    host::graphics_set_color(r, graphics, 65280_i32)?;
                    let canvas_map_13 = r.get(canvas, field::canvas::MAP)?.i();
                    let remainder_value_7 =
                        arithmetic::remainder_int(r, canvas_map_13.wrapping_sub(1_i32), 4_i32)?;
                    let canvas_map_14 = r.get(canvas, field::canvas::MAP)?.i();
                    let quotient_value_7 =
                        arithmetic::divide_int(r, canvas_map_14.wrapping_sub(1_i32), 4_i32)?;
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        (10_i32).wrapping_add(remainder_value_7.wrapping_shl((1_i32) as u32)),
                        (number_3.wrapping_sub(16_i32))
                            .wrapping_add(quotient_value_7.wrapping_shl((1_i32) as u32)),
                        2_i32,
                        2_i32,
                    )?;
                    host::graphics_set_color(r, graphics, 16711680_i32)?;
                    let canvas_player_11 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_triforce_pieces_11 =
                        r.get(canvas_player_11, field::player::TRIFORCE_PIECES)?.r();
                    let element_value_11 = r.array_get(player_triforce_pieces_11, 4_i32)?.i();
                    if element_value_11 != 0 {
                        return Ok(());
                    }
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        10_i32,
                        (number_3.wrapping_sub(16_i32)).wrapping_add(2_i32),
                        2_i32,
                        2_i32,
                    )?;
                    return Ok(());
                }
                number_4 = 0_i32;
                'draw_settings_loop_26: loop {
                    if number_4 >= 4_i32 {
                        break 'draw_settings_loop_26;
                    }
                    let canvas_state_bd = r.get(canvas, field::canvas::STATE_BD)?.r();
                    let element_value_27 = r.array_get(canvas_state_bd, number_2)?.r();
                    let element_value_28 = r.array_get(element_value_27, number_4)?.i();
                    if !(element_value_28 != 1_i32) {
                        host::graphics_fill_rect(
                            r,
                            graphics,
                            (10_i32).wrapping_add(number_4.wrapping_shl((1_i32) as u32)),
                            ((number_2.wrapping_shl((1_i32) as u32)).wrapping_add(number_3))
                                .wrapping_sub(16_i32),
                            2_i32,
                            2_i32,
                        )?;
                    }
                    number_4 = number_4.wrapping_add(1);
                    continue 'draw_settings_loop_26;
                }
                number_2 = number_2.wrapping_add(1);
                continue 'draw_settings_loop_25;
            }
        }
        8 => {
            let canvas_get_width_value_8 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_9 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_8,
                canvas_get_height_value_9,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 153_i32)?;
            number_2 = 0_i32;
            'draw_settings_loop_30: loop {
                if number_2 >= 8_i32 {
                    host::graphics_set_color(r, graphics, 65280_i32)?;
                    let canvas_map_15 = r.get(canvas, field::canvas::MAP)?.i();
                    let remainder_value_8 =
                        arithmetic::remainder_int(r, canvas_map_15.wrapping_sub(1_i32), 6_i32)?;
                    let canvas_map_16 = r.get(canvas, field::canvas::MAP)?.i();
                    let quotient_value_8 =
                        arithmetic::divide_int(r, canvas_map_16.wrapping_sub(1_i32), 6_i32)?;
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        (10_i32).wrapping_add(remainder_value_8.wrapping_shl((1_i32) as u32)),
                        (number_3.wrapping_sub(16_i32))
                            .wrapping_add(quotient_value_8.wrapping_shl((1_i32) as u32)),
                        2_i32,
                        2_i32,
                    )?;
                    host::graphics_set_color(r, graphics, 16711680_i32)?;
                    let canvas_player_12 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_triforce_pieces_12 =
                        r.get(canvas_player_12, field::player::TRIFORCE_PIECES)?.r();
                    let element_value_12 = r.array_get(player_triforce_pieces_12, 5_i32)?.i();
                    if element_value_12 != 0 {
                        return Ok(());
                    }
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        18_i32,
                        number_3.wrapping_sub(16_i32),
                        2_i32,
                        2_i32,
                    )?;
                    return Ok(());
                }
                number_4 = 0_i32;
                'draw_settings_loop_31: loop {
                    if number_4 >= 6_i32 {
                        break 'draw_settings_loop_31;
                    }
                    let canvas_state_be = r.get(canvas, field::canvas::STATE_BE)?.r();
                    let element_value_29 = r.array_get(canvas_state_be, number_2)?.r();
                    let element_value_30 = r.array_get(element_value_29, number_4)?.i();
                    if !(element_value_30 != 1_i32) {
                        host::graphics_fill_rect(
                            r,
                            graphics,
                            (10_i32).wrapping_add(number_4.wrapping_shl((1_i32) as u32)),
                            ((number_2.wrapping_shl((1_i32) as u32)).wrapping_add(number_3))
                                .wrapping_sub(16_i32),
                            2_i32,
                            2_i32,
                        )?;
                    }
                    number_4 = number_4.wrapping_add(1);
                    continue 'draw_settings_loop_31;
                }
                number_2 = number_2.wrapping_add(1);
                continue 'draw_settings_loop_30;
            }
        }
        9 => {
            let canvas_get_width_value_9 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_10 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_9,
                canvas_get_height_value_10,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 153_i32)?;
            number_2 = 0_i32;
            'draw_settings_loop_35: loop {
                if number_2 >= 8_i32 {
                    host::graphics_set_color(r, graphics, 65280_i32)?;
                    let canvas_map_17 = r.get(canvas, field::canvas::MAP)?.i();
                    let remainder_value_9 =
                        arithmetic::remainder_int(r, canvas_map_17.wrapping_sub(1_i32), 6_i32)?;
                    let canvas_map_18 = r.get(canvas, field::canvas::MAP)?.i();
                    let quotient_value_9 =
                        arithmetic::divide_int(r, canvas_map_18.wrapping_sub(1_i32), 6_i32)?;
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        (10_i32).wrapping_add(remainder_value_9.wrapping_shl((1_i32) as u32)),
                        (number_3.wrapping_sub(16_i32))
                            .wrapping_add(quotient_value_9.wrapping_shl((1_i32) as u32)),
                        2_i32,
                        2_i32,
                    )?;
                    host::graphics_set_color(r, graphics, 16711680_i32)?;
                    let canvas_player_13 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_triforce_pieces_13 =
                        r.get(canvas_player_13, field::player::TRIFORCE_PIECES)?.r();
                    let element_value_13 = r.array_get(player_triforce_pieces_13, 6_i32)?.i();
                    if element_value_13 != 0 {
                        return Ok(());
                    }
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        16_i32,
                        (number_3.wrapping_sub(16_i32)).wrapping_add(4_i32),
                        2_i32,
                        2_i32,
                    )?;
                    return Ok(());
                }
                number_4 = 0_i32;
                'draw_settings_loop_36: loop {
                    if number_4 >= 6_i32 {
                        break 'draw_settings_loop_36;
                    }
                    let canvas_state_bf = r.get(canvas, field::canvas::STATE_BF)?.r();
                    let element_value_31 = r.array_get(canvas_state_bf, number_2)?.r();
                    let element_value_32 = r.array_get(element_value_31, number_4)?.i();
                    if !(element_value_32 != 1_i32) {
                        host::graphics_fill_rect(
                            r,
                            graphics,
                            (10_i32).wrapping_add(number_4.wrapping_shl((1_i32) as u32)),
                            ((number_2.wrapping_shl((1_i32) as u32)).wrapping_add(number_3))
                                .wrapping_sub(16_i32),
                            2_i32,
                            2_i32,
                        )?;
                    }
                    number_4 = number_4.wrapping_add(1);
                    continue 'draw_settings_loop_36;
                }
                number_2 = number_2.wrapping_add(1);
                continue 'draw_settings_loop_35;
            }
        }
        10 => {
            let canvas_get_width_value_10 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_11 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_10,
                canvas_get_height_value_11,
            )?;
            host::graphics_set_color(r, graphics, 0_i32)?;
            host::graphics_fill_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            host::graphics_draw_rect(
                r,
                graphics,
                2_i32,
                number_3.wrapping_sub(18_i32),
                32_i32,
                16_i32,
            )?;
            host::graphics_set_color(r, graphics, 153_i32)?;
            number_2 = 0_i32;
            'draw_settings_loop_40: loop {
                if number_2 >= 8_i32 {
                    host::graphics_set_color(r, graphics, 65280_i32)?;
                    let canvas_map_19 = r.get(canvas, field::canvas::MAP)?.i();
                    let remainder_value_10 =
                        arithmetic::remainder_int(r, canvas_map_19.wrapping_sub(1_i32), 5_i32)?;
                    let canvas_map_20 = r.get(canvas, field::canvas::MAP)?.i();
                    let quotient_value_10 =
                        arithmetic::divide_int(r, canvas_map_20.wrapping_sub(1_i32), 5_i32)?;
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        (10_i32).wrapping_add(remainder_value_10.wrapping_shl((1_i32) as u32)),
                        (number_3.wrapping_sub(16_i32))
                            .wrapping_add(quotient_value_10.wrapping_shl((1_i32) as u32)),
                        2_i32,
                        2_i32,
                    )?;
                    host::graphics_set_color(r, graphics, 16711680_i32)?;
                    let canvas_player_14 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_triforce_pieces_14 =
                        r.get(canvas_player_14, field::player::TRIFORCE_PIECES)?.r();
                    let element_value_14 = r.array_get(player_triforce_pieces_14, 7_i32)?.i();
                    if element_value_14 != 0 {
                        return Ok(());
                    }
                    host::graphics_fill_rect(
                        r,
                        graphics,
                        12_i32,
                        (number_3.wrapping_sub(16_i32)).wrapping_add(4_i32),
                        2_i32,
                        2_i32,
                    )?;
                    return Ok(());
                }
                number_4 = 0_i32;
                'draw_settings_loop_41: loop {
                    if number_4 >= 5_i32 {
                        break 'draw_settings_loop_41;
                    }
                    let canvas_state_bg = r.get(canvas, field::canvas::STATE_BG)?.r();
                    let element_value_33 = r.array_get(canvas_state_bg, number_2)?.r();
                    let element_value_34 = r.array_get(element_value_33, number_4)?.i();
                    if !(element_value_34 != 1_i32) {
                        host::graphics_fill_rect(
                            r,
                            graphics,
                            (10_i32).wrapping_add(number_4.wrapping_shl((1_i32) as u32)),
                            ((number_2.wrapping_shl((1_i32) as u32)).wrapping_add(number_3))
                                .wrapping_sub(16_i32),
                            2_i32,
                            2_i32,
                        )?;
                    }
                    number_4 = number_4.wrapping_add(1);
                    continue 'draw_settings_loop_41;
                }
                number_2 = number_2.wrapping_add(1);
                continue 'draw_settings_loop_40;
            }
        }
        _ => {
            return Ok(());
        }
    }
}
