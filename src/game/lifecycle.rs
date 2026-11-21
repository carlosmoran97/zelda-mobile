//! Application lifecycle and softkey commands.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `Zelda.<init>()V`.
pub(super) fn initialize(r: &mut Runtime, application: u32) -> Result<(), u32> {
    host::m_i_dlet_initialize(r, application)?;
    let frame_timer_value = r.alloc("m", Native::Empty);
    super::frame_timer::initialize(r, frame_timer_value, application)?;
    r.set(
        application,
        field::lifecycle::FRAME_TIMER,
        V::R(frame_timer_value),
    )?;
    let object_value = r.alloc("java/util/Timer", Native::Empty);
    host::timer_initialize(r, object_value)?;
    r.set(application, field::lifecycle::TIMER, V::R(object_value))?;
    let object_value_2 = r.alloc("java/util/Timer", Native::Empty);
    host::timer_initialize(r, object_value_2)?;
    let second_timer_value = r.alloc("a", Native::Empty);
    super::second_timer::initialize(r, second_timer_value, application)?;
    r.set(
        application,
        field::lifecycle::SECOND_TIMER,
        V::R(second_timer_value),
    )?;
    let display_get_display_value = host::display_get_display(r, application)?;
    r.set(
        application,
        field::lifecycle::DISPLAY,
        V::R(display_get_display_value),
    )?;
    let canvas_value = r.alloc("k", Native::Empty);
    let lifecycle_display = r.get(application, field::lifecycle::DISPLAY)?.r();
    super::canvas::initialize(r, canvas_value, lifecycle_display, application)?;
    r.set(application, field::lifecycle::CANVAS, V::R(canvas_value))?;
    let object_value_3 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value = r.intern("Start");
    host::command_initialize(r, object_value_3, text_value, 1, 1)?;
    r.set(
        application,
        field::lifecycle::START_COMMAND,
        V::R(object_value_3),
    )?;
    let object_value_4 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value_2 = r.intern("Play");
    host::command_initialize(r, object_value_4, text_value_2, 1, 1)?;
    r.set(
        application,
        field::lifecycle::PLAY_COMMAND,
        V::R(object_value_4),
    )?;
    let object_value_5 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value_3 = r.intern("Menu");
    host::command_initialize(r, object_value_5, text_value_3, 1, 1)?;
    r.set(
        application,
        field::lifecycle::MENU_COMMAND,
        V::R(object_value_5),
    )?;
    let object_value_6 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value_4 = r.intern("Settings");
    host::command_initialize(r, object_value_6, text_value_4, 1, 2)?;
    r.set(
        application,
        field::lifecycle::SETTINGS_COMMAND,
        V::R(object_value_6),
    )?;
    let object_value_7 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value_5 = r.intern("Exit");
    host::command_initialize(r, object_value_7, text_value_5, 1, 2)?;
    r.set(
        application,
        field::lifecycle::EXIT_COMMAND,
        V::R(object_value_7),
    )?;
    let object_value_8 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value_6 = r.intern("Save & exit");
    host::command_initialize(r, object_value_8, text_value_6, 1, 2)?;
    r.set(
        application,
        field::lifecycle::SAVE_EXIT_COMMAND,
        V::R(object_value_8),
    )?;
    let object_value_9 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value_7 = r.intern("Return");
    host::command_initialize(r, object_value_9, text_value_7, 1, 1)?;
    r.set(
        application,
        field::lifecycle::RETURN_COMMAND,
        V::R(object_value_9),
    )?;
    let object_value_10 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value_8 = r.intern("Default");
    host::command_initialize(r, object_value_10, text_value_8, 1, 1)?;
    r.set(
        application,
        field::lifecycle::DEFAULT_COMMAND,
        V::R(object_value_10),
    )?;
    let object_value_11 = r.alloc("javax/microedition/lcdui/Command", Native::Empty);
    let text_value_9 = r.intern("Main Menu");
    host::command_initialize(r, object_value_11, text_value_9, 1, 1)?;
    r.set(
        application,
        field::lifecycle::MAIN_MENU_COMMAND,
        V::R(object_value_11),
    )?;
    let lifecycle_canvas_2 = r.get(application, field::lifecycle::CANVAS)?.r();
    let lifecycle_start_command = r.get(application, field::lifecycle::START_COMMAND)?.r();
    host::canvas_add_command(r, lifecycle_canvas_2, lifecycle_start_command)?;
    let lifecycle_canvas_3 = r.get(application, field::lifecycle::CANVAS)?.r();
    host::canvas_set_command_listener(r, lifecycle_canvas_3, application)?;
    let lifecycle_timer = r.get(application, field::lifecycle::TIMER)?.r();
    let lifecycle_frame_timer_2 = r.get(application, field::lifecycle::FRAME_TIMER)?.r();
    host::timer_schedule_at_fixed_rate(r, lifecycle_timer, lifecycle_frame_timer_2, 100, 70)?;
    let lifecycle_timer_2 = r.get(application, field::lifecycle::TIMER)?.r();
    let lifecycle_second_timer_2 = r.get(application, field::lifecycle::SECOND_TIMER)?.r();
    host::timer_schedule_at_fixed_rate(r, lifecycle_timer_2, lifecycle_second_timer_2, 100, 1000)?;
    return Ok(());
}

