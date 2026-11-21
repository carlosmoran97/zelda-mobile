//! Typed platform-service boundary for game code.
use crate::runtime::{Runtime, V};

pub(super) fn draw_entity(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
    argument_5: i32,
) -> Result<(), u32> {
    r.invoke(
        1,
        &[
            V::R(object),
            V::R(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
            V::I(argument_5),
        ],
        true,
    )?;
    Ok(())
}

pub(super) fn object_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(2, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn math_abs(r: &mut Runtime, argument_1: i32) -> Result<i32, u32> {
    Ok(r.invoke(7, &[V::I(argument_1)], false)?.i())
}

pub(super) fn random_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(8, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn random_next_int(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(9, &[V::R(object)], false)?.i())
}

pub(super) fn graphics_clip_rect(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
) -> Result<(), u32> {
    r.invoke(
        10,
        &[
            V::R(object),
            V::I(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_draw_image(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
) -> Result<(), u32> {
    r.invoke(
        11,
        &[
            V::R(object),
            V::R(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_draw_region(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
    argument_5: i32,
    argument_6: i32,
    argument_7: i32,
    argument_8: i32,
    argument_9: i32,
) -> Result<(), u32> {
    r.invoke(
        12,
        &[
            V::R(object),
            V::R(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
            V::I(argument_5),
            V::I(argument_6),
            V::I(argument_7),
            V::I(argument_8),
            V::I(argument_9),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_get_translate_x(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(13, &[V::R(object)], false)?.i())
}

pub(super) fn graphics_get_translate_y(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(14, &[V::R(object)], false)?.i())
}

pub(super) fn graphics_set_clip(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
) -> Result<(), u32> {
    r.invoke(
        15,
        &[
            V::R(object),
            V::I(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_translate(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
) -> Result<(), u32> {
    r.invoke(
        16,
        &[V::R(object), V::I(argument_1), V::I(argument_2)],
        false,
    )?;
    Ok(())
}

pub(super) fn system_current_time_millis(r: &mut Runtime) -> Result<i64, u32> {
    Ok(r.invoke(21, &[], false)?.j())
}

pub(super) fn system_gc(r: &mut Runtime) -> Result<(), u32> {
    r.invoke(22, &[], false)?;
    Ok(())
}

pub(super) fn graphics_draw_string(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
) -> Result<(), u32> {
    r.invoke(
        23,
        &[
            V::R(object),
            V::R(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_set_color(r: &mut Runtime, object: u32, argument_1: i32) -> Result<(), u32> {
    r.invoke(24, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn image_create_image(r: &mut Runtime, argument_1: u32) -> Result<u32, u32> {
    Ok(r.invoke(25, &[V::R(argument_1)], false)?.r())
}

pub(super) fn data_input_stream_initialize(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(48, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn data_input_stream_read(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
    argument_3: i32,
) -> Result<i32, u32> {
    Ok(r.invoke(
        49,
        &[
            V::R(object),
            V::R(argument_1),
            V::I(argument_2),
            V::I(argument_3),
        ],
        false,
    )?
    .i())
}

pub(super) fn data_input_stream_read_byte(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(50, &[V::R(object)], false)?.i())
}

pub(super) fn data_input_stream_read_unsigned_byte(
    r: &mut Runtime,
    object: u32,
) -> Result<i32, u32> {
    Ok(r.invoke(51, &[V::R(object)], false)?.i())
}

pub(super) fn print_stream_println(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(52, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn class_get_resource_as_stream(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<u32, u32> {
    Ok(r.invoke(53, &[V::R(object), V::R(argument_1)], false)?.r())
}

pub(super) fn object_get_class(r: &mut Runtime, object: u32) -> Result<u32, u32> {
    Ok(r.invoke(54, &[V::R(object)], false)?.r())
}

pub(super) fn string_initialize(r: &mut Runtime, object: u32, argument_1: u32) -> Result<(), u32> {
    r.invoke(55, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn string_length(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(56, &[V::R(object)], false)?.i())
}

pub(super) fn string_substring(r: &mut Runtime, object: u32, argument_1: i32) -> Result<u32, u32> {
    Ok(r.invoke(57, &[V::R(object), V::I(argument_1)], false)?.r())
}

pub(super) fn string_substring_2(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
) -> Result<u32, u32> {
    Ok(r.invoke(
        58,
        &[V::R(object), V::I(argument_1), V::I(argument_2)],
        false,
    )?
    .r())
}

pub(super) fn string_buffer_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(59, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn string_buffer_append(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<u32, u32> {
    Ok(r.invoke(60, &[V::R(object), V::I(argument_1)], false)?.r())
}

pub(super) fn string_buffer_append_2(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<u32, u32> {
    Ok(r.invoke(61, &[V::R(object), V::R(argument_1)], false)?.r())
}

pub(super) fn string_buffer_append_3(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<u32, u32> {
    Ok(r.invoke(62, &[V::R(object), V::I(argument_1)], false)?.r())
}

pub(super) fn string_buffer_to_string(r: &mut Runtime, object: u32) -> Result<u32, u32> {
    Ok(r.invoke(63, &[V::R(object)], false)?.r())
}

pub(super) fn random_next_int_2(r: &mut Runtime, object: u32, argument_1: i32) -> Result<i32, u32> {
    Ok(r.invoke(64, &[V::R(object), V::I(argument_1)], false)?.i())
}

pub(super) fn random_set_seed(r: &mut Runtime, object: u32, argument_1: i64) -> Result<(), u32> {
    r.invoke(65, &[V::R(object), V::J(argument_1)], false)?;
    Ok(())
}

pub(super) fn vector_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(66, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn vector_add_element(r: &mut Runtime, object: u32, argument_1: u32) -> Result<(), u32> {
    r.invoke(67, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn vector_element_at(r: &mut Runtime, object: u32, argument_1: i32) -> Result<u32, u32> {
    Ok(r.invoke(68, &[V::R(object), V::I(argument_1)], false)?.r())
}

pub(super) fn vector_first_element(r: &mut Runtime, object: u32) -> Result<u32, u32> {
    Ok(r.invoke(69, &[V::R(object)], false)?.r())
}

pub(super) fn vector_remove_all_elements(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(70, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn vector_remove_element_at(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<(), u32> {
    r.invoke(71, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn vector_set_element_at(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
) -> Result<(), u32> {
    r.invoke(
        72,
        &[V::R(object), V::R(argument_1), V::I(argument_2)],
        false,
    )?;
    Ok(())
}

pub(super) fn vector_set_size(r: &mut Runtime, object: u32, argument_1: i32) -> Result<(), u32> {
    r.invoke(73, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn vector_size(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(74, &[V::R(object)], false)?.i())
}

pub(super) fn font_get_height(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(75, &[V::R(object)], false)?.i())
}

pub(super) fn font_substring_width(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
    argument_3: i32,
) -> Result<i32, u32> {
    Ok(r.invoke(
        76,
        &[
            V::R(object),
            V::R(argument_1),
            V::I(argument_2),
            V::I(argument_3),
        ],
        false,
    )?
    .i())
}

pub(super) fn graphics_draw_rect(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
) -> Result<(), u32> {
    r.invoke(
        77,
        &[
            V::R(object),
            V::I(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_fill_rect(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
) -> Result<(), u32> {
    r.invoke(
        78,
        &[
            V::R(object),
            V::I(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_get_font(r: &mut Runtime, object: u32) -> Result<u32, u32> {
    Ok(r.invoke(79, &[V::R(object)], false)?.r())
}

pub(super) fn exception_to_string(r: &mut Runtime, object: u32) -> Result<u32, u32> {
    Ok(r.invoke(102, &[V::R(object)], false)?.r())
}

pub(super) fn thread_initialize(r: &mut Runtime, object: u32, argument_1: u32) -> Result<(), u32> {
    r.invoke(103, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn thread_set_priority(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<(), u32> {
    r.invoke(104, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn thread_sleep(r: &mut Runtime, argument_1: i64) -> Result<(), u32> {
    r.invoke(105, &[V::J(argument_1)], false)?;
    Ok(())
}

pub(super) fn thread_start(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(106, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn manager_create_player(
    r: &mut Runtime,
    argument_1: u32,
    argument_2: u32,
) -> Result<u32, u32> {
    Ok(r.invoke(107, &[V::R(argument_1), V::R(argument_2)], false)?
        .r())
}

pub(super) fn player_add_player_listener(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(108, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn player_close(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(109, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn player_get_control(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<u32, u32> {
    Ok(r.invoke(110, &[V::R(object), V::R(argument_1)], false)?.r())
}

pub(super) fn player_get_state(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(111, &[V::R(object)], false)?.i())
}

pub(super) fn player_prefetch(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(112, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn player_realize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(113, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn player_remove_player_listener(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(114, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn player_set_loop_count(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<(), u32> {
    r.invoke(115, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn player_set_media_time(
    r: &mut Runtime,
    object: u32,
    argument_1: i64,
) -> Result<i64, u32> {
    Ok(r.invoke(116, &[V::R(object), V::J(argument_1)], false)?.j())
}

pub(super) fn player_start(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(117, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn player_stop(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(118, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn volume_control_set_level(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<i32, u32> {
    Ok(r.invoke(119, &[V::R(object), V::I(argument_1)], false)?.i())
}

pub(super) fn graphics_draw_line(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
) -> Result<(), u32> {
    r.invoke(
        122,
        &[
            V::R(object),
            V::I(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_set_color_2(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
) -> Result<(), u32> {
    r.invoke(
        123,
        &[
            V::R(object),
            V::I(argument_1),
            V::I(argument_2),
            V::I(argument_3),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn byte_array_input_stream_initialize(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(128, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn byte_array_input_stream_close(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(129, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn byte_array_input_stream_reset(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(130, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn byte_array_output_stream_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(131, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn byte_array_output_stream_close(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(132, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn byte_array_output_stream_reset(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(133, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn byte_array_output_stream_to_byte_array(
    r: &mut Runtime,
    object: u32,
) -> Result<u32, u32> {
    Ok(r.invoke(134, &[V::R(object)], false)?.r())
}

pub(super) fn data_input_stream_close(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(135, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn data_input_stream_read_2(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(136, &[V::R(object)], false)?.i())
}

pub(super) fn data_input_stream_read_boolean(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(137, &[V::R(object)], false)?.i())
}

pub(super) fn data_input_stream_read_int(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(138, &[V::R(object)], false)?.i())
}

pub(super) fn data_output_stream_initialize(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(139, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn data_output_stream_close(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(140, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn data_output_stream_flush(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(141, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn data_output_stream_write(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<(), u32> {
    r.invoke(142, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn data_output_stream_write_boolean(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<(), u32> {
    r.invoke(143, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn data_output_stream_write_int(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<(), u32> {
    r.invoke(144, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn runtime_get_runtime(r: &mut Runtime) -> Result<u32, u32> {
    Ok(r.invoke(146, &[], false)?.r())
}

pub(super) fn runtime_total_memory(r: &mut Runtime, object: u32) -> Result<i64, u32> {
    Ok(r.invoke(147, &[V::R(object)], false)?.j())
}

pub(super) fn timer_task_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(160, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn canvas_add_command(r: &mut Runtime, object: u32, argument_1: u32) -> Result<(), u32> {
    r.invoke(161, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn canvas_remove_command(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(162, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn canvas_repaint(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(163, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn m_i_dlet_notify_destroyed(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(170, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn timer_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(171, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn timer_schedule_at_fixed_rate(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i64,
    argument_3: i64,
) -> Result<(), u32> {
    r.invoke(
        172,
        &[
            V::R(object),
            V::R(argument_1),
            V::J(argument_2),
            V::J(argument_3),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn command_initialize(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
    argument_3: i32,
) -> Result<(), u32> {
    r.invoke(
        173,
        &[
            V::R(object),
            V::R(argument_1),
            V::I(argument_2),
            V::I(argument_3),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn display_get_display(r: &mut Runtime, argument_1: u32) -> Result<u32, u32> {
    Ok(r.invoke(174, &[V::R(argument_1)], false)?.r())
}

pub(super) fn display_set_current(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(175, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn m_i_dlet_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(176, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn canvas_set_command_listener(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(184, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn integer_to_string(r: &mut Runtime, argument_1: i32) -> Result<u32, u32> {
    Ok(r.invoke(203, &[V::I(argument_1)], false)?.r())
}

pub(super) fn vector_insert_element_at(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
) -> Result<(), u32> {
    r.invoke(
        204,
        &[V::R(object), V::R(argument_1), V::I(argument_2)],
        false,
    )?;
    Ok(())
}

pub(super) fn canvas_initialize(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(205, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn font_get_font(
    r: &mut Runtime,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
) -> Result<u32, u32> {
    Ok(r.invoke(
        206,
        &[V::I(argument_1), V::I(argument_2), V::I(argument_3)],
        false,
    )?
    .r())
}

pub(super) fn font_string_width(r: &mut Runtime, object: u32, argument_1: u32) -> Result<i32, u32> {
    Ok(r.invoke(207, &[V::R(object), V::R(argument_1)], false)?.i())
}

pub(super) fn graphics_fill_round_rect(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
    argument_5: i32,
    argument_6: i32,
) -> Result<(), u32> {
    r.invoke(
        208,
        &[
            V::R(object),
            V::I(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
            V::I(argument_5),
            V::I(argument_6),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_fill_triangle(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: i32,
    argument_3: i32,
    argument_4: i32,
    argument_5: i32,
    argument_6: i32,
) -> Result<(), u32> {
    r.invoke(
        209,
        &[
            V::R(object),
            V::I(argument_1),
            V::I(argument_2),
            V::I(argument_3),
            V::I(argument_4),
            V::I(argument_5),
            V::I(argument_6),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn graphics_get_clip_height(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(210, &[V::R(object)], false)?.i())
}

pub(super) fn graphics_get_clip_width(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(211, &[V::R(object)], false)?.i())
}

pub(super) fn graphics_get_clip_x(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(212, &[V::R(object)], false)?.i())
}

pub(super) fn graphics_get_clip_y(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(213, &[V::R(object)], false)?.i())
}

pub(super) fn graphics_set_font(r: &mut Runtime, object: u32, argument_1: u32) -> Result<(), u32> {
    r.invoke(214, &[V::R(object), V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn image_create_image_2(
    r: &mut Runtime,
    argument_1: i32,
    argument_2: i32,
) -> Result<u32, u32> {
    Ok(r.invoke(215, &[V::I(argument_1), V::I(argument_2)], false)?
        .r())
}

pub(super) fn image_get_graphics(r: &mut Runtime, object: u32) -> Result<u32, u32> {
    Ok(r.invoke(216, &[V::R(object)], false)?.r())
}

pub(super) fn image_get_width(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(217, &[V::R(object)], false)?.i())
}

pub(super) fn record_store_add_record(
    r: &mut Runtime,
    object: u32,
    argument_1: u32,
    argument_2: i32,
    argument_3: i32,
) -> Result<i32, u32> {
    Ok(r.invoke(
        218,
        &[
            V::R(object),
            V::R(argument_1),
            V::I(argument_2),
            V::I(argument_3),
        ],
        false,
    )?
    .i())
}

pub(super) fn record_store_close_record_store(r: &mut Runtime, object: u32) -> Result<(), u32> {
    r.invoke(219, &[V::R(object)], false)?;
    Ok(())
}

pub(super) fn record_store_delete_record_store(
    r: &mut Runtime,
    argument_1: u32,
) -> Result<(), u32> {
    r.invoke(220, &[V::R(argument_1)], false)?;
    Ok(())
}

pub(super) fn record_store_get_num_records(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(221, &[V::R(object)], false)?.i())
}

pub(super) fn record_store_get_record(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<u32, u32> {
    Ok(r.invoke(222, &[V::R(object), V::I(argument_1)], false)?.r())
}

pub(super) fn record_store_list_record_stores(r: &mut Runtime) -> Result<u32, u32> {
    Ok(r.invoke(223, &[], false)?.r())
}

pub(super) fn record_store_open_record_store(
    r: &mut Runtime,
    argument_1: u32,
    argument_2: i32,
) -> Result<u32, u32> {
    Ok(r.invoke(224, &[V::R(argument_1), V::I(argument_2)], false)?
        .r())
}

pub(super) fn record_store_set_record(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
    argument_2: u32,
    argument_3: i32,
    argument_4: i32,
) -> Result<(), u32> {
    r.invoke(
        225,
        &[
            V::R(object),
            V::I(argument_1),
            V::R(argument_2),
            V::I(argument_3),
            V::I(argument_4),
        ],
        false,
    )?;
    Ok(())
}

pub(super) fn record_store_exception_to_string(r: &mut Runtime, object: u32) -> Result<u32, u32> {
    Ok(r.invoke(226, &[V::R(object)], false)?.r())
}

pub(super) fn canvas_get_game_action(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<i32, u32> {
    Ok(r.invoke(227, &[V::R(object), V::I(argument_1)], false)?.i())
}

pub(super) fn canvas_get_height(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(228, &[V::R(object)], false)?.i())
}

pub(super) fn canvas_get_width(r: &mut Runtime, object: u32) -> Result<i32, u32> {
    Ok(r.invoke(229, &[V::R(object)], false)?.i())
}

pub(super) fn canvas_set_full_screen_mode(
    r: &mut Runtime,
    object: u32,
    argument_1: i32,
) -> Result<(), u32> {
    r.invoke(230, &[V::R(object), V::I(argument_1)], false)?;
    Ok(())
}

pub(super) fn check_cast(r: &mut Runtime, object: u32, class: &str) -> Result<(), u32> {
    if object != 0 && !r.is_instance(object, class) {
        return Err(r.exception("java/lang/ClassCastException", "invalid cast"));
    }
    Ok(())
}
