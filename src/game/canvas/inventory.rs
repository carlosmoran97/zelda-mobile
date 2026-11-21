//! Canvas inventory.
use super::*;

/// Set inventory open. Original reference: `k.a(Z)V`.
pub(in crate::game) fn set_inventory_open(
    r: &mut Runtime,
    canvas: u32,
    argument_1: i32,
) -> Result<(), u32> {
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    r.set(canvas, field::canvas::INVENTORY_OPEN, V::I(argument_1))?;
    if argument_1 == 0 {
        r.set(canvas, field::canvas::STATE_U_UPPER, V::R(0))?;
        r.set(canvas, field::canvas::STATE_Z, V::R(0))?;
        r.set(canvas, field::canvas::STATE_AA, V::R(0))?;
        r.set(canvas, field::canvas::STATE_AB, V::R(0))?;
        r.set(canvas, field::canvas::STATE_AC, V::R(0))?;
        host::system_gc(r)?;
        return Ok(());
    }
    let canvas_state_u_upper = r.get(canvas, field::canvas::STATE_U_UPPER)?.r();
    if canvas_state_u_upper != 0 {
        return Ok(());
    }
    let text_value = r.intern("/Images/Menuitems.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value)?)) })();
    (carry_0_r,) = (canvas,);
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
        field::canvas::STATE_U_UPPER,
        V::R(image_create_image_value),
    )?;
    let text_value_2 = r.intern("/Images/Menu01.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_2)?)) })();
    (carry_0_r,) = (canvas,);
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
        field::canvas::STATE_Z,
        V::R(image_create_image_value_2),
    )?;
    let text_value_3 = r.intern("/Images/MenuSelection01.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_3)?)) })();
    (carry_0_r,) = (canvas,);
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
        field::canvas::STATE_AA,
        V::R(image_create_image_value_3),
    )?;
    let text_value_4 = r.intern("/Images/MenuSelectionArrows.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_4)?)) })();
    (carry_0_r,) = (canvas,);
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
        field::canvas::STATE_AB,
        V::R(image_create_image_value_4),
    )?;
    r.set(canvas, field::canvas::REDRAW, V::I(1))?;
    return Ok(());
}

