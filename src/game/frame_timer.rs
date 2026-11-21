//! The 70 ms repaint and command callback.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `m.<init>(LZelda;)V`.
pub(super) fn initialize(r: &mut Runtime, timer: u32, application: u32) -> Result<(), u32> {
    host::timer_task_initialize(r, timer)?;
    r.set(timer, field::frame_timer::APPLICATION, V::R(application))?;
    return Ok(());
}

/// Tick. Original reference: `m.run()V`.
pub(super) fn tick(r: &mut Runtime, timer: u32) -> Result<(), u32> {
    let frame_timer_application = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value = super::lifecycle::canvas(r, frame_timer_application)?;
    host::canvas_repaint(r, canvas_value)?;
    let frame_timer_application_2 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_2 = super::lifecycle::canvas(r, frame_timer_application_2)?;
    let canvas_refresh_commands = r.get(canvas_value_2, field::canvas::REFRESH_COMMANDS)?.i();
    if canvas_refresh_commands == 0 {
        let frame_timer_application_3_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_3 = super::lifecycle::canvas(r, frame_timer_application_3_read)?;
        let canvas_exit_requested_read = r.get(canvas_value_3, field::canvas::EXIT_REQUESTED)?.i();
        if canvas_exit_requested_read == 0 {
            return Ok(());
        }
        let frame_timer_application_5_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
        super::lifecycle::destroy(r, frame_timer_application_5_read, 0)?;
        return Ok(());
    }
    let frame_timer_application_4 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_4 = super::lifecycle::canvas(r, frame_timer_application_4)?;
    let canvas_screen_state = r.get(canvas_value_4, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state == 0 {
        let frame_timer_application_7 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_6 = super::lifecycle::canvas(r, frame_timer_application_7)?;
        let frame_timer_application_8 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let exit_command_value = super::lifecycle::exit_command(r, frame_timer_application_8)?;
        host::canvas_remove_command(r, canvas_value_6, exit_command_value)?;
        let frame_timer_application_9 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_7 = super::lifecycle::canvas(r, frame_timer_application_9)?;
        let frame_timer_application_10 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let menu_command_value = super::lifecycle::menu_command(r, frame_timer_application_10)?;
        host::canvas_remove_command(r, canvas_value_7, menu_command_value)?;
        let frame_timer_application_11 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_8 = super::lifecycle::canvas(r, frame_timer_application_11)?;
        let frame_timer_application_12 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let save_exit_command_value =
            super::lifecycle::save_exit_command(r, frame_timer_application_12)?;
        host::canvas_remove_command(r, canvas_value_8, save_exit_command_value)?;
        let frame_timer_application_13 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_9 = super::lifecycle::canvas(r, frame_timer_application_13)?;
        let frame_timer_application_14 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let return_command_value = super::lifecycle::return_command(r, frame_timer_application_14)?;
        host::canvas_remove_command(r, canvas_value_9, return_command_value)?;
        let frame_timer_application_15 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_10 = super::lifecycle::canvas(r, frame_timer_application_15)?;
        let frame_timer_application_16 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let start_command_value = super::lifecycle::start_command(r, frame_timer_application_16)?;
        host::canvas_add_command(r, canvas_value_10, start_command_value)?;
        let frame_timer_application_17 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_11 = super::lifecycle::canvas(r, frame_timer_application_17)?;
        let frame_timer_application_18 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let settings_command_value =
            super::lifecycle::settings_command(r, frame_timer_application_18)?;
        host::canvas_remove_command(r, canvas_value_11, settings_command_value)?;
        let frame_timer_application_19 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_12 = super::lifecycle::canvas(r, frame_timer_application_19)?;
        let frame_timer_application_20 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let default_command_value =
            super::lifecycle::default_command(r, frame_timer_application_20)?;
        host::canvas_remove_command(r, canvas_value_12, default_command_value)?;
    }
    let frame_timer_application_6 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_5 = super::lifecycle::canvas(r, frame_timer_application_6)?;
    let canvas_screen_state_2 = r.get(canvas_value_5, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_2 == 7 {
        let frame_timer_application_22 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_14 = super::lifecycle::canvas(r, frame_timer_application_22)?;
        let frame_timer_application_23 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let return_command_value_2 =
            super::lifecycle::return_command(r, frame_timer_application_23)?;
        host::canvas_remove_command(r, canvas_value_14, return_command_value_2)?;
        let frame_timer_application_24 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_15 = super::lifecycle::canvas(r, frame_timer_application_24)?;
        let frame_timer_application_25 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let start_command_value_2 = super::lifecycle::start_command(r, frame_timer_application_25)?;
        host::canvas_remove_command(r, canvas_value_15, start_command_value_2)?;
    }
    let frame_timer_application_21 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_13 = super::lifecycle::canvas(r, frame_timer_application_21)?;
    let canvas_screen_state_3 = r.get(canvas_value_13, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_3 == 30 {
        let frame_timer_application_27 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_17 = super::lifecycle::canvas(r, frame_timer_application_27)?;
        let frame_timer_application_28 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let play_command_value = super::lifecycle::play_command(r, frame_timer_application_28)?;
        host::canvas_add_command(r, canvas_value_17, play_command_value)?;
    }
    let frame_timer_application_26 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_16 = super::lifecycle::canvas(r, frame_timer_application_26)?;
    let canvas_screen_state_4 = r.get(canvas_value_16, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_4 == 100 {
        let frame_timer_application_30 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_19 = super::lifecycle::canvas(r, frame_timer_application_30)?;
        let frame_timer_application_31 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let exit_command_value_2 = super::lifecycle::exit_command(r, frame_timer_application_31)?;
        host::canvas_remove_command(r, canvas_value_19, exit_command_value_2)?;
        let frame_timer_application_32 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_20 = super::lifecycle::canvas(r, frame_timer_application_32)?;
        let frame_timer_application_33 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let start_command_value_3 = super::lifecycle::start_command(r, frame_timer_application_33)?;
        host::canvas_remove_command(r, canvas_value_20, start_command_value_3)?;
        let frame_timer_application_34 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_21 = super::lifecycle::canvas(r, frame_timer_application_34)?;
        let frame_timer_application_35 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let menu_command_value_2 = super::lifecycle::menu_command(r, frame_timer_application_35)?;
        host::canvas_remove_command(r, canvas_value_21, menu_command_value_2)?;
        let frame_timer_application_36 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_22 = super::lifecycle::canvas(r, frame_timer_application_36)?;
        let frame_timer_application_37 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let save_exit_command_value_2 =
            super::lifecycle::save_exit_command(r, frame_timer_application_37)?;
        host::canvas_remove_command(r, canvas_value_22, save_exit_command_value_2)?;
        let frame_timer_application_38 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_23 = super::lifecycle::canvas(r, frame_timer_application_38)?;
        let frame_timer_application_39 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let return_command_value_3 =
            super::lifecycle::return_command(r, frame_timer_application_39)?;
        host::canvas_remove_command(r, canvas_value_23, return_command_value_3)?;
        let frame_timer_application_40 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_24 = super::lifecycle::canvas(r, frame_timer_application_40)?;
        let frame_timer_application_41 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let settings_command_value_2 =
            super::lifecycle::settings_command(r, frame_timer_application_41)?;
        host::canvas_remove_command(r, canvas_value_24, settings_command_value_2)?;
        let frame_timer_application_42 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_25 = super::lifecycle::canvas(r, frame_timer_application_42)?;
        let frame_timer_application_43 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let play_command_value_2 = super::lifecycle::play_command(r, frame_timer_application_43)?;
        host::canvas_remove_command(r, canvas_value_25, play_command_value_2)?;
        let frame_timer_application_44 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_26 = super::lifecycle::canvas(r, frame_timer_application_44)?;
        let frame_timer_application_45 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let default_command_value_2 =
            super::lifecycle::default_command(r, frame_timer_application_45)?;
        host::canvas_add_command(r, canvas_value_26, default_command_value_2)?;
    }
    let frame_timer_application_29 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_18 = super::lifecycle::canvas(r, frame_timer_application_29)?;
    let canvas_screen_state_5 = r.get(canvas_value_18, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_5 == 8 {
        let frame_timer_application_47 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_28 = super::lifecycle::canvas(r, frame_timer_application_47)?;
        let frame_timer_application_48 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let start_command_value_4 = super::lifecycle::start_command(r, frame_timer_application_48)?;
        host::canvas_remove_command(r, canvas_value_28, start_command_value_4)?;
        let frame_timer_application_49 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_29 = super::lifecycle::canvas(r, frame_timer_application_49)?;
        let frame_timer_application_50 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let return_command_value_4 =
            super::lifecycle::return_command(r, frame_timer_application_50)?;
        host::canvas_add_command(r, canvas_value_29, return_command_value_4)?;
    }
    let frame_timer_application_46 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_27 = super::lifecycle::canvas(r, frame_timer_application_46)?;
    let canvas_screen_state_6 = r.get(canvas_value_27, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_6 == 10 {
        let frame_timer_application_52 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_31 = super::lifecycle::canvas(r, frame_timer_application_52)?;
        let frame_timer_application_53 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let exit_command_value_3 = super::lifecycle::exit_command(r, frame_timer_application_53)?;
        host::canvas_remove_command(r, canvas_value_31, exit_command_value_3)?;
        let frame_timer_application_54 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_32 = super::lifecycle::canvas(r, frame_timer_application_54)?;
        let frame_timer_application_55 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let menu_command_value_3 = super::lifecycle::menu_command(r, frame_timer_application_55)?;
        host::canvas_remove_command(r, canvas_value_32, menu_command_value_3)?;
        let frame_timer_application_56 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_33 = super::lifecycle::canvas(r, frame_timer_application_56)?;
        let frame_timer_application_57 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let save_exit_command_value_3 =
            super::lifecycle::save_exit_command(r, frame_timer_application_57)?;
        host::canvas_remove_command(r, canvas_value_33, save_exit_command_value_3)?;
        let frame_timer_application_58 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_34 = super::lifecycle::canvas(r, frame_timer_application_58)?;
        let frame_timer_application_59 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let start_command_value_5 = super::lifecycle::start_command(r, frame_timer_application_59)?;
        host::canvas_remove_command(r, canvas_value_34, start_command_value_5)?;
        let frame_timer_application_60 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_35 = super::lifecycle::canvas(r, frame_timer_application_60)?;
        let frame_timer_application_61 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let settings_command_value_3 =
            super::lifecycle::settings_command(r, frame_timer_application_61)?;
        host::canvas_remove_command(r, canvas_value_35, settings_command_value_3)?;
    }
    let frame_timer_application_51 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_30 = super::lifecycle::canvas(r, frame_timer_application_51)?;
    let canvas_screen_state_7 = r.get(canvas_value_30, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_7 == 1 {
        let frame_timer_application_63 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_37 = super::lifecycle::canvas(r, frame_timer_application_63)?;
        let frame_timer_application_64 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let start_command_value_6 = super::lifecycle::start_command(r, frame_timer_application_64)?;
        host::canvas_remove_command(r, canvas_value_37, start_command_value_6)?;
        let frame_timer_application_65 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_38 = super::lifecycle::canvas(r, frame_timer_application_65)?;
        let frame_timer_application_66 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let menu_command_value_4 = super::lifecycle::menu_command(r, frame_timer_application_66)?;
        host::canvas_remove_command(r, canvas_value_38, menu_command_value_4)?;
        let frame_timer_application_67 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_39 = super::lifecycle::canvas(r, frame_timer_application_67)?;
        let frame_timer_application_68 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let save_exit_command_value_4 =
            super::lifecycle::save_exit_command(r, frame_timer_application_68)?;
        host::canvas_remove_command(r, canvas_value_39, save_exit_command_value_4)?;
        let frame_timer_application_69 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_40 = super::lifecycle::canvas(r, frame_timer_application_69)?;
        let frame_timer_application_70 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let return_command_value_5 =
            super::lifecycle::return_command(r, frame_timer_application_70)?;
        host::canvas_remove_command(r, canvas_value_40, return_command_value_5)?;
        let frame_timer_application_71 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_41 = super::lifecycle::canvas(r, frame_timer_application_71)?;
        let frame_timer_application_72 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let exit_command_value_4 = super::lifecycle::exit_command(r, frame_timer_application_72)?;
        host::canvas_add_command(r, canvas_value_41, exit_command_value_4)?;
        let frame_timer_application_73 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_42 = super::lifecycle::canvas(r, frame_timer_application_73)?;
        let frame_timer_application_74 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let settings_command_value_4 =
            super::lifecycle::settings_command(r, frame_timer_application_74)?;
        host::canvas_add_command(r, canvas_value_42, settings_command_value_4)?;
        let frame_timer_application_75 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_43 = super::lifecycle::canvas(r, frame_timer_application_75)?;
        let frame_timer_application_76 = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let play_command_value_3 = super::lifecycle::play_command(r, frame_timer_application_76)?;
        host::canvas_remove_command(r, canvas_value_43, play_command_value_3)?;
    }
    let frame_timer_application_62 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_36 = super::lifecycle::canvas(r, frame_timer_application_62)?;
    let canvas_screen_state_8 = r.get(canvas_value_36, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_8 != 1 {
        let frame_timer_application_77_read_read =
            r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_44 = super::lifecycle::canvas(r, frame_timer_application_77_read_read)?;
        r.set(canvas_value_44, field::canvas::REFRESH_COMMANDS, V::I(0))?;
        let frame_timer_application_3_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_3 = super::lifecycle::canvas(r, frame_timer_application_3_read)?;
        let canvas_exit_requested_read = r.get(canvas_value_3, field::canvas::EXIT_REQUESTED)?.i();
        if canvas_exit_requested_read == 0 {
            return Ok(());
        }
        let frame_timer_application_5_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
        super::lifecycle::destroy(r, frame_timer_application_5_read, 0)?;
        return Ok(());
    }
    let frame_timer_application_78 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_45 = super::lifecycle::canvas(r, frame_timer_application_78)?;
    let canvas_inventory_open = r.get(canvas_value_45, field::canvas::INVENTORY_OPEN)?.i();
    if canvas_inventory_open == 0 {
        let frame_timer_application_77_read_read =
            r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_44 = super::lifecycle::canvas(r, frame_timer_application_77_read_read)?;
        r.set(canvas_value_44, field::canvas::REFRESH_COMMANDS, V::I(0))?;
        let frame_timer_application_3_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
        let canvas_value_3 = super::lifecycle::canvas(r, frame_timer_application_3_read)?;
        let canvas_exit_requested_read = r.get(canvas_value_3, field::canvas::EXIT_REQUESTED)?.i();
        if canvas_exit_requested_read == 0 {
            return Ok(());
        }
        let frame_timer_application_5_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
        super::lifecycle::destroy(r, frame_timer_application_5_read, 0)?;
        return Ok(());
    }
    let frame_timer_application_79 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_46 = super::lifecycle::canvas(r, frame_timer_application_79)?;
    let frame_timer_application_80 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let exit_command_value_5 = super::lifecycle::exit_command(r, frame_timer_application_80)?;
    host::canvas_remove_command(r, canvas_value_46, exit_command_value_5)?;
    let frame_timer_application_81 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_47 = super::lifecycle::canvas(r, frame_timer_application_81)?;
    let frame_timer_application_82 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let start_command_value_7 = super::lifecycle::start_command(r, frame_timer_application_82)?;
    host::canvas_remove_command(r, canvas_value_47, start_command_value_7)?;
    let frame_timer_application_83 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_48 = super::lifecycle::canvas(r, frame_timer_application_83)?;
    let frame_timer_application_84 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let menu_command_value_5 = super::lifecycle::menu_command(r, frame_timer_application_84)?;
    host::canvas_remove_command(r, canvas_value_48, menu_command_value_5)?;
    let frame_timer_application_85 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_49 = super::lifecycle::canvas(r, frame_timer_application_85)?;
    let frame_timer_application_86 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let save_exit_command_value_5 =
        super::lifecycle::save_exit_command(r, frame_timer_application_86)?;
    host::canvas_remove_command(r, canvas_value_49, save_exit_command_value_5)?;
    let frame_timer_application_87 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_50 = super::lifecycle::canvas(r, frame_timer_application_87)?;
    let frame_timer_application_88 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let return_command_value_6 = super::lifecycle::return_command(r, frame_timer_application_88)?;
    host::canvas_remove_command(r, canvas_value_50, return_command_value_6)?;
    let frame_timer_application_89 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_51 = super::lifecycle::canvas(r, frame_timer_application_89)?;
    let frame_timer_application_90 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let settings_command_value_5 =
        super::lifecycle::settings_command(r, frame_timer_application_90)?;
    host::canvas_remove_command(r, canvas_value_51, settings_command_value_5)?;
    let frame_timer_application_91 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_52 = super::lifecycle::canvas(r, frame_timer_application_91)?;
    let frame_timer_application_92 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let menu_command_value_6 = super::lifecycle::menu_command(r, frame_timer_application_92)?;
    host::canvas_add_command(r, canvas_value_52, menu_command_value_6)?;
    let frame_timer_application_93 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_53 = super::lifecycle::canvas(r, frame_timer_application_93)?;
    let frame_timer_application_94 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let save_exit_command_value_6 =
        super::lifecycle::save_exit_command(r, frame_timer_application_94)?;
    host::canvas_add_command(r, canvas_value_53, save_exit_command_value_6)?;
    let frame_timer_application_95 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_54 = super::lifecycle::canvas(r, frame_timer_application_95)?;
    let frame_timer_application_96 = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let play_command_value_4 = super::lifecycle::play_command(r, frame_timer_application_96)?;
    host::canvas_remove_command(r, canvas_value_54, play_command_value_4)?;
    let frame_timer_application_77_read_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_44 = super::lifecycle::canvas(r, frame_timer_application_77_read_read)?;
    r.set(canvas_value_44, field::canvas::REFRESH_COMMANDS, V::I(0))?;
    let frame_timer_application_3_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
    let canvas_value_3 = super::lifecycle::canvas(r, frame_timer_application_3_read)?;
    let canvas_exit_requested_read = r.get(canvas_value_3, field::canvas::EXIT_REQUESTED)?.i();
    if canvas_exit_requested_read == 0 {
        return Ok(());
    }
    let frame_timer_application_5_read = r.get(timer, field::frame_timer::APPLICATION)?.r();
    super::lifecycle::destroy(r, frame_timer_application_5_read, 0)?;
    return Ok(());
}
