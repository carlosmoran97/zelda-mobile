//! Canvas controls.
use super::*;

/// Pointer pressed. Original reference: `k.pointerPressed(II)V`.
pub(in crate::game) fn pointer_pressed(
    r: &mut Runtime,
    canvas: u32,
    argument_1: i32,
    argument_2: i32,
) -> Result<(), u32> {
    r.set(canvas, field::canvas::STATE_AR, V::I(1))?;
    r.set(
        canvas,
        field::canvas::STATE_AS,
        V::I(argument_1.wrapping_sub(12)),
    )?;
    r.set(
        canvas,
        field::canvas::STATE_AT,
        V::I((argument_2.wrapping_sub(32)).wrapping_add(16)),
    )?;
    let system_current_time_millis_value = host::system_current_time_millis(r)?;
    r.set(
        canvas,
        field::canvas::STATE_AU,
        V::J(system_current_time_millis_value),
    )?;
    return Ok(());
}

/// Pointer dragged. Original reference: `k.pointerDragged(II)V`.
pub(in crate::game) fn pointer_dragged(
    r: &mut Runtime,
    canvas: u32,
    argument_1: i32,
    argument_2: i32,
) -> Result<(), u32> {
    r.set(canvas, field::canvas::STATE_AR, V::I(1))?;
    r.set(
        canvas,
        field::canvas::STATE_AS,
        V::I(argument_1.wrapping_sub(12)),
    )?;
    r.set(
        canvas,
        field::canvas::STATE_AT,
        V::I((argument_2.wrapping_sub(32)).wrapping_add(16)),
    )?;
    let canvas_left_key = r.get(canvas, field::canvas::LEFT_KEY)?.i();
    super::canvas::release_control(r, canvas, canvas_left_key)?;
    let canvas_right_key = r.get(canvas, field::canvas::RIGHT_KEY)?.i();
    super::canvas::release_control(r, canvas, canvas_right_key)?;
    let canvas_up_key = r.get(canvas, field::canvas::UP_KEY)?.i();
    super::canvas::release_control(r, canvas, canvas_up_key)?;
    let canvas_down_key = r.get(canvas, field::canvas::DOWN_KEY)?.i();
    super::canvas::release_control(r, canvas, canvas_down_key)?;
    return Ok(());
}

/// Pointer released. Original reference: `k.pointerReleased(II)V`.
pub(in crate::game) fn pointer_released(
    r: &mut Runtime,
    canvas: u32,
    mut argument_1: i32,
    mut argument_2: i32,
) -> Result<(), u32> {
    r.set(canvas, field::canvas::STATE_AR, V::I(0))?;
    let canvas_state_au = r.get(canvas, field::canvas::STATE_AU)?.j();
    let system_current_time_millis_value = host::system_current_time_millis(r)?;
    if (if canvas_state_au > system_current_time_millis_value.wrapping_sub(150) {
        1
    } else if canvas_state_au == system_current_time_millis_value.wrapping_sub(150) {
        0
    } else if canvas_state_au < system_current_time_millis_value.wrapping_sub(150) {
        -1
    } else {
        -1
    }) <= 0
    {
        let canvas_left_key_read = r.get(canvas, field::canvas::LEFT_KEY)?.i();
        super::canvas::release_control(r, canvas, canvas_left_key_read)?;
        let canvas_right_key_read = r.get(canvas, field::canvas::RIGHT_KEY)?.i();
        super::canvas::release_control(r, canvas, canvas_right_key_read)?;
        let canvas_up_key_read = r.get(canvas, field::canvas::UP_KEY)?.i();
        super::canvas::release_control(r, canvas, canvas_up_key_read)?;
        let canvas_down_key_read = r.get(canvas, field::canvas::DOWN_KEY)?.i();
        super::canvas::release_control(r, canvas, canvas_down_key_read)?;
        return Ok(());
    }
    let canvas_state_as = r.get(canvas, field::canvas::STATE_AS)?.i();
    let canvas_player = r.get(canvas, field::canvas::PLAYER)?.r();
    let drawable_x = r.get(canvas_player, field::drawable::X)?.i();
    let canvas_state_p = r.get(canvas, field::canvas::STATE_P)?.i();
    argument_1 = canvas_state_as.wrapping_sub(drawable_x.wrapping_sub(canvas_state_p));
    let canvas_state_at = r.get(canvas, field::canvas::STATE_AT)?.i();
    let canvas_player_2 = r.get(canvas, field::canvas::PLAYER)?.r();
    let drawable_y = r.get(canvas_player_2, field::drawable::Y)?.i();
    let canvas_state_q = r.get(canvas, field::canvas::STATE_Q)?.i();
    argument_2 = canvas_state_at.wrapping_sub(drawable_y.wrapping_sub(canvas_state_q));
    let math_abs_value = host::math_abs(r, argument_1)?;
    let math_abs_value_2 = host::math_abs(r, argument_2)?;
    if math_abs_value <= math_abs_value_2 {
        if argument_2 >= 0 {
            let canvas_player_3 = r.get(canvas, field::canvas::PLAYER)?.r();
            r.set(canvas_player_3, field::player::FACING, V::I(2))?;
        } else {
            let canvas_player_4 = r.get(canvas, field::canvas::PLAYER)?.r();
            r.set(canvas_player_4, field::player::FACING, V::I(0))?;
        }
    } else {
        if argument_1 >= 0 {
            let canvas_player_5 = r.get(canvas, field::canvas::PLAYER)?.r();
            r.set(canvas_player_5, field::player::FACING, V::I(1))?;
        } else {
            let canvas_player_6 = r.get(canvas, field::canvas::PLAYER)?.r();
            r.set(canvas_player_6, field::player::FACING, V::I(3))?;
        }
    }
    let canvas_primary_key = r.get(canvas, field::canvas::PRIMARY_KEY)?.i();
    super::canvas::press_control(r, canvas, canvas_primary_key)?;
    let canvas_left_key_read = r.get(canvas, field::canvas::LEFT_KEY)?.i();
    super::canvas::release_control(r, canvas, canvas_left_key_read)?;
    let canvas_right_key_read = r.get(canvas, field::canvas::RIGHT_KEY)?.i();
    super::canvas::release_control(r, canvas, canvas_right_key_read)?;
    let canvas_up_key_read = r.get(canvas, field::canvas::UP_KEY)?.i();
    super::canvas::release_control(r, canvas, canvas_up_key_read)?;
    let canvas_down_key_read = r.get(canvas, field::canvas::DOWN_KEY)?.i();
    super::canvas::release_control(r, canvas, canvas_down_key_read)?;
    return Ok(());
}

/// Key pressed. Original reference: `k.keyPressed(I)V`.
pub(in crate::game) fn key_pressed(
    r: &mut Runtime,
    canvas: u32,
    argument_1: i32,
) -> Result<(), u32> {
    let mut number_2: i32 = 0;
    let canvas_screen_state = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state < 20 {
        let canvas_state_w_read = r.get(canvas, field::canvas::STATE_W)?.i();
        if canvas_state_w_read == 0 {
            super::canvas::press_control(r, canvas, argument_1)?;
            return Ok(());
        }
        let canvas_get_game_action_value = host::canvas_get_game_action(r, canvas, argument_1)?;
        number_2 = canvas_get_game_action_value;
        match canvas_get_game_action_value {
            1 => {
                let canvas_up_key_read = r.get(canvas, field::canvas::UP_KEY)?.i();
                super::canvas::press_control(r, canvas, canvas_up_key_read)?;
                return Ok(());
            }
            2 => {
                let canvas_left_key_read = r.get(canvas, field::canvas::LEFT_KEY)?.i();
                super::canvas::press_control(r, canvas, canvas_left_key_read)?;
                return Ok(());
            }
            5 => {
                let canvas_right_key_read = r.get(canvas, field::canvas::RIGHT_KEY)?.i();
                super::canvas::press_control(r, canvas, canvas_right_key_read)?;
                return Ok(());
            }
            6 => {
                let canvas_down_key_read = r.get(canvas, field::canvas::DOWN_KEY)?.i();
                super::canvas::press_control(r, canvas, canvas_down_key_read)?;
                return Ok(());
            }
            8 => {
                let canvas_primary_key_read = r.get(canvas, field::canvas::PRIMARY_KEY)?.i();
                super::canvas::press_control(r, canvas, canvas_primary_key_read)?;
                return Ok(());
            }
            _ => {
                super::canvas::press_control(r, canvas, argument_1)?;
                return Ok(());
            }
        }
    }
    let canvas_screen_state_2 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_2 <= 26 {
        super::canvas::press_control(r, canvas, argument_1)?;
        return Ok(());
    }
    let canvas_state_w_read = r.get(canvas, field::canvas::STATE_W)?.i();
    if canvas_state_w_read == 0 {
        super::canvas::press_control(r, canvas, argument_1)?;
        return Ok(());
    }
    let canvas_get_game_action_value = host::canvas_get_game_action(r, canvas, argument_1)?;
    number_2 = canvas_get_game_action_value;
    match canvas_get_game_action_value {
        1 => {
            let canvas_up_key_read = r.get(canvas, field::canvas::UP_KEY)?.i();
            super::canvas::press_control(r, canvas, canvas_up_key_read)?;
            return Ok(());
        }
        2 => {
            let canvas_left_key_read = r.get(canvas, field::canvas::LEFT_KEY)?.i();
            super::canvas::press_control(r, canvas, canvas_left_key_read)?;
            return Ok(());
        }
        5 => {
            let canvas_right_key_read = r.get(canvas, field::canvas::RIGHT_KEY)?.i();
            super::canvas::press_control(r, canvas, canvas_right_key_read)?;
            return Ok(());
        }
        6 => {
            let canvas_down_key_read = r.get(canvas, field::canvas::DOWN_KEY)?.i();
            super::canvas::press_control(r, canvas, canvas_down_key_read)?;
            return Ok(());
        }
        8 => {
            let canvas_primary_key_read = r.get(canvas, field::canvas::PRIMARY_KEY)?.i();
            super::canvas::press_control(r, canvas, canvas_primary_key_read)?;
            return Ok(());
        }
        _ => {
            super::canvas::press_control(r, canvas, argument_1)?;
            return Ok(());
        }
    }
}

/// Key released. Original reference: `k.keyReleased(I)V`.
pub(in crate::game) fn key_released(
    r: &mut Runtime,
    canvas: u32,
    argument_1: i32,
) -> Result<(), u32> {
    let mut number_2: i32 = 0;
    let canvas_state_w = r.get(canvas, field::canvas::STATE_W)?.i();
    if canvas_state_w == 0 {
        super::canvas::release_control(r, canvas, argument_1)?;
        return Ok(());
    }
    let canvas_get_game_action_value = host::canvas_get_game_action(r, canvas, argument_1)?;
    number_2 = canvas_get_game_action_value;
    match canvas_get_game_action_value {
        1 => {
            let canvas_up_key = r.get(canvas, field::canvas::UP_KEY)?.i();
            super::canvas::release_control(r, canvas, canvas_up_key)?;
            return Ok(());
        }
        2 => {
            let canvas_left_key = r.get(canvas, field::canvas::LEFT_KEY)?.i();
            super::canvas::release_control(r, canvas, canvas_left_key)?;
            return Ok(());
        }
        5 => {
            let canvas_right_key = r.get(canvas, field::canvas::RIGHT_KEY)?.i();
            super::canvas::release_control(r, canvas, canvas_right_key)?;
            return Ok(());
        }
        6 => {
            let canvas_down_key = r.get(canvas, field::canvas::DOWN_KEY)?.i();
            super::canvas::release_control(r, canvas, canvas_down_key)?;
            return Ok(());
        }
        _ => {
            super::canvas::release_control(r, canvas, argument_1)?;
            return Ok(());
        }
    }
}

/// Set menu state. Original reference: `k.a(II)V`.
pub(in crate::game) fn center_camera(
    r: &mut Runtime,
    canvas: u32,
    argument_1: i32,
    argument_2: i32,
) -> Result<(), u32> {
    let canvas_state_l_upper = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let quotient_value = arithmetic::divide_int(r, canvas_state_l_upper, 2_i32)?;
    r.set(
        canvas,
        field::canvas::STATE_P,
        V::I(argument_1.wrapping_sub(quotient_value)),
    )?;
    let canvas_state_m = r.get(canvas, field::canvas::STATE_M)?.i();
    let quotient_value_2 = arithmetic::divide_int(r, canvas_state_m, 2_i32)?;
    r.set(
        canvas,
        field::canvas::STATE_Q,
        V::I((argument_2.wrapping_sub(quotient_value_2)).wrapping_add(32_i32)),
    )?;
    let canvas_state_p = r.get(canvas, field::canvas::STATE_P)?.i();
    if !(canvas_state_p >= 0) {
        r.set(canvas, field::canvas::STATE_P, V::I(0_i32))?;
    }
    let canvas_state_q = r.get(canvas, field::canvas::STATE_Q)?.i();
    if !(canvas_state_q >= 0) {
        r.set(canvas, field::canvas::STATE_Q, V::I(0_i32))?;
    }
    let canvas_state_p_2 = r.get(canvas, field::canvas::STATE_P)?.i();
    let canvas_state_al = r.get(canvas, field::canvas::STATE_AL)?.i();
    if !(canvas_state_p_2 <= canvas_state_al) {
        let canvas_state_al_2 = r.get(canvas, field::canvas::STATE_AL)?.i();
        r.set(canvas, field::canvas::STATE_P, V::I(canvas_state_al_2))?;
    }
    let canvas_state_q_2 = r.get(canvas, field::canvas::STATE_Q)?.i();
    let canvas_state_am = r.get(canvas, field::canvas::STATE_AM)?.i();
    if !(canvas_state_q_2 <= canvas_state_am) {
        let canvas_state_am_2 = r.get(canvas, field::canvas::STATE_AM)?.i();
        r.set(canvas, field::canvas::STATE_Q, V::I(canvas_state_am_2))?;
    }
    let canvas_state_l_upper_2 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    if !(canvas_state_l_upper_2 <= 256_i32) {
        r.set(canvas, field::canvas::STATE_P, V::I(0_i32))?;
    }
    let canvas_state_m_2 = r.get(canvas, field::canvas::STATE_M)?.i();
    if canvas_state_m_2 <= 208_i32 {
        return Ok(());
    }
    r.set(canvas, field::canvas::STATE_Q, V::I(0_i32))?;
    return Ok(());
}