/// Load inventory images. Original reference: `k.b()V`.
pub(in crate::game) fn load_inventory_images(r: &mut Runtime, canvas: u32) -> Result<(), u32> {
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let canvas_inventory_open = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
    if canvas_inventory_open == 0 {
        return Ok(());
    }
    let canvas_inventory_page = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
    if canvas_inventory_page == 1 {
        r.set(canvas, field::canvas::STATE_Z, V::R(0))?;
        host::system_gc(r)?;
        let text_value = r.intern("/Images/Menu01.png");
        outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value)?)) })();
        (carry_0_r,) = (canvas,);
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
            field::canvas::STATE_Z,
            V::R(image_create_image_value),
        )?;
    }
    let canvas_inventory_page_2 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
    if canvas_inventory_page_2 == 2 {
        r.set(canvas, field::canvas::STATE_Z, V::R(0))?;
        host::system_gc(r)?;
        let text_value_2 = r.intern("/Images/Menu02.png");
        outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_2)?)) })();
        (carry_0_r,) = (canvas,);
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
            field::canvas::STATE_Z,
            V::R(image_create_image_value_2),
        )?;
    }
    let canvas_inventory_page_3 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
    if canvas_inventory_page_3 == 3 {
        r.set(canvas, field::canvas::STATE_Z, V::R(0))?;
        r.set(canvas, field::canvas::STATE_AC, V::R(0))?;
        host::system_gc(r)?;
        let canvas_zone = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone == 0 {
            let text_value_6 = r.intern("/Images/OverworldMap.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_6)?)) })();
            (carry_0_r,) = (canvas,);
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
                field::canvas::STATE_Z,
                V::R(image_create_image_value_6),
            )?;
        } else {
            let canvas_zone_2 = r.get(canvas, field::canvas::ZONE)?.i();
            if canvas_zone_2 == 1 {
                let text_value_6 = r.intern("/Images/OverworldMap.png");
                outcome =
                    (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_6)?)) })();
                (carry_0_r,) = (canvas,);
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
                    field::canvas::STATE_Z,
                    V::R(image_create_image_value_6),
                )?;
            }
        }
        let canvas_zone_3 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_3 == 3 {
            let text_value_7 = r.intern("/Images/Dungeon01.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_7)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_7 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_7),
            )?;
        }
        let canvas_zone_4 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_4 == 4 {
            let text_value_8 = r.intern("/Images/Dungeon02.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_8)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_8 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_8),
            )?;
        }
        let canvas_zone_5 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_5 == 5 {
            let text_value_9 = r.intern("/Images/Dungeon03.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_9)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_9 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_9),
            )?;
        }
        let canvas_zone_6 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_6 == 6 {
            let text_value_10 = r.intern("/Images/Dungeon04.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_10)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_10 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_10),
            )?;
        }
        let canvas_zone_7 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_7 == 7 {
            let text_value_11 = r.intern("/Images/Dungeon05.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_11)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_11 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_11),
            )?;
        }
        let canvas_zone_8 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_8 == 8 {
            let text_value_12 = r.intern("/Images/Dungeon06.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_12)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_12 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_12),
            )?;
        }
        let canvas_zone_9 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_9 == 9 {
            let text_value_13 = r.intern("/Images/Dungeon07.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_13)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_13 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_13),
            )?;
        }
        let canvas_zone_10 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_10 == 10 {
            let text_value_14 = r.intern("/Images/Dungeon08.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_14)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_14 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_14),
            )?;
        }
        let canvas_zone_11 = r.get(canvas, field::canvas::ZONE)?.i();
        if canvas_zone_11 == 11 {
            let text_value_16 = r.intern("/Images/Dungeon09.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_16)?)) })();
            (carry_0_r,) = (canvas,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(());
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value_16 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Z,
                V::R(image_create_image_value_16),
            )?;
        }
        let text_value_15 = r.intern("/Images/MapItems.png");
        outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_15)?)) })();
        (carry_0_r,) = (canvas,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if r.is_instance(carry_0_r, "java/io/IOException") {
                return Ok(());
            }
            return Err(outcome.unwrap_err());
        }
        let image_create_image_value_15 = outcome.unwrap().r();
        r.set(
            carry_0_r,
            field::canvas::STATE_AC,
            V::R(image_create_image_value_15),
        )?;
    }
    let canvas_inventory_page_4 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
    if canvas_inventory_page_4 != 4 {
        r.set(canvas, field::canvas::REDRAW, V::I(1))?;
        return Ok(());
    }
    r.set(canvas, field::canvas::STATE_AA, V::R(0))?;
    r.set(canvas, field::canvas::STATE_AB, V::R(0))?;
    r.set(canvas, field::canvas::STATE_AD, V::R(0))?;
    host::system_gc(r)?;
    let text_value_3 = r.intern("/Images/Menu03.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_3)?)) })();
    (carry_0_r,) = (canvas,);
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
        field::canvas::STATE_AA,
        V::R(image_create_image_value_3),
    )?;
    let text_value_4 = r.intern("/Images/MenuLink.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_4)?)) })();
    (carry_0_r,) = (canvas,);
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
        field::canvas::STATE_AB,
        V::R(image_create_image_value_4),
    )?;
    let text_value_5 = r.intern("/Images/Life2.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_5)?)) })();
    (carry_0_r,) = (canvas,);
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
        field::canvas::STATE_AD,
        V::R(image_create_image_value_5),
    )?;
    r.set(canvas, field::canvas::REDRAW, V::I(1))?;
    return Ok(());
}