/// Start. Original reference: `Zelda.startApp()V`.
pub(super) fn start(r: &mut Runtime, mut application: u32) -> Result<(), u32> {
    let lifecycle_canvas = r.get(application, field::lifecycle::CANVAS)?.r();
    application = lifecycle_canvas;
    let canvas_state_c = r.get(lifecycle_canvas, field::canvas::STATE_C)?.i();
    if canvas_state_c == 0 {
        return Ok(());
    }
    r.set(application, field::canvas::STATE_C, V::I(0))?;
    let canvas_display = r.get(application, field::canvas::DISPLAY)?.r();
    host::display_set_current(r, canvas_display, application)?;
    return Ok(());
}

/// Pause. Original reference: `Zelda.pauseApp()V`.
pub(super) fn pause(r: &mut Runtime, mut application: u32) -> Result<(), u32> {
    let lifecycle_canvas = r.get(application, field::lifecycle::CANVAS)?.r();
    application = lifecycle_canvas;
    let canvas_state_c = r.get(lifecycle_canvas, field::canvas::STATE_C)?.i();
    if canvas_state_c != 0 {
        return Ok(());
    }
    r.set(application, field::canvas::STATE_C, V::I(1))?;
    return Ok(());
}

/// Destroy. Original reference: `Zelda.destroyApp(Z)V`.
pub(super) fn destroy(r: &mut Runtime, application: u32, argument_1: i32) -> Result<(), u32> {
    let mut object_1: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let lifecycle_canvas = r.get(application, field::lifecycle::CANVAS)?.r();
    let object_get_class_value = host::object_get_class(r, lifecycle_canvas)?;
    let lifecycle_canvas_2 = r.get(application, field::lifecycle::CANVAS)?.r();
    let canvas_sound = r.get(lifecycle_canvas_2, field::canvas::SOUND)?.r();
    object_1 = canvas_sound;
    let sound_enabled = r.get(canvas_sound, field::sound::ENABLED)?.i();
    if sound_enabled == 0 {
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    outcome = (|| -> JResult {
        r.set(object_1, field::sound::STOP_REQUESTED, V::I(1))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    outcome = (|| -> JResult {
        host::thread_sleep(r, 100)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(object_1, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    let music_player_value = outcome.unwrap().r();
    outcome = (|| -> JResult { Ok(V::I(host::player_get_state(r, music_player_value)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    let player_get_state_value = outcome.unwrap().i();
    if player_get_state_value == 400 {
        outcome = (|| -> JResult { Ok(V::R(r.get(object_1, field::sound::MUSIC_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            object_1 = carry_0_r;
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, object_1)?;
            host::print_stream_println(r, state_out_value, exception_to_string_value)?;
            let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
            super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
            let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
            host::display_set_current(r, lifecycle_display_read, 0)?;
            host::system_gc(r)?;
            host::m_i_dlet_notify_destroyed(r, application)?;
            return Ok(());
        }
        let music_player_value_2 = outcome.unwrap().r();
        outcome = (|| -> JResult {
            host::player_stop(r, music_player_value_2)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            object_1 = carry_0_r;
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, object_1)?;
            host::print_stream_println(r, state_out_value, exception_to_string_value)?;
            let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
            super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
            let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
            host::display_set_current(r, lifecycle_display_read, 0)?;
            host::system_gc(r)?;
            host::m_i_dlet_notify_destroyed(r, application)?;
            return Ok(());
        }
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(object_1, field::sound::EFFECT_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    let effect_player_value = outcome.unwrap().r();
    outcome = (|| -> JResult { Ok(V::I(host::player_get_state(r, effect_player_value)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    let player_get_state_value_2 = outcome.unwrap().i();
    if player_get_state_value_2 == 400 {
        outcome = (|| -> JResult { Ok(V::R(r.get(object_1, field::sound::EFFECT_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            object_1 = carry_0_r;
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, object_1)?;
            host::print_stream_println(r, state_out_value, exception_to_string_value)?;
            let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
            super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
            let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
            host::display_set_current(r, lifecycle_display_read, 0)?;
            host::system_gc(r)?;
            host::m_i_dlet_notify_destroyed(r, application)?;
            return Ok(());
        }
        let effect_player_value_3 = outcome.unwrap().r();
        outcome = (|| -> JResult {
            host::player_stop(r, effect_player_value_3)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            object_1 = carry_0_r;
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, object_1)?;
            host::print_stream_println(r, state_out_value, exception_to_string_value)?;
            let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
            super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
            let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
            host::display_set_current(r, lifecycle_display_read, 0)?;
            host::system_gc(r)?;
            host::m_i_dlet_notify_destroyed(r, application)?;
            return Ok(());
        }
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(object_1, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    let music_player_value_3 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_close(r, music_player_value_3)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(object_1, field::sound::EFFECT_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }
    let effect_player_value_2 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_close(r, effect_player_value_2)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value, exception_to_string_value)?;
        let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
        let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
        host::display_set_current(r, lifecycle_display_read, 0)?;
        host::system_gc(r)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
        return Ok(());
    }

    let lifecycle_canvas_3_read = r.get(application, field::lifecycle::CANVAS)?.r();
    super::canvas::save_settings(r, lifecycle_canvas_3_read)?;
    let lifecycle_display_read = r.get(application, field::lifecycle::DISPLAY)?.r();
    host::display_set_current(r, lifecycle_display_read, 0)?;
    host::system_gc(r)?;
    host::m_i_dlet_notify_destroyed(r, application)?;
    return Ok(());
}

/// Command. Original reference: `Zelda.commandAction(Ljavax/microedition/lcdui/Command;Ljavax/microedition/lcdui/Displayable;)V`.
pub(super) fn command(
    r: &mut Runtime,
    application: u32,
    argument_1: u32,
    argument_2: u32,
) -> Result<(), u32> {
    let lifecycle_start_command = r.get(application, field::lifecycle::START_COMMAND)?.r();
    if argument_1 == lifecycle_start_command {
        let lifecycle_canvas = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_start_command_2 = r.get(application, field::lifecycle::START_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas, lifecycle_start_command_2)?;
        let lifecycle_canvas_2 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_default_command_2 =
            r.get(application, field::lifecycle::DEFAULT_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas_2, lifecycle_default_command_2)?;
        let lifecycle_canvas_3 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_3, field::canvas::SCREEN_STATE, V::I(30))?;
        let lifecycle_canvas_4 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_4, field::canvas::REFRESH_COMMANDS, V::I(1))?;
        let lifecycle_canvas_5 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_5, field::canvas::MENU_SELECTION, V::I(0))?;
        let lifecycle_canvas_6 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_6, field::canvas::INVENTORY_OPEN, V::I(1))?;
        let lifecycle_canvas_7 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_7, field::canvas::INVENTORY_PAGE, V::I(4))?;
        let lifecycle_canvas_8 = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::load_inventory_images(r, lifecycle_canvas_8)?;
        let lifecycle_canvas_9 = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::load_save_slots(r, lifecycle_canvas_9)?;
        let lifecycle_canvas_10 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_10, field::canvas::INVENTORY_OPEN, V::I(0))?;
        let lifecycle_canvas_11 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_11, field::canvas::INVENTORY_PAGE, V::I(0))?;
    }
    let lifecycle_default_command = r.get(application, field::lifecycle::DEFAULT_COMMAND)?.r();
    if argument_1 == lifecycle_default_command {
        let lifecycle_canvas_12 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_12, field::canvas::UP_KEY, V::I(50))?;
        let lifecycle_canvas_13 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_13, field::canvas::DOWN_KEY, V::I(56))?;
        let lifecycle_canvas_14 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_14, field::canvas::LEFT_KEY, V::I(52))?;
        let lifecycle_canvas_15 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_15, field::canvas::RIGHT_KEY, V::I(54))?;
        let lifecycle_canvas_16 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_16, field::canvas::PRIMARY_KEY, V::I(53))?;
        let lifecycle_canvas_17 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_17, field::canvas::SECONDARY_KEY, V::I(55))?;
        let lifecycle_canvas_18 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_18, field::canvas::MENU_KEY, V::I(57))?;
        let lifecycle_canvas_19 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_19, field::canvas::STATE_W, V::I(1))?;
    }
    let lifecycle_exit_command = r.get(application, field::lifecycle::EXIT_COMMAND)?.r();
    if argument_1 == lifecycle_exit_command {
        super::lifecycle::destroy(r, application, 0)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
    }
    let lifecycle_save_exit_command = r.get(application, field::lifecycle::SAVE_EXIT_COMMAND)?.r();
    if argument_1 == lifecycle_save_exit_command {
        let lifecycle_canvas_20 = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::save_game(r, lifecycle_canvas_20)?;
        super::lifecycle::destroy(r, application, 0)?;
        host::m_i_dlet_notify_destroyed(r, application)?;
    }
    let lifecycle_settings_command = r.get(application, field::lifecycle::SETTINGS_COMMAND)?.r();
    if argument_1 == lifecycle_settings_command {
        let lifecycle_canvas_21 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_settings_command_2 =
            r.get(application, field::lifecycle::SETTINGS_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas_21, lifecycle_settings_command_2)?;
        let lifecycle_canvas_22 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_menu_command = r.get(application, field::lifecycle::MENU_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas_22, lifecycle_menu_command)?;
        let lifecycle_canvas_23 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_return_command = r.get(application, field::lifecycle::RETURN_COMMAND)?.r();
        host::canvas_add_command(r, lifecycle_canvas_23, lifecycle_return_command)?;
        let lifecycle_canvas_24 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_default_command_3 =
            r.get(application, field::lifecycle::DEFAULT_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas_24, lifecycle_default_command_3)?;
        let lifecycle_canvas_25 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_canvas_26 = r.get(application, field::lifecycle::CANVAS)?.r();
        let canvas_screen_state = r.get(lifecycle_canvas_26, field::canvas::SCREEN_STATE)?.i();
        r.set(
            lifecycle_canvas_25,
            field::canvas::PREVIOUS_SCREEN,
            V::I(canvas_screen_state),
        )?;
        let lifecycle_canvas_27 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_27, field::canvas::SCREEN_STATE, V::I(7))?;
    }
    let lifecycle_play_command = r.get(application, field::lifecycle::PLAY_COMMAND)?.r();
    if argument_1 == lifecycle_play_command {
        let lifecycle_canvas_28 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_canvas_29 = r.get(application, field::lifecycle::CANVAS)?.r();
        let canvas_primary_key = r.get(lifecycle_canvas_29, field::canvas::PRIMARY_KEY)?.i();
        super::canvas::press_control(r, lifecycle_canvas_28, canvas_primary_key)?;
        let lifecycle_canvas_30 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_play_command_2 = r.get(application, field::lifecycle::PLAY_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas_30, lifecycle_play_command_2)?;
    }
    let lifecycle_return_command_2 = r.get(application, field::lifecycle::RETURN_COMMAND)?.r();
    if argument_1 == lifecycle_return_command_2 {
        let lifecycle_canvas_31 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_return_command_3 = r.get(application, field::lifecycle::RETURN_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas_31, lifecycle_return_command_3)?;
        let lifecycle_canvas_32 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_save_exit_command_2 =
            r.get(application, field::lifecycle::SAVE_EXIT_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas_32, lifecycle_save_exit_command_2)?;
        let lifecycle_canvas_33 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_menu_command_3 = r.get(application, field::lifecycle::MENU_COMMAND)?.r();
        host::canvas_add_command(r, lifecycle_canvas_33, lifecycle_menu_command_3)?;
        let lifecycle_canvas_34 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_default_command_4 =
            r.get(application, field::lifecycle::DEFAULT_COMMAND)?.r();
        host::canvas_remove_command(r, lifecycle_canvas_34, lifecycle_default_command_4)?;
        let lifecycle_canvas_35 = r.get(application, field::lifecycle::CANVAS)?.r();
        let lifecycle_settings_command_3 =
            r.get(application, field::lifecycle::SETTINGS_COMMAND)?.r();
        host::canvas_add_command(r, lifecycle_canvas_35, lifecycle_settings_command_3)?;
        let lifecycle_canvas_36 = r.get(application, field::lifecycle::CANVAS)?.r();
        super::canvas::set_inventory_open(r, lifecycle_canvas_36, 0)?;
        let lifecycle_canvas_37 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_37, field::canvas::SCREEN_STATE, V::I(1))?;
        let lifecycle_canvas_38 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_38, field::canvas::INVENTORY_PAGE, V::I(0))?;
        let lifecycle_canvas_39 = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_39, field::canvas::INVENTORY_OPEN, V::I(0))?;
    }
    let lifecycle_menu_command_2 = r.get(application, field::lifecycle::MENU_COMMAND)?.r();
    if argument_1 != lifecycle_menu_command_2 {
        let lifecycle_main_menu_command_read =
            r.get(application, field::lifecycle::MAIN_MENU_COMMAND)?.r();
        if argument_1 != lifecycle_main_menu_command_read {
            return Ok(());
        }
        let lifecycle_canvas_41_read = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(
            lifecycle_canvas_41_read,
            field::canvas::SCREEN_STATE,
            V::I(0),
        )?;
        let lifecycle_canvas_42_read = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_42_read, field::canvas::STATE_U, V::I(0))?;
        let lifecycle_canvas_43_read = r.get(application, field::lifecycle::CANVAS)?.r();
        r.set(lifecycle_canvas_43_read, field::canvas::STATE_V, V::I(1))?;
        return Ok(());
    }
    let lifecycle_canvas_40 = r.get(application, field::lifecycle::CANVAS)?.r();
    let canvas_screen_state_2 = r.get(lifecycle_canvas_40, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_2 == 0 {
        return Ok(());
    }
    let lifecycle_canvas_44 = r.get(application, field::lifecycle::CANVAS)?.r();
    let canvas_screen_state_3 = r.get(lifecycle_canvas_44, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_3 == -1 {
        return Ok(());
    }
    let lifecycle_canvas_45 = r.get(application, field::lifecycle::CANVAS)?.r();
    let lifecycle_menu_command_4 = r.get(application, field::lifecycle::MENU_COMMAND)?.r();
    host::canvas_remove_command(r, lifecycle_canvas_45, lifecycle_menu_command_4)?;
    let lifecycle_canvas_46 = r.get(application, field::lifecycle::CANVAS)?.r();
    let lifecycle_settings_command_4 = r.get(application, field::lifecycle::SETTINGS_COMMAND)?.r();
    host::canvas_remove_command(r, lifecycle_canvas_46, lifecycle_settings_command_4)?;
    let lifecycle_canvas_47 = r.get(application, field::lifecycle::CANVAS)?.r();
    let lifecycle_save_exit_command_3 =
        r.get(application, field::lifecycle::SAVE_EXIT_COMMAND)?.r();
    host::canvas_add_command(r, lifecycle_canvas_47, lifecycle_save_exit_command_3)?;
    let lifecycle_canvas_48 = r.get(application, field::lifecycle::CANVAS)?.r();
    let lifecycle_return_command_4 = r.get(application, field::lifecycle::RETURN_COMMAND)?.r();
    host::canvas_add_command(r, lifecycle_canvas_48, lifecycle_return_command_4)?;
    let lifecycle_canvas_49 = r.get(application, field::lifecycle::CANVAS)?.r();
    let lifecycle_default_command_5 = r.get(application, field::lifecycle::DEFAULT_COMMAND)?.r();
    host::canvas_remove_command(r, lifecycle_canvas_49, lifecycle_default_command_5)?;
    let lifecycle_canvas_50 = r.get(application, field::lifecycle::CANVAS)?.r();
    r.set(lifecycle_canvas_50, field::canvas::INVENTORY_PAGE, V::I(1))?;
    let lifecycle_canvas_51 = r.get(application, field::lifecycle::CANVAS)?.r();
    r.set(lifecycle_canvas_51, field::canvas::INVENTORY_OPEN, V::I(1))?;
    let lifecycle_canvas_52 = r.get(application, field::lifecycle::CANVAS)?.r();
    super::canvas::set_inventory_open(r, lifecycle_canvas_52, 1)?;
    let lifecycle_main_menu_command_read =
        r.get(application, field::lifecycle::MAIN_MENU_COMMAND)?.r();
    if argument_1 != lifecycle_main_menu_command_read {
        return Ok(());
    }
    let lifecycle_canvas_41_read = r.get(application, field::lifecycle::CANVAS)?.r();
    r.set(
        lifecycle_canvas_41_read,
        field::canvas::SCREEN_STATE,
        V::I(0),
    )?;
    let lifecycle_canvas_42_read = r.get(application, field::lifecycle::CANVAS)?.r();
    r.set(lifecycle_canvas_42_read, field::canvas::STATE_U, V::I(0))?;
    let lifecycle_canvas_43_read = r.get(application, field::lifecycle::CANVAS)?.r();
    r.set(lifecycle_canvas_43_read, field::canvas::STATE_V, V::I(1))?;
    return Ok(());
}

/// Canvas. Original reference: `Zelda.a(LZelda;)Lk;`.
pub(super) fn canvas(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_canvas = r.get(application, field::lifecycle::CANVAS)?.r();
    return Ok(lifecycle_canvas);
}

/// Exit command. Original reference: `Zelda.b(LZelda;)Ljavax/microedition/lcdui/Command;`.
pub(super) fn exit_command(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_menu_command = r.get(application, field::lifecycle::MENU_COMMAND)?.r();
    return Ok(lifecycle_menu_command);
}

/// Menu command. Original reference: `Zelda.c(LZelda;)Ljavax/microedition/lcdui/Command;`.
pub(super) fn menu_command(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_return_command = r.get(application, field::lifecycle::RETURN_COMMAND)?.r();
    return Ok(lifecycle_return_command);
}

/// Save exit command. Original reference: `Zelda.d(LZelda;)Ljavax/microedition/lcdui/Command;`.
pub(super) fn save_exit_command(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_save_exit_command = r.get(application, field::lifecycle::SAVE_EXIT_COMMAND)?.r();
    return Ok(lifecycle_save_exit_command);
}

/// Return command. Original reference: `Zelda.e(LZelda;)Ljavax/microedition/lcdui/Command;`.
pub(super) fn return_command(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_default_command = r.get(application, field::lifecycle::DEFAULT_COMMAND)?.r();
    return Ok(lifecycle_default_command);
}

/// Start command. Original reference: `Zelda.f(LZelda;)Ljavax/microedition/lcdui/Command;`.
pub(super) fn start_command(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_start_command = r.get(application, field::lifecycle::START_COMMAND)?.r();
    return Ok(lifecycle_start_command);
}

/// Settings command. Original reference: `Zelda.g(LZelda;)Ljavax/microedition/lcdui/Command;`.
pub(super) fn settings_command(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_settings_command = r.get(application, field::lifecycle::SETTINGS_COMMAND)?.r();
    return Ok(lifecycle_settings_command);
}

/// Default command. Original reference: `Zelda.h(LZelda;)Ljavax/microedition/lcdui/Command;`.
pub(super) fn default_command(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_main_menu_command = r.get(application, field::lifecycle::MAIN_MENU_COMMAND)?.r();
    return Ok(lifecycle_main_menu_command);
}

/// Play command. Original reference: `Zelda.i(LZelda;)Ljavax/microedition/lcdui/Command;`.
pub(super) fn play_command(r: &mut Runtime, application: u32) -> Result<u32, u32> {
    let lifecycle_play_command = r.get(application, field::lifecycle::PLAY_COMMAND)?.r();
    return Ok(lifecycle_play_command);
}