/// Press control. Original reference: `k.a(I)V`.
pub(in crate::game) fn press_control(
    r: &mut Runtime,
    mut canvas: u32,
    mut key: i32,
) -> Result<(), u32> {
    let mut object_1: u32 = 0;
    let mut number_2: i32 = 0;
    let mut object_2: u32 = 0;
    let mut number_3: i32 = 0;
    let mut number_4: i32 = 0;
    let mut number_5: i32 = 0;
    let mut object_5: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut carry_1_i: i32 = 0;
    let canvas_player = r.get(canvas, field::canvas::PLAYER)?.r();
    if !(canvas_player == 0) {
        let canvas_player_2 = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_holding_item = r.get(canvas_player_2, field::player::HOLDING_ITEM)?.i();
        if !(player_holding_item == 0) {
            let canvas_screen_state_3 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if !(canvas_screen_state_3 != 1_i32) {
                r.set(canvas, field::canvas::STATE_AE, V::I(0_i32))?;
                r.set(canvas, field::canvas::STATE_AD_UPPER, V::I(0_i32))?;
                return Ok(());
            }
        }
    }
    let canvas_screen_state = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state != 20_i32) {
        r.set(canvas, field::canvas::UP_KEY, V::I(key))?;
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(8_i32))?;
        return Ok(());
    }
    let canvas_screen_state_2 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_2 != 21_i32) {
        r.set(canvas, field::canvas::DOWN_KEY, V::I(key))?;
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(8_i32))?;
        return Ok(());
    }
    let canvas_screen_state_4 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_4 != 22_i32) {
        r.set(canvas, field::canvas::LEFT_KEY, V::I(key))?;
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(8_i32))?;
        return Ok(());
    }
    let canvas_screen_state_5 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_5 != 23_i32) {
        r.set(canvas, field::canvas::RIGHT_KEY, V::I(key))?;
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(8_i32))?;
        return Ok(());
    }
    let canvas_screen_state_6 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_6 != 24_i32) {
        r.set(canvas, field::canvas::PRIMARY_KEY, V::I(key))?;
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(8_i32))?;
        return Ok(());
    }
    let canvas_screen_state_7 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_7 != 25_i32) {
        r.set(canvas, field::canvas::SECONDARY_KEY, V::I(key))?;
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(8_i32))?;
        return Ok(());
    }
    let canvas_screen_state_8 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_8 != 26_i32) {
        r.set(canvas, field::canvas::MENU_KEY, V::I(key))?;
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(8_i32))?;
        return Ok(());
    }
    let canvas_left_key = r.get(canvas, field::canvas::LEFT_KEY)?.i();
    if !(key != canvas_left_key) {
        let canvas_screen_state_9 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_9 != 10_i32) {
            return Ok(());
        }
        let canvas_screen_state_11 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_11 != 100_i32) {
            return Ok(());
        }
        let canvas_screen_state_14 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_14 != 30_i32) {
            let canvas_menu_selection = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection == 2_i32 {
                let canvas_menu_selection_3_read =
                    r.get(canvas, field::canvas::MENU_SELECTION)?.i();
                if canvas_menu_selection_3_read >= 0 {
                    return Ok(());
                }
                r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_4 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_4 == 4_i32) {
                let canvas_menu_selection_9 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
                r.set(
                    canvas,
                    field::canvas::MENU_SELECTION,
                    V::I(canvas_menu_selection_9.wrapping_sub(1_i32)),
                )?;
            }
            let canvas_menu_selection_3_read = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_3_read >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_inventory_open = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
        if canvas_inventory_open == 0 {
            let canvas_screen_state_21 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if !(canvas_screen_state_21 != 0) {
                return Ok(());
            }
            let canvas_screen_state_25 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if !(canvas_screen_state_25 != 5_i32) {
                let canvas_key_configuration_step =
                    r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
                if !(canvas_key_configuration_step != 0) {
                    let canvas_sound = r.get(canvas, field::canvas::SOUND)?.r();
                    let set_effect_volume_value =
                        super::sound::set_effect_volume(r, canvas_sound, -10_i32)?;
                }
                let canvas_key_configuration_step_3 =
                    r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
                if canvas_key_configuration_step_3 != 1_i32 {
                    return Ok(());
                }
                let canvas_sound_3 = r.get(canvas, field::canvas::SOUND)?.r();
                super::sound::set_music_volume(r, canvas_sound_3, -10_i32)?;
                return Ok(());
            }
            let canvas_screen_state_31 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if canvas_screen_state_31 != 12_i32 {
                let canvas_player_3 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_fishing = r.get(canvas_player_3, field::player::FISHING)?.i();
                if player_fishing == 0 {
                    let canvas_state_aq = r.get(canvas, field::canvas::STATE_AQ)?.i();
                    if !(canvas_state_aq == 0) {
                        r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(-1_i32))?;
                        r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(0_i32))?;
                        return Ok(());
                    }
                    let canvas_player_15 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_state_ag = r.get(canvas_player_15, field::player::STATE_AG)?.i();
                    if !(player_state_ag == 0) {
                        return Ok(());
                    }
                    r.set(canvas, field::canvas::STATE_AD_UPPER, V::I(-3_i32))?;
                    return Ok(());
                }
                let canvas_player_6 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_state_ac_lower =
                    r.get(canvas_player_6, field::player::STATE_AC_LOWER)?.i();
                r.set(
                    canvas_player_6,
                    field::player::STATE_AC_LOWER,
                    V::I(player_state_ac_lower.wrapping_add(1_i32)),
                )?;
                let canvas_player_7 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_state_ac_lower_2 =
                    r.get(canvas_player_7, field::player::STATE_AC_LOWER)?.i();
                if player_state_ac_lower_2 <= 6_i32 {
                    return Ok(());
                }
                let canvas_player_16 = r.get(canvas, field::canvas::PLAYER)?.r();
                r.set(canvas_player_16, field::player::STATE_AC_LOWER, V::I(1_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_16 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_16 != 0 {
                let canvas_menu_selection_21_read =
                    r.get(canvas, field::canvas::MENU_SELECTION)?.i();
                if canvas_menu_selection_21_read != 1_i32 {
                    return Ok(());
                }
                let canvas_music_volume_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
                r.set(
                    canvas,
                    field::canvas::MUSIC_VOLUME,
                    V::I(canvas_music_volume_read.wrapping_sub(10_i32)),
                )?;
                let canvas_music_volume_2_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
                if !(canvas_music_volume_2_read >= 0) {
                    r.set(canvas, field::canvas::MUSIC_VOLUME, V::I(0_i32))?;
                }
                let canvas_sound_7_read = r.get(canvas, field::canvas::SOUND)?.r();
                let canvas_music_volume_5_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
                super::sound::set_music_volume(r, canvas_sound_7_read, canvas_music_volume_5_read)?;
                return Ok(());
            }
            let canvas_effect_volume = r.get(canvas, field::canvas::EFFECT_VOLUME)?.i();
            r.set(
                canvas,
                field::canvas::EFFECT_VOLUME,
                V::I(canvas_effect_volume.wrapping_sub(10_i32)),
            )?;
            let canvas_effect_volume_2 = r.get(canvas, field::canvas::EFFECT_VOLUME)?.i();
            if !(canvas_effect_volume_2 >= 0) {
                r.set(canvas, field::canvas::EFFECT_VOLUME, V::I(0_i32))?;
            }
            let canvas_sound_5 = r.get(canvas, field::canvas::SOUND)?.r();
            let canvas_effect_volume_5 = r.get(canvas, field::canvas::EFFECT_VOLUME)?.i();
            let set_effect_volume_value_3 =
                super::sound::set_effect_volume(r, canvas_sound_5, canvas_effect_volume_5)?;
            let canvas_menu_selection_21_read = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_21_read != 1_i32 {
                return Ok(());
            }
            let canvas_music_volume_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
            r.set(
                canvas,
                field::canvas::MUSIC_VOLUME,
                V::I(canvas_music_volume_read.wrapping_sub(10_i32)),
            )?;
            let canvas_music_volume_2_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
            if !(canvas_music_volume_2_read >= 0) {
                r.set(canvas, field::canvas::MUSIC_VOLUME, V::I(0_i32))?;
            }
            let canvas_sound_7_read = r.get(canvas, field::canvas::SOUND)?.r();
            let canvas_music_volume_5_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
            super::sound::set_music_volume(r, canvas_sound_7_read, canvas_music_volume_5_read)?;
            return Ok(());
        }
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        let canvas_inventory_page = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
        if !(canvas_inventory_page != 2_i32) {
            let canvas_state_ax = r.get(canvas, field::canvas::STATE_AX)?.i();
            r.set(
                canvas,
                field::canvas::STATE_AX,
                V::I(canvas_state_ax.wrapping_sub(1_i32)),
            )?;
            let canvas_state_ax_2 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_2 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(1_i32))?;
            r.set(canvas, field::canvas::STATE_AX, V::I(10_i32))?;
            super::canvas::load_inventory_images(r, canvas)?;
            return Ok(());
        }
        let canvas_inventory_page_3 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
        if !(canvas_inventory_page_3 != 3_i32) {
            let canvas_state_ax_7 = r.get(canvas, field::canvas::STATE_AX)?.i();
            r.set(
                canvas,
                field::canvas::STATE_AX,
                V::I(canvas_state_ax_7.wrapping_sub(1_i32)),
            )?;
            let canvas_state_ax_8 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_8 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(2_i32))?;
            r.set(canvas, field::canvas::STATE_AX, V::I(0_i32))?;
            super::canvas::load_inventory_images(r, canvas)?;
            return Ok(());
        }
        let canvas_state_ax_6 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_6 == 4_i32) {
            let canvas_state_ax_16 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if !(canvas_state_ax_16 == 11_i32) {
                let canvas_state_ax_24 = r.get(canvas, field::canvas::STATE_AX)?.i();
                if !(canvas_state_ax_24 == 10_i32) {
                    let canvas_state_ax_29 = r.get(canvas, field::canvas::STATE_AX)?.i();
                    if canvas_state_ax_29 != 8_i32 {
                        let canvas_state_ax_34 = r.get(canvas, field::canvas::STATE_AX)?.i();
                        r.set(
                            canvas,
                            field::canvas::STATE_AX,
                            V::I(canvas_state_ax_34.wrapping_sub(1_i32)),
                        )?;
                        let canvas_state_ax_28_read = r.get(canvas, field::canvas::STATE_AX)?.i();
                        if !(canvas_state_ax_28_read >= 0) {
                            r.set(canvas, field::canvas::STATE_AX, V::I(10_i32))?;
                        }
                        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
                        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                        let canvas_player_25_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        r.set(canvas_player_25_read, field::player::STATE_A, V::I(0_i32))?;
                        let canvas_player_26_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        r.set(
                            canvas_player_26_read,
                            field::player::SHIELD_RAISED,
                            V::I(0_i32),
                        )?;
                        let canvas_state_ax_33_read = r.get(canvas, field::canvas::STATE_AX)?.i();
                        if canvas_state_ax_33_read != 6_i32 {
                            return Ok(());
                        }
                        let canvas_player_33_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        let has_mirror_shield_value_3 = r
                            .get(canvas_player_33_read, field::player::HAS_MIRROR_SHIELD)?
                            .i();
                        if !(has_mirror_shield_value_3 == 0) {
                            let canvas_player_40_read = r.get(canvas, field::canvas::PLAYER)?.r();
                            r.set(
                                canvas_player_40_read,
                                field::player::SHIELD_RAISED,
                                V::I(1_i32),
                            )?;
                            return Ok(());
                        }
                        let canvas_player_39_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        r.set(
                            canvas_player_39_read,
                            field::player::SHIELD_RAISED,
                            V::I(0_i32),
                        )?;
                        return Ok(());
                    }
                }
            }
        }
        let canvas_state_ax_15 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if canvas_state_ax_15 == 11_i32 {
            r.set(canvas, field::canvas::STATE_AX, V::I(3_i32))?;
        } else {
            let canvas_state_ax_23 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_23 != 10_i32 {
                r.set(canvas, field::canvas::STATE_AX, V::I(10_i32))?;
            } else {
                r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(3_i32))?;
                r.set(canvas, field::canvas::STATE_AX, V::I(0_i32))?;
                super::canvas::load_inventory_images(r, canvas)?;
            }
        }
        let canvas_state_ax_28_read = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_28_read >= 0) {
            r.set(canvas, field::canvas::STATE_AX, V::I(10_i32))?;
        }
        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        let canvas_player_25_read = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_25_read, field::player::STATE_A, V::I(0_i32))?;
        let canvas_player_26_read = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(
            canvas_player_26_read,
            field::player::SHIELD_RAISED,
            V::I(0_i32),
        )?;
        let canvas_state_ax_33_read = r.get(canvas, field::canvas::STATE_AX)?.i();
        if canvas_state_ax_33_read != 6_i32 {
            return Ok(());
        }
        let canvas_player_33_read = r.get(canvas, field::canvas::PLAYER)?.r();
        let has_mirror_shield_value_3 = r
            .get(canvas_player_33_read, field::player::HAS_MIRROR_SHIELD)?
            .i();
        if !(has_mirror_shield_value_3 == 0) {
            let canvas_player_40_read = r.get(canvas, field::canvas::PLAYER)?.r();
            r.set(
                canvas_player_40_read,
                field::player::SHIELD_RAISED,
                V::I(1_i32),
            )?;
            return Ok(());
        }
        let canvas_player_39_read = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(
            canvas_player_39_read,
            field::player::SHIELD_RAISED,
            V::I(0_i32),
        )?;
        return Ok(());
    }
    let canvas_right_key = r.get(canvas, field::canvas::RIGHT_KEY)?.i();
    if !(key != canvas_right_key) {
        let canvas_screen_state_10 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_10 != 10_i32) {
            return Ok(());
        }
        let canvas_screen_state_13 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_13 != 100_i32) {
            return Ok(());
        }
        let canvas_screen_state_17 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_17 != 30_i32) {
            let canvas_menu_selection_2 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_2 == 1_i32 {
                let canvas_menu_selection_7_read =
                    r.get(canvas, field::canvas::MENU_SELECTION)?.i();
                if canvas_menu_selection_7_read <= 5_i32 {
                    return Ok(());
                }
                r.set(canvas, field::canvas::MENU_SELECTION, V::I(5_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_8 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_8 == 3_i32) {
                let canvas_menu_selection_12 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
                r.set(
                    canvas,
                    field::canvas::MENU_SELECTION,
                    V::I(canvas_menu_selection_12.wrapping_add(1_i32)),
                )?;
            }
            let canvas_menu_selection_7_read = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_7_read <= 5_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(5_i32))?;
            return Ok(());
        }
        let canvas_inventory_open_2 = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
        if canvas_inventory_open_2 == 0 {
            let canvas_screen_state_24 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if !(canvas_screen_state_24 != 0) {
                return Ok(());
            }
            let canvas_screen_state_30 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if !(canvas_screen_state_30 != 5_i32) {
                let canvas_key_configuration_step_2 =
                    r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
                if !(canvas_key_configuration_step_2 != 0) {
                    let canvas_sound_2 = r.get(canvas, field::canvas::SOUND)?.r();
                    let set_effect_volume_value_2 =
                        super::sound::set_effect_volume(r, canvas_sound_2, 10_i32)?;
                }
                let canvas_key_configuration_step_6 =
                    r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
                if canvas_key_configuration_step_6 != 1_i32 {
                    return Ok(());
                }
                let canvas_sound_4 = r.get(canvas, field::canvas::SOUND)?.r();
                super::sound::set_music_volume(r, canvas_sound_4, 10_i32)?;
                return Ok(());
            }
            let canvas_screen_state_35 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if canvas_screen_state_35 != 12_i32 {
                let canvas_player_5 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_fishing_2 = r.get(canvas_player_5, field::player::FISHING)?.i();
                if player_fishing_2 == 0 {
                    let canvas_state_aq_2 = r.get(canvas, field::canvas::STATE_AQ)?.i();
                    if !(canvas_state_aq_2 == 0) {
                        r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(1_i32))?;
                        r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(0_i32))?;
                        return Ok(());
                    }
                    let canvas_player_23 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_state_ag_2 = r.get(canvas_player_23, field::player::STATE_AG)?.i();
                    if !(player_state_ag_2 == 0) {
                        return Ok(());
                    }
                    r.set(canvas, field::canvas::STATE_AD_UPPER, V::I(3_i32))?;
                    return Ok(());
                }
                let canvas_player_13 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_state_ac_lower_3 =
                    r.get(canvas_player_13, field::player::STATE_AC_LOWER)?.i();
                r.set(
                    canvas_player_13,
                    field::player::STATE_AC_LOWER,
                    V::I(player_state_ac_lower_3.wrapping_sub(1_i32)),
                )?;
                let canvas_player_14 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_state_ac_lower_4 =
                    r.get(canvas_player_14, field::player::STATE_AC_LOWER)?.i();
                if player_state_ac_lower_4 >= 1_i32 {
                    return Ok(());
                }
                let canvas_player_24 = r.get(canvas, field::canvas::PLAYER)?.r();
                r.set(canvas_player_24, field::player::STATE_AC_LOWER, V::I(6_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_20 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_20 != 0 {
                let canvas_menu_selection_28_read =
                    r.get(canvas, field::canvas::MENU_SELECTION)?.i();
                if canvas_menu_selection_28_read != 1_i32 {
                    return Ok(());
                }
                let canvas_music_volume_3_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
                r.set(
                    canvas,
                    field::canvas::MUSIC_VOLUME,
                    V::I(canvas_music_volume_3_read.wrapping_add(10_i32)),
                )?;
                let canvas_music_volume_4_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
                if !(canvas_music_volume_4_read <= 100_i32) {
                    r.set(canvas, field::canvas::MUSIC_VOLUME, V::I(100_i32))?;
                }
                let canvas_sound_8_read = r.get(canvas, field::canvas::SOUND)?.r();
                let canvas_music_volume_6_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
                super::sound::set_music_volume(r, canvas_sound_8_read, canvas_music_volume_6_read)?;
                return Ok(());
            }
            let canvas_effect_volume_3 = r.get(canvas, field::canvas::EFFECT_VOLUME)?.i();
            r.set(
                canvas,
                field::canvas::EFFECT_VOLUME,
                V::I(canvas_effect_volume_3.wrapping_add(10_i32)),
            )?;
            let canvas_effect_volume_4 = r.get(canvas, field::canvas::EFFECT_VOLUME)?.i();
            if !(canvas_effect_volume_4 <= 100_i32) {
                r.set(canvas, field::canvas::EFFECT_VOLUME, V::I(100_i32))?;
            }
            let canvas_sound_6 = r.get(canvas, field::canvas::SOUND)?.r();
            let canvas_effect_volume_6 = r.get(canvas, field::canvas::EFFECT_VOLUME)?.i();
            let set_effect_volume_value_4 =
                super::sound::set_effect_volume(r, canvas_sound_6, canvas_effect_volume_6)?;
            let canvas_menu_selection_28_read = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_28_read != 1_i32 {
                return Ok(());
            }
            let canvas_music_volume_3_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
            r.set(
                canvas,
                field::canvas::MUSIC_VOLUME,
                V::I(canvas_music_volume_3_read.wrapping_add(10_i32)),
            )?;
            let canvas_music_volume_4_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
            if !(canvas_music_volume_4_read <= 100_i32) {
                r.set(canvas, field::canvas::MUSIC_VOLUME, V::I(100_i32))?;
            }
            let canvas_sound_8_read = r.get(canvas, field::canvas::SOUND)?.r();
            let canvas_music_volume_6_read = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
            super::sound::set_music_volume(r, canvas_sound_8_read, canvas_music_volume_6_read)?;
            return Ok(());
        }
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        let canvas_inventory_page_2 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
        if !(canvas_inventory_page_2 != 2_i32) {
            let canvas_state_ax_4 = r.get(canvas, field::canvas::STATE_AX)?.i();
            r.set(
                canvas,
                field::canvas::STATE_AX,
                V::I(canvas_state_ax_4.wrapping_add(1_i32)),
            )?;
            let canvas_state_ax_5 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_5 <= 1_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(3_i32))?;
            r.set(canvas, field::canvas::STATE_AX, V::I(1_i32))?;
            super::canvas::load_inventory_images(r, canvas)?;
            return Ok(());
        }
        let canvas_inventory_page_5 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
        if !(canvas_inventory_page_5 != 3_i32) {
            let canvas_state_ax_13 = r.get(canvas, field::canvas::STATE_AX)?.i();
            r.set(
                canvas,
                field::canvas::STATE_AX,
                V::I(canvas_state_ax_13.wrapping_add(1_i32)),
            )?;
            let canvas_state_ax_14 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_14 <= 1_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(1_i32))?;
            r.set(canvas, field::canvas::STATE_AX, V::I(11_i32))?;
            super::canvas::load_inventory_images(r, canvas)?;
            return Ok(());
        }
        let canvas_state_ax_12 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_12 == 3_i32) {
            let canvas_state_ax_22 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if !(canvas_state_ax_22 == 7_i32) {
                let canvas_state_ax_27 = r.get(canvas, field::canvas::STATE_AX)?.i();
                if !(canvas_state_ax_27 == 10_i32) {
                    let canvas_state_ax_32 = r.get(canvas, field::canvas::STATE_AX)?.i();
                    if canvas_state_ax_32 != 9_i32 {
                        let canvas_state_ax_37 = r.get(canvas, field::canvas::STATE_AX)?.i();
                        r.set(
                            canvas,
                            field::canvas::STATE_AX,
                            V::I(canvas_state_ax_37.wrapping_add(1_i32)),
                        )?;
                        let canvas_state_ax_31_read = r.get(canvas, field::canvas::STATE_AX)?.i();
                        if !(canvas_state_ax_31_read <= 11_i32) {
                            r.set(canvas, field::canvas::STATE_AX, V::I(1_i32))?;
                            r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(2_i32))?;
                            super::canvas::load_inventory_images(r, canvas)?;
                        }
                        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
                        let canvas_player_32_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        r.set(canvas_player_32_read, field::player::STATE_A, V::I(0_i32))?;
                        let canvas_state_ax_36_read = r.get(canvas, field::canvas::STATE_AX)?.i();
                        if canvas_state_ax_36_read != 6_i32 {
                            let canvas_player_37_read = r.get(canvas, field::canvas::PLAYER)?.r();
                            r.set(
                                canvas_player_37_read,
                                field::player::SHIELD_RAISED,
                                V::I(0_i32),
                            )?;
                            return Ok(());
                        }
                        let canvas_player_38_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        let has_mirror_shield_value_4 = r
                            .get(canvas_player_38_read, field::player::HAS_MIRROR_SHIELD)?
                            .i();
                        if has_mirror_shield_value_4 == 0 {
                            let canvas_player_45_read = r.get(canvas, field::canvas::PLAYER)?.r();
                            r.set(
                                canvas_player_45_read,
                                field::player::SHIELD_RAISED,
                                V::I(0_i32),
                            )?;
                            return Ok(());
                        }
                        let canvas_player_46_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        r.set(
                            canvas_player_46_read,
                            field::player::SHIELD_RAISED,
                            V::I(1_i32),
                        )?;
                        return Ok(());
                    }
                }
            }
        }
        let canvas_state_ax_21 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if canvas_state_ax_21 == 10_i32 {
            r.set(canvas, field::canvas::STATE_AX, V::I(0_i32))?;
        } else {
            r.set(canvas, field::canvas::STATE_AX, V::I(11_i32))?;
        }
        let canvas_state_ax_31_read = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_31_read <= 11_i32) {
            r.set(canvas, field::canvas::STATE_AX, V::I(1_i32))?;
            r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(2_i32))?;
            super::canvas::load_inventory_images(r, canvas)?;
        }
        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
        let canvas_player_32_read = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_32_read, field::player::STATE_A, V::I(0_i32))?;
        let canvas_state_ax_36_read = r.get(canvas, field::canvas::STATE_AX)?.i();
        if canvas_state_ax_36_read != 6_i32 {
            let canvas_player_37_read = r.get(canvas, field::canvas::PLAYER)?.r();
            r.set(
                canvas_player_37_read,
                field::player::SHIELD_RAISED,
                V::I(0_i32),
            )?;
            return Ok(());
        }
        let canvas_player_38_read = r.get(canvas, field::canvas::PLAYER)?.r();
        let has_mirror_shield_value_4 = r
            .get(canvas_player_38_read, field::player::HAS_MIRROR_SHIELD)?
            .i();
        if has_mirror_shield_value_4 == 0 {
            let canvas_player_45_read = r.get(canvas, field::canvas::PLAYER)?.r();
            r.set(
                canvas_player_45_read,
                field::player::SHIELD_RAISED,
                V::I(0_i32),
            )?;
            return Ok(());
        }
        let canvas_player_46_read = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(
            canvas_player_46_read,
            field::player::SHIELD_RAISED,
            V::I(1_i32),
        )?;
        return Ok(());
    }
    let canvas_up_key = r.get(canvas, field::canvas::UP_KEY)?.i();
    if !(key != canvas_up_key) {
        let canvas_screen_state_12 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_12 != 10_i32) {
            return Ok(());
        }
        let canvas_screen_state_16 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_16 != 100_i32) {
            return Ok(());
        }
        let canvas_screen_state_20 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_20 != 30_i32) {
            let canvas_menu_selection_5 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_5.wrapping_sub(2_i32)),
            )?;
            let canvas_menu_selection_6 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_6 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_inventory_open_4 = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
        if !(canvas_inventory_open_4 == 0) {
            let canvas_inventory_page_4 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
            if !(canvas_inventory_page_4 != 1_i32) {
                let canvas_state_ax_10 = r.get(canvas, field::canvas::STATE_AX)?.i();
                r.set(
                    canvas,
                    field::canvas::STATE_AX,
                    V::I(canvas_state_ax_10.wrapping_sub(4_i32)),
                )?;
                let canvas_state_ax_11 = r.get(canvas, field::canvas::STATE_AX)?.i();
                if !(canvas_state_ax_11 >= 0) {
                    r.set(canvas, field::canvas::STATE_AX, V::I(0_i32))?;
                }
                r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                let canvas_player_4 = r.get(canvas, field::canvas::PLAYER)?.r();
                r.set(canvas_player_4, field::player::STATE_A, V::I(0_i32))?;
                let canvas_state_ax_20 = r.get(canvas, field::canvas::STATE_AX)?.i();
                if canvas_state_ax_20 != 6_i32 {
                    let canvas_player_11 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_11, field::player::SHIELD_RAISED, V::I(0_i32))?;
                    return Ok(());
                }
                let canvas_player_12 = r.get(canvas, field::canvas::PLAYER)?.r();
                let has_mirror_shield_value = r
                    .get(canvas_player_12, field::player::HAS_MIRROR_SHIELD)?
                    .i();
                if has_mirror_shield_value == 0 {
                    let canvas_player_21 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_21, field::player::SHIELD_RAISED, V::I(0_i32))?;
                    return Ok(());
                }
                let canvas_player_22 = r.get(canvas, field::canvas::PLAYER)?.r();
                r.set(canvas_player_22, field::player::SHIELD_RAISED, V::I(1_i32))?;
                return Ok(());
            }
        }
        let canvas_screen_state_29 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_29 != 7_i32) {
            let canvas_menu_selection_14 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_14.wrapping_sub(1_i32)),
            )?;
            let canvas_menu_selection_15 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_15 >= 0 {
                return Ok(());
            }
            let canvas_previous_screen = r.get(canvas, field::canvas::PREVIOUS_SCREEN)?.i();
            if !(canvas_previous_screen == 1_i32) {
                r.set(canvas, field::canvas::MENU_SELECTION, V::I(4_i32))?;
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(6_i32))?;
            return Ok(());
        }
        let canvas_screen_state_34 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_34 != 8_i32) {
            let canvas_menu_selection_18 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_18.wrapping_sub(1_i32)),
            )?;
            let canvas_menu_selection_19 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_19 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(8_i32))?;
            return Ok(());
        }
        let canvas_screen_state_38 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_38 != 9_i32) {
            let canvas_menu_selection_26 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_26.wrapping_sub(1_i32)),
            )?;
            let canvas_menu_selection_27 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_27 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(5_i32))?;
            return Ok(());
        }
        let canvas_screen_state_41 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_41 != 13_i32) {
            let canvas_menu_selection_34 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_34.wrapping_sub(1_i32)),
            )?;
            let canvas_menu_selection_35 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_35 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_screen_state_44 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_44 != 0) {
            let canvas_key_configuration_step_7 =
                r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
            r.set(
                canvas,
                field::canvas::KEY_CONFIGURATION_STEP,
                V::I(canvas_key_configuration_step_7.wrapping_sub(1_i32)),
            )?;
            let canvas_key_configuration_step_8 =
                r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
            if canvas_key_configuration_step_8 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(2_i32))?;
            return Ok(());
        }
        let canvas_screen_state_47 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_47 != 5_i32) {
            let canvas_key_configuration_step_9 =
                r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
            r.set(
                canvas,
                field::canvas::KEY_CONFIGURATION_STEP,
                V::I(canvas_key_configuration_step_9.wrapping_sub(1_i32)),
            )?;
            let canvas_key_configuration_step_10 =
                r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
            if canvas_key_configuration_step_10 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_screen_state_50 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_50 != 12_i32) {
            let canvas_menu_selection_51 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_51.wrapping_sub(1_i32)),
            )?;
            let canvas_menu_selection_52 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_52 >= 0 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(2_i32))?;
            return Ok(());
        }
        let canvas_screen_state_53 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if canvas_screen_state_53 != 11_i32 {
            let canvas_player_53 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_fishing_3 = r.get(canvas_player_53, field::player::FISHING)?.i();
            if !(player_fishing_3 == 0) {
                return Ok(());
            }
            let canvas_state_aq_3 = r.get(canvas, field::canvas::STATE_AQ)?.i();
            if !(canvas_state_aq_3 == 0) {
                r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(0_i32))?;
                r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(-1_i32))?;
                return Ok(());
            }
            let canvas_player_66 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_state_ag_3 = r.get(canvas_player_66, field::player::STATE_AG)?.i();
            if !(player_state_ag_3 == 0) {
                return Ok(());
            }
            r.set(canvas, field::canvas::STATE_AE, V::I(-3_i32))?;
            return Ok(());
        }
        let canvas_key_configuration_step_11 =
            r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
        r.set(
            canvas,
            field::canvas::KEY_CONFIGURATION_STEP,
            V::I(canvas_key_configuration_step_11.wrapping_sub(1_i32)),
        )?;
        let canvas_key_configuration_step_12 =
            r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
        if canvas_key_configuration_step_12 >= 0 {
            return Ok(());
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(1_i32))?;
        return Ok(());
    }
    let canvas_down_key = r.get(canvas, field::canvas::DOWN_KEY)?.i();
    if !(key != canvas_down_key) {
        let canvas_screen_state_15 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_15 != 10_i32) {
            return Ok(());
        }
        let canvas_screen_state_19 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_19 != 100_i32) {
            return Ok(());
        }
        let canvas_screen_state_23 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_23 != 30_i32) {
            let canvas_menu_selection_10 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_10.wrapping_add(2_i32)),
            )?;
            let canvas_menu_selection_11 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_11 <= 5_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(4_i32))?;
            return Ok(());
        }
        let canvas_inventory_open_5 = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
        if !(canvas_inventory_open_5 == 0) {
            let canvas_inventory_page_8 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
            if !(canvas_inventory_page_8 != 1_i32) {
                let canvas_state_ax_18 = r.get(canvas, field::canvas::STATE_AX)?.i();
                r.set(
                    canvas,
                    field::canvas::STATE_AX,
                    V::I(canvas_state_ax_18.wrapping_add(4_i32)),
                )?;
                let canvas_state_ax_19 = r.get(canvas, field::canvas::STATE_AX)?.i();
                if !(canvas_state_ax_19 <= 9_i32) {
                    r.set(canvas, field::canvas::STATE_AX, V::I(9_i32))?;
                }
                r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                let canvas_player_9 = r.get(canvas, field::canvas::PLAYER)?.r();
                r.set(canvas_player_9, field::player::STATE_A, V::I(0_i32))?;
                let canvas_player_10 = r.get(canvas, field::canvas::PLAYER)?.r();
                r.set(canvas_player_10, field::player::DIGGING, V::I(0_i32))?;
                let canvas_state_ax_26 = r.get(canvas, field::canvas::STATE_AX)?.i();
                if canvas_state_ax_26 != 6_i32 {
                    let canvas_player_19 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_19, field::player::SHIELD_RAISED, V::I(0_i32))?;
                    return Ok(());
                }
                let canvas_player_20 = r.get(canvas, field::canvas::PLAYER)?.r();
                let has_mirror_shield_value_2 = r
                    .get(canvas_player_20, field::player::HAS_MIRROR_SHIELD)?
                    .i();
                if has_mirror_shield_value_2 == 0 {
                    let canvas_player_30 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_30, field::player::SHIELD_RAISED, V::I(0_i32))?;
                    return Ok(());
                }
                let canvas_player_31 = r.get(canvas, field::canvas::PLAYER)?.r();
                r.set(canvas_player_31, field::player::SHIELD_RAISED, V::I(1_i32))?;
                return Ok(());
            }
        }
        let canvas_screen_state_33 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_33 != 0) {
            let canvas_key_configuration_step_4 =
                r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
            r.set(
                canvas,
                field::canvas::KEY_CONFIGURATION_STEP,
                V::I(canvas_key_configuration_step_4.wrapping_add(1_i32)),
            )?;
            let canvas_key_configuration_step_5 =
                r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
            if canvas_key_configuration_step_5 <= 2_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_screen_state_37 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_37 != 5_i32) {
            let canvas_menu_selection_24 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_24.wrapping_add(1_i32)),
            )?;
            let canvas_menu_selection_25 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_25 <= 1_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_screen_state_40 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_40 != 12_i32) {
            let canvas_menu_selection_32 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_32.wrapping_add(1_i32)),
            )?;
            let canvas_menu_selection_33 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_33 <= 2_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_screen_state_43 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_43 != 7_i32) {
            let canvas_menu_selection_38 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_38.wrapping_add(1_i32)),
            )?;
            let canvas_previous_screen_2 = r.get(canvas, field::canvas::PREVIOUS_SCREEN)?.i();
            if canvas_previous_screen_2 == 1_i32 {
                let canvas_menu_selection_44 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
                if canvas_menu_selection_44 <= 6_i32 {
                    return Ok(());
                }
                r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_45 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_45 <= 4_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_screen_state_46 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_46 != 8_i32) {
            let canvas_menu_selection_42 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_42.wrapping_add(1_i32)),
            )?;
            let canvas_menu_selection_43 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_43 <= 8_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_screen_state_49 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_49 != 9_i32) {
            let canvas_menu_selection_49 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_49.wrapping_add(1_i32)),
            )?;
            let canvas_menu_selection_50 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_50 <= 5_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_screen_state_52 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_52 != 13_i32) {
            let canvas_menu_selection_57 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            r.set(
                canvas,
                field::canvas::MENU_SELECTION,
                V::I(canvas_menu_selection_57.wrapping_add(1_i32)),
            )?;
            let canvas_menu_selection_58 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_58 <= 1_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_screen_state_55 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if canvas_screen_state_55 != 11_i32 {
            let canvas_player_60 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_fishing_4 = r.get(canvas_player_60, field::player::FISHING)?.i();
            if !(player_fishing_4 == 0) {
                return Ok(());
            }
            let canvas_state_aq_4 = r.get(canvas, field::canvas::STATE_AQ)?.i();
            if !(canvas_state_aq_4 == 0) {
                r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(0_i32))?;
                r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(1_i32))?;
                return Ok(());
            }
            let canvas_player_88 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_state_ag_4 = r.get(canvas_player_88, field::player::STATE_AG)?.i();
            if !(player_state_ag_4 == 0) {
                return Ok(());
            }
            r.set(canvas, field::canvas::STATE_AE, V::I(3_i32))?;
            return Ok(());
        }
        let canvas_key_configuration_step_13 =
            r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
        r.set(
            canvas,
            field::canvas::KEY_CONFIGURATION_STEP,
            V::I(canvas_key_configuration_step_13.wrapping_add(1_i32)),
        )?;
        let canvas_key_configuration_step_14 =
            r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
        if canvas_key_configuration_step_14 <= 1_i32 {
            return Ok(());
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0_i32))?;
        return Ok(());
    }
    let canvas_primary_key = r.get(canvas, field::canvas::PRIMARY_KEY)?.i();
    if key != canvas_primary_key {
        let canvas_menu_key = r.get(canvas, field::canvas::MENU_KEY)?.i();
        if !(key != canvas_menu_key) {
            let canvas_screen_state_22 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if !(canvas_screen_state_22 != 10_i32) {
                return Ok(());
            }
            let canvas_screen_state_27 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
            if !(canvas_screen_state_27 != 100_i32) {
                return Ok(());
            }
            r.set(canvas, field::canvas::STATE_AQ, V::I(0_i32))?;
            let canvas_inventory_page_6 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
            r.set(
                canvas,
                field::canvas::INVENTORY_PAGE,
                V::I(canvas_inventory_page_6.wrapping_add(1_i32)),
            )?;
            let canvas_inventory_page_7 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
            if !(canvas_inventory_page_7 <= 2_i32) {
                r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(0_i32))?;
            }
            let canvas_inventory_page_9 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
            if canvas_inventory_page_9 == 0 {
                r.set(canvas, field::canvas::INVENTORY_OPEN, V::I(0_i32))?;
                let canvas_inventory_open_7 = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
                super::canvas::set_inventory_open(r, canvas, canvas_inventory_open_7)?;
                r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
                return Ok(());
            }
            r.set(canvas, field::canvas::INVENTORY_OPEN, V::I(1_i32))?;
            let canvas_inventory_open_8 = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
            super::canvas::set_inventory_open(r, canvas, canvas_inventory_open_8)?;
            r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_secondary_key = r.get(canvas, field::canvas::SECONDARY_KEY)?.i();
        if key != canvas_secondary_key {
            return Ok(());
        }
        let canvas_screen_state_26 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
        if !(canvas_screen_state_26 != 10_i32) {
            return Ok(());
        }
        let canvas_inventory_open_6 = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
        if !(canvas_inventory_open_6 == 0) {
            return Ok(());
        }
        let canvas_state_ax_17 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_17 != 0) {
            let canvas_player_8 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_bombs = r.get(canvas_player_8, field::player::HAS_BOMBS)?.i();
            if !(player_has_bombs == 0) {
                let canvas_player_18 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_on_raft = r.get(canvas_player_18, field::player::ON_RAFT)?.i();
                if !(player_on_raft != 0) {
                    let canvas_world = r.get(canvas, field::canvas::WORLD)?.r();
                    let canvas_player_29 = r.get(canvas, field::canvas::PLAYER)?.r();
                    object_2 = canvas_player_29;
                    object_1 = canvas_world;
                    let player_bombs = r.get(object_2, field::player::BOMBS)?.i();
                    (carry_0_r,) = (canvas,);
                    if player_bombs <= 0 {
                        (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
                        r.set(carry_0_r, field::canvas::REDRAW_WORLD, V::I(carry_1_i))?;
                        return Ok(());
                    }
                    number_4 = 0_i32;
                    number_5 = 0_i32;
                    let player_facing = r.get(object_2, field::player::FACING)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if !(player_facing != 0) {
                        number_5 = -8_i32;
                        (carry_0_r,) = (carry_0_r,);
                    }
                    let player_facing_2 = r.get(object_2, field::player::FACING)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if !(player_facing_2 != 1_i32) {
                        number_4 = 16_i32;
                        (carry_0_r,) = (carry_0_r,);
                    }
                    let player_facing_5 = r.get(object_2, field::player::FACING)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if !(player_facing_5 != 2_i32) {
                        number_5 = 16_i32;
                        (carry_0_r,) = (carry_0_r,);
                    }
                    let player_facing_9 = r.get(object_2, field::player::FACING)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if !(player_facing_9 != 3_i32) {
                        number_4 = -8_i32;
                        (carry_0_r,) = (carry_0_r,);
                    }
                    let item_value = r.alloc("c", Native::Empty);
                    let world_resources = r.get(object_1, field::world::RESOURCES)?.r();
                    let item_image_value = super::resources::item_image(r, world_resources, 4_i32)?;
                    let drawable_x = r.get(object_2, field::drawable::X)?.i();
                    let drawable_y = r.get(object_2, field::drawable::Y)?.i();
                    super::item::initialize(
                        r,
                        item_value,
                        item_image_value,
                        4_i32,
                        drawable_x.wrapping_add(number_4),
                        (drawable_y.wrapping_add(number_5)).wrapping_sub(32_i32),
                        1_i32,
                        object_2,
                    )?;
                    canvas = item_value;
                    let world_items = r.get(object_1, field::world::ITEMS)?.r();
                    host::vector_add_element(r, world_items, canvas)?;
                    let player_bombs_2 = r.get(object_2, field::player::BOMBS)?.i();
                    r.set(
                        object_2,
                        field::player::BOMBS,
                        V::I((player_bombs_2.wrapping_sub(1_i32)) as i8 as i32),
                    )?;
                    (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
                    r.set(carry_0_r, field::canvas::REDRAW_WORLD, V::I(carry_1_i))?;
                    return Ok(());
                }
            }
        }
        let canvas_state_ax_25 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_25 != 1_i32) {
            let canvas_player_17 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_boomerang = r.get(canvas_player_17, field::player::HAS_BOOMERANG)?.i();
            if !(player_has_boomerang == 0) {
                let canvas_player_28 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_on_raft_2 = r.get(canvas_player_28, field::player::ON_RAFT)?.i();
                if !(player_on_raft_2 != 0) {
                    let canvas_player_36 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_boomerang_active = r
                        .get(canvas_player_36, field::player::BOOMERANG_ACTIVE)?
                        .i();
                    if player_boomerang_active != 0 {
                        return Ok(());
                    }
                    let canvas_world_3 = r.get(canvas, field::canvas::WORLD)?.r();
                    let canvas_player_44 = r.get(canvas, field::canvas::PLAYER)?.r();
                    object_2 = canvas_player_44;
                    object_1 = canvas_world_3;
                    let player_boomerang_active_2 =
                        r.get(object_2, field::player::BOOMERANG_ACTIVE)?.i();
                    if player_boomerang_active_2 != 0 {
                        r.set(canvas, field::canvas::STATE_AQ, V::I(1_i32))?;
                        r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(0_i32))?;
                        r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(0_i32))?;
                        let canvas_player_52_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        let player_facing_3_read =
                            r.get(canvas_player_52_read, field::player::FACING)?.i();
                        if !(player_facing_3_read != 0) {
                            r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(-1_i32))?;
                        }
                        let canvas_player_59_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        let player_facing_7_read =
                            r.get(canvas_player_59_read, field::player::FACING)?.i();
                        if !(player_facing_7_read != 1_i32) {
                            r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(1_i32))?;
                        }
                        let canvas_player_65_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        let player_facing_11_read =
                            r.get(canvas_player_65_read, field::player::FACING)?.i();
                        if !(player_facing_11_read != 2_i32) {
                            r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(1_i32))?;
                        }
                        let canvas_player_69_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        let player_facing_14_read =
                            r.get(canvas_player_69_read, field::player::FACING)?.i();
                        if player_facing_14_read != 3_i32 {
                            return Ok(());
                        }
                        r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(-1_i32))?;
                        return Ok(());
                    }
                    number_3 = 0_i32;
                    number_4 = 0_i32;
                    let player_facing_4 = r.get(object_2, field::player::FACING)?.i();
                    if !(player_facing_4 != 0) {
                        number_4 = -1_i32;
                    }
                    let player_facing_8 = r.get(object_2, field::player::FACING)?.i();
                    if !(player_facing_8 != 1_i32) {
                        number_3 = 1_i32;
                    }
                    let player_facing_12 = r.get(object_2, field::player::FACING)?.i();
                    if !(player_facing_12 != 2_i32) {
                        number_4 = 1_i32;
                    }
                    let player_facing_15 = r.get(object_2, field::player::FACING)?.i();
                    if !(player_facing_15 != 3_i32) {
                        number_3 = -1_i32;
                    }
                    let projectile_value = r.alloc("p", Native::Empty);
                    let world_resources_3 = r.get(object_1, field::world::RESOURCES)?.r();
                    let projectile_image_value_2 =
                        super::resources::projectile_image(r, world_resources_3, 8_i32)?;
                    super::projectile::initialize(
                        r,
                        projectile_value,
                        projectile_image_value_2,
                        8_i32,
                        number_3,
                        number_4,
                    )?;
                    object_5 = projectile_value;
                    let drawable_x_5 = r.get(object_2, field::drawable::X)?.i();
                    r.set(
                        projectile_value,
                        field::drawable::X,
                        V::I(
                            (drawable_x_5.wrapping_add(8_i32))
                                .wrapping_add(number_3.wrapping_mul(12_i32)),
                        ),
                    )?;
                    let drawable_y_5 = r.get(object_2, field::drawable::Y)?.i();
                    r.set(
                        object_5,
                        field::drawable::Y,
                        V::I(
                            (drawable_y_5.wrapping_add(12_i32))
                                .wrapping_add(number_4.wrapping_mul(16_i32)),
                        ),
                    )?;
                    r.set(object_2, field::player::BOOMERANG_ACTIVE, V::I(1_i32))?;
                    r.set(object_5, field::projectile::ENEMY_OWNED, V::I(0_i32))?;
                    let world_projectiles = r.get(object_1, field::world::PROJECTILES)?.r();
                    host::vector_add_element(r, world_projectiles, object_5)?;
                    r.set(object_2, field::player::ANIMATION, V::I(6_i32))?;
                    let drawable_x_6 = r.get(object_5, field::drawable::X)?.i();
                    r.set(object_1, field::world::HOOKSHOT_X, V::I(drawable_x_6))?;
                    let drawable_y_6 = r.get(object_5, field::drawable::Y)?.i();
                    r.set(object_1, field::world::HOOKSHOT_Y, V::I(drawable_y_6))?;
                    r.set(canvas, field::canvas::STATE_AQ, V::I(1_i32))?;
                    r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(0_i32))?;
                    r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(0_i32))?;
                    let canvas_player_52_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_facing_3_read =
                        r.get(canvas_player_52_read, field::player::FACING)?.i();
                    if !(player_facing_3_read != 0) {
                        r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(-1_i32))?;
                    }
                    let canvas_player_59_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_facing_7_read =
                        r.get(canvas_player_59_read, field::player::FACING)?.i();
                    if !(player_facing_7_read != 1_i32) {
                        r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(1_i32))?;
                    }
                    let canvas_player_65_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_facing_11_read =
                        r.get(canvas_player_65_read, field::player::FACING)?.i();
                    if !(player_facing_11_read != 2_i32) {
                        r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(1_i32))?;
                    }
                    let canvas_player_69_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_facing_14_read =
                        r.get(canvas_player_69_read, field::player::FACING)?.i();
                    if player_facing_14_read != 3_i32 {
                        return Ok(());
                    }
                    r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(-1_i32))?;
                    return Ok(());
                }
            }
        }
        let canvas_state_ax_30 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_30 != 2_i32) {
            let canvas_player_27 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_bow = r.get(canvas_player_27, field::player::HAS_BOW)?.i();
            if !(player_has_bow == 0) {
                let canvas_player_35 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_on_raft_3 = r.get(canvas_player_35, field::player::ON_RAFT)?.i();
                if !(player_on_raft_3 != 0) {
                    let canvas_world_2 = r.get(canvas, field::canvas::WORLD)?.r();
                    let canvas_player_43 = r.get(canvas, field::canvas::PLAYER)?.r();
                    object_2 = canvas_player_43;
                    object_1 = canvas_world_2;
                    let player_arrows = r.get(object_2, field::player::ARROWS)?.i();
                    (carry_0_r,) = (canvas,);
                    if player_arrows <= 0 {
                        (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
                        r.set(carry_0_r, field::canvas::REDRAW_WORLD, V::I(carry_1_i))?;
                        return Ok(());
                    }
                    let player_projectile_cooldown =
                        r.get(object_2, field::player::PROJECTILE_COOLDOWN)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if player_projectile_cooldown > 0 {
                        (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
                        r.set(carry_0_r, field::canvas::REDRAW_WORLD, V::I(carry_1_i))?;
                        return Ok(());
                    }
                    let player_arrows_2 = r.get(object_2, field::player::ARROWS)?.i();
                    r.set(
                        object_2,
                        field::player::ARROWS,
                        V::I((player_arrows_2.wrapping_sub(1_i32)) as i8 as i32),
                    )?;
                    number_3 = 0_i32;
                    number_4 = 0_i32;
                    let player_facing_6 = r.get(object_2, field::player::FACING)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if !(player_facing_6 != 0) {
                        number_3 = -1_i32;
                        (carry_0_r,) = (carry_0_r,);
                    }
                    let player_facing_10 = r.get(object_2, field::player::FACING)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if !(player_facing_10 != 1_i32) {
                        number_4 = 1_i32;
                        (carry_0_r,) = (carry_0_r,);
                    }
                    let player_facing_13 = r.get(object_2, field::player::FACING)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if !(player_facing_13 != 2_i32) {
                        number_3 = 1_i32;
                        (carry_0_r,) = (carry_0_r,);
                    }
                    let player_facing_17 = r.get(object_2, field::player::FACING)?.i();
                    (carry_0_r,) = (carry_0_r,);
                    if !(player_facing_17 != 3_i32) {
                        number_4 = -1_i32;
                        (carry_0_r,) = (carry_0_r,);
                    }
                    r.set(object_2, field::player::PROJECTILE_COOLDOWN, V::I(10_i32))?;
                    let projectile_value_2 = r.alloc("p", Native::Empty);
                    let world_resources_4 = r.get(object_1, field::world::RESOURCES)?.r();
                    let projectile_image_value_3 =
                        super::resources::projectile_image(r, world_resources_4, 3_i32)?;
                    super::projectile::initialize(
                        r,
                        projectile_value_2,
                        projectile_image_value_3,
                        3_i32,
                        number_4,
                        number_3,
                    )?;
                    object_5 = projectile_value_2;
                    let drawable_x_7 = r.get(object_2, field::drawable::X)?.i();
                    r.set(
                        projectile_value_2,
                        field::drawable::X,
                        V::I(
                            (drawable_x_7.wrapping_add(8_i32))
                                .wrapping_add(number_4.wrapping_mul(12_i32)),
                        ),
                    )?;
                    let drawable_y_7 = r.get(object_2, field::drawable::Y)?.i();
                    r.set(
                        object_5,
                        field::drawable::Y,
                        V::I(
                            (drawable_y_7.wrapping_add(12_i32))
                                .wrapping_add(number_3.wrapping_mul(16_i32)),
                        ),
                    )?;
                    r.set(object_5, field::projectile::ENEMY_OWNED, V::I(0_i32))?;
                    let world_projectiles_2 = r.get(object_1, field::world::PROJECTILES)?.r();
                    host::vector_add_element(r, world_projectiles_2, object_5)?;
                    (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
                    r.set(carry_0_r, field::canvas::REDRAW_WORLD, V::I(carry_1_i))?;
                    return Ok(());
                }
            }
        }
        let canvas_state_ax_35 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_35 != 3_i32) {
            let canvas_player_34 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_candle = r.get(canvas_player_34, field::player::HAS_CANDLE)?.i();
            if !(player_has_candle == 0) {
                let canvas_player_42 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_on_raft_4 = r.get(canvas_player_42, field::player::ON_RAFT)?.i();
                if !(player_on_raft_4 != 0) {
                    let canvas_player_50 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let canvas_player_51 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_state_a = r.get(canvas_player_51, field::player::STATE_A)?.i();
                    (carry_0_r,) = (canvas_player_50,);
                    if player_state_a != 0 {
                        (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
                    } else {
                        (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
                    }
                    r.set(carry_0_r, field::player::STATE_A, V::I(carry_1_i))?;
                    return Ok(());
                }
            }
        }
        let canvas_state_ax_38 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_38 != 4_i32) {
            let canvas_player_41 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_potion = r.get(canvas_player_41, field::player::HAS_POTION)?.i();
            if !(player_has_potion == 0) {
                let canvas_player_49 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_on_raft_6 = r.get(canvas_player_49, field::player::ON_RAFT)?.i();
                if !(player_on_raft_6 != 0) {
                    let canvas_player_56 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let canvas_player_57 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_max_health = r.get(canvas_player_57, field::player::MAX_HEALTH)?.i();
                    r.set(
                        canvas_player_56,
                        field::player::HEALTH,
                        V::I(player_max_health),
                    )?;
                    let canvas_player_58 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_58, field::player::HAS_POTION, V::I(0_i32))?;
                    r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
                    return Ok(());
                }
            }
        }
        let canvas_state_ax_39 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_39 != 5_i32) {
            let canvas_player_48 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_raft = r.get(canvas_player_48, field::player::HAS_RAFT)?.i();
            if !(player_has_raft == 0) {
                let canvas_player_55 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_on_raft_7 = r.get(canvas_player_55, field::player::ON_RAFT)?.i();
                if !(player_on_raft_7 != 0) {
                    let canvas_player_64 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let hookshot_extending_value = r
                        .get(canvas_player_64, field::player::HOOKSHOT_EXTENDING)?
                        .i();
                    if hookshot_extending_value != 0 {
                        return Ok(());
                    }
                    let canvas_player_68 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_action_sprite =
                        r.get(canvas_player_68, field::player::ACTION_SPRITE)?.r();
                    if !(player_action_sprite != 0) {
                        let canvas_player_102 = r.get(canvas, field::canvas::PLAYER)?.r();
                        let canvas_world_7 = r.get(canvas, field::canvas::WORLD)?.r();
                        let world_resources_2 = r.get(canvas_world_7, field::world::RESOURCES)?.r();
                        let projectile_image_value =
                            super::resources::projectile_image(r, world_resources_2, 4_i32)?;
                        r.set(
                            canvas_player_102,
                            field::player::ACTION_SPRITE,
                            V::R(projectile_image_value),
                        )?;
                    }
                    let canvas_player_96 = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_hookshot_trail =
                        r.get(canvas_player_96, field::player::HOOKSHOT_TRAIL)?.r();
                    host::vector_remove_all_elements(r, player_hookshot_trail)?;
                    let canvas_player_97 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(
                        canvas_player_97,
                        field::player::HOOKSHOT_RETURNING,
                        V::I(0_i32),
                    )?;
                    let canvas_player_98 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_98, field::player::ACTION_TIMER, V::I(0_i32))?;
                    let canvas_player_99 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_99, field::player::STATE_AH, V::I(0_i32))?;
                    let canvas_player_100 = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(
                        canvas_player_100,
                        field::player::HOOKSHOT_EXTENDING,
                        V::I(1_i32),
                    )?;
                    let canvas_world_6 = r.get(canvas, field::canvas::WORLD)?.r();
                    let canvas_player_101 = r.get(canvas, field::canvas::PLAYER)?.r();
                    super::world::update_hookshot(r, canvas_world_6, canvas_player_101)?;
                    return Ok(());
                }
            }
        }
        let canvas_player_47 = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_on_raft_5 = r.get(canvas_player_47, field::player::ON_RAFT)?.i();
        if player_on_raft_5 == 0 {
            let canvas_state_ax_40_read = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_40_read != 7_i32 {
                return Ok(());
            }
            let canvas_player_61_read = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_shovel_read =
                r.get(canvas_player_61_read, field::player::HAS_SHOVEL)?.i();
            if player_has_shovel_read == 0 {
                return Ok(());
            }
            let canvas_player_67_read = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_on_raft_8_read = r.get(canvas_player_67_read, field::player::ON_RAFT)?.i();
            if player_on_raft_8_read != 0 {
                return Ok(());
            }
            let canvas_player_89_read = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_digging_read = r.get(canvas_player_89_read, field::player::DIGGING)?.i();
            if player_digging_read != 0 {
                return Ok(());
            }
            let canvas_world_9_read = r.get(canvas, field::canvas::WORLD)?.r();
            let canvas_player_104_read = r.get(canvas, field::canvas::PLAYER)?.r();
            super::world::dig(r, canvas_world_9_read, canvas_player_104_read)?;
            return Ok(());
        }
        let canvas_player_54 = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_has_fish = r.get(canvas_player_54, field::player::HAS_FISH)?.i();
        if player_has_fish == 0 {
            let canvas_state_ax_40_read = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_40_read != 7_i32 {
                return Ok(());
            }
            let canvas_player_61_read = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_shovel_read =
                r.get(canvas_player_61_read, field::player::HAS_SHOVEL)?.i();
            if player_has_shovel_read == 0 {
                return Ok(());
            }
            let canvas_player_67_read = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_on_raft_8_read = r.get(canvas_player_67_read, field::player::ON_RAFT)?.i();
            if player_on_raft_8_read != 0 {
                return Ok(());
            }
            let canvas_player_89_read = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_digging_read = r.get(canvas_player_89_read, field::player::DIGGING)?.i();
            if player_digging_read != 0 {
                return Ok(());
            }
            let canvas_world_9_read = r.get(canvas, field::canvas::WORLD)?.r();
            let canvas_player_104_read = r.get(canvas, field::canvas::PLAYER)?.r();
            super::world::dig(r, canvas_world_9_read, canvas_player_104_read)?;
            return Ok(());
        }
        let canvas_player_62 = r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_player_63 = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_fishing_5 = r.get(canvas_player_63, field::player::FISHING)?.i();
        (carry_0_r,) = (canvas_player_62,);
        if player_fishing_5 != 0 {
            (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
        } else {
            (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
        }
        r.set(carry_0_r, field::player::FISHING, V::I(carry_1_i))?;
        let canvas_player_90 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_90, field::player::STATE_AC_LOWER, V::I(1_i32))?;
        let canvas_player_91 = r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_player_92 = r.get(canvas, field::canvas::PLAYER)?.r();
        let drawable_x_4 = r.get(canvas_player_92, field::drawable::X)?.i();
        r.set(
            canvas_player_91,
            field::player::STATE_AA,
            V::I(drawable_x_4),
        )?;
        let canvas_player_93 = r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_player_94 = r.get(canvas, field::canvas::PLAYER)?.r();
        let drawable_y_4 = r.get(canvas_player_94, field::drawable::Y)?.i();
        r.set(
            canvas_player_93,
            field::player::STATE_AB_LOWER,
            V::I(drawable_y_4.wrapping_add(16_i32)),
        )?;
        let canvas_player_95 = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_facing_16 = r.get(canvas_player_95, field::player::FACING)?.i();
        if !(player_facing_16 != 1_i32) {
            let canvas_player_106 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_state_aa = r.get(canvas_player_106, field::player::STATE_AA)?.i();
            r.set(
                canvas_player_106,
                field::player::STATE_AA,
                V::I(player_state_aa.wrapping_add(20_i32)),
            )?;
        }
        let canvas_player_105 = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_facing_18 = r.get(canvas_player_105, field::player::FACING)?.i();
        if player_facing_18 != 3_i32 {
            return Ok(());
        }
        let canvas_player_108 = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_state_aa_2 = r.get(canvas_player_108, field::player::STATE_AA)?.i();
        r.set(
            canvas_player_108,
            field::player::STATE_AA,
            V::I(player_state_aa_2.wrapping_sub(20_i32)),
        )?;
        return Ok(());
    }
    let canvas_screen_state_18 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_18 != 10_i32) {
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(0_i32))?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        return Ok(());
    }
    r.set(canvas, field::canvas::STATE_AQ, V::I(0_i32))?;
    let canvas_inventory_open_3 = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
    if !(canvas_inventory_open_3 == 0) {
        let canvas_state_ax_3 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_3 != 8_i32) {
            super::canvas::save_game(r, canvas)?;
            r.set(canvas, field::canvas::STATE_AN_UPPER, V::I(1_i32))?;
            r.set(canvas, field::canvas::STATE_AM_UPPER, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_state_ax_9 = r.get(canvas, field::canvas::STATE_AX)?.i();
        if !(canvas_state_ax_9 != 9_i32) {
            r.set(canvas, field::canvas::EXIT_REQUESTED, V::I(1_i32))?;
            return Ok(());
        }
        r.set(canvas, field::canvas::INVENTORY_OPEN, V::I(0_i32))?;
        r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
        return Ok(());
    }
    let canvas_screen_state_28 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_28 != 30_i32) {
        let canvas_menu_selection_13 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_13 != 0) {
            r.set(canvas, field::canvas::SAVE_SLOT, V::I(0_i32))?;
            let canvas_save_slots = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            let element_value = r.array_get(canvas_save_slots, 0_i32)?.r();
            if element_value == 0 {
                super::canvas::start_game(r, canvas, 1_i32)?;
            } else {
                super::canvas::start_game(r, canvas, 0_i32)?;
            }
        }
        let canvas_menu_selection_17 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_17 != 1_i32) {
            super::canvas::delete_save_slot(r, canvas, 0_i32)?;
        }
        let canvas_menu_selection_23 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_23 != 2_i32) {
            r.set(canvas, field::canvas::SAVE_SLOT, V::I(1_i32))?;
            let canvas_save_slots_2 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            let element_value_2 = r.array_get(canvas_save_slots_2, 1_i32)?.r();
            if element_value_2 == 0 {
                super::canvas::start_game(r, canvas, 1_i32)?;
            } else {
                super::canvas::start_game(r, canvas, 0_i32)?;
            }
        }
        let canvas_menu_selection_31 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_31 != 3_i32) {
            super::canvas::delete_save_slot(r, canvas, 1_i32)?;
        }
        let canvas_menu_selection_37 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if canvas_menu_selection_37 != 4_i32 {
            let canvas_menu_selection_41_read = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_41_read != 5_i32 {
                return Ok(());
            }
            super::canvas::delete_save_slot(r, canvas, 2_i32)?;
            return Ok(());
        }
        r.set(canvas, field::canvas::SAVE_SLOT, V::I(2_i32))?;
        let canvas_save_slots_3 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
        let element_value_3 = r.array_get(canvas_save_slots_3, 2_i32)?.r();
        if element_value_3 == 0 {
            super::canvas::start_game(r, canvas, 1_i32)?;
        } else {
            super::canvas::start_game(r, canvas, 0_i32)?;
        }
        let canvas_menu_selection_41_read = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if canvas_menu_selection_41_read != 5_i32 {
            return Ok(());
        }
        super::canvas::delete_save_slot(r, canvas, 2_i32)?;
        return Ok(());
    }
    let canvas_screen_state_32 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_32 != 10_i32) {
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(0_i32))?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        return Ok(());
    }
    let canvas_screen_state_36 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_36 != 13_i32) {
        let canvas_menu_selection_22 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_22 != 0) {
            let canvas_state_ak = r.get(canvas, field::canvas::STATE_AK)?.i();
            (carry_0_r,) = (canvas,);
            if canvas_state_ak != 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
            }
            r.set(carry_0_r, field::canvas::STATE_AK, V::I(carry_1_i))?;
            r.set(canvas, field::canvas::SECOND_ELAPSED, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_30 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_30 != 1_i32) {
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(7_i32))?;
            r.set(canvas, field::canvas::SECOND_ELAPSED, V::I(1_i32))?;
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(3_i32))?;
            return Ok(());
        }
    }
    let canvas_screen_state_39 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_39 != 12_i32) {
        let canvas_menu_selection_29 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if canvas_menu_selection_29 != 2_i32 {
            return Ok(());
        }
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(7_i32))?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        r.set(canvas, field::canvas::MENU_SELECTION, V::I(1_i32))?;
        return Ok(());
    }
    let canvas_screen_state_42 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_42 != 7_i32) {
        let canvas_menu_selection_36 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_36 != 4_i32) {
            let canvas_previous_screen_3 = r.get(canvas, field::canvas::PREVIOUS_SCREEN)?.i();
            r.set(
                canvas,
                field::canvas::SCREEN_STATE,
                V::I(canvas_previous_screen_3),
            )?;
            r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
            r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_40 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_40 != 0) {
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(8_i32))?;
            r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
            r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_48 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_48 != 1_i32) {
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(12_i32))?;
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0_i32))?;
            r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
            r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_56 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_56 != 2_i32) {
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(9_i32))?;
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
            r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_61 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_61 != 3_i32) {
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(13_i32))?;
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
            r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
            r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_64 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if canvas_menu_selection_64 != 5_i32 {
            let canvas_menu_selection_67 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_67 != 6_i32 {
                return Ok(());
            }
            super::canvas::save_game(r, canvas)?;
            r.set(canvas, field::canvas::EXIT_REQUESTED, V::I(1_i32))?;
            return Ok(());
        }
        r.set(canvas, field::canvas::MAP, V::I(120_i32))?;
        r.set(canvas, field::canvas::ZONE, V::I(0_i32))?;
        let canvas_player_80 = r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_zone = r.get(canvas, field::canvas::ZONE)?.i();
        r.set(canvas_player_80, field::player::ZONE, V::I(canvas_zone))?;
        let canvas_player_81 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_81, field::player::ON_RAFT, V::I(0_i32))?;
        let canvas_player_82 = r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_map = r.get(canvas, field::canvas::MAP)?.i();
        r.set(canvas_player_82, field::player::MAP, V::I(canvas_map))?;
        let canvas_player_83 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_83, field::drawable::X, V::I(110_i32))?;
        let canvas_player_84 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_84, field::drawable::Y, V::I(110_i32))?;
        let canvas_world_5 = r.get(canvas, field::canvas::WORLD)?.r();
        let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
        host::string_buffer_initialize(r, object_value)?;
        let text_value_2 = r.intern("/Maps/Map");
        let string_buffer_append_2_value =
            host::string_buffer_append_2(r, object_value, text_value_2)?;
        let canvas_map_2 = r.get(canvas, field::canvas::MAP)?.i();
        let string_buffer_append_value =
            host::string_buffer_append(r, string_buffer_append_2_value, canvas_map_2)?;
        let text_value_3 = r.intern("-");
        let string_buffer_append_2_value_2 =
            host::string_buffer_append_2(r, string_buffer_append_value, text_value_3)?;
        let canvas_zone_2 = r.get(canvas, field::canvas::ZONE)?.i();
        let string_buffer_append_value_2 =
            host::string_buffer_append(r, string_buffer_append_2_value_2, canvas_zone_2)?;
        let text_value_4 = r.intern(".xmap.xmob");
        let string_buffer_append_2_value_3 =
            host::string_buffer_append_2(r, string_buffer_append_value_2, text_value_4)?;
        let string_buffer_to_string_value =
            host::string_buffer_to_string(r, string_buffer_append_2_value_3)?;
        let canvas_scene_image_2 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
        let image_get_graphics_value_2 = host::image_get_graphics(r, canvas_scene_image_2)?;
        let canvas_player_85 = r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_zone_3 = r.get(canvas, field::canvas::ZONE)?.i();
        let canvas_map_3 = r.get(canvas, field::canvas::MAP)?.i();
        let canvas_get_width_value_2 = host::canvas_get_width(r, canvas)?;
        let canvas_get_height_value_2 = host::canvas_get_height(r, canvas)?;
        let load_map_value_2 = super::world::load_map(
            r,
            canvas_world_5,
            string_buffer_to_string_value,
            image_get_graphics_value_2,
            canvas_player_85,
            canvas_zone_3,
            canvas_map_3,
            canvas_get_width_value_2,
            canvas_get_height_value_2,
            1_i32,
        )?;
        let canvas_player_86 = r.get(canvas, field::canvas::PLAYER)?.r();
        let drawable_x_3 = r.get(canvas_player_86, field::drawable::X)?.i();
        let canvas_state_l_upper_2 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let quotient_value_3 = arithmetic::divide_int(r, canvas_state_l_upper_2, 2_i32)?;
        r.set(
            canvas,
            field::canvas::STATE_P,
            V::I(drawable_x_3.wrapping_sub(quotient_value_3)),
        )?;
        let canvas_player_87 = r.get(canvas, field::canvas::PLAYER)?.r();
        let drawable_y_3 = r.get(canvas_player_87, field::drawable::Y)?.i();
        let canvas_state_m_2 = r.get(canvas, field::canvas::STATE_M)?.i();
        let quotient_value_4 =
            arithmetic::divide_int(r, canvas_state_m_2.wrapping_add(32_i32), 2_i32)?;
        r.set(
            canvas,
            field::canvas::STATE_Q,
            V::I(drawable_y_3.wrapping_sub(quotient_value_4)),
        )?;
        let canvas_state_p_2 = r.get(canvas, field::canvas::STATE_P)?.i();
        if !(canvas_state_p_2 >= 0) {
            r.set(canvas, field::canvas::STATE_P, V::I(0_i32))?;
        }
        let canvas_state_q_2 = r.get(canvas, field::canvas::STATE_Q)?.i();
        if !(canvas_state_q_2 >= 0) {
            r.set(canvas, field::canvas::STATE_Q, V::I(0_i32))?;
        }
        let canvas_state_p_4 = r.get(canvas, field::canvas::STATE_P)?.i();
        let canvas_state_al_2 = r.get(canvas, field::canvas::STATE_AL)?.i();
        if !(canvas_state_p_4 <= canvas_state_al_2) {
            let canvas_state_al_4 = r.get(canvas, field::canvas::STATE_AL)?.i();
            r.set(canvas, field::canvas::STATE_P, V::I(canvas_state_al_4))?;
        }
        let canvas_state_q_4 = r.get(canvas, field::canvas::STATE_Q)?.i();
        let canvas_state_am_2 = r.get(canvas, field::canvas::STATE_AM)?.i();
        if !(canvas_state_q_4 <= canvas_state_am_2) {
            let canvas_state_am_4 = r.get(canvas, field::canvas::STATE_AM)?.i();
            r.set(canvas, field::canvas::STATE_Q, V::I(canvas_state_am_4))?;
        }
        let canvas_state_m_7 = r.get(canvas, field::canvas::STATE_M)?.i();
        if !(canvas_state_m_7 <= 208_i32) {
            r.set(canvas, field::canvas::STATE_Q, V::I(0_i32))?;
        }
        let canvas_state_l_upper_4 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        if !(canvas_state_l_upper_4 <= 256_i32) {
            r.set(canvas, field::canvas::STATE_P, V::I(0_i32))?;
        }
        let canvas_sound_12 = r.get(canvas, field::canvas::SOUND)?.r();
        let canvas_zone_4 = r.get(canvas, field::canvas::ZONE)?.i();
        let canvas_map_5 = r.get(canvas, field::canvas::MAP)?.i();
        super::sound::select_map_music(r, canvas_sound_12, canvas_zone_4, canvas_map_5)?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
        r.set(canvas, field::canvas::STATE_AP, V::I(0_i32))?;
        r.set(canvas, field::canvas::STATE_AV, V::I(0_i32))?;
        let canvas_previous_screen_4 = r.get(canvas, field::canvas::PREVIOUS_SCREEN)?.i();
        r.set(
            canvas,
            field::canvas::SCREEN_STATE,
            V::I(canvas_previous_screen_4),
        )?;
        r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        return Ok(());
    }
    let canvas_screen_state_45 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_45 != 8_i32) {
        let canvas_menu_selection_39 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if canvas_menu_selection_39 != 7_i32 {
            let canvas_menu_selection_47 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_47 != 8_i32) {
                r.set(canvas, field::canvas::SCREEN_STATE, V::I(7_i32))?;
                r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                r.set(canvas, field::canvas::MENU_SELECTION, V::I(0_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_55 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_55 != 0) {
                r.set(canvas, field::canvas::SCREEN_STATE, V::I(20_i32))?;
                r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_60 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_60 != 1_i32) {
                r.set(canvas, field::canvas::SCREEN_STATE, V::I(21_i32))?;
                r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_63 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_63 != 2_i32) {
                r.set(canvas, field::canvas::SCREEN_STATE, V::I(22_i32))?;
                r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_66 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_66 != 3_i32) {
                r.set(canvas, field::canvas::SCREEN_STATE, V::I(23_i32))?;
                r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_69 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_69 != 4_i32) {
                r.set(canvas, field::canvas::SCREEN_STATE, V::I(24_i32))?;
                r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_70 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if !(canvas_menu_selection_70 != 5_i32) {
                r.set(canvas, field::canvas::SCREEN_STATE, V::I(25_i32))?;
                r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
                r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
                return Ok(());
            }
            let canvas_menu_selection_71 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_71 != 6_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(26_i32))?;
            r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
            r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
            return Ok(());
        }
        r.set(canvas, field::canvas::SECOND_ELAPSED, V::I(1_i32))?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        let canvas_state_w = r.get(canvas, field::canvas::STATE_W)?.i();
        (carry_0_r,) = (canvas,);
        if canvas_state_w != 0 {
            (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
        } else {
            (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
        }
        r.set(carry_0_r, field::canvas::STATE_W, V::I(carry_1_i))?;
        return Ok(());
    }
    let canvas_screen_state_48 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_48 != 9_i32) {
        let canvas_menu_selection_46 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_46 != 0) {
            let canvas_state_ap_upper = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            (carry_0_r,) = (canvas,);
            if canvas_state_ap_upper != 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
            }
            r.set(carry_0_r, field::canvas::STATE_AP_UPPER, V::I(carry_1_i))?;
            let canvas_state_ap_upper_2 = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            host::canvas_set_full_screen_mode(r, canvas, canvas_state_ap_upper_2)?;
            r.set(canvas, field::canvas::SECOND_ELAPSED, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_54 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_54 != 1_i32) {
            let canvas_use_phone_buttons = r.get(canvas, field::canvas::USE_PHONE_BUTTONS)?.i();
            (carry_0_r,) = (canvas,);
            if canvas_use_phone_buttons != 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
            }
            r.set(carry_0_r, field::canvas::USE_PHONE_BUTTONS, V::I(carry_1_i))?;
            r.set(canvas, field::canvas::SECOND_ELAPSED, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_59 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_59 != 2_i32) {
            let canvas_state_ao_upper = r.get(canvas, field::canvas::STATE_AO_UPPER)?.i();
            (carry_0_r,) = (canvas,);
            if canvas_state_ao_upper != 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
            }
            r.set(carry_0_r, field::canvas::STATE_AO_UPPER, V::I(carry_1_i))?;
            r.set(canvas, field::canvas::SECOND_ELAPSED, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_62 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if !(canvas_menu_selection_62 != 3_i32) {
            let canvas_visual_effects = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
            (carry_0_r,) = (canvas,);
            if canvas_visual_effects != 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
            }
            r.set(carry_0_r, field::canvas::VISUAL_EFFECTS, V::I(carry_1_i))?;
            let canvas_world_10 = r.get(canvas, field::canvas::WORLD)?.r();
            r.set(canvas_world_10, field::world::STATE_C, V::R(0))?;
            r.set(canvas, field::canvas::SECOND_ELAPSED, V::I(1_i32))?;
            return Ok(());
        }
        let canvas_menu_selection_65 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if canvas_menu_selection_65 != 4_i32 {
            let canvas_menu_selection_68 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
            if canvas_menu_selection_68 != 5_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(7_i32))?;
            r.set(canvas, field::canvas::MENU_SELECTION, V::I(2_i32))?;
            return Ok(());
        }
        let canvas_state_x = r.get(canvas, field::canvas::STATE_X)?.i();
        (carry_0_r,) = (canvas,);
        if canvas_state_x != 0 {
            (carry_0_r, carry_1_i) = (carry_0_r, 0_i32);
        } else {
            (carry_0_r, carry_1_i) = (carry_0_r, 1_i32);
        }
        r.set(carry_0_r, field::canvas::STATE_X, V::I(carry_1_i))?;
        return Ok(());
    }
    let canvas_screen_state_51 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_51 != 5_i32) {
        let canvas_menu_selection_53 = r.get(canvas, field::canvas::MENU_SELECTION)?.i();
        if canvas_menu_selection_53 != 2_i32 {
            return Ok(());
        }
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(6_i32))?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        r.set(canvas, field::canvas::MENU_SELECTION, V::I(3_i32))?;
        return Ok(());
    }
    let canvas_screen_state_54 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_54 != 11_i32) {
        let canvas_state_ak_upper = r.get(canvas, field::canvas::STATE_AK_UPPER)?.i();
        if !(canvas_state_ak_upper >= 0) {
            return Ok(());
        }
        let canvas_key_configuration_step_16 =
            r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
        if canvas_key_configuration_step_16 != 0 {
            let canvas_key_configuration_step_18 =
                r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
            if canvas_key_configuration_step_18 != 1_i32 {
                return Ok(());
            }
            r.set(canvas, field::canvas::EXIT_REQUESTED, V::I(1_i32))?;
            let canvas_application = r.get(canvas, field::canvas::APPLICATION)?.r();
            super::lifecycle::destroy(r, canvas_application, 0_i32)?;
            return Ok(());
        }
        let canvas_player_71 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_71, field::player::HEALTH, V::I(12_i32))?;
        let canvas_player_72 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_72, field::player::ZONE, V::I(0_i32))?;
        let canvas_player_73 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_73, field::player::ON_RAFT, V::I(0_i32))?;
        let canvas_player_74 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_74, field::player::MAP, V::I(120_i32))?;
        let canvas_player_75 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_75, field::drawable::X, V::I(75_i32))?;
        let canvas_player_76 = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(canvas_player_76, field::drawable::Y, V::I(85_i32))?;
        r.set(canvas, field::canvas::MAP, V::I(120_i32))?;
        r.set(canvas, field::canvas::ZONE, V::I(0_i32))?;
        super::canvas::save_game(r, canvas)?;
        let canvas_world_4 = r.get(canvas, field::canvas::WORLD)?.r();
        let text_value = r.intern("/Maps/Map120-0.xmap.xmob");
        let canvas_scene_image = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
        let image_get_graphics_value = host::image_get_graphics(r, canvas_scene_image)?;
        let canvas_player_77 = r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_get_width_value = host::canvas_get_width(r, canvas)?;
        let canvas_get_height_value = host::canvas_get_height(r, canvas)?;
        let load_map_value = super::world::load_map(
            r,
            canvas_world_4,
            text_value,
            image_get_graphics_value,
            canvas_player_77,
            0_i32,
            120_i32,
            canvas_get_width_value,
            canvas_get_height_value,
            1_i32,
        )?;
        let canvas_player_78 = r.get(canvas, field::canvas::PLAYER)?.r();
        let drawable_x_2 = r.get(canvas_player_78, field::drawable::X)?.i();
        let canvas_state_l_upper = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let quotient_value = arithmetic::divide_int(r, canvas_state_l_upper, 2_i32)?;
        r.set(
            canvas,
            field::canvas::STATE_P,
            V::I(drawable_x_2.wrapping_sub(quotient_value)),
        )?;
        let canvas_player_79 = r.get(canvas, field::canvas::PLAYER)?.r();
        let drawable_y_2 = r.get(canvas_player_79, field::drawable::Y)?.i();
        let canvas_state_m = r.get(canvas, field::canvas::STATE_M)?.i();
        let quotient_value_2 =
            arithmetic::divide_int(r, canvas_state_m.wrapping_add(32_i32), 2_i32)?;
        r.set(
            canvas,
            field::canvas::STATE_Q,
            V::I(drawable_y_2.wrapping_sub(quotient_value_2)),
        )?;
        let canvas_state_p = r.get(canvas, field::canvas::STATE_P)?.i();
        if !(canvas_state_p >= 0) {
            r.set(canvas, field::canvas::STATE_P, V::I(0_i32))?;
        }
        let canvas_state_q = r.get(canvas, field::canvas::STATE_Q)?.i();
        if !(canvas_state_q >= 0) {
            r.set(canvas, field::canvas::STATE_Q, V::I(0_i32))?;
        }
        let canvas_state_p_3 = r.get(canvas, field::canvas::STATE_P)?.i();
        let canvas_state_al = r.get(canvas, field::canvas::STATE_AL)?.i();
        if !(canvas_state_p_3 <= canvas_state_al) {
            let canvas_state_al_3 = r.get(canvas, field::canvas::STATE_AL)?.i();
            r.set(canvas, field::canvas::STATE_P, V::I(canvas_state_al_3))?;
        }
        let canvas_state_q_3 = r.get(canvas, field::canvas::STATE_Q)?.i();
        let canvas_state_am = r.get(canvas, field::canvas::STATE_AM)?.i();
        if !(canvas_state_q_3 <= canvas_state_am) {
            let canvas_state_am_3 = r.get(canvas, field::canvas::STATE_AM)?.i();
            r.set(canvas, field::canvas::STATE_Q, V::I(canvas_state_am_3))?;
        }
        let canvas_state_m_6 = r.get(canvas, field::canvas::STATE_M)?.i();
        if !(canvas_state_m_6 <= 208_i32) {
            r.set(canvas, field::canvas::STATE_Q, V::I(0_i32))?;
        }
        let canvas_state_l_upper_3 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        if !(canvas_state_l_upper_3 <= 256_i32) {
            r.set(canvas, field::canvas::STATE_P, V::I(0_i32))?;
        }
        let canvas_sound_11 = r.get(canvas, field::canvas::SOUND)?.r();
        super::sound::select_map_music(r, canvas_sound_11, 0_i32, 120_i32)?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
        r.set(canvas, field::canvas::STATE_AP, V::I(0_i32))?;
        r.set(canvas, field::canvas::STATE_AV, V::I(0_i32))?;
        r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1_i32))?;
        r.set(canvas, field::canvas::REDRAW, V::I(1_i32))?;
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(1_i32))?;
        return Ok(());
    }
    let canvas_screen_state_56 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if !(canvas_screen_state_56 != 0) {
        let canvas_key_configuration_step_15 =
            r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
        if !(canvas_key_configuration_step_15 != 0) {
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(30_i32))?;
            r.set(canvas, field::canvas::INVENTORY_OPEN, V::I(1_i32))?;
            r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(4_i32))?;
            super::canvas::load_inventory_images(r, canvas)?;
            super::canvas::load_save_slots(r, canvas)?;
            r.set(canvas, field::canvas::INVENTORY_OPEN, V::I(0_i32))?;
            r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_key_configuration_step_17 =
            r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
        if !(canvas_key_configuration_step_17 != 1_i32) {
            r.set(canvas, field::canvas::SCREEN_STATE, V::I(7_i32))?;
        }
        let canvas_key_configuration_step_19 =
            r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
        if canvas_key_configuration_step_19 != 2_i32 {
            return Ok(());
        }
        r.set(canvas, field::canvas::SCREEN_STATE, V::I(10_i32))?;
        return Ok(());
    }
    let canvas_state_i = r.get(canvas, field::canvas::STATE_I)?.i();
    if canvas_state_i != 0 {
        let canvas_player_70_read_read = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(
            canvas_player_70_read_read,
            field::player::ATTACKING,
            V::I(0_i32),
        )?;
    } else {
        let canvas_show_item_message = r.get(canvas, field::canvas::SHOW_ITEM_MESSAGE)?.i();
        if canvas_show_item_message != 0 {
            let canvas_player_70_read_read = r.get(canvas, field::canvas::PLAYER)?.r();
            r.set(
                canvas_player_70_read_read,
                field::player::ATTACKING,
                V::I(0_i32),
            )?;
        } else {
            let canvas_world_8 = r.get(canvas, field::canvas::WORLD)?.r();
            let world_state_m_3 = r.get(canvas_world_8, field::world::STATE_M)?.i();
            if !(world_state_m_3 == 0) {
                let canvas_player_70_read_read = r.get(canvas, field::canvas::PLAYER)?.r();
                r.set(
                    canvas_player_70_read_read,
                    field::player::ATTACKING,
                    V::I(0_i32),
                )?;
            }
        }
    }
    let canvas_player_103 = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_attacking = r.get(canvas_player_103, field::player::ATTACKING)?.i();
    if !(player_attacking != 0) {
        let canvas_state_i_2 = r.get(canvas, field::canvas::STATE_I)?.i();
        if !(canvas_state_i_2 != 0) {
            let canvas_show_item_message_3 = r.get(canvas, field::canvas::SHOW_ITEM_MESSAGE)?.i();
            if !(canvas_show_item_message_3 != 0) {
                let canvas_world_14 = r.get(canvas, field::canvas::WORLD)?.r();
                let world_state_m_5 = r.get(canvas_world_14, field::world::STATE_M)?.i();
                if world_state_m_5 == 0 {
                    let canvas_player_107_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_has_sword_read =
                        r.get(canvas_player_107_read, field::player::HAS_SWORD)?.i();
                    if player_has_sword_read == 0 {
                        r.set(canvas, field::canvas::STATE_I, V::I(0_i32))?;
                        r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(0_i32))?;
                        let canvas_world_11_read = r.get(canvas, field::canvas::WORLD)?.r();
                        r.set(canvas_world_11_read, field::world::STATE_M, V::I(0_i32))?;
                        r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(0_i32))?;
                        let canvas_world_12_read = r.get(canvas, field::canvas::WORLD)?.r();
                        let canvas_player_109_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        let canvas_sound_9_read = r.get(canvas, field::canvas::SOUND)?.r();
                        let canvas_player_110_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        let player_has_sword_2_read =
                            r.get(canvas_player_110_read, field::player::HAS_SWORD)?.i();
                        let interact_value = super::world::interact(
                            r,
                            canvas_world_12_read,
                            canvas_player_109_read,
                            canvas_sound_9_read,
                            player_has_sword_2_read,
                        )?;
                        key = interact_value;
                        if interact_value <= 0 {
                            return Ok(());
                        }
                        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
                        return Ok(());
                    }
                    let canvas_player_111_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_attacking_2_read =
                        r.get(canvas_player_111_read, field::player::ATTACKING)?.i();
                    if !(player_attacking_2_read == 0) {
                        return Ok(());
                    }
                    let canvas_player_112_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_state_ah_read =
                        r.get(canvas_player_112_read, field::player::STATE_AH)?.i();
                    if !(player_state_ah_read == 0) {
                        return Ok(());
                    }
                    r.set(canvas, field::canvas::STATE_I, V::I(0_i32))?;
                    r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(0_i32))?;
                    let canvas_world_15_read = r.get(canvas, field::canvas::WORLD)?.r();
                    r.set(canvas_world_15_read, field::world::STATE_M, V::I(0_i32))?;
                    r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(0_i32))?;
                    let canvas_player_113_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(
                        canvas_player_113_read,
                        field::player::ATTACKING,
                        V::I(1_i32),
                    )?;
                    let canvas_player_114_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_114_read, field::player::DIGGING, V::I(0_i32))?;
                    let canvas_player_115_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(
                        canvas_player_115_read,
                        field::player::DIG_TIMER,
                        V::I(0_i32),
                    )?;
                    let canvas_player_116_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(
                        canvas_player_116_read,
                        field::player::HOOKSHOT_EXTENDING,
                        V::I(0_i32),
                    )?;
                    let canvas_player_117_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_hookshot_trail_2_read = r
                        .get(canvas_player_117_read, field::player::HOOKSHOT_TRAIL)?
                        .r();
                    host::vector_remove_all_elements(r, player_hookshot_trail_2_read)?;
                    let canvas_player_118_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(
                        canvas_player_118_read,
                        field::player::HOOKSHOT_RETURNING,
                        V::I(0_i32),
                    )?;
                    let canvas_player_119_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(
                        canvas_player_119_read,
                        field::player::ACTION_TIMER,
                        V::I(0_i32),
                    )?;
                    let canvas_player_120_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(canvas_player_120_read, field::player::STATE_AH, V::I(0_i32))?;
                    let canvas_player_121_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    r.set(
                        canvas_player_121_read,
                        field::player::ANIMATION,
                        V::I(1_i32),
                    )?;
                    let canvas_world_16_read = r.get(canvas, field::canvas::WORLD)?.r();
                    let canvas_player_122_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let canvas_sound_10_read = r.get(canvas, field::canvas::SOUND)?.r();
                    let canvas_player_123_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_has_sword_3_read =
                        r.get(canvas_player_123_read, field::player::HAS_SWORD)?.i();
                    let interact_value_2 = super::world::interact(
                        r,
                        canvas_world_16_read,
                        canvas_player_122_read,
                        canvas_sound_10_read,
                        player_has_sword_3_read,
                    )?;
                    key = interact_value_2;
                    if !(interact_value_2 <= 0) {
                        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
                    }

                    if !(key != 2_i32) {
                        let canvas_player_125_read = r.get(canvas, field::canvas::PLAYER)?.r();
                        r.set(
                            canvas_player_125_read,
                            field::player::ATTACKING,
                            V::I(0_i32),
                        )?;
                    }
                    let canvas_player_124_read = r.get(canvas, field::canvas::PLAYER)?.r();
                    let player_attacking_3_read =
                        r.get(canvas_player_124_read, field::player::ATTACKING)?.i();
                    if player_attacking_3_read == 0 {
                        return Ok(());
                    }
                    let canvas_sound_13_read = r.get(canvas, field::canvas::SOUND)?.r();
                    key = 1_i32;
                    canvas = canvas_sound_13_read;
                    r.set(
                        canvas_sound_13_read,
                        field::sound::PENDING_EFFECT,
                        V::I(key),
                    )?;
                    return Ok(());
                }
            }
        }
        let canvas_show_item_message_2 = r.get(canvas, field::canvas::SHOW_ITEM_MESSAGE)?.i();
        if !(canvas_show_item_message_2 == 0) {
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(0_i32))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_world_13 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_state_m_4 = r.get(canvas_world_13, field::world::STATE_M)?.i();
        if !(world_state_m_4 == 0) {
            let canvas_world_18 = r.get(canvas, field::canvas::WORLD)?.r();
            r.set(canvas_world_18, field::world::STATE_M, V::I(0_i32))?;
            return Ok(());
        }
        let canvas_world_17 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects = r.get(canvas_world_17, field::world::OBJECTS)?.r();
        let canvas_state_l = r.get(canvas, field::canvas::STATE_L)?.i();
        let vector_element_at_value = host::vector_element_at(r, world_objects, canvas_state_l)?;
        host::check_cast(r, vector_element_at_value, "n")?;
        object_1 = vector_element_at_value;
        r.set(canvas, field::canvas::STATE_I, V::I(0_i32))?;
        r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(0_i32))?;
        let map_object_kind = r.get(object_1, field::map_object::KIND)?.i();
        if !(map_object_kind != 16_i32) {
            return Ok(());
        }
        let canvas_map_4 = r.get(canvas, field::canvas::MAP)?.i();
        let quotient_value_5 = arithmetic::divide_int(r, canvas_map_4, 32_i32)?;
        number_2 = quotient_value_5;
        let map_object_persistence_id = r.get(object_1, field::map_object::PERSISTENCE_ID)?.i();
        let remainder_value = arithmetic::remainder_int(r, map_object_persistence_id, 32_i32)?;
        number_3 = remainder_value;
        let map_object_persistence_id_2 = r.get(object_1, field::map_object::PERSISTENCE_ID)?.i();
        if map_object_persistence_id_2 != 0 {
            let canvas_zone_5 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_5 != 0) {
                let canvas_player_126 = r.get(canvas, field::canvas::PLAYER)?.r();
                let overworld_events_value = r
                    .get(canvas_player_126, field::player::OVERWORLD_EVENTS)?
                    .r();
                let element_value_4 = r.array_get(overworld_events_value, number_2)?.i();
                r.array_set(
                    overworld_events_value,
                    number_2,
                    V::I((element_value_4 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_6 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_6 != 1_i32) {
                let canvas_player_127 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_cave_events = r.get(canvas_player_127, field::player::CAVE_EVENTS)?.r();
                let element_value_5 = r.array_get(player_cave_events, number_2)?.i();
                r.array_set(
                    player_cave_events,
                    number_2,
                    V::I((element_value_5 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_7 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_7 != 3_i32) {
                let canvas_player_128 = r.get(canvas, field::canvas::PLAYER)?.r();
                let dungeon_1_events_value = r
                    .get(canvas_player_128, field::player::DUNGEON_1_EVENTS)?
                    .r();
                let element_value_6 = r.array_get(dungeon_1_events_value, number_2)?.i();
                r.array_set(
                    dungeon_1_events_value,
                    number_2,
                    V::I((element_value_6 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_8 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_8 != 4_i32) {
                let canvas_player_129 = r.get(canvas, field::canvas::PLAYER)?.r();
                let dungeon_2_events_value = r
                    .get(canvas_player_129, field::player::DUNGEON_2_EVENTS)?
                    .r();
                let element_value_7 = r.array_get(dungeon_2_events_value, number_2)?.i();
                r.array_set(
                    dungeon_2_events_value,
                    number_2,
                    V::I((element_value_7 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_9 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_9 != 5_i32) {
                let canvas_player_130 = r.get(canvas, field::canvas::PLAYER)?.r();
                let dungeon_3_events_value = r
                    .get(canvas_player_130, field::player::DUNGEON_3_EVENTS)?
                    .r();
                let element_value_8 = r.array_get(dungeon_3_events_value, number_2)?.i();
                r.array_set(
                    dungeon_3_events_value,
                    number_2,
                    V::I((element_value_8 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_10 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_10 != 6_i32) {
                let canvas_player_131 = r.get(canvas, field::canvas::PLAYER)?.r();
                let dungeon_4_events_value = r
                    .get(canvas_player_131, field::player::DUNGEON_4_EVENTS)?
                    .r();
                let element_value_9 = r.array_get(dungeon_4_events_value, number_2)?.i();
                r.array_set(
                    dungeon_4_events_value,
                    number_2,
                    V::I((element_value_9 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_11 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_11 != 7_i32) {
                let canvas_player_132 = r.get(canvas, field::canvas::PLAYER)?.r();
                let dungeon_5_events_value = r
                    .get(canvas_player_132, field::player::DUNGEON_5_EVENTS)?
                    .r();
                let element_value_10 = r.array_get(dungeon_5_events_value, number_2)?.i();
                r.array_set(
                    dungeon_5_events_value,
                    number_2,
                    V::I((element_value_10 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_12 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_12 != 8_i32) {
                let canvas_player_133 = r.get(canvas, field::canvas::PLAYER)?.r();
                let dungeon_6_events_value = r
                    .get(canvas_player_133, field::player::DUNGEON_6_EVENTS)?
                    .r();
                let element_value_11 = r.array_get(dungeon_6_events_value, number_2)?.i();
                r.array_set(
                    dungeon_6_events_value,
                    number_2,
                    V::I((element_value_11 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_13 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_13 != 9_i32) {
                let canvas_player_134 = r.get(canvas, field::canvas::PLAYER)?.r();
                let dungeon_7_events_value = r
                    .get(canvas_player_134, field::player::DUNGEON_7_EVENTS)?
                    .r();
                let element_value_12 = r.array_get(dungeon_7_events_value, number_2)?.i();
                r.array_set(
                    dungeon_7_events_value,
                    number_2,
                    V::I((element_value_12 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_14 = r.get(canvas, field::canvas::ZONE)?.i();
            if !(canvas_zone_14 != 10_i32) {
                let canvas_player_135 = r.get(canvas, field::canvas::PLAYER)?.r();
                let dungeon_8_events_value = r
                    .get(canvas_player_135, field::player::DUNGEON_8_EVENTS)?
                    .r();
                let element_value_13 = r.array_get(dungeon_8_events_value, number_2)?.i();
                r.array_set(
                    dungeon_8_events_value,
                    number_2,
                    V::I((element_value_13 | (1_i32).wrapping_shl((number_3) as u32))),
                )?;
            }
            let canvas_zone_15 = r.get(canvas, field::canvas::ZONE)?.i();
            if canvas_zone_15 != 11_i32 {
                let canvas_world_20_read = r.get(canvas, field::canvas::WORLD)?.r();
                let world_objects_3_read = r.get(canvas_world_20_read, field::world::OBJECTS)?.r();
                let canvas_state_l_3_read = r.get(canvas, field::canvas::STATE_L)?.i();
                host::vector_remove_element_at(r, world_objects_3_read, canvas_state_l_3_read)?;
                return Ok(());
            }
            let canvas_player_136 = r.get(canvas, field::canvas::PLAYER)?.r();
            let dungeon_9_events_value = r
                .get(canvas_player_136, field::player::DUNGEON_9_EVENTS)?
                .r();
            let element_value_14 = r.array_get(dungeon_9_events_value, number_2)?.i();
            r.array_set(
                dungeon_9_events_value,
                number_2,
                V::I((element_value_14 | (1_i32).wrapping_shl((number_3) as u32))),
            )?;
            let canvas_world_20_read = r.get(canvas, field::canvas::WORLD)?.r();
            let world_objects_3_read = r.get(canvas_world_20_read, field::world::OBJECTS)?.r();
            let canvas_state_l_3_read = r.get(canvas, field::canvas::STATE_L)?.i();
            host::vector_remove_element_at(r, world_objects_3_read, canvas_state_l_3_read)?;
            return Ok(());
        }
        let map_object_has_dialogue = r.get(object_1, field::map_object::HAS_DIALOGUE)?.i();
        if map_object_has_dialogue >= 2_i32 {
            return Ok(());
        }
        let canvas_world_19 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_2 = r.get(canvas_world_19, field::world::OBJECTS)?.r();
        let canvas_state_l_2 = r.get(canvas, field::canvas::STATE_L)?.i();
        host::vector_remove_element_at(r, world_objects_2, canvas_state_l_2)?;
        return Ok(());
    }
    let canvas_player_107_read = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_has_sword_read = r.get(canvas_player_107_read, field::player::HAS_SWORD)?.i();
    if player_has_sword_read == 0 {
        r.set(canvas, field::canvas::STATE_I, V::I(0_i32))?;
        r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(0_i32))?;
        let canvas_world_11_read = r.get(canvas, field::canvas::WORLD)?.r();
        r.set(canvas_world_11_read, field::world::STATE_M, V::I(0_i32))?;
        r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(0_i32))?;
        let canvas_world_12_read = r.get(canvas, field::canvas::WORLD)?.r();
        let canvas_player_109_read = r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_sound_9_read = r.get(canvas, field::canvas::SOUND)?.r();
        let canvas_player_110_read = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_has_sword_2_read = r.get(canvas_player_110_read, field::player::HAS_SWORD)?.i();
        let interact_value = super::world::interact(
            r,
            canvas_world_12_read,
            canvas_player_109_read,
            canvas_sound_9_read,
            player_has_sword_2_read,
        )?;
        key = interact_value;
        if interact_value <= 0 {
            return Ok(());
        }
        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
        return Ok(());
    }
    let canvas_player_111_read = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_attacking_2_read = r.get(canvas_player_111_read, field::player::ATTACKING)?.i();
    if !(player_attacking_2_read == 0) {
        return Ok(());
    }
    let canvas_player_112_read = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_state_ah_read = r.get(canvas_player_112_read, field::player::STATE_AH)?.i();
    if !(player_state_ah_read == 0) {
        return Ok(());
    }
    r.set(canvas, field::canvas::STATE_I, V::I(0_i32))?;
    r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(0_i32))?;
    let canvas_world_15_read = r.get(canvas, field::canvas::WORLD)?.r();
    r.set(canvas_world_15_read, field::world::STATE_M, V::I(0_i32))?;
    r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(0_i32))?;
    let canvas_player_113_read = r.get(canvas, field::canvas::PLAYER)?.r();
    r.set(
        canvas_player_113_read,
        field::player::ATTACKING,
        V::I(1_i32),
    )?;
    let canvas_player_114_read = r.get(canvas, field::canvas::PLAYER)?.r();
    r.set(canvas_player_114_read, field::player::DIGGING, V::I(0_i32))?;
    let canvas_player_115_read = r.get(canvas, field::canvas::PLAYER)?.r();
    r.set(
        canvas_player_115_read,
        field::player::DIG_TIMER,
        V::I(0_i32),
    )?;
    let canvas_player_116_read = r.get(canvas, field::canvas::PLAYER)?.r();
    r.set(
        canvas_player_116_read,
        field::player::HOOKSHOT_EXTENDING,
        V::I(0_i32),
    )?;
    let canvas_player_117_read = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_hookshot_trail_2_read = r
        .get(canvas_player_117_read, field::player::HOOKSHOT_TRAIL)?
        .r();
    host::vector_remove_all_elements(r, player_hookshot_trail_2_read)?;
    let canvas_player_118_read = r.get(canvas, field::canvas::PLAYER)?.r();
    r.set(
        canvas_player_118_read,
        field::player::HOOKSHOT_RETURNING,
        V::I(0_i32),
    )?;
    let canvas_player_119_read = r.get(canvas, field::canvas::PLAYER)?.r();
    r.set(
        canvas_player_119_read,
        field::player::ACTION_TIMER,
        V::I(0_i32),
    )?;
    let canvas_player_120_read = r.get(canvas, field::canvas::PLAYER)?.r();
    r.set(canvas_player_120_read, field::player::STATE_AH, V::I(0_i32))?;
    let canvas_player_121_read = r.get(canvas, field::canvas::PLAYER)?.r();
    r.set(
        canvas_player_121_read,
        field::player::ANIMATION,
        V::I(1_i32),
    )?;
    let canvas_world_16_read = r.get(canvas, field::canvas::WORLD)?.r();
    let canvas_player_122_read = r.get(canvas, field::canvas::PLAYER)?.r();
    let canvas_sound_10_read = r.get(canvas, field::canvas::SOUND)?.r();
    let canvas_player_123_read = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_has_sword_3_read = r.get(canvas_player_123_read, field::player::HAS_SWORD)?.i();
    let interact_value_2 = super::world::interact(
        r,
        canvas_world_16_read,
        canvas_player_122_read,
        canvas_sound_10_read,
        player_has_sword_3_read,
    )?;
    key = interact_value_2;
    if !(interact_value_2 <= 0) {
        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1_i32))?;
    }

    if !(key != 2_i32) {
        let canvas_player_125_read = r.get(canvas, field::canvas::PLAYER)?.r();
        r.set(
            canvas_player_125_read,
            field::player::ATTACKING,
            V::I(0_i32),
        )?;
    }
    let canvas_player_124_read = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_attacking_3_read = r.get(canvas_player_124_read, field::player::ATTACKING)?.i();
    if player_attacking_3_read == 0 {
        return Ok(());
    }
    let canvas_sound_13_read = r.get(canvas, field::canvas::SOUND)?.r();
    key = 1_i32;
    canvas = canvas_sound_13_read;
    r.set(
        canvas_sound_13_read,
        field::sound::PENDING_EFFECT,
        V::I(key),
    )?;
    return Ok(());
}

/// Release control. Original reference: `k.c(I)V`.
pub(in crate::game) fn release_control(r: &mut Runtime, canvas: u32, key: i32) -> Result<(), u32> {
    let canvas_left_key = r.get(canvas, field::canvas::LEFT_KEY)?.i();
    if !(key != canvas_left_key) {
        let canvas_state_ad_upper = r.get(canvas, field::canvas::STATE_AD_UPPER)?.i();
        if !(canvas_state_ad_upper != -3_i32) {
            r.set(canvas, field::canvas::STATE_AD_UPPER, V::I(0_i32))?;
        }
        let canvas_state_aq = r.get(canvas, field::canvas::STATE_AQ)?.i();
        if canvas_state_aq == 0 {
            return Ok(());
        }
        return Ok(());
    }
    let canvas_right_key = r.get(canvas, field::canvas::RIGHT_KEY)?.i();
    if !(key != canvas_right_key) {
        let canvas_state_ad_upper_2 = r.get(canvas, field::canvas::STATE_AD_UPPER)?.i();
        if !(canvas_state_ad_upper_2 != 3_i32) {
            r.set(canvas, field::canvas::STATE_AD_UPPER, V::I(0_i32))?;
        }
        let canvas_state_aq_2 = r.get(canvas, field::canvas::STATE_AQ)?.i();
        if canvas_state_aq_2 == 0 {
            return Ok(());
        }
        return Ok(());
    }
    let canvas_up_key = r.get(canvas, field::canvas::UP_KEY)?.i();
    if !(key != canvas_up_key) {
        let canvas_state_ae = r.get(canvas, field::canvas::STATE_AE)?.i();
        if !(canvas_state_ae != -3_i32) {
            r.set(canvas, field::canvas::STATE_AE, V::I(0_i32))?;
        }
        let canvas_state_aq_3 = r.get(canvas, field::canvas::STATE_AQ)?.i();
        if canvas_state_aq_3 == 0 {
            return Ok(());
        }
        return Ok(());
    }
    let canvas_down_key = r.get(canvas, field::canvas::DOWN_KEY)?.i();
    if key != canvas_down_key {
        let canvas_secondary_key = r.get(canvas, field::canvas::SECONDARY_KEY)?.i();
        if key != canvas_secondary_key {
            return Ok(());
        }
        r.set(canvas, field::canvas::STATE_AQ, V::I(0_i32))?;
        r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(0_i32))?;
        r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(0_i32))?;
        return Ok(());
    }
    let canvas_state_ae_2 = r.get(canvas, field::canvas::STATE_AE)?.i();
    if !(canvas_state_ae_2 != 3_i32) {
        r.set(canvas, field::canvas::STATE_AE, V::I(0_i32))?;
    }
    let canvas_state_aq_4 = r.get(canvas, field::canvas::STATE_AQ)?.i();
    if canvas_state_aq_4 == 0 {
        return Ok(());
    }
    return Ok(());
}