/// Draw inventory. Original reference: `k.a(Ljavax/microedition/lcdui/Graphics;Z)V`.
#[rustfmt::skip]
pub(in crate::game) fn draw_inventory(
    r: &mut Runtime,
    canvas: u32,
    graphics: u32,
    argument_2: i32,
) -> Result<(), u32> {
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    'check_canvas_zone_2: {
    'check_canvas_running_6: {
    'check_canvas_state_aq_upper_6: {
    'handle_result_9: {
    'handle_result_7: {
    'handle_result_8: {
    'check_canvas_running_3: {
    'check_canvas_state_aq_upper_3: {
    'graphics_get_translate_x: {
    'check_canvas_screen_state: {
    'check_canvas_state_v: {
    'next_step: {
    'check_canvas_running_8: {
    'check_canvas_state_aq_upper_8: {
    'handle_result_4: {
    'handle_result_11: {
    'handle_result_10: {
    'check_canvas_running_7: {
    'check_canvas_state_aq_upper_7: {
    'check_canvas_state_w_upper_3: {
    'check_canvas_map_6: {
    'check_canvas_running_5: {
    'check_canvas_state_aq_upper_5: {
    'handle_result_6: {
    'handle_result_5: {
    'check_canvas_running_2: {
    'check_canvas_state_aq_upper_2: {
    'check_canvas_state_w_upper_2: {
    'check_canvas_map_4: {
    'check_canvas_map_2: {
    'check_canvas_zone: {
    'next_step_2: {
    'check_canvas_running_10: {
    'check_canvas_state_aq_upper_10: {
    'handle_result: {
    'handle_result_13: {
    'handle_result_12: {
    'check_canvas_running_9: {
    'check_canvas_state_aq_upper_9: {
    'check_canvas_state_w_upper_4: {
    'check_canvas_map_10: {
    'check_canvas_map_8: {
    'check_canvas_running_4: {
    'check_canvas_state_aq_upper_4: {
    'handle_result_3: {
    'handle_result_2: {
    'check_canvas_running: {
    'check_canvas_state_aq_upper: {
    'check_canvas_state_w_upper: {
    'check_canvas_map_19: {
    'check_canvas_map_18: {
    'check_canvas_map_17: {
    'check_canvas_map_16: {
    'check_canvas_map_15: {
    'check_canvas_map_14: {
    'check_canvas_map_13: {
    'check_canvas_map_12: {
    'check_canvas_map_11: {
    'check_canvas_map_9: {
    'check_canvas_map_7: {
    'check_canvas_map_5: {
    'check_canvas_map_3: {
    'check_canvas_map: {
    let canvas_state_ao_upper = r.get(canvas, field::canvas::STATE_AO_UPPER)?.i();
    if canvas_state_ao_upper == 0 {
return Ok(());
}
    let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value.wrapping_neg(), graphics_get_translate_y_value.wrapping_neg())?;
    host::graphics_translate(r, graphics, 0, 32)?;
    let canvas_state_l_upper = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper, canvas_state_m)?;
    let canvas_state_l_upper_2 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_2 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_2, canvas_state_m_2)?;
    let canvas_zone = r.get(canvas, field::canvas::ZONE)?.i();
    if canvas_zone != 0 { break 'check_canvas_zone; }
    } // check_canvas_map
    let canvas_map = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map == 12 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_3
    let canvas_map_3 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_3 == 81 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_5
    let canvas_map_5 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_5 == 82 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_7
    let canvas_map_7 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_7 == 83 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_9
    let canvas_map_9 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_9 == 84 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_11
    let canvas_map_11 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_11 == 97 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_12
    let canvas_map_12 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_12 == 98 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_13
    let canvas_map_13 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_13 == 99 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_14
    let canvas_map_14 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_14 == 100 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_15
    let canvas_map_15 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_15 == 113 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_16
    let canvas_map_16 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_16 == 114 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_17
    let canvas_map_17 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_17 == 115 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_18
    let canvas_map_18 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_18 == 116 { break 'check_canvas_state_w_upper; }
    } // check_canvas_map_19
    let canvas_map_19 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_19 != 117 { break 'check_canvas_map_8; }
    } // check_canvas_state_w_upper
    let canvas_state_w_upper_2 = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
    if canvas_state_w_upper_2 == 0 { break 'check_canvas_running; }
    } // check_canvas_state_aq_upper
    let canvas_state_aq_upper_2 = r.get(canvas, field::canvas::STATE_AQ_UPPER)?.i();
    if canvas_state_aq_upper_2 == 1 { break 'check_canvas_running_4; }
    } // check_canvas_running
    let canvas_visual_effects_3 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_3 == 0 { break 'check_canvas_state_aq_upper_4; }
    } // handle_result_2
    let text_value_2 = r.intern("/Images/filter1.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_2)?)) })();
    (carry_0_r,) = (canvas,);
    if outcome.is_err() { break 'handle_result; }
    } // handle_result_3
    let image_create_image_value_2 = outcome.unwrap().r();
    r.set(carry_0_r, field::canvas::STATE_W_UPPER, V::R(image_create_image_value_2))?;
    } // check_canvas_state_aq_upper_4
    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(1))?;
    } // check_canvas_running_4
    let canvas_visual_effects_6 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_6 != 0 {
let canvas_state_w_upper_7 = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
let canvas_state_p_2 = r.get(canvas, field::canvas::STATE_P)?.i();
let canvas_state_q_2 = r.get(canvas, field::canvas::STATE_Q)?.i();
host::graphics_draw_image(r, graphics, canvas_state_w_upper_7, canvas_state_p_2.wrapping_neg(), canvas_state_q_2.wrapping_neg(), 0)?;
}
    } // check_canvas_map_8
    let canvas_map_8 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_8 == 58 { break 'check_canvas_state_w_upper_4; }
    } // check_canvas_map_10
    let canvas_map_10 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_10 != 68 { break 'next_step_2; }
    } // check_canvas_state_w_upper_4
    let canvas_state_w_upper_10 = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
    if canvas_state_w_upper_10 == 0 { break 'check_canvas_running_9; }
    } // check_canvas_state_aq_upper_9
    let canvas_state_aq_upper_6 = r.get(canvas, field::canvas::STATE_AQ_UPPER)?.i();
    if canvas_state_aq_upper_6 == 3 { break 'check_canvas_running_10; }
    } // check_canvas_running_9
    let canvas_visual_effects_11 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_11 == 0 { break 'check_canvas_state_aq_upper_10; }
    } // handle_result_12
    let text_value_6 = r.intern("/Images/Filter4.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_6)?)) })();
    (carry_0_r,) = (canvas,);
    if outcome.is_err() { break 'handle_result; }
    } // handle_result_13
    let image_create_image_value_6 = outcome.unwrap().r();
    r.set(carry_0_r, field::canvas::STATE_W_UPPER, V::R(image_create_image_value_6))?;
    break 'check_canvas_state_aq_upper_10;
    } // handle_result
    carry_0_r = outcome.unwrap_err();
    if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
    r.set(canvas, field::canvas::STATE_W_UPPER, V::R(0))?;
    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AO_UPPER, V::I(0))?;
    break 'check_canvas_zone;
    } // check_canvas_state_aq_upper_10
    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(3))?;
    } // check_canvas_running_10
    let canvas_visual_effects_12 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_12 != 0 {
let canvas_state_w_upper_12 = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
let canvas_state_p_5 = r.get(canvas, field::canvas::STATE_P)?.i();
let canvas_state_q_5 = r.get(canvas, field::canvas::STATE_Q)?.i();
host::graphics_draw_image(r, graphics, canvas_state_w_upper_12, canvas_state_p_5.wrapping_neg(), canvas_state_q_5.wrapping_neg(), 0)?;
}
    } // next_step_2
    } // check_canvas_zone
    let canvas_zone_2 = r.get(canvas, field::canvas::ZONE)?.i();
    if canvas_zone_2 != 100 { break 'check_canvas_state_v; }
    } // check_canvas_map_2
    let canvas_map_2 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_2 == 1 { break 'check_canvas_state_w_upper_2; }
    } // check_canvas_map_4
    let canvas_map_4 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_4 != 2 { break 'check_canvas_map_6; }
    } // check_canvas_state_w_upper_2
    let canvas_state_w_upper_3 = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
    if canvas_state_w_upper_3 == 0 { break 'check_canvas_running_2; }
    } // check_canvas_state_aq_upper_2
    let canvas_state_aq_upper_3 = r.get(canvas, field::canvas::STATE_AQ_UPPER)?.i();
    if canvas_state_aq_upper_3 == 2 { break 'check_canvas_running_5; }
    } // check_canvas_running_2
    let canvas_visual_effects_4 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_4 == 0 { break 'check_canvas_state_aq_upper_5; }
    } // handle_result_5
    let text_value_3 = r.intern("/Images/filter1.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_3)?)) })();
    (carry_0_r,) = (canvas,);
    if outcome.is_err() { break 'handle_result_4; }
    } // handle_result_6
    let image_create_image_value_3 = outcome.unwrap().r();
    r.set(carry_0_r, field::canvas::STATE_W_UPPER, V::R(image_create_image_value_3))?;
    } // check_canvas_state_aq_upper_5
    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(2))?;
    } // check_canvas_running_5
    let canvas_visual_effects_7 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_7 != 0 {
let canvas_state_w_upper_8 = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
let canvas_state_p_3 = r.get(canvas, field::canvas::STATE_P)?.i();
let canvas_state_q_3 = r.get(canvas, field::canvas::STATE_Q)?.i();
host::graphics_draw_image(r, graphics, canvas_state_w_upper_8, canvas_state_p_3.wrapping_neg(), canvas_state_q_3.wrapping_neg(), 0)?;
}
    } // check_canvas_map_6
    let canvas_map_6 = r.get(canvas, field::canvas::MAP)?.i();
    if canvas_map_6
        != 3
    {
        break 'next_step;
    }
    } // check_canvas_state_w_upper_3
    let canvas_state_w_upper_6 = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
    if canvas_state_w_upper_6 == 0 { break 'check_canvas_running_7; }
    } // check_canvas_state_aq_upper_7
    let canvas_state_aq_upper_5 = r.get(canvas, field::canvas::STATE_AQ_UPPER)?.i();
    if canvas_state_aq_upper_5 == 3 { break 'check_canvas_running_8; }
    } // check_canvas_running_7
    let canvas_visual_effects_9 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_9 == 0 {
        break 'check_canvas_state_aq_upper_8;
    }
    } // handle_result_10
    let text_value_5 = r.intern(
        "/Images/Filter4.png",
    );
    outcome = (|| -> JResult {
        Ok(V::R(host::image_create_image(r, text_value_5)?))
    })(
    );
    (carry_0_r,) = (canvas,);
    if outcome.is_err() {
        break 'handle_result_4;
    }
    } // handle_result_11
    let image_create_image_value_5 =
        outcome.unwrap().r();
    r.set(
        carry_0_r,
        field::canvas::STATE_W_UPPER,
        V::R(
            image_create_image_value_5,
        ),
    )?;
    break 'check_canvas_state_aq_upper_8;
    } // handle_result_4
    carry_0_r = outcome.unwrap_err();
    if !(r.is_instance(
        carry_0_r,
        "java/io/IOException",
    )) {
        return Err(outcome.unwrap_err());
    }
    r.set(
        canvas,
        field::canvas::STATE_W_UPPER,
        V::R(0),
    )?;
    r.set(
        canvas,
        field::canvas::STATE_AQ_UPPER,
        V::I(0),
    )?;
    r.set(
        canvas,
        field::canvas::STATE_AO_UPPER,
        V::I(0),
    )?;
    break 'check_canvas_state_v;
    } // check_canvas_state_aq_upper_8
    r.set(
        canvas,
        field::canvas::STATE_AQ_UPPER,
        V::I(3),
    )?;
    } // check_canvas_running_8
    let running_value_10 = r
        .get(canvas, field::canvas::VISUAL_EFFECTS)?
        .i();
    if running_value_10 != 0 {
        let state_w_upper_value_11 = r
            .get(
                canvas,
                field::canvas::STATE_W_UPPER,
            )?
            .r();
        let state_p_value_4 = r
            .get(canvas, field::canvas::STATE_P)?
            .i();
        let state_q_value_4 = r
            .get(canvas, field::canvas::STATE_Q)?
            .i();
        host::graphics_draw_image(
            r,
            graphics,
            state_w_upper_value_11,
            state_p_value_4.wrapping_neg(),
            state_q_value_4.wrapping_neg(),
            0,
        )?;
    }
    } // next_step
    } // check_canvas_state_v
    let canvas_state_v =
        r.get(canvas, field::canvas::STATE_V)?.i();
    if canvas_state_v == 0 {
        break 'check_canvas_zone_2;
    }
    } // check_canvas_screen_state
    let canvas_screen_state =
        r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state != 0 {
        break 'check_canvas_zone_2;
    }
    } // graphics_get_translate_x
    let graphics_get_translate_x_value_2 =
        host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_2 =
        host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_2.wrapping_neg(),
        graphics_get_translate_y_value_2.wrapping_neg(),
    )?;
    let canvas_state_l_upper_3 =
        r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_3 =
        r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(
        r,
        graphics,
        0,
        0,
        canvas_state_l_upper_3,
        canvas_state_m_3,
    )?;
    let canvas_state_l_upper_4 =
        r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_4 =
        r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(
        r,
        graphics,
        0,
        0,
        canvas_state_l_upper_4,
        canvas_state_m_4,
    )?;
    let canvas_state_w_upper_4 =
        r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
    if canvas_state_w_upper_4 == 0 {
        break 'check_canvas_running_3;
    }
    } // check_canvas_state_aq_upper_3
    let canvas_state_aq_upper_4 =
        r.get(canvas, field::canvas::STATE_AQ_UPPER)?.i();
    if canvas_state_aq_upper_4 == 3 {
        break 'check_canvas_running_6;
    }
    } // check_canvas_running_3
    let canvas_visual_effects_5 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_5 == 0 {
        break 'check_canvas_state_aq_upper_6;
    }
    } // handle_result_8
    let text_value_4 = r.intern("/Images/Filter4.png");
    outcome = (|| -> JResult {
        Ok(V::R(host::image_create_image(r, text_value_4)?))
    })();
    (carry_0_r,) = (canvas,);
    if outcome.is_ok() {
        break 'handle_result_9;
    }
    } // handle_result_7
    carry_0_r = outcome.unwrap_err();
    if !(r.is_instance(carry_0_r, "java/io/IOException")) {
        return Err(outcome.unwrap_err());
    }
    r.set(canvas, field::canvas::STATE_W_UPPER, V::R(0))?;
    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AO_UPPER, V::I(0))?;
    break 'check_canvas_zone_2;
    } // handle_result_9
    let image_create_image_value_4 = outcome.unwrap().r();
    r.set(
        carry_0_r,
        field::canvas::STATE_W_UPPER,
        V::R(image_create_image_value_4),
    )?;
    } // check_canvas_state_aq_upper_6
    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(3))?;
    } // check_canvas_running_6
    let canvas_visual_effects_8 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_8 != 0 {
        let canvas_state_w_upper_9 = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
        let canvas_state_ar_upper = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        host::graphics_draw_image(
            r,
            graphics,
            canvas_state_w_upper_9,
            canvas_state_ar_upper.wrapping_neg(),
            0,
            0,
        )?;
    }
    } // check_canvas_zone_2
    let canvas_zone_3 = r.get(canvas, field::canvas::ZONE)?.i();
    if canvas_zone_3 != 12 {
        return Ok(());
    }
    let canvas_state_w_upper = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
    if canvas_state_w_upper != 0 {
        let canvas_state_aq_upper = r.get(canvas, field::canvas::STATE_AQ_UPPER)?.i();
        if canvas_state_aq_upper == 10 {
            let canvas_visual_effects_read = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
            if canvas_visual_effects_read == 0 {
                return Ok(());
            }
            let canvas_state_w_upper_5_read = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
            let canvas_state_p_read = r.get(canvas, field::canvas::STATE_P)?.i();
            let canvas_random_read = r.get(canvas, field::canvas::RANDOM)?.r();
            let random_next_int_value = host::random_next_int(r, canvas_random_read)?;
            let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 5)?;
            let canvas_state_q_read = r.get(canvas, field::canvas::STATE_Q)?.i();
            let canvas_random_2_read = r.get(canvas, field::canvas::RANDOM)?.r();
            let random_next_int_value_2 = host::random_next_int(r, canvas_random_2_read)?;
            let remainder_value_2 = arithmetic::remainder_int(r, random_next_int_value_2, 5)?;
            host::graphics_draw_image(
                r,
                graphics,
                canvas_state_w_upper_5_read,
                (canvas_state_p_read.wrapping_neg()).wrapping_add(remainder_value),
                (canvas_state_q_read.wrapping_neg()).wrapping_add(remainder_value_2),
                0,
            )?;
            return Ok(());
        }
    }
    let canvas_visual_effects_2 = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_2 == 0 {
        r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(10))?;
        let canvas_visual_effects_read = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
        if canvas_visual_effects_read == 0 {
            return Ok(());
        }
        let canvas_state_w_upper_5_read = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
        let canvas_state_p_read = r.get(canvas, field::canvas::STATE_P)?.i();
        let canvas_random_read = r.get(canvas, field::canvas::RANDOM)?.r();
        let random_next_int_value = host::random_next_int(r, canvas_random_read)?;
        let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 5)?;
        let canvas_state_q_read = r.get(canvas, field::canvas::STATE_Q)?.i();
        let canvas_random_2_read = r.get(canvas, field::canvas::RANDOM)?.r();
        let random_next_int_value_2 = host::random_next_int(r, canvas_random_2_read)?;
        let remainder_value_2 = arithmetic::remainder_int(r, random_next_int_value_2, 5)?;
        host::graphics_draw_image(
            r,
            graphics,
            canvas_state_w_upper_5_read,
            (canvas_state_p_read.wrapping_neg()).wrapping_add(remainder_value),
            (canvas_state_q_read.wrapping_neg()).wrapping_add(remainder_value_2),
            0,
        )?;
        return Ok(());
    }
    let text_value = r.intern("/Images/Dark.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value)?)) })();
    (carry_0_r,) = (canvas,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        r.set(canvas, field::canvas::STATE_W_UPPER, V::R(0))?;
        r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(0))?;
        r.set(canvas, field::canvas::STATE_AO_UPPER, V::I(0))?;
        r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(10))?;
        let canvas_visual_effects_read = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
        if canvas_visual_effects_read == 0 {
            return Ok(());
        }
        let canvas_state_w_upper_5_read = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
        let canvas_state_p_read = r.get(canvas, field::canvas::STATE_P)?.i();
        let canvas_random_read = r.get(canvas, field::canvas::RANDOM)?.r();
        let random_next_int_value = host::random_next_int(r, canvas_random_read)?;
        let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 5)?;
        let canvas_state_q_read = r.get(canvas, field::canvas::STATE_Q)?.i();
        let canvas_random_2_read = r.get(canvas, field::canvas::RANDOM)?.r();
        let random_next_int_value_2 = host::random_next_int(r, canvas_random_2_read)?;
        let remainder_value_2 = arithmetic::remainder_int(r, random_next_int_value_2, 5)?;
        host::graphics_draw_image(
            r,
            graphics,
            canvas_state_w_upper_5_read,
            (canvas_state_p_read.wrapping_neg()).wrapping_add(remainder_value),
            (canvas_state_q_read.wrapping_neg()).wrapping_add(remainder_value_2),
            0,
        )?;
        return Ok(());
    }
    let image_create_image_value = outcome.unwrap().r();
    r.set(
        carry_0_r,
        field::canvas::STATE_W_UPPER,
        V::R(image_create_image_value),
    )?;

    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(10))?;
    let canvas_visual_effects_read = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
    if canvas_visual_effects_read == 0 {
        return Ok(());
    }
    let canvas_state_w_upper_5_read = r.get(canvas, field::canvas::STATE_W_UPPER)?.r();
    let canvas_state_p_read = r.get(canvas, field::canvas::STATE_P)?.i();
    let canvas_random_read = r.get(canvas, field::canvas::RANDOM)?.r();
    let random_next_int_value = host::random_next_int(r, canvas_random_read)?;
    let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 5)?;
    let canvas_state_q_read = r.get(canvas, field::canvas::STATE_Q)?.i();
    let canvas_random_2_read = r.get(canvas, field::canvas::RANDOM)?.r();
    let random_next_int_value_2 = host::random_next_int(r, canvas_random_2_read)?;
    let remainder_value_2 = arithmetic::remainder_int(r, random_next_int_value_2, 5)?;
    host::graphics_draw_image(
        r,
        graphics,
        canvas_state_w_upper_5_read,
        (canvas_state_p_read.wrapping_neg()).wrapping_add(remainder_value),
        (canvas_state_q_read.wrapping_neg()).wrapping_add(remainder_value_2),
        0,
    )?;
    return Ok(());
}

/// Remove menu entry. Original reference: `k.a(Ljava/lang/Object;)V`.
pub(in crate::game) fn remove_menu_entry(
    r: &mut Runtime,
    canvas: u32,
    argument_1: u32,
) -> Result<(), u32> {
    let mut number_2: i32 = 0;
    number_2 = 0;
    loop {
        let canvas_state_aw = r.get(canvas, field::canvas::STATE_AW)?.r();
        let vector_size_value = host::vector_size(r, canvas_state_aw)?;
        if number_2 >= vector_size_value {
            let canvas_state_aw_2 = r.get(canvas, field::canvas::STATE_AW)?.r();
            host::vector_add_element(r, canvas_state_aw_2, argument_1)?;
            return Ok(());
        }
        let canvas_state_aw_3 = r.get(canvas, field::canvas::STATE_AW)?.r();
        let vector_element_at_value = host::vector_element_at(r, canvas_state_aw_3, number_2)?;
        host::check_cast(r, vector_element_at_value, "f")?;
        let drawable_y = r.get(vector_element_at_value, field::drawable::Y)?.i();
        let canvas_state_aw_4 = r.get(canvas, field::canvas::STATE_AW)?.r();
        let vector_element_at_value_2 = host::vector_element_at(r, canvas_state_aw_4, number_2)?;
        host::check_cast(r, vector_element_at_value_2, "f")?;
        let half_height_value = r
            .get(vector_element_at_value_2, field::drawable::HALF_HEIGHT)?
            .i();
        let canvas_state_aw_5 = r.get(canvas, field::canvas::STATE_AW)?.r();
        let vector_element_at_value_3 = host::vector_element_at(r, canvas_state_aw_5, number_2)?;
        host::check_cast(r, vector_element_at_value_3, "f")?;
        let draw_order_value = r
            .get(vector_element_at_value_3, field::drawable::DRAW_ORDER)?
            .i();
        host::check_cast(r, argument_1, "f")?;
        let drawable_y_2 = r.get(argument_1, field::drawable::Y)?.i();
        host::check_cast(r, argument_1, "f")?;
        let drawable_half_height_2 = r.get(argument_1, field::drawable::HALF_HEIGHT)?.i();
        host::check_cast(r, argument_1, "f")?;
        let drawable_draw_order_2 = r.get(argument_1, field::drawable::DRAW_ORDER)?.i();
        if !((drawable_y.wrapping_add(half_height_value)).wrapping_add(draw_order_value)
            <= (drawable_y_2.wrapping_add(drawable_half_height_2))
                .wrapping_add(drawable_draw_order_2))
        {
            let canvas_state_aw_6 = r.get(canvas, field::canvas::STATE_AW)?.r();
            host::vector_insert_element_at(r, canvas_state_aw_6, argument_1, number_2)?;
            return Ok(());
        }
        number_2 = number_2.wrapping_add(1);
    }
}

/// Draw map marker. Original reference: `k.a(Ljavax/microedition/lcdui/Graphics;II)V`.
pub(in crate::game) fn draw_map_marker(
    r: &mut Runtime,
    canvas: u32,
    graphics: u32,
    mut argument_2: i32,
    mut argument_3: i32,
) -> Result<(), u32> {
    let mut number_4: i32 = 0;
    let mut number_5: i32 = 0;
    let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
    number_4 = graphics_get_translate_x_value;
    if argument_3 == 0 {
        host::graphics_clip_rect(r, graphics, 0, 0, 3, 7)?;
        host::graphics_set_clip(r, graphics, 0, 0, 3, 7)?;
        let canvas_state_x_upper = r.get(canvas, field::canvas::STATE_X_UPPER)?.r();
        host::graphics_draw_image(r, graphics, canvas_state_x_upper, 0, 0, 0)?;
        host::graphics_translate(r, graphics, 4, 0)?;
    }

    if argument_3 == 1 {
        host::graphics_clip_rect(r, graphics, 0, 0, 9, 7)?;
        host::graphics_set_clip(r, graphics, 0, 0, 9, 7)?;
        let canvas_state_x_upper_2 = r.get(canvas, field::canvas::STATE_X_UPPER)?.r();
        host::graphics_draw_image(r, graphics, canvas_state_x_upper_2, -88, 0, 0)?;
        host::graphics_translate(r, graphics, 9, 0)?;
    }
    argument_3 = argument_2;
    if argument_2 >= 100 {
        let quotient_value = arithmetic::divide_int(r, argument_2, 100)?;
        number_5 = quotient_value;
        host::graphics_clip_rect(r, graphics, 0, 0, 7, 7)?;
        host::graphics_set_clip(r, graphics, 0, 0, 7, 7)?;
        let canvas_state_x_upper_3 = r.get(canvas, field::canvas::STATE_X_UPPER)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            canvas_state_x_upper_3,
            ((number_5.wrapping_mul(7)).wrapping_add(3)).wrapping_neg(),
            0,
            0,
        )?;
        host::graphics_translate(r, graphics, 7, 0)?;
        argument_2 = argument_2.wrapping_sub(number_5.wrapping_mul(100));
    }

    if !((argument_2 < 10) && (argument_3 < 100)) {
        let quotient_value_2 = arithmetic::divide_int(r, argument_2, 10)?;
        number_5 = quotient_value_2;
        host::graphics_clip_rect(r, graphics, 0, 0, 7, 7)?;
        host::graphics_set_clip(r, graphics, 0, 0, 7, 7)?;
        let canvas_state_x_upper_4 = r.get(canvas, field::canvas::STATE_X_UPPER)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            canvas_state_x_upper_4,
            ((number_5.wrapping_mul(7)).wrapping_add(3)).wrapping_neg(),
            0,
            0,
        )?;
        host::graphics_translate(r, graphics, 7, 0)?;
        argument_2 = argument_2.wrapping_sub(number_5.wrapping_mul(10));
    }

    if argument_2 < 0 {
        let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, graphics)?;
        host::graphics_translate(
            r,
            graphics,
            number_4.wrapping_sub(graphics_get_translate_x_value_2),
            0,
        )?;
        return Ok(());
    }
    let remainder_value = arithmetic::remainder_int(r, argument_2, 10)?;
    number_5 = remainder_value;
    host::graphics_clip_rect(r, graphics, 0, 0, 7, 7)?;
    host::graphics_set_clip(r, graphics, 0, 0, 7, 7)?;
    let canvas_state_x_upper_5 = r.get(canvas, field::canvas::STATE_X_UPPER)?.r();
    host::graphics_draw_image(
        r,
        graphics,
        canvas_state_x_upper_5,
        ((number_5.wrapping_mul(7)).wrapping_add(3)).wrapping_neg(),
        0,
        0,
    )?;
    let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        number_4.wrapping_sub(graphics_get_translate_x_value_2),
        0,
    )?;
    return Ok(());
}
