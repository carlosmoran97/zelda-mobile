//! Canvas rendering.
use super::*;

/// Paint. Original reference: `k.paint(Ljavax/microedition/lcdui/Graphics;)V`.
#[rustfmt::skip]
pub(in crate::game) fn paint(
    r: &mut Runtime,
    canvas: u32,
    graphics: u32,
) -> Result<(), u32> {
    let mut number_2: i32 = 0;
    let mut object_2: u32 = 0;
    let mut number_3: i32 = 0;
    let mut object_3: u32 = 0;
    let mut number_4: i32 = 0;
    let mut object_4: u32 = 0;
    let mut number_5: i32 = 0;
    let mut object_5: u32 = 0;
    let mut number_6: i32 = 0;
    let mut object_6: u32 = 0;
    let mut number_7: i32 = 0;
    let mut number_8: i32 = 0;
    let mut object_8: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut carry_1_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    'check_canvas_world_22: {
    'check_canvas_scene_image_8: {
    'check_canvas_state_at_upper: {
    'check_canvas_state_f_3: {
    'check_canvas_state_f: {
    'check_canvas_state_f_4: {
    'check_canvas_random: {
    'check_canvas_state_f_2: {
    'check_canvas_state_i: {
    'next_step_11: {
    'check_canvas_player_2: {
    'check_canvas_player_3: {
    'check_portal_destination_zone: {
    'next_step_9: {
    'check_canvas_world_19: {
    'check_canvas_world_20: {
    'check_canvas_world_18: {
    'check_player_moving: {
    'next_step_10: {
    'check_player_digging: {
    'check_player_hookshot_extending: {
    'next_step_8: {
    'check_canvas_world_21: {
    'check_canvas_show_item_message: {
    'graphics_get_clip_x: {
    'check_canvas_redraw_world: {
    'graphics_get_translate_x_7: {
    'graphics_set_color_3: {
    'graphics_get_translate_x_8: {
    'check_canvas_zone_4: {
    'check_canvas_player_4: {
    'check_canvas_zone_3: {
    'check_canvas_zone_2: {
    'check_canvas_map: {
    'check_canvas_zone: {
    'next_step_7: {
    'next_step_6: {
    'check_canvas_inventory_page_3: {
    'next_step_5: {
    'check_canvas_inventory_page_2: {
    'next_step_4: {
    'check_canvas_inventory_page: {
    'check_canvas_screen_state_9: {
    'graphics_clip_rect_2: {
    'graphics_translate_6: {
    'graphics_translate_5: {
    'check_canvas_menu_selection_4: {
    'check_canvas_menu_selection_3: {
    'graphics_translate_4: {
    'graphics_clip_rect: {
    'graphics_translate_3: {
    'graphics_translate_2: {
    'check_canvas_menu_selection_2: {
    'check_canvas_menu_selection: {
    'graphics_translate: {
    'next_step_3: {
    'check_canvas_screen_state_8: {
    'check_canvas_state_v_upper: {
    'check_canvas_world_16: {
    'check_canvas_scene_image_7: {
    'check_canvas_state_u_8: {
    'check_canvas_screen_state_7: {
    'graphics_get_translate_x_6: {
    'check_canvas_world_11: {
    'check_canvas_scene_image_6: {
    'check_canvas_state_u_7: {
    'check_canvas_screen_state_6: {
    'graphics_get_translate_x_5: {
    'check_canvas_world_8: {
    'check_canvas_scene_image_5: {
    'check_canvas_state_u_6: {
    'check_canvas_screen_state_5: {
    'graphics_get_translate_x_3: {
    'check_canvas_world_5: {
    'check_canvas_scene_image_3: {
    'check_canvas_state_u_4: {
    'check_canvas_screen_state_4: {
    'graphics_get_translate_x_2: {
    'check_canvas_world_3: {
    'check_canvas_scene_image_2: {
    'check_canvas_state_u_2: {
    'check_canvas_screen_state_3: {
    'graphics_get_translate_x: {
    'check_canvas_world_2: {
    'check_canvas_scene_image: {
    'check_canvas_state_u: {
    'check_canvas_screen_state_2: {
    'check_canvas_state_l_upper_2: {
    'graphics_get_translate_x_4: {
    'check_canvas_world_13: {
    'next_step_2: {
    'check_canvas_world_7: {
    'check_canvas_scene_image_4: {
    'check_canvas_state_u_5: {
    'graphics_set_color_2: {
    'check_canvas_configure_phone_keys: {
    'next_step: {
    'check_canvas_player: {
    'check_canvas_state_u_3: {
    'check_canvas_world: {
    'check_canvas_graphics: {
    'check_canvas_screen_state: {
    'check_canvas_second_elapsed: {
    'graphics_set_color: {
    'check_canvas_state_l_upper: {
    'canvas_get_width: {
    let canvas_state_ay = r.get(canvas, field::canvas::STATE_AY)?.i();
    if canvas_state_ay == 0 {
let sound_value = r.alloc("b", Native::Empty);
let canvas_music_volume = r.get(canvas, field::canvas::MUSIC_VOLUME)?.i();
let canvas_effect_volume = r.get(canvas, field::canvas::EFFECT_VOLUME)?.i();
super::sound::initialize(r, sound_value, 0, 1, -2, 0, canvas_music_volume, canvas_effect_volume)?;
r.set(canvas, field::canvas::SOUND, V::R(sound_value))?;
r.set(canvas, field::canvas::STATE_AY, V::I(1))?;
}
    let font_get_font_value = host::font_get_font(r, 0, 0, 8)?;
    object_2 = font_get_font_value;
    host::graphics_set_font(r, graphics, object_2)?;
    let canvas_redraw = r.get(canvas, field::canvas::REDRAW)?.i();
    if canvas_redraw != 0 {
host::graphics_set_color(r, graphics, 0)?;
let canvas_get_width_value = host::canvas_get_width(r, canvas)?;
let canvas_get_height_value = host::canvas_get_height(r, canvas)?;
host::graphics_fill_rect(r, graphics, 0, 0, canvas_get_width_value, canvas_get_height_value)?;
r.set(canvas, field::canvas::REDRAW, V::I(0))?;
}
    object_2 = canvas;
    let canvas_state_ar = r.get(canvas, field::canvas::STATE_AR)?.i();
    if canvas_state_ar != 0 {
let canvas_state_au = r.get(object_2, field::canvas::STATE_AU)?.j();
let system_current_time_millis_value = host::system_current_time_millis(r)?;
if !((if canvas_state_au > system_current_time_millis_value.wrapping_sub(150) { 1 } else if canvas_state_au == system_current_time_millis_value.wrapping_sub(150) { 0 } else if canvas_state_au < system_current_time_millis_value.wrapping_sub(150) { -1 } else { -1 }) > 0) {
let canvas_state_as = r.get(object_2, field::canvas::STATE_AS)?.i();
let canvas_player = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x = r.get(canvas_player, field::drawable::X)?.i();
let canvas_state_p = r.get(object_2, field::canvas::STATE_P)?.i();
number_3 = canvas_state_as.wrapping_sub(drawable_x.wrapping_sub(canvas_state_p));
let canvas_state_at = r.get(object_2, field::canvas::STATE_AT)?.i();
let canvas_player_2 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y = r.get(canvas_player_2, field::drawable::Y)?.i();
let canvas_state_q = r.get(object_2, field::canvas::STATE_Q)?.i();
number_4 = canvas_state_at.wrapping_sub(drawable_y.wrapping_sub(canvas_state_q));
let math_abs_value = host::math_abs(r, number_3)?;
let math_abs_value_2 = host::math_abs(r, number_4)?;
if math_abs_value <= math_abs_value_2 {
let math_abs_value_3 = host::math_abs(r, number_4)?;
if math_abs_value_3 <= 4 {
let canvas_up_key = r.get(object_2, field::canvas::UP_KEY)?.i();
super::canvas::release_control(r, object_2, canvas_up_key)?;
let canvas_down_key = r.get(object_2, field::canvas::DOWN_KEY)?.i();
super::canvas::release_control(r, object_2, canvas_down_key)?;
} else {

if number_4 >= 0 {
let canvas_down_key_2 = r.get(object_2, field::canvas::DOWN_KEY)?.i();
super::canvas::press_control(r, object_2, canvas_down_key_2)?;
} else {
let canvas_up_key_2 = r.get(object_2, field::canvas::UP_KEY)?.i();
super::canvas::press_control(r, object_2, canvas_up_key_2)?;
}
}
let math_abs_value_5 = host::math_abs(r, number_3)?;
if math_abs_value_5 <= 4 {
let canvas_left_key_3 = r.get(object_2, field::canvas::LEFT_KEY)?.i();
super::canvas::release_control(r, object_2, canvas_left_key_3)?;
let canvas_right_key_3 = r.get(object_2, field::canvas::RIGHT_KEY)?.i();
super::canvas::release_control(r, object_2, canvas_right_key_3)?;
} else {

if number_3 >= 0 {
let canvas_right_key_4 = r.get(object_2, field::canvas::RIGHT_KEY)?.i();
super::canvas::press_control(r, object_2, canvas_right_key_4)?;
} else {
let canvas_left_key_4 = r.get(object_2, field::canvas::LEFT_KEY)?.i();
super::canvas::press_control(r, object_2, canvas_left_key_4)?;
}
}
} else {
let math_abs_value_4 = host::math_abs(r, number_3)?;
if math_abs_value_4 <= 4 {
let canvas_left_key = r.get(object_2, field::canvas::LEFT_KEY)?.i();
super::canvas::release_control(r, object_2, canvas_left_key)?;
let canvas_right_key = r.get(object_2, field::canvas::RIGHT_KEY)?.i();
super::canvas::release_control(r, object_2, canvas_right_key)?;
} else {

if number_3 >= 0 {
let canvas_right_key_2 = r.get(object_2, field::canvas::RIGHT_KEY)?.i();
super::canvas::press_control(r, object_2, canvas_right_key_2)?;
} else {
let canvas_left_key_2 = r.get(object_2, field::canvas::LEFT_KEY)?.i();
super::canvas::press_control(r, object_2, canvas_left_key_2)?;
}
}
let math_abs_value_6 = host::math_abs(r, number_4)?;
if math_abs_value_6 <= 4 {
let canvas_up_key_3 = r.get(object_2, field::canvas::UP_KEY)?.i();
super::canvas::release_control(r, object_2, canvas_up_key_3)?;
let canvas_down_key_3 = r.get(object_2, field::canvas::DOWN_KEY)?.i();
super::canvas::release_control(r, object_2, canvas_down_key_3)?;
} else {

if number_4 >= 0 {
let canvas_down_key_4 = r.get(object_2, field::canvas::DOWN_KEY)?.i();
super::canvas::press_control(r, object_2, canvas_down_key_4)?;
} else {
let canvas_up_key_5 = r.get(object_2, field::canvas::UP_KEY)?.i();
super::canvas::press_control(r, object_2, canvas_up_key_5)?;
}
}
}
}
}
    let canvas_second_elapsed = r.get(canvas, field::canvas::SECOND_ELAPSED)?.i();
    if canvas_second_elapsed == 0 { break 'check_canvas_screen_state; }
    } // canvas_get_width
    let canvas_get_width_value_2 = host::canvas_get_width(r, canvas)?;
    r.set(canvas, field::canvas::STATE_L_UPPER, V::I(canvas_get_width_value_2))?;
    let canvas_get_height_value_2 = host::canvas_get_height(r, canvas)?;
    r.set(canvas, field::canvas::STATE_M, V::I(canvas_get_height_value_2))?;
    let canvas_state_m = r.get(canvas, field::canvas::STATE_M)?.i();
    let canvas_state_o = r.get(canvas, field::canvas::STATE_O)?.i();
    if canvas_state_m != canvas_state_o { break 'graphics_set_color; }
    } // check_canvas_state_l_upper
    let canvas_state_l_upper_3 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_n = r.get(canvas, field::canvas::STATE_N)?.i();
    if canvas_state_l_upper_3 == canvas_state_n { break 'check_canvas_second_elapsed; }
    } // graphics_set_color
    host::graphics_set_color(r, graphics, 0)?;
    let canvas_get_width_value_3 = host::canvas_get_width(r, canvas)?;
    let canvas_get_height_value_3 = host::canvas_get_height(r, canvas)?;
    host::graphics_fill_rect(r, graphics, 0, 0, canvas_get_width_value_3, canvas_get_height_value_3)?;
    let canvas_state_m_2 = r.get(canvas, field::canvas::STATE_M)?.i();
    r.set(canvas, field::canvas::STATE_O, V::I(canvas_state_m_2))?;
    let canvas_state_l_upper = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    r.set(canvas, field::canvas::STATE_N, V::I(canvas_state_l_upper))?;
    r.set(canvas, field::canvas::STATE_AN, V::I(32))?;
    let canvas_state_l_upper_2 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    r.set(canvas, field::canvas::STATE_AL, V::I((256_i32).wrapping_sub(canvas_state_l_upper_2)))?;
    let canvas_state_m_3 = r.get(canvas, field::canvas::STATE_M)?.i();
    let canvas_state_an = r.get(canvas, field::canvas::STATE_AN)?.i();
    r.set(canvas, field::canvas::STATE_AM, V::I((176_i32).wrapping_sub(canvas_state_m_3.wrapping_sub(canvas_state_an))))?;
    } // check_canvas_second_elapsed
    r.set(canvas, field::canvas::SECOND_ELAPSED, V::I(0))?;
    r.set(canvas, field::canvas::REDRAW, V::I(0))?;
    } // check_canvas_screen_state
    let canvas_screen_state = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state != 0 { break 'check_canvas_screen_state_2; }
    } // check_canvas_graphics
    r.set(canvas, field::canvas::GRAPHICS, V::R(graphics))?;
    let canvas_state_v = r.get(canvas, field::canvas::STATE_V)?.i();
    if canvas_state_v == 0 { break 'check_canvas_configure_phone_keys; }
    } // check_canvas_world
    let canvas_world = r.get(canvas, field::canvas::WORLD)?.r();
    if canvas_world != 0 { break 'next_step; }
    } // check_canvas_state_u_3
    let canvas_state_u_3 = r.get(canvas, field::canvas::STATE_U)?.i();
    if canvas_state_u_3 != 0 { break 'next_step; }
    } // check_canvas_player
    r.set(canvas, field::canvas::PLAYER, V::R(0))?;
    r.set(canvas, field::canvas::STATE_U, V::I(1))?;
    let world_value_3 = r.alloc("j", Native::Empty);
    let canvas_scene_image_3 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let image_get_graphics_value = host::image_get_graphics(r, canvas_scene_image_3)?;
    let canvas_graphics = r.get(canvas, field::canvas::GRAPHICS)?.r();
    let canvas_player_3 = r.get(canvas, field::canvas::PLAYER)?.r();
    super::world::initialize(r, world_value_3, image_get_graphics_value, canvas_graphics, canvas_player_3, canvas)?;
    r.set(canvas, field::canvas::WORLD, V::R(world_value_3))?;
    break 'check_canvas_configure_phone_keys;
    } // next_step
    let text_value_3 = r.intern("Loading..");
    host::graphics_draw_string(r, graphics, text_value_3, 0, 0, 20)?;
    } // check_canvas_configure_phone_keys
    let canvas_configure_phone_keys = r.get(canvas, field::canvas::CONFIGURE_PHONE_KEYS)?.i();
    if canvas_configure_phone_keys != 0 {
r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(1))?;
}
    let canvas_state_v_2 = r.get(canvas, field::canvas::STATE_V)?.i();
    if canvas_state_v_2 != 0 { break 'check_canvas_state_u_5; }
    } // graphics_set_color_2
    host::graphics_set_color(r, graphics, 0)?;
    let canvas_get_width_value_4 = host::canvas_get_width(r, canvas)?;
    let canvas_get_height_value_4 = host::canvas_get_height(r, canvas)?;
    host::graphics_fill_rect(r, graphics, 0, 0, canvas_get_width_value_4, canvas_get_height_value_4)?;
    let canvas_get_width_value_5 = host::canvas_get_width(r, canvas)?;
    let quotient_value = arithmetic::divide_int(r, canvas_get_width_value_5, 2)?;
    host::graphics_translate(r, graphics, quotient_value.wrapping_sub(64), 0)?;
    break 'check_canvas_state_l_upper_2;
    } // check_canvas_state_u_5
    let canvas_state_u_5 = r.get(canvas, field::canvas::STATE_U)?.i();
    if canvas_state_u_5 == 0 { break 'graphics_get_translate_x_4; }
    } // check_canvas_scene_image_4
    let canvas_scene_image_5 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let canvas_state_ar_upper_7 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    host::graphics_draw_image(r, graphics, canvas_scene_image_5, canvas_state_ar_upper_7.wrapping_neg(), 0, 0)?;
    let graphics_get_translate_x_value_11 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_11 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_11.wrapping_neg(), graphics_get_translate_y_value_11.wrapping_neg())?;
    number_2 = 0;
    } // check_canvas_world_7
    'paint_loop_16: loop {
        'check_canvas_world_10: {
        let canvas_world_8 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_6 = r.get(canvas_world_8, field::world::OBJECTS)?.r();
        let vector_size_value_4 = host::vector_size(r, world_objects_6)?;
        if number_2 >= vector_size_value_4 { break 'next_step_2; }
        } // check_canvas_world_10
        let canvas_world_11 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_9 = r.get(canvas_world_11, field::world::OBJECTS)?.r();
        let vector_element_at_value_4 = host::vector_element_at(r, world_objects_9, number_2)?;
        host::check_cast(r, vector_element_at_value_4, "n")?;
        let canvas_state_ar_upper_13 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        let canvas_state_l_upper_28 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let canvas_state_m_25 = r.get(canvas, field::canvas::STATE_M)?.i();
        super::map_object::draw(r, vector_element_at_value_4, graphics, canvas_state_ar_upper_13, 32, canvas_state_l_upper_28, canvas_state_m_25)?;
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_16;
    }
    } // next_step_2
    number_2 = 0;
    } // check_canvas_world_13
    'paint_loop_19: loop {
        'check_canvas_world_15: {
        let canvas_world_14 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_items = r.get(canvas_world_14, field::world::ITEMS)?.r();
        let vector_size_value_7 = host::vector_size(r, world_items)?;
        if number_2 >= vector_size_value_7 { break 'graphics_get_translate_x_4; }
        } // check_canvas_world_15
        let canvas_world_16 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_items_2 = r.get(canvas_world_16, field::world::ITEMS)?.r();
        let vector_element_at_value_7 = host::vector_element_at(r, world_items_2, number_2)?;
        host::check_cast(r, vector_element_at_value_7, "c")?;
        let canvas_state_ar_upper_19 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        let canvas_state_l_upper_34 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let canvas_state_m_31 = r.get(canvas, field::canvas::STATE_M)?.i();
        super::item::draw(r, vector_element_at_value_7, graphics, canvas_state_ar_upper_19, 32, canvas_state_l_upper_34, canvas_state_m_31)?;
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_19;
    }
    } // graphics_get_translate_x_4
    let graphics_get_translate_x_value_10 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_10 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_10.wrapping_neg(), graphics_get_translate_y_value_10.wrapping_neg())?;
    let canvas_state_l_upper_14 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_14 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_14, canvas_state_m_14)?;
    let canvas_state_l_upper_15 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_15 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_15, canvas_state_m_15)?;
    super::canvas::draw_inventory(r, canvas, graphics, 1)?;
    let canvas_state_ar_upper_5 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    let canvas_state_as_upper = r.get(canvas, field::canvas::STATE_AS_UPPER)?.i();
    r.set(canvas, field::canvas::STATE_AR_UPPER, V::I(canvas_state_ar_upper_5.wrapping_add(canvas_state_as_upper)))?;
    let canvas_state_ar_upper_6 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    let canvas_state_l_upper_16 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    if !(canvas_state_ar_upper_6.wrapping_add(canvas_state_l_upper_16) <= 256) {
r.set(canvas, field::canvas::STATE_AS_UPPER, V::I(-1))?;
}
    let canvas_state_ar_upper_10 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    if canvas_state_ar_upper_10 < 0 {
r.set(canvas, field::canvas::STATE_AS_UPPER, V::I(1))?;
}
    let canvas_state_l_upper_27 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    if canvas_state_l_upper_27 >= 250 {
r.set(canvas, field::canvas::STATE_AR_UPPER, V::I(0))?;
}
    } // check_canvas_state_l_upper_2
    let canvas_state_l_upper_17 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let quotient_value_2 = arithmetic::divide_int(r, canvas_state_l_upper_17, 2)?;
    let canvas_state_v_upper = r.get(canvas, field::canvas::STATE_V_UPPER)?.r();
    let image_get_width_value = host::image_get_width(r, canvas_state_v_upper)?;
    let quotient_value_3 = arithmetic::divide_int(r, image_get_width_value, 2)?;
    number_2 = quotient_value_2.wrapping_sub(quotient_value_3);
    let canvas_state_v_upper_2 = r.get(canvas, field::canvas::STATE_V_UPPER)?.r();
    host::graphics_draw_image(r, graphics, canvas_state_v_upper_2, number_2, 2, 0)?;
    let graphics_get_translate_x_value_12 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_12 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_12.wrapping_neg(), graphics_get_translate_y_value_12.wrapping_neg())?;
    host::graphics_translate(r, graphics, number_2.wrapping_add(10), 60)?;
    host::graphics_set_color(r, graphics, 16777215)?;
    let canvas_key_configuration_step = r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
    if canvas_key_configuration_step == 0 {
host::graphics_set_color(r, graphics, 16711680)?;
}
    host::graphics_translate(r, graphics, 0, 10)?;
    let text_value_13 = r.intern("Start..");
    host::graphics_draw_string(r, graphics, text_value_13, 0, 0, 20)?;
    host::graphics_set_color(r, graphics, 16777215)?;
    let canvas_key_configuration_step_2 = r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
    if canvas_key_configuration_step_2 == 1 {
host::graphics_set_color(r, graphics, 16711680)?;
}
    host::graphics_translate(r, graphics, 0, 10)?;
    let text_value_55 = r.intern("Config");
    host::graphics_draw_string(r, graphics, text_value_55, 0, 0, 20)?;
    host::graphics_set_color(r, graphics, 16777215)?;
    host::graphics_set_color(r, graphics, 16777215)?;
    let canvas_key_configuration_step_3 = r.get(canvas, field::canvas::KEY_CONFIGURATION_STEP)?.i();
    if canvas_key_configuration_step_3 == 2 {
host::graphics_set_color(r, graphics, 16711680)?;
}
    host::graphics_translate(r, graphics, 0, 10)?;
    let text_value_60 = r.intern("Credits");
    host::graphics_draw_string(r, graphics, text_value_60, 0, 0, 20)?;
    host::graphics_set_color(r, graphics, 16777215)?;
    let graphics_get_translate_x_value_23 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_21 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_23.wrapping_neg(), graphics_get_translate_y_value_21.wrapping_neg())?;
    let canvas_get_height_value_5 = host::canvas_get_height(r, canvas)?;
    host::graphics_translate(r, graphics, 10, canvas_get_height_value_5.wrapping_sub(35))?;
    host::graphics_set_color(r, graphics, 13421772)?;
    let text_value_61 = r.intern("This game is FREE!");
    host::graphics_draw_string(r, graphics, text_value_61, 0, 0, 20)?;
    host::graphics_translate(r, graphics, 0, 10)?;
    let text_value_62 = r.intern("Email me: gorthwogh@gmail.com");
    host::graphics_draw_string(r, graphics, text_value_62, 0, 0, 20)?;
    host::graphics_translate(r, graphics, 0, 10)?;
    let text_value_63 = r.intern("(c) - Gorthwogh - 20010 (Beta 4)");
    host::graphics_draw_string(r, graphics, text_value_63, 0, 0, 20)?;
    host::system_gc(r)?;
    return Ok(());
    } // check_canvas_screen_state_2
    let canvas_screen_state_2 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_2 != 7 { break 'check_canvas_screen_state_3; }
    } // check_canvas_state_u
    let canvas_state_u = r.get(canvas, field::canvas::STATE_U)?.i();
    if canvas_state_u == 0 { break 'graphics_get_translate_x; }
    } // check_canvas_scene_image
    let canvas_scene_image = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let canvas_state_ar_upper = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    host::graphics_draw_image(r, graphics, canvas_scene_image, canvas_state_ar_upper.wrapping_neg(), 0, 0)?;
    let graphics_get_translate_x_value_3 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_3 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_3.wrapping_neg(), graphics_get_translate_y_value_3.wrapping_neg())?;
    number_2 = 0;
    } // check_canvas_world_2
    'paint_loop_26: loop {
        'check_canvas_world_4: {
        let canvas_world_2 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects = r.get(canvas_world_2, field::world::OBJECTS)?.r();
        let vector_size_value = host::vector_size(r, world_objects)?;
        if number_2 >= vector_size_value { break 'graphics_get_translate_x; }
        } // check_canvas_world_4
        let canvas_world_5 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_3 = r.get(canvas_world_5, field::world::OBJECTS)?.r();
        let vector_element_at_value = host::vector_element_at(r, world_objects_3, number_2)?;
        host::check_cast(r, vector_element_at_value, "n")?;
        let canvas_state_ar_upper_4 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        let canvas_state_l_upper_13 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let canvas_state_m_13 = r.get(canvas, field::canvas::STATE_M)?.i();
        super::map_object::draw(r, vector_element_at_value, graphics, canvas_state_ar_upper_4, 32, canvas_state_l_upper_13, canvas_state_m_13)?;
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_26;
    }
    } // graphics_get_translate_x
    let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value.wrapping_neg(), graphics_get_translate_y_value.wrapping_neg())?;
    let canvas_state_l_upper_4 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_4 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_4, canvas_state_m_4)?;
    let canvas_state_l_upper_5 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_5 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_5, canvas_state_m_5)?;
    object_3 = graphics;
    object_2 = canvas;
    let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_2 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_2.wrapping_neg(), graphics_get_translate_y_value_2.wrapping_neg())?;
    let canvas_state_m_6 = r.get(object_2, field::canvas::STATE_M)?.i();
    number_4 = canvas_state_m_6;
    let canvas_state_l_upper_6 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
    number_5 = canvas_state_l_upper_6;
    let font_get_font_value_2 = host::font_get_font(r, 64, 0, 8)?;
    object_6 = font_get_font_value_2;
    host::graphics_set_font(r, object_3, object_6)?;
    let font_get_height_value = host::font_get_height(r, object_6)?;
    number_7 = font_get_height_value;
    host::graphics_set_color(r, object_3, 13421772)?;
    host::graphics_fill_round_rect(r, object_3, 10, 10, number_5.wrapping_sub(20), number_4.wrapping_sub(10), 10, 10)?;
    host::graphics_set_color(r, object_3, 5592405)?;
    host::graphics_fill_round_rect(r, object_3, 12, 12, number_5.wrapping_sub(22), number_4.wrapping_sub(12), 10, 10)?;
    host::graphics_translate(r, object_3, 20, 0)?;
    host::graphics_translate(r, object_3, 0, 20)?;
    host::graphics_set_color(r, object_3, 16711680)?;
    let text_value_2 = r.intern("Config");
    host::graphics_draw_string(r, object_3, text_value_2, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_7)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection == 0 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_5 = r.intern("Key Config");
    host::graphics_draw_string(r, object_3, text_value_5, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_7)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_3 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_3 == 1 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_8 = r.intern("Sound");
    host::graphics_draw_string(r, object_3, text_value_8, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_7)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_6 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_6 == 2 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_12 = r.intern("Graphics");
    host::graphics_draw_string(r, object_3, text_value_12, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_7)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_10 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_10 == 3 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_54 = r.intern("GameSettings");
    host::graphics_draw_string(r, object_3, text_value_54, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_7)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_15 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_15 == 4 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_59 = r.intern("Back..");
    host::graphics_draw_string(r, object_3, text_value_59, 0, 0, 20)?;
    let canvas_previous_screen = r.get(object_2, field::canvas::PREVIOUS_SCREEN)?.i();
    if canvas_previous_screen != 1 {
let canvas_previous_screen_2_read = r.get(object_2, field::canvas::PREVIOUS_SCREEN)?.i();
if canvas_previous_screen_2_read != 1 {
return Ok(());
}
host::graphics_translate(r, object_3, 0, number_7)?;
host::graphics_set_color(r, object_3, 16777215)?;
let canvas_menu_selection_27_read = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
if canvas_menu_selection_27_read == 6 {
host::graphics_set_color(r, object_3, 65280)?;
}
let text_value_80 = r.intern("Save & Exit");
host::graphics_draw_string(r, object_3, text_value_80, 0, 0, 20)?;
return Ok(());
}
    host::graphics_translate(r, object_3, 0, number_7)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_23 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_23 == 5 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_75 = r.intern("Reset position..");
    host::graphics_draw_string(r, object_3, text_value_75, 0, 0, 20)?;
    let canvas_previous_screen_2_read = r.get(object_2, field::canvas::PREVIOUS_SCREEN)?.i();
    if canvas_previous_screen_2_read != 1 {
return Ok(());
}
    host::graphics_translate(r, object_3, 0, number_7)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_27_read = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_27_read == 6 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_80 = r.intern("Save & Exit");
    host::graphics_draw_string(r, object_3, text_value_80, 0, 0, 20)?;
    return Ok(());
    } // check_canvas_screen_state_3
    let canvas_screen_state_3 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_3 != 8 { break 'check_canvas_screen_state_4; }
    } // check_canvas_state_u_2
    let canvas_state_u_2 = r.get(canvas, field::canvas::STATE_U)?.i();
    if canvas_state_u_2 == 0 { break 'graphics_get_translate_x_2; }
    } // check_canvas_scene_image_2
    let canvas_scene_image_2 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let canvas_state_ar_upper_2 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    host::graphics_draw_image(r, graphics, canvas_scene_image_2, canvas_state_ar_upper_2.wrapping_neg(), 0, 0)?;
    let graphics_get_translate_x_value_6 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_6 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_6.wrapping_neg(), graphics_get_translate_y_value_6.wrapping_neg())?;
    number_2 = 0;
    } // check_canvas_world_3
    'paint_loop_32: loop {
        'check_canvas_world_6: {
        let canvas_world_4 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_2 = r.get(canvas_world_4, field::world::OBJECTS)?.r();
        let vector_size_value_2 = host::vector_size(r, world_objects_2)?;
        if number_2 >= vector_size_value_2 { break 'graphics_get_translate_x_2; }
        } // check_canvas_world_6
        let canvas_world_7 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_5 = r.get(canvas_world_7, field::world::OBJECTS)?.r();
        let vector_element_at_value_2 = host::vector_element_at(r, world_objects_5, number_2)?;
        host::check_cast(r, vector_element_at_value_2, "n")?;
        let canvas_state_ar_upper_9 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        let canvas_state_l_upper_21 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let canvas_state_m_19 = r.get(canvas, field::canvas::STATE_M)?.i();
        super::map_object::draw(r, vector_element_at_value_2, graphics, canvas_state_ar_upper_9, 32, canvas_state_l_upper_21, canvas_state_m_19)?;
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_32;
    }
    } // graphics_get_translate_x_2
    let graphics_get_translate_x_value_4 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_4 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_4.wrapping_neg(), graphics_get_translate_y_value_4.wrapping_neg())?;
    let canvas_state_l_upper_7 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_7 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_7, canvas_state_m_7)?;
    let canvas_state_l_upper_8 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_8 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_8, canvas_state_m_8)?;
    object_3 = graphics;
    object_2 = canvas;
    let font_get_font_value_3 = host::font_get_font(r, 64, 0, 8)?;
    object_4 = font_get_font_value_3;
    host::graphics_set_font(r, object_3, object_4)?;
    let font_get_height_value_2 = host::font_get_height(r, object_4)?;
    number_5 = font_get_height_value_2;
    let graphics_get_translate_x_value_5 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_5 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_5.wrapping_neg(), graphics_get_translate_y_value_5.wrapping_neg())?;
    let canvas_state_m_9 = r.get(object_2, field::canvas::STATE_M)?.i();
    number_6 = canvas_state_m_9;
    let canvas_state_l_upper_9 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
    number_7 = canvas_state_l_upper_9;
    host::graphics_set_color(r, object_3, 13421772)?;
    host::graphics_fill_round_rect(r, object_3, 0, 0, number_7, number_6, 10, 10)?;
    host::graphics_set_color(r, object_3, 5592405)?;
    host::graphics_fill_round_rect(r, object_3, 2, 2, number_7.wrapping_sub(2), number_6.wrapping_sub(2), 10, 10)?;
    host::graphics_translate(r, object_3, 10, 0)?;
    host::graphics_translate(r, object_3, 0, 5)?;
    host::graphics_set_color(r, object_3, 16711680)?;
    let text_value_4 = r.intern("Key Config");
    host::graphics_draw_string(r, object_3, text_value_4, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 5, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_2 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_2 == 0 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value)?;
    let text_value_7 = r.intern("Button UP: ");
    let string_buffer_append_2_value = host::string_buffer_append_2(r, object_value, text_value_7)?;
    let canvas_up_key_4 = r.get(object_2, field::canvas::UP_KEY)?.i();
    let string_buffer_append_value = host::string_buffer_append(r, string_buffer_append_2_value, canvas_up_key_4)?;
    let string_buffer_to_string_value = host::string_buffer_to_string(r, string_buffer_append_value)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_5 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_5 == 1 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_3 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_3)?;
    let text_value_11 = r.intern("Button DOWN: ");
    let string_buffer_append_2_value_3 = host::string_buffer_append_2(r, object_value_3, text_value_11)?;
    let canvas_down_key_5 = r.get(object_2, field::canvas::DOWN_KEY)?.i();
    let string_buffer_append_value_2 = host::string_buffer_append(r, string_buffer_append_2_value_3, canvas_down_key_5)?;
    let string_buffer_to_string_value_3 = host::string_buffer_to_string(r, string_buffer_append_value_2)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_3, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_9 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_9 == 2 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_5 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_5)?;
    let text_value_53 = r.intern("Button LEFT: ");
    let string_buffer_append_2_value_5 = host::string_buffer_append_2(r, object_value_5, text_value_53)?;
    let canvas_left_key_5 = r.get(object_2, field::canvas::LEFT_KEY)?.i();
    let string_buffer_append_value_3 = host::string_buffer_append(r, string_buffer_append_2_value_5, canvas_left_key_5)?;
    let string_buffer_to_string_value_5 = host::string_buffer_to_string(r, string_buffer_append_value_3)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_5, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_14 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_14 == 3 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_8 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_8)?;
    let text_value_58 = r.intern("Button RIGHT: ");
    let string_buffer_append_2_value_8 = host::string_buffer_append_2(r, object_value_8, text_value_58)?;
    let canvas_right_key_5 = r.get(object_2, field::canvas::RIGHT_KEY)?.i();
    let string_buffer_append_value_4 = host::string_buffer_append(r, string_buffer_append_2_value_8, canvas_right_key_5)?;
    let string_buffer_to_string_value_8 = host::string_buffer_to_string(r, string_buffer_append_value_4)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_8, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_19 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_19 == 4 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_10 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_10)?;
    let text_value_72 = r.intern("Button A: ");
    let string_buffer_append_2_value_10 = host::string_buffer_append_2(r, object_value_10, text_value_72)?;
    let canvas_primary_key = r.get(object_2, field::canvas::PRIMARY_KEY)?.i();
    let string_buffer_append_value_5 = host::string_buffer_append(r, string_buffer_append_2_value_10, canvas_primary_key)?;
    let string_buffer_to_string_value_10 = host::string_buffer_to_string(r, string_buffer_append_value_5)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_10, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_22 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_22 == 5 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_12 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_12)?;
    let text_value_74 = r.intern("Button B: ");
    let string_buffer_append_2_value_12 = host::string_buffer_append_2(r, object_value_12, text_value_74)?;
    let canvas_secondary_key = r.get(object_2, field::canvas::SECONDARY_KEY)?.i();
    let string_buffer_append_value_6 = host::string_buffer_append(r, string_buffer_append_2_value_12, canvas_secondary_key)?;
    let string_buffer_to_string_value_12 = host::string_buffer_to_string(r, string_buffer_append_value_6)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_12, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_26 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_26 == 6 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_13 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_13)?;
    let text_value_79 = r.intern("Button MENU: ");
    let string_buffer_append_2_value_13 = host::string_buffer_append_2(r, object_value_13, text_value_79)?;
    let canvas_menu_key = r.get(object_2, field::canvas::MENU_KEY)?.i();
    let string_buffer_append_value_7 = host::string_buffer_append(r, string_buffer_append_2_value_13, canvas_menu_key)?;
    let string_buffer_to_string_value_13 = host::string_buffer_to_string(r, string_buffer_append_value_7)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_13, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_28 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_28 == 7 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_15 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_15)?;
    let text_value_120 = r.intern("UseJavaButtons: ");
    let string_buffer_append_2_value_15 = host::string_buffer_append_2(r, object_value_15, text_value_120)?;
    let canvas_state_w = r.get(object_2, field::canvas::STATE_W)?.i();
    let string_buffer_append_3_value_7 = host::string_buffer_append_3(r, string_buffer_append_2_value_15, canvas_state_w)?;
    let string_buffer_to_string_value_15 = host::string_buffer_to_string(r, string_buffer_append_3_value_7)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_15, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_29 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_29 == 8 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_122 = r.intern("Back... ");
    host::graphics_draw_string(r, object_3, text_value_122, 0, 0, 20)?;
    return Ok(());
    } // check_canvas_screen_state_4
    let canvas_screen_state_4 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_4 != 9 { break 'check_canvas_screen_state_5; }
    } // check_canvas_state_u_4
    let canvas_state_u_4 = r.get(canvas, field::canvas::STATE_U)?.i();
    if canvas_state_u_4 == 0 { break 'graphics_get_translate_x_3; }
    } // check_canvas_scene_image_3
    let canvas_scene_image_4 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let canvas_state_ar_upper_3 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    host::graphics_draw_image(r, graphics, canvas_scene_image_4, canvas_state_ar_upper_3.wrapping_neg(), 0, 0)?;
    let graphics_get_translate_x_value_9 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_9 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_9.wrapping_neg(), graphics_get_translate_y_value_9.wrapping_neg())?;
    number_2 = 0;
    } // check_canvas_world_5
    'paint_loop_38: loop {
        'check_canvas_world_9: {
        let canvas_world_6 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_4 = r.get(canvas_world_6, field::world::OBJECTS)?.r();
        let vector_size_value_3 = host::vector_size(r, world_objects_4)?;
        if number_2 >= vector_size_value_3 { break 'graphics_get_translate_x_3; }
        } // check_canvas_world_9
        let canvas_world_10 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_8 = r.get(canvas_world_10, field::world::OBJECTS)?.r();
        let vector_element_at_value_3 = host::vector_element_at(r, world_objects_8, number_2)?;
        host::check_cast(r, vector_element_at_value_3, "n")?;
        let canvas_state_ar_upper_12 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        let canvas_state_l_upper_26 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let canvas_state_m_24 = r.get(canvas, field::canvas::STATE_M)?.i();
        super::map_object::draw(r, vector_element_at_value_3, graphics, canvas_state_ar_upper_12, 32, canvas_state_l_upper_26, canvas_state_m_24)?;
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_38;
    }
    } // graphics_get_translate_x_3
    let graphics_get_translate_x_value_7 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_7 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_7.wrapping_neg(), graphics_get_translate_y_value_7.wrapping_neg())?;
    let canvas_state_l_upper_10 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_10 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_10, canvas_state_m_10)?;
    let canvas_state_l_upper_11 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_11 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_11, canvas_state_m_11)?;
    object_3 = graphics;
    object_2 = canvas;
    let font_get_font_value_4 = host::font_get_font(r, 64, 0, 8)?;
    object_4 = font_get_font_value_4;
    host::graphics_set_font(r, object_3, object_4)?;
    let font_get_height_value_3 = host::font_get_height(r, object_4)?;
    number_5 = font_get_height_value_3;
    let graphics_get_translate_x_value_8 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_8 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_8.wrapping_neg(), graphics_get_translate_y_value_8.wrapping_neg())?;
    let canvas_state_m_12 = r.get(object_2, field::canvas::STATE_M)?.i();
    number_6 = canvas_state_m_12;
    let canvas_state_l_upper_12 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
    number_7 = canvas_state_l_upper_12;
    host::graphics_set_color(r, object_3, 13421772)?;
    host::graphics_fill_round_rect(r, object_3, 0, 0, number_7, number_6, 10, 10)?;
    host::graphics_set_color(r, object_3, 5592405)?;
    host::graphics_fill_round_rect(r, object_3, 2, 2, number_7.wrapping_sub(2), number_6.wrapping_sub(2), 10, 10)?;
    host::graphics_translate(r, object_3, 10, 0)?;
    host::graphics_translate(r, object_3, 0, 5)?;
    host::graphics_set_color(r, object_3, 16711680)?;
    let text_value_6 = r.intern("Graphics Config");
    host::graphics_draw_string(r, object_3, text_value_6, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 5, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_4 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_4 == 0 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_2 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_2)?;
    let text_value_10 = r.intern("FullScreen: ");
    let string_buffer_append_2_value_2 = host::string_buffer_append_2(r, object_value_2, text_value_10)?;
    let canvas_state_ap_upper = r.get(object_2, field::canvas::STATE_AP_UPPER)?.i();
    let string_buffer_append_3_value = host::string_buffer_append_3(r, string_buffer_append_2_value_2, canvas_state_ap_upper)?;
    let string_buffer_to_string_value_2 = host::string_buffer_to_string(r, string_buffer_append_3_value)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_2, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_8 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_8 == 1 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_4 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_4)?;
    let text_value_52 = r.intern("RainEffects: ");
    let string_buffer_append_2_value_4 = host::string_buffer_append_2(r, object_value_4, text_value_52)?;
    let canvas_use_phone_buttons = r.get(object_2, field::canvas::USE_PHONE_BUTTONS)?.i();
    let string_buffer_append_3_value_2 = host::string_buffer_append_3(r, string_buffer_append_2_value_4, canvas_use_phone_buttons)?;
    let string_buffer_to_string_value_4 = host::string_buffer_to_string(r, string_buffer_append_3_value_2)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_4, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_13 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_13 == 2 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_7 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_7)?;
    let text_value_57 = r.intern("FogEffects: ");
    let string_buffer_append_2_value_7 = host::string_buffer_append_2(r, object_value_7, text_value_57)?;
    let canvas_state_ao_upper = r.get(object_2, field::canvas::STATE_AO_UPPER)?.i();
    let string_buffer_append_3_value_4 = host::string_buffer_append_3(r, string_buffer_append_2_value_7, canvas_state_ao_upper)?;
    let string_buffer_to_string_value_7 = host::string_buffer_to_string(r, string_buffer_append_3_value_4)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_7, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_18 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_18 == 3 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_9 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_9)?;
    let text_value_71 = r.intern("AlphaEffects: ");
    let string_buffer_append_2_value_9 = host::string_buffer_append_2(r, object_value_9, text_value_71)?;
    let canvas_visual_effects = r.get(object_2, field::canvas::VISUAL_EFFECTS)?.i();
    let string_buffer_append_3_value_5 = host::string_buffer_append_3(r, string_buffer_append_2_value_9, canvas_visual_effects)?;
    let string_buffer_to_string_value_9 = host::string_buffer_to_string(r, string_buffer_append_3_value_5)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_9, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_21 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_21 == 4 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_11 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_11)?;
    let text_value_73 = r.intern("AnimatedWater: ");
    let string_buffer_append_2_value_11 = host::string_buffer_append_2(r, object_value_11, text_value_73)?;
    let canvas_state_x = r.get(object_2, field::canvas::STATE_X)?.i();
    let string_buffer_append_3_value_6 = host::string_buffer_append_3(r, string_buffer_append_2_value_11, canvas_state_x)?;
    let string_buffer_to_string_value_11 = host::string_buffer_to_string(r, string_buffer_append_3_value_6)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_11, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_25 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_25 == 5 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_78 = r.intern("Back.. ");
    host::graphics_draw_string(r, object_3, text_value_78, 0, 0, 20)?;
    return Ok(());
    } // check_canvas_screen_state_5
    let canvas_screen_state_5 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_5 != 12 { break 'check_canvas_screen_state_6; }
    } // check_canvas_state_u_6
    let canvas_state_u_6 = r.get(canvas, field::canvas::STATE_U)?.i();
    if canvas_state_u_6 == 0 { break 'graphics_get_translate_x_5; }
    } // check_canvas_scene_image_5
    let canvas_scene_image_6 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let canvas_state_ar_upper_8 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    host::graphics_draw_image(r, graphics, canvas_scene_image_6, canvas_state_ar_upper_8.wrapping_neg(), 0, 0)?;
    let graphics_get_translate_x_value_15 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_15 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_15.wrapping_neg(), graphics_get_translate_y_value_15.wrapping_neg())?;
    number_2 = 0;
    } // check_canvas_world_8
    'paint_loop_44: loop {
        'check_canvas_world_12: {
        let canvas_world_9 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_7 = r.get(canvas_world_9, field::world::OBJECTS)?.r();
        let vector_size_value_5 = host::vector_size(r, world_objects_7)?;
        if number_2 >= vector_size_value_5 { break 'graphics_get_translate_x_5; }
        } // check_canvas_world_12
        let canvas_world_13 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_11 = r.get(canvas_world_13, field::world::OBJECTS)?.r();
        let vector_element_at_value_5 = host::vector_element_at(r, world_objects_11, number_2)?;
        host::check_cast(r, vector_element_at_value_5, "n")?;
        let canvas_state_ar_upper_14 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        let canvas_state_l_upper_29 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let canvas_state_m_27 = r.get(canvas, field::canvas::STATE_M)?.i();
        super::map_object::draw(r, vector_element_at_value_5, graphics, canvas_state_ar_upper_14, 32, canvas_state_l_upper_29, canvas_state_m_27)?;
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_44;
    }
    } // graphics_get_translate_x_5
    let graphics_get_translate_x_value_13 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_13 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_13.wrapping_neg(), graphics_get_translate_y_value_13.wrapping_neg())?;
    let canvas_state_l_upper_18 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_16 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_18, canvas_state_m_16)?;
    let canvas_state_l_upper_19 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_17 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_19, canvas_state_m_17)?;
    object_3 = graphics;
    object_2 = canvas;
    let graphics_get_translate_x_value_14 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_14 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_14.wrapping_neg(), graphics_get_translate_y_value_14.wrapping_neg())?;
    let canvas_state_m_18 = r.get(object_2, field::canvas::STATE_M)?.i();
    number_4 = canvas_state_m_18;
    let canvas_state_l_upper_20 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
    number_5 = canvas_state_l_upper_20;
    let font_get_font_value_5 = host::font_get_font(r, 64, 0, 8)?;
    object_6 = font_get_font_value_5;
    host::graphics_set_font(r, object_3, object_6)?;
    let font_get_height_value_4 = host::font_get_height(r, object_6)?;
    number_7 = font_get_height_value_4;
    host::graphics_set_color(r, object_3, 13421772)?;
    host::graphics_fill_round_rect(r, object_3, 10, 10, number_5.wrapping_sub(20), number_4.wrapping_sub(10), 10, 10)?;
    host::graphics_set_color(r, object_3, 5592405)?;
    host::graphics_fill_round_rect(r, object_3, 12, 12, number_5.wrapping_sub(22), number_4.wrapping_sub(12), 10, 10)?;
    host::graphics_translate(r, object_3, 20, 0)?;
    host::graphics_translate(r, object_3, 0, 20)?;
    host::graphics_set_color(r, object_3, 16711680)?;
    let text_value_9 = r.intern("Sound settings");
    host::graphics_draw_string(r, object_3, text_value_9, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_7)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_7 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_7 == 0 {
host::graphics_set_color(r, object_3, 65280)?;
}
    host::graphics_translate(r, object_3, 0, number_7)?;
    let text_value_51 = r.intern("SoundFX volume");
    host::graphics_draw_string(r, object_3, text_value_51, 0, 0, 20)?;
    let graphics_get_translate_x_value_20 = host::graphics_get_translate_x(r, object_3)?;
    let quotient_value_4 = arithmetic::divide_int(r, (number_5.wrapping_sub(graphics_get_translate_x_value_20)).wrapping_sub(20), 10)?;
    number_8 = quotient_value_4;
    host::graphics_set_color(r, object_3, 11184810)?;
    host::graphics_translate(r, object_3, 0, number_7.wrapping_add(4))?;
    let graphics_get_translate_x_value_21 = host::graphics_get_translate_x(r, object_3)?;
    host::graphics_fill_rect(r, object_3, 0, 0, (number_5.wrapping_sub(graphics_get_translate_x_value_21)).wrapping_sub(20), 10)?;
    let canvas_menu_selection_12 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_12 == 0 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let graphics_get_translate_x_value_22 = host::graphics_get_translate_x(r, object_3)?;
    host::graphics_draw_rect(r, object_3, 0, 0, (number_5.wrapping_sub(graphics_get_translate_x_value_22)).wrapping_sub(20), 10)?;
    host::graphics_set_color(r, object_3, 8912896)?;
    let canvas_effect_volume_2 = r.get(object_2, field::canvas::EFFECT_VOLUME)?.i();
    let quotient_value_5 = arithmetic::divide_int(r, canvas_effect_volume_2, 10)?;
    host::graphics_translate(r, object_3, number_8.wrapping_mul(quotient_value_5), 0)?;
    host::graphics_fill_rect(r, object_3, 1, -1, 5, 12)?;
    let canvas_effect_volume_3 = r.get(object_2, field::canvas::EFFECT_VOLUME)?.i();
    let quotient_value_6 = arithmetic::divide_int(r, canvas_effect_volume_3, 10)?;
    host::graphics_translate(r, object_3, (number_8.wrapping_neg()).wrapping_mul(quotient_value_6), 0)?;
    host::graphics_translate(r, object_3, 0, 10)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_17 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_17 == 1 {
host::graphics_set_color(r, object_3, 65280)?;
}
    host::graphics_translate(r, object_3, 0, number_7)?;
    let text_value_70 = r.intern("Music volume");
    host::graphics_draw_string(r, object_3, text_value_70, 0, 0, 20)?;
    host::graphics_set_color(r, object_3, 11184810)?;
    host::graphics_translate(r, object_3, 0, number_7.wrapping_add(4))?;
    let graphics_get_translate_x_value_26 = host::graphics_get_translate_x(r, object_3)?;
    host::graphics_fill_rect(r, object_3, 0, 0, (number_5.wrapping_sub(graphics_get_translate_x_value_26)).wrapping_sub(20), 10)?;
    let canvas_menu_selection_20 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_20 == 1 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let graphics_get_translate_x_value_27 = host::graphics_get_translate_x(r, object_3)?;
    host::graphics_draw_rect(r, object_3, 0, 0, (number_5.wrapping_sub(graphics_get_translate_x_value_27)).wrapping_sub(20), 10)?;
    host::graphics_set_color(r, object_3, 8912896)?;
    let canvas_music_volume_2 = r.get(object_2, field::canvas::MUSIC_VOLUME)?.i();
    let quotient_value_9 = arithmetic::divide_int(r, canvas_music_volume_2, 10)?;
    host::graphics_translate(r, object_3, number_8.wrapping_mul(quotient_value_9), 0)?;
    host::graphics_fill_rect(r, object_3, 1, -1, 5, 12)?;
    let canvas_music_volume_3 = r.get(object_2, field::canvas::MUSIC_VOLUME)?.i();
    let quotient_value_10 = arithmetic::divide_int(r, canvas_music_volume_3, 10)?;
    host::graphics_translate(r, object_3, (number_8.wrapping_neg()).wrapping_mul(quotient_value_10), 0)?;
    host::graphics_translate(r, object_3, 0, 10)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_24 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_24 == 2 {
host::graphics_set_color(r, object_3, 65280)?;
}
    host::graphics_translate(r, object_3, 0, number_7)?;
    let text_value_77 = r.intern("Back..");
    host::graphics_draw_string(r, object_3, text_value_77, 0, 0, 20)?;
    return Ok(());
    } // check_canvas_screen_state_6
    let canvas_screen_state_6 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_6 != 13 { break 'check_canvas_screen_state_7; }
    } // check_canvas_state_u_7
    let canvas_state_u_7 = r.get(canvas, field::canvas::STATE_U)?.i();
    if canvas_state_u_7 == 0 { break 'graphics_get_translate_x_6; }
    } // check_canvas_scene_image_6
    let canvas_scene_image_7 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let canvas_state_ar_upper_11 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    host::graphics_draw_image(r, graphics, canvas_scene_image_7, canvas_state_ar_upper_11.wrapping_neg(), 0, 0)?;
    let graphics_get_translate_x_value_19 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_20 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_19.wrapping_neg(), graphics_get_translate_y_value_20.wrapping_neg())?;
    number_2 = 0;
    } // check_canvas_world_11
    'paint_loop_50: loop {
        'check_canvas_world_14: {
        let canvas_world_12 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_10 = r.get(canvas_world_12, field::world::OBJECTS)?.r();
        let vector_size_value_6 = host::vector_size(r, world_objects_10)?;
        if number_2 >= vector_size_value_6 { break 'graphics_get_translate_x_6; }
        } // check_canvas_world_14
        let canvas_world_15 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_12 = r.get(canvas_world_15, field::world::OBJECTS)?.r();
        let vector_element_at_value_6 = host::vector_element_at(r, world_objects_12, number_2)?;
        host::check_cast(r, vector_element_at_value_6, "n")?;
        let canvas_state_ar_upper_18 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        let canvas_state_l_upper_33 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let canvas_state_m_30 = r.get(canvas, field::canvas::STATE_M)?.i();
        super::map_object::draw(r, vector_element_at_value_6, graphics, canvas_state_ar_upper_18, 32, canvas_state_l_upper_33, canvas_state_m_30)?;
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_50;
    }
    } // graphics_get_translate_x_6
    let graphics_get_translate_x_value_17 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_18 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_17.wrapping_neg(), graphics_get_translate_y_value_18.wrapping_neg())?;
    let canvas_state_l_upper_23 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_21 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_23, canvas_state_m_21)?;
    let canvas_state_l_upper_24 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_22 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_24, canvas_state_m_22)?;
    object_3 = graphics;
    object_2 = canvas;
    let font_get_font_value_6 = host::font_get_font(r, 64, 0, 8)?;
    object_4 = font_get_font_value_6;
    host::graphics_set_font(r, object_3, object_4)?;
    let font_get_height_value_5 = host::font_get_height(r, object_4)?;
    number_5 = font_get_height_value_5;
    let graphics_get_translate_x_value_18 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_19 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_18.wrapping_neg(), graphics_get_translate_y_value_19.wrapping_neg())?;
    let canvas_state_m_23 = r.get(object_2, field::canvas::STATE_M)?.i();
    number_6 = canvas_state_m_23;
    let canvas_state_l_upper_25 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
    number_7 = canvas_state_l_upper_25;
    host::graphics_set_color(r, object_3, 13421772)?;
    host::graphics_fill_round_rect(r, object_3, 0, 0, number_7, number_6, 10, 10)?;
    host::graphics_set_color(r, object_3, 5592405)?;
    host::graphics_fill_round_rect(r, object_3, 2, 2, number_7.wrapping_sub(2), number_6.wrapping_sub(2), 10, 10)?;
    host::graphics_translate(r, object_3, 10, 0)?;
    host::graphics_translate(r, object_3, 0, 5)?;
    host::graphics_set_color(r, object_3, 16711680)?;
    let text_value_50 = r.intern("Game settings");
    host::graphics_draw_string(r, object_3, text_value_50, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 5, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_11 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_11 == 0 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let object_value_6 = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_6)?;
    let text_value_56 = r.intern("Dying: ");
    let string_buffer_append_2_value_6 = host::string_buffer_append_2(r, object_value_6, text_value_56)?;
    let canvas_state_ak = r.get(object_2, field::canvas::STATE_AK)?.i();
    let string_buffer_append_3_value_3 = host::string_buffer_append_3(r, string_buffer_append_2_value_6, canvas_state_ak)?;
    let string_buffer_to_string_value_6 = host::string_buffer_to_string(r, string_buffer_append_3_value_3)?;
    host::graphics_draw_string(r, object_3, string_buffer_to_string_value_6, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, number_5)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    let canvas_menu_selection_16 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_16 == 1 {
host::graphics_set_color(r, object_3, 65280)?;
}
    let text_value_69 = r.intern("Back.. ");
    host::graphics_draw_string(r, object_3, text_value_69, 0, 0, 20)?;
    return Ok(());
    } // check_canvas_screen_state_7
    let canvas_screen_state_7 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_7 == 100 {
object_3 = graphics;
object_2 = canvas;
let graphics_get_translate_x_value_16 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_16 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_16.wrapping_neg(), graphics_get_translate_y_value_16.wrapping_neg())?;
let canvas_state_m_20 = r.get(object_2, field::canvas::STATE_M)?.i();
number_4 = canvas_state_m_20;
let canvas_state_l_upper_22 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
number_5 = canvas_state_l_upper_22;
host::graphics_set_color(r, object_3, 13421772)?;
host::graphics_fill_round_rect(r, object_3, 10, 0, number_5.wrapping_sub(20), number_4, 10, 10)?;
host::graphics_set_color(r, object_3, 5592405)?;
host::graphics_fill_round_rect(r, object_3, 12, 2, number_5.wrapping_sub(22), number_4.wrapping_sub(2), 10, 10)?;
host::graphics_translate(r, object_3, 20, 0)?;
let canvas_state_al_upper = r.get(object_2, field::canvas::STATE_AL_UPPER)?.i();
host::graphics_translate(r, object_3, 0, canvas_state_al_upper)?;
host::graphics_set_color(r, object_3, 16711680)?;
let text_value_14 = r.intern("Thank you for playing");
host::graphics_draw_string(r, object_3, text_value_14, 0, 0, 20)?;
host::graphics_set_color(r, object_3, 16777215)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_set_color(r, object_3, 16711680)?;
let text_value_15 = r.intern("Credits");
host::graphics_draw_string(r, object_3, text_value_15, 0, 0, 20)?;
host::graphics_set_color(r, object_3, 16777215)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_16 = r.intern("Code:");
host::graphics_draw_string(r, object_3, text_value_16, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_17 = r.intern("Gorthwogh");
host::graphics_draw_string(r, object_3, text_value_17, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_18 = r.intern("Music:");
host::graphics_draw_string(r, object_3, text_value_18, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_19 = r.intern("Floaf (X-FormZ)");
host::graphics_draw_string(r, object_3, text_value_19, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_20 = r.intern("Graphics:");
host::graphics_draw_string(r, object_3, text_value_20, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_21 = r.intern("Nintendo.. duh!");
host::graphics_draw_string(r, object_3, text_value_21, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_22 = r.intern("Gorthwogh");
host::graphics_draw_string(r, object_3, text_value_22, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_23 = r.intern("Hansson");
host::graphics_draw_string(r, object_3, text_value_23, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_24 = r.intern("Edwin Bento");
host::graphics_draw_string(r, object_3, text_value_24, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_25 = r.intern("Graphics: (rips)");
host::graphics_draw_string(r, object_3, text_value_25, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_26 = r.intern("Gorthwogh");
host::graphics_draw_string(r, object_3, text_value_26, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_27 = r.intern("xfinium");
host::graphics_draw_string(r, object_3, text_value_27, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_28 = r.intern("Polar koala");
host::graphics_draw_string(r, object_3, text_value_28, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_29 = r.intern("Helios");
host::graphics_draw_string(r, object_3, text_value_29, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_30 = r.intern("TRM");
host::graphics_draw_string(r, object_3, text_value_30, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_31 = r.intern("more.. :P");
host::graphics_draw_string(r, object_3, text_value_31, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_32 = r.intern("Testers: ");
host::graphics_draw_string(r, object_3, text_value_32, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_33 = r.intern("D-Flax");
host::graphics_draw_string(r, object_3, text_value_33, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_34 = r.intern("Floaf");
host::graphics_draw_string(r, object_3, text_value_34, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_35 = r.intern("Linda - love you.. :P");
host::graphics_draw_string(r, object_3, text_value_35, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_36 = r.intern("Nex");
host::graphics_draw_string(r, object_3, text_value_36, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_37 = r.intern("Uppy");
host::graphics_draw_string(r, object_3, text_value_37, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_38 = r.intern("Fox");
host::graphics_draw_string(r, object_3, text_value_38, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_39 = r.intern("Lance");
host::graphics_draw_string(r, object_3, text_value_39, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_40 = r.intern("Justin");
host::graphics_draw_string(r, object_3, text_value_40, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_41 = r.intern("Sovereign");
host::graphics_draw_string(r, object_3, text_value_41, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_42 = r.intern("Taarna23");
host::graphics_draw_string(r, object_3, text_value_42, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_43 = r.intern("zfgc.com");
host::graphics_draw_string(r, object_3, text_value_43, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_44 = r.intern("Getjar.com");
host::graphics_draw_string(r, object_3, text_value_44, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_45 = r.intern("Thank you all");
host::graphics_draw_string(r, object_3, text_value_45, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_46 = r.intern("for your support!");
host::graphics_draw_string(r, object_3, text_value_46, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_47 = r.intern("Gorthwogh - 2009");
host::graphics_draw_string(r, object_3, text_value_47, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_48 = r.intern("zeldamobile.gorthwogh.com");
host::graphics_draw_string(r, object_3, text_value_48, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
let text_value_49 = r.intern("Gorthwogh@gmail.com");
host::graphics_draw_string(r, object_3, text_value_49, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 16)?;
host::graphics_translate(r, object_3, 0, 16)?;
let graphics_get_translate_y_value_17 = host::graphics_get_translate_y(r, object_3)?;
if graphics_get_translate_y_value_17 < 0 {
let canvas_state_m_26 = r.get(object_2, field::canvas::STATE_M)?.i();
r.set(object_2, field::canvas::STATE_AL_UPPER, V::I(canvas_state_m_26))?;
}
let canvas_state_al_upper_2 = r.get(object_2, field::canvas::STATE_AL_UPPER)?.i();
r.set(object_2, field::canvas::STATE_AL_UPPER, V::I(canvas_state_al_upper_2.wrapping_sub(1)))?;
return Ok(());
}
    let canvas_screen_state_8 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_8 != 10 { break 'check_canvas_screen_state_8; }
    } // check_canvas_state_u_8
    let canvas_state_u_8 = r.get(canvas, field::canvas::STATE_U)?.i();
    if canvas_state_u_8 == 0 { break 'check_canvas_state_v_upper; }
    } // check_canvas_scene_image_7
    let canvas_scene_image_8 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let canvas_state_ar_upper_17 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    host::graphics_draw_image(r, graphics, canvas_scene_image_8, canvas_state_ar_upper_17.wrapping_neg(), 0, 0)?;
    let graphics_get_translate_x_value_25 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_23 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_25.wrapping_neg(), graphics_get_translate_y_value_23.wrapping_neg())?;
    number_2 = 0;
    } // check_canvas_world_16
    'paint_loop_56: loop {
        'check_canvas_world_17: {
        let canvas_world_17 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_13 = r.get(canvas_world_17, field::world::OBJECTS)?.r();
        let vector_size_value_8 = host::vector_size(r, world_objects_13)?;
        if number_2 >= vector_size_value_8 { break 'check_canvas_state_v_upper; }
        } // check_canvas_world_17
        let canvas_world_18 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_14 = r.get(canvas_world_18, field::world::OBJECTS)?.r();
        let vector_element_at_value_8 = host::vector_element_at(r, world_objects_14, number_2)?;
        host::check_cast(r, vector_element_at_value_8, "n")?;
        let canvas_state_ar_upper_21 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
        let canvas_state_l_upper_36 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let canvas_state_m_32 = r.get(canvas, field::canvas::STATE_M)?.i();
        super::map_object::draw(r, vector_element_at_value_8, graphics, canvas_state_ar_upper_21, 32, canvas_state_l_upper_36, canvas_state_m_32)?;
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_56;
    }
    } // check_canvas_state_v_upper
    let canvas_state_v_upper_3 = r.get(canvas, field::canvas::STATE_V_UPPER)?.r();
    let image_get_width_value_2 = host::image_get_width(r, canvas_state_v_upper_3)?;
    let graphics_get_translate_x_value_24 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_22 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_24.wrapping_neg(), graphics_get_translate_y_value_22.wrapping_neg())?;
    let canvas_state_l_upper_30 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_28 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_30, canvas_state_m_28)?;
    let canvas_state_l_upper_31 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_29 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_31, canvas_state_m_29)?;
    let canvas_state_ar_upper_15 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    let canvas_state_as_upper_2 = r.get(canvas, field::canvas::STATE_AS_UPPER)?.i();
    r.set(canvas, field::canvas::STATE_AR_UPPER, V::I(canvas_state_ar_upper_15.wrapping_add(canvas_state_as_upper_2)))?;
    let canvas_state_ar_upper_16 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    let canvas_state_l_upper_32 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    if !(canvas_state_ar_upper_16.wrapping_add(canvas_state_l_upper_32) <= 256) {
r.set(canvas, field::canvas::STATE_AS_UPPER, V::I(-1))?;
}
    let canvas_state_ar_upper_20 = r.get(canvas, field::canvas::STATE_AR_UPPER)?.i();
    if canvas_state_ar_upper_20 < 0 {
r.set(canvas, field::canvas::STATE_AS_UPPER, V::I(1))?;
}
    let canvas_state_l_upper_35 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    if canvas_state_l_upper_35 >= 250 {
r.set(canvas, field::canvas::STATE_AR_UPPER, V::I(0))?;
}
    object_3 = graphics;
    object_2 = canvas;
    let graphics_get_translate_x_value_29 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_25 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_29.wrapping_neg(), graphics_get_translate_y_value_25.wrapping_neg())?;
    let canvas_state_m_33 = r.get(object_2, field::canvas::STATE_M)?.i();
    number_4 = canvas_state_m_33;
    let canvas_state_l_upper_37 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
    number_5 = canvas_state_l_upper_37;
    host::graphics_set_color(r, object_3, 13421772)?;
    host::graphics_fill_round_rect(r, object_3, 10, 0, number_5.wrapping_sub(20), number_4, 10, 10)?;
    host::graphics_set_color(r, object_3, 5592405)?;
    host::graphics_fill_round_rect(r, object_3, 12, 2, number_5.wrapping_sub(22), number_4.wrapping_sub(2), 10, 10)?;
    host::graphics_translate(r, object_3, 20, 0)?;
    let canvas_state_al_upper_3 = r.get(object_2, field::canvas::STATE_AL_UPPER)?.i();
    host::graphics_translate(r, object_3, 0, canvas_state_al_upper_3)?;
    host::graphics_set_color(r, object_3, 16711680)?;
    let text_value_83 = r.intern("Credits");
    host::graphics_draw_string(r, object_3, text_value_83, 0, 0, 20)?;
    host::graphics_set_color(r, object_3, 16777215)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_84 = r.intern("Code:");
    host::graphics_draw_string(r, object_3, text_value_84, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_85 = r.intern("Gorthwogh");
    host::graphics_draw_string(r, object_3, text_value_85, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_86 = r.intern("Music:");
    host::graphics_draw_string(r, object_3, text_value_86, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_87 = r.intern("Floaf (X-FormZ)");
    host::graphics_draw_string(r, object_3, text_value_87, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_88 = r.intern("Graphics:");
    host::graphics_draw_string(r, object_3, text_value_88, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_89 = r.intern("Nintendo.. duh!");
    host::graphics_draw_string(r, object_3, text_value_89, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_90 = r.intern("Gorthwogh");
    host::graphics_draw_string(r, object_3, text_value_90, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_91 = r.intern("Hansson");
    host::graphics_draw_string(r, object_3, text_value_91, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_92 = r.intern("Edwin Bento");
    host::graphics_draw_string(r, object_3, text_value_92, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_93 = r.intern("Graphics: (rips)");
    host::graphics_draw_string(r, object_3, text_value_93, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_94 = r.intern("Gorthwogh");
    host::graphics_draw_string(r, object_3, text_value_94, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_95 = r.intern("xfinium");
    host::graphics_draw_string(r, object_3, text_value_95, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_96 = r.intern("Polar koala");
    host::graphics_draw_string(r, object_3, text_value_96, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_97 = r.intern("Helios");
    host::graphics_draw_string(r, object_3, text_value_97, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_98 = r.intern("TRM");
    host::graphics_draw_string(r, object_3, text_value_98, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_99 = r.intern("more.. :P");
    host::graphics_draw_string(r, object_3, text_value_99, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_100 = r.intern("Testers: ");
    host::graphics_draw_string(r, object_3, text_value_100, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_101 = r.intern("D-Flax");
    host::graphics_draw_string(r, object_3, text_value_101, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_102 = r.intern("Floaf");
    host::graphics_draw_string(r, object_3, text_value_102, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_103 = r.intern("Linda - love you.. :P");
    host::graphics_draw_string(r, object_3, text_value_103, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_104 = r.intern("Nex");
    host::graphics_draw_string(r, object_3, text_value_104, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_105 = r.intern("Uppy");
    host::graphics_draw_string(r, object_3, text_value_105, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_106 = r.intern("Fox");
    host::graphics_draw_string(r, object_3, text_value_106, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_107 = r.intern("Lance");
    host::graphics_draw_string(r, object_3, text_value_107, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_108 = r.intern("Justin");
    host::graphics_draw_string(r, object_3, text_value_108, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_109 = r.intern("Sovereign");
    host::graphics_draw_string(r, object_3, text_value_109, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_110 = r.intern("Taarna23");
    host::graphics_draw_string(r, object_3, text_value_110, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_111 = r.intern("zfgc.com");
    host::graphics_draw_string(r, object_3, text_value_111, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_112 = r.intern("Getjar.com");
    host::graphics_draw_string(r, object_3, text_value_112, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_113 = r.intern("Thank you all");
    host::graphics_draw_string(r, object_3, text_value_113, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_114 = r.intern("for your support!");
    host::graphics_draw_string(r, object_3, text_value_114, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_115 = r.intern("Gorthwogh - 2007");
    host::graphics_draw_string(r, object_3, text_value_115, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_116 = r.intern("zeldamobile.gorthwogh.com");
    host::graphics_draw_string(r, object_3, text_value_116, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_117 = r.intern("Gorthwogh@gmail.com");
    host::graphics_draw_string(r, object_3, text_value_117, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_118 = r.intern("CU in the next release..");
    host::graphics_draw_string(r, object_3, text_value_118, 0, 0, 20)?;
    host::graphics_translate(r, object_3, 0, 16)?;
    let text_value_119 = r.intern("DEC - 2008");
    host::graphics_draw_string(r, object_3, text_value_119, 0, 0, 20)?;
    let graphics_get_translate_y_value_26 = host::graphics_get_translate_y(r, object_3)?;
    if graphics_get_translate_y_value_26 < 0 {
let canvas_state_m_34 = r.get(object_2, field::canvas::STATE_M)?.i();
r.set(object_2, field::canvas::STATE_AL_UPPER, V::I(canvas_state_m_34))?;
}
    let canvas_state_al_upper_4 = r.get(object_2, field::canvas::STATE_AL_UPPER)?.i();
    r.set(object_2, field::canvas::STATE_AL_UPPER, V::I(canvas_state_al_upper_4.wrapping_sub(1)))?;
    return Ok(());
    } // check_canvas_screen_state_8
    let canvas_screen_state_9 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_9 == -1 {
host::graphics_set_color(r, graphics, 0)?;
let canvas_get_width_value_6 = host::canvas_get_width(r, canvas)?;
let canvas_get_height_value_6 = host::canvas_get_height(r, canvas)?;
host::graphics_fill_rect(r, graphics, 0, 0, canvas_get_width_value_6, canvas_get_height_value_6)?;
host::graphics_set_color(r, graphics, 16777215)?;
let canvas_get_width_value_7 = host::canvas_get_width(r, canvas)?;
let quotient_value_7 = arithmetic::divide_int(r, canvas_get_width_value_7, 2)?;
let canvas_get_height_value_7 = host::canvas_get_height(r, canvas)?;
let quotient_value_8 = arithmetic::divide_int(r, canvas_get_height_value_7, 3)?;
host::graphics_translate(r, graphics, quotient_value_7.wrapping_sub(50), quotient_value_8.wrapping_sub(10))?;
let text_value_64 = r.intern("Out of memory!");
host::graphics_draw_string(r, graphics, text_value_64, 0, 0, 20)?;
host::graphics_set_color(r, graphics, 16777215)?;
host::graphics_translate(r, graphics, 0, 10)?;
let text_value_65 = r.intern("Try Restarting..");
host::graphics_draw_string(r, graphics, text_value_65, 0, 0, 20)?;
host::graphics_translate(r, graphics, 0, 10)?;
let text_value_66 = r.intern("3 times.. then report this..");
host::graphics_draw_string(r, graphics, text_value_66, 0, 0, 20)?;
host::graphics_translate(r, graphics, 0, 10)?;
let text_value_67 = r.intern("in compability forum");
host::graphics_draw_string(r, graphics, text_value_67, 0, 0, 20)?;
host::graphics_translate(r, graphics, 0, 20)?;
let text_value_68 = r.intern("sorry.. ");
host::graphics_draw_string(r, graphics, text_value_68, 0, 0, 20)?;
return Ok(());
}
    let canvas_screen_state_10 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_10 != 30 { break 'check_canvas_screen_state_9; }
    } // next_step_3
    object_3 = graphics;
    object_2 = canvas;
    let canvas_state_aa = r.get(canvas, field::canvas::STATE_AA)?.r();
    if canvas_state_aa == 0 {
let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
let text_value_76 = r.intern("is null");
host::print_stream_println(r, state_out_value_2, text_value_76)?;
return Ok(());
}
    let canvas_state_ab = r.get(object_2, field::canvas::STATE_AB)?.r();
    if canvas_state_ab == 0 {
let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
let text_value_76 = r.intern("is null");
host::print_stream_println(r, state_out_value_2, text_value_76)?;
return Ok(());
}
    let canvas_state_ad = r.get(object_2, field::canvas::STATE_AD)?.r();
    if canvas_state_ad == 0 {
let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
let text_value_76 = r.intern("is null");
host::print_stream_println(r, state_out_value_2, text_value_76)?;
return Ok(());
}
    let graphics_get_translate_x_value_32 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_29 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_32.wrapping_neg(), graphics_get_translate_y_value_29.wrapping_neg())?;
    let canvas_get_width_value_10 = host::canvas_get_width(r, object_2)?;
    let quotient_value_15 = arithmetic::divide_int(r, canvas_get_width_value_10, 2)?;
    host::graphics_translate(r, object_3, quotient_value_15.wrapping_sub(64), 10)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 128, 41)?;
    host::graphics_set_clip(r, object_3, 0, 0, 128, 41)?;
    let canvas_state_aa_2 = r.get(object_2, field::canvas::STATE_AA)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_aa_2, 0, -26, 0)?;
    host::graphics_translate(r, object_3, 4, 6)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 12, 25)?;
    host::graphics_set_clip(r, object_3, 0, 0, 12, 25)?;
    let canvas_state_aa_3 = r.get(object_2, field::canvas::STATE_AA)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_aa_3, 0, 0, 0)?;
    host::graphics_translate(r, object_3, -4, -6)?;
    super::canvas::draw_key_prompt(r, object_2, object_3, 0)?;
    let canvas_menu_selection_30 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_30 <= 1 { break 'check_canvas_menu_selection; }
    } // graphics_translate
    host::graphics_translate(r, object_3, 0, 42)?;
    break 'graphics_clip_rect;
    } // check_canvas_menu_selection
    let canvas_menu_selection_31 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_31 == 0 { break 'graphics_translate_2; }
    } // check_canvas_menu_selection_2
    let canvas_menu_selection_33 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_33 != 1 { break 'graphics_translate_3; }
    } // graphics_translate_2
    host::graphics_translate(r, object_3, 30, 35)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 31, 13)?;
    host::graphics_set_clip(r, object_3, 0, 0, 31, 13)?;
    let canvas_menu_selection_32 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_32 != 0 {
let canvas_state_aa_6 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_6, -36, 0, 0)?;
} else {
let canvas_state_aa_7 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_7, -67, 0, 0)?;
}
    host::graphics_translate(r, object_3, 40, 0)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 31, 13)?;
    host::graphics_set_clip(r, object_3, 0, 0, 31, 13)?;
    let canvas_menu_selection_36 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_36 != 1 {
let canvas_state_aa_11 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_11, -36, -13, 0)?;
} else {
let canvas_state_aa_12 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_12, -67, -13, 0)?;
}
    host::graphics_translate(r, object_3, -70, 15)?;
    break 'graphics_clip_rect;
    } // graphics_translate_3
    host::graphics_translate(r, object_3, 0, 50)?;
    } // graphics_clip_rect
    host::graphics_clip_rect(r, object_3, 0, 0, 128, 41)?;
    host::graphics_set_clip(r, object_3, 0, 0, 128, 41)?;
    let canvas_state_aa_4 = r.get(object_2, field::canvas::STATE_AA)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_aa_4, 0, -26, 0)?;
    host::graphics_translate(r, object_3, 4, 6)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 12, 25)?;
    host::graphics_set_clip(r, object_3, 0, 0, 12, 25)?;
    let canvas_state_aa_5 = r.get(object_2, field::canvas::STATE_AA)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_aa_5, -12, 0, 0)?;
    host::graphics_translate(r, object_3, -4, -6)?;
    super::canvas::draw_key_prompt(r, object_2, object_3, 1)?;
    let canvas_menu_selection_34 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_34 <= 3 { break 'check_canvas_menu_selection_3; }
    } // graphics_translate_4
    host::graphics_translate(r, object_3, 0, 42)?;
    break 'graphics_clip_rect_2;
    } // check_canvas_menu_selection_3
    let canvas_menu_selection_35 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_35 == 2 { break 'graphics_translate_5; }
    } // check_canvas_menu_selection_4
    let canvas_menu_selection_38 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_38 != 3 { break 'graphics_translate_6; }
    } // graphics_translate_5
    host::graphics_translate(r, object_3, 30, 35)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 31, 13)?;
    host::graphics_set_clip(r, object_3, 0, 0, 31, 13)?;
    let canvas_menu_selection_37 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_37 != 2 {
let canvas_state_aa_13 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_13, -36, 0, 0)?;
} else {
let canvas_state_aa_14 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_14, -67, 0, 0)?;
}
    host::graphics_translate(r, object_3, 40, 0)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 31, 13)?;
    host::graphics_set_clip(r, object_3, 0, 0, 31, 13)?;
    let canvas_menu_selection_42 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_42 != 3 {
let canvas_state_aa_20 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_20, -36, -13, 0)?;
} else {
let canvas_state_aa_21 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_21, -67, -13, 0)?;
}
    host::graphics_translate(r, object_3, -70, 15)?;
    break 'graphics_clip_rect_2;
    } // graphics_translate_6
    host::graphics_translate(r, object_3, 0, 50)?;
    } // graphics_clip_rect_2
    host::graphics_clip_rect(r, object_3, 0, 0, 128, 41)?;
    host::graphics_set_clip(r, object_3, 0, 0, 128, 41)?;
    let canvas_state_aa_8 = r.get(object_2, field::canvas::STATE_AA)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_aa_8, 0, -26, 0)?;
    host::graphics_translate(r, object_3, 4, 6)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 12, 25)?;
    host::graphics_set_clip(r, object_3, 0, 0, 12, 25)?;
    let canvas_state_aa_9 = r.get(object_2, field::canvas::STATE_AA)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_aa_9, -24, 0, 0)?;
    host::graphics_translate(r, object_3, -4, -6)?;
    super::canvas::draw_key_prompt(r, object_2, object_3, 2)?;
    let canvas_menu_selection_39 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_39 != 4 {
let canvas_menu_selection_41 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
if canvas_menu_selection_41 != 5 {
host::graphics_translate(r, object_3, 0, 50)?;
r.set(object_2, field::canvas::REDRAW, V::I(1))?;
return Ok(());
}
}
    host::graphics_translate(r, object_3, 30, 35)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 31, 13)?;
    host::graphics_set_clip(r, object_3, 0, 0, 31, 13)?;
    let canvas_menu_selection_40 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_40 != 4 {
let canvas_state_aa_16 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_16, -36, 0, 0)?;
} else {
let canvas_state_aa_17 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_17, -67, 0, 0)?;
}
    host::graphics_translate(r, object_3, 40, 0)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 31, 13)?;
    host::graphics_set_clip(r, object_3, 0, 0, 31, 13)?;
    let canvas_menu_selection_43 = r.get(object_2, field::canvas::MENU_SELECTION)?.i();
    if canvas_menu_selection_43 != 5 {
let canvas_state_aa_23 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_23, -36, -13, 0)?;
} else {
let canvas_state_aa_24 = r.get(object_2, field::canvas::STATE_AA)?.r();
host::graphics_draw_image(r, object_3, canvas_state_aa_24, -67, -13, 0)?;
}
    host::graphics_translate(r, object_3, -70, 15)?;
    r.set(object_2, field::canvas::REDRAW, V::I(1))?;
    return Ok(());
    } // check_canvas_screen_state_9
    let canvas_screen_state_11 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_11 == 6 {
object_3 = graphics;
object_2 = canvas;
let canvas_screen_state_13 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
if canvas_screen_state_13 != 6 {
return Ok(());
}
let graphics_get_font_value = host::graphics_get_font(r, object_3)?;
let font_get_font_value_7 = host::font_get_font(r, 0, 0, 8)?;
object_5 = font_get_font_value_7;
host::graphics_set_font(r, object_3, object_5)?;
let graphics_get_translate_x_value_28 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_24 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_28.wrapping_neg(), graphics_get_translate_y_value_24.wrapping_neg())?;
let canvas_get_width_value_8 = host::canvas_get_width(r, object_2)?;
let quotient_value_11 = arithmetic::divide_int(r, canvas_get_width_value_8, 2)?;
let canvas_get_height_value_8 = host::canvas_get_height(r, object_2)?;
let quotient_value_12 = arithmetic::divide_int(r, canvas_get_height_value_8, 2)?;
host::graphics_translate(r, object_3, quotient_value_11.wrapping_sub(50), quotient_value_12.wrapping_sub(40))?;
host::graphics_set_color(r, object_3, 0)?;
host::graphics_fill_round_rect(r, object_3, 0, 0, 100, 100, 10, 10)?;
host::graphics_set_color(r, object_3, 16777215)?;
host::graphics_translate(r, object_3, 5, 0)?;
let text_value_81 = r.intern("Press New B button");
host::graphics_draw_string(r, object_3, text_value_81, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 15)?;
let object_value_14 = r.alloc("java/lang/StringBuffer", Native::Empty);
host::string_buffer_initialize(r, object_value_14)?;
let text_value_82 = r.intern("Current button: ");
let string_buffer_append_2_value_14 = host::string_buffer_append_2(r, object_value_14, text_value_82)?;
let canvas_secondary_key_2 = r.get(object_2, field::canvas::SECONDARY_KEY)?.i();
let string_buffer_append_value_8 = host::string_buffer_append(r, string_buffer_append_2_value_14, canvas_secondary_key_2)?;
let string_buffer_to_string_value_14 = host::string_buffer_to_string(r, string_buffer_append_value_8)?;
host::graphics_draw_string(r, object_3, string_buffer_to_string_value_14, 0, 0, 20)?;
return Ok(());
}
    let canvas_screen_state_12 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_12 >= 20 {
let canvas_screen_state_15 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
if canvas_screen_state_15 <= 26 {
host::graphics_set_color(r, graphics, 16777215)?;
let canvas_get_width_value_9 = host::canvas_get_width(r, canvas)?;
let quotient_value_13 = arithmetic::divide_int(r, canvas_get_width_value_9, 2)?;
let canvas_get_height_value_9 = host::canvas_get_height(r, canvas)?;
let quotient_value_14 = arithmetic::divide_int(r, canvas_get_height_value_9, 3)?;
host::graphics_translate(r, graphics, quotient_value_13.wrapping_sub(50), quotient_value_14.wrapping_sub(10))?;
let text_value_121 = r.intern("Press new button");
host::graphics_draw_string(r, graphics, text_value_121, 0, 0, 20)?;
return Ok(());
}
}
    let canvas_screen_state_14 = r.get(canvas, field::canvas::SCREEN_STATE)?.i();
    if canvas_screen_state_14 == 11 {
host::graphics_translate(r, graphics, 0, 32)?;
let canvas_scene_image_9 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
let canvas_state_p_2 = r.get(canvas, field::canvas::STATE_P)?.i();
let canvas_state_q_2 = r.get(canvas, field::canvas::STATE_Q)?.i();
host::graphics_draw_image(r, graphics, canvas_scene_image_9, canvas_state_p_2.wrapping_neg(), canvas_state_q_2.wrapping_neg(), 0)?;
let graphics_get_translate_x_value_30 = host::graphics_get_translate_x(r, graphics)?;
let graphics_get_translate_y_value_27 = host::graphics_get_translate_y(r, graphics)?;
host::graphics_translate(r, graphics, graphics_get_translate_x_value_30.wrapping_neg(), graphics_get_translate_y_value_27.wrapping_neg())?;
let canvas_state_l_upper_38 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_38, 32)?;
let canvas_state_l_upper_39 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_39, 32)?;
let canvas_state_s = r.get(canvas, field::canvas::STATE_S)?.r();
host::graphics_draw_image(r, graphics, canvas_state_s, 0, 0, 0)?;
super::canvas::draw_settings(r, canvas, graphics)?;
object_3 = graphics;
object_2 = canvas;
let graphics_get_translate_x_value_31 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_28 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_31.wrapping_neg(), graphics_get_translate_y_value_28.wrapping_neg())?;
let graphics_get_font_value_2 = host::graphics_get_font(r, object_3)?;
object_4 = graphics_get_font_value_2;
let font_get_font_value_8 = host::font_get_font(r, 64, 0, 8)?;
object_5 = font_get_font_value_8;
host::graphics_set_font(r, object_3, object_5)?;
let canvas_state_m_35 = r.get(object_2, field::canvas::STATE_M)?.i();
number_6 = canvas_state_m_35;
let canvas_state_l_upper_40 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
number_7 = canvas_state_l_upper_40;
let canvas_state_l_upper_41 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
let canvas_state_m_36 = r.get(object_2, field::canvas::STATE_M)?.i();
host::graphics_clip_rect(r, object_3, 0, 0, canvas_state_l_upper_41, canvas_state_m_36)?;
let canvas_state_l_upper_42 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
let canvas_state_m_37 = r.get(object_2, field::canvas::STATE_M)?.i();
host::graphics_set_clip(r, object_3, 0, 0, canvas_state_l_upper_42, canvas_state_m_37)?;
number_8 = 40;
let canvas_state_aq_upper = r.get(object_2, field::canvas::STATE_AQ_UPPER)?.i();
if canvas_state_aq_upper != 5 {
let text_value_123 = r.intern("/Images/Filter5.png");
outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_123)?)) })();
(carry_0_r,) = (object_2,);
if outcome.is_err() {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
r.set(object_2, field::canvas::STATE_W_UPPER, V::R(0))?;
r.set(object_2, field::canvas::STATE_AQ_UPPER, V::I(0))?;
return Ok(());
}
let image_create_image_value = outcome.unwrap().r();
r.set(carry_0_r, field::canvas::STATE_W_UPPER, V::R(image_create_image_value))?;
r.set(object_2, field::canvas::STATE_AQ_UPPER, V::I(5))?;
}
let canvas_state_ak_upper = r.get(object_2, field::canvas::STATE_AK_UPPER)?.i();
r.set(object_2, field::canvas::STATE_AK_UPPER, V::I(canvas_state_ak_upper.wrapping_add(8)))?;
let canvas_state_ak_upper_2 = r.get(object_2, field::canvas::STATE_AK_UPPER)?.i();
if canvas_state_ak_upper_2 > 0 {
r.set(object_2, field::canvas::STATE_AK_UPPER, V::I(0))?;
}
let canvas_state_ak_upper_3 = r.get(object_2, field::canvas::STATE_AK_UPPER)?.i();
host::graphics_translate(r, object_3, 0, canvas_state_ak_upper_3)?;
let canvas_state_w_upper = r.get(object_2, field::canvas::STATE_W_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_w_upper, 0, 0, 0)?;
let canvas_state_m_38 = r.get(object_2, field::canvas::STATE_M)?.i();
if canvas_state_m_38 > 208 {
number_6 = 208;
}
let canvas_state_l_upper_43 = r.get(object_2, field::canvas::STATE_L_UPPER)?.i();
if canvas_state_l_upper_43 > 256 {
number_7 = 256;
}
let text_value_124 = r.intern("Game Over");
outcome = (|| -> JResult { Ok(V::I(host::font_string_width(r, object_4, text_value_124)?)) })();
if outcome.is_err() {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
r.set(object_2, field::canvas::STATE_W_UPPER, V::R(0))?;
r.set(object_2, field::canvas::STATE_AQ_UPPER, V::I(0))?;
return Ok(());
}
let font_string_width_value = outcome.unwrap().i();
if font_string_width_value > 60 {
let text_value_127 = r.intern("Game Over");
outcome = (|| -> JResult { Ok(V::I(host::font_string_width(r, object_4, text_value_127)?)) })();
if outcome.is_err() {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
r.set(object_2, field::canvas::STATE_W_UPPER, V::R(0))?;
r.set(object_2, field::canvas::STATE_AQ_UPPER, V::I(0))?;
return Ok(());
}
let font_string_width_value_2 = outcome.unwrap().i();
number_8 = font_string_width_value_2.wrapping_add(10);
}
let quotient_value_37 = arithmetic::divide_int(r, number_7, 2)?;
let quotient_value_38 = arithmetic::divide_int(r, number_6, 2)?;
host::graphics_translate(r, object_3, quotient_value_37.wrapping_sub(number_8), quotient_value_38.wrapping_sub(50))?;
host::graphics_set_color(r, object_3, 11184810)?;
host::graphics_fill_round_rect(r, object_3, 4, 5, number_8.wrapping_shl((1) as u32), 100, 10, 10)?;
host::graphics_set_color(r, object_3, 6710886)?;
host::graphics_fill_round_rect(r, object_3, 6, 7, (number_8.wrapping_shl((1) as u32)).wrapping_sub(2), 98, 10, 10)?;
host::graphics_set_color(r, object_3, 16777215)?;
host::graphics_translate(r, object_3, 20, 8)?;
let text_value_126 = r.intern("Game Over");
host::graphics_draw_string(r, object_3, text_value_126, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 20)?;
let canvas_key_configuration_step_4 = r.get(object_2, field::canvas::KEY_CONFIGURATION_STEP)?.i();
if canvas_key_configuration_step_4 != 0 {
host::graphics_set_color(r, object_3, 16777215)?;
} else {
host::graphics_set_color(r, object_3, 65280)?;
host::graphics_translate(r, object_3, -12, 2)?;
host::graphics_fill_round_rect(r, object_3, 0, 0, 10, 10, 10, 10)?;
host::graphics_translate(r, object_3, 12, -2)?;
}
let text_value_131 = r.intern("Continue");
host::graphics_draw_string(r, object_3, text_value_131, 0, 0, 20)?;
host::graphics_translate(r, object_3, 0, 20)?;
let canvas_key_configuration_step_5 = r.get(object_2, field::canvas::KEY_CONFIGURATION_STEP)?.i();
if canvas_key_configuration_step_5 != 1 {
host::graphics_set_color(r, object_3, 16777215)?;
} else {
host::graphics_set_color(r, object_3, 65280)?;
host::graphics_translate(r, object_3, -12, 2)?;
host::graphics_fill_round_rect(r, object_3, 0, 0, 10, 10, 10, 10)?;
host::graphics_translate(r, object_3, 12, -2)?;
}
let text_value_140 = r.intern("Quit");
host::graphics_draw_string(r, object_3, text_value_140, 0, 0, 20)?;
return Ok(());
}
    let canvas_inventory_open = r.get(canvas, field::canvas::INVENTORY_OPEN)?.i();
    if canvas_inventory_open == 0 { break 'graphics_get_clip_x; }
    } // check_canvas_inventory_page
    let canvas_inventory_page = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
    if canvas_inventory_page != 1 { break 'check_canvas_inventory_page_2; }
    } // next_step_4
    object_3 = graphics;
    object_2 = canvas;
    let canvas_state_aj = r.get(canvas, field::canvas::STATE_AJ)?.i();
    r.set(canvas, field::canvas::STATE_AJ, V::I(canvas_state_aj.wrapping_add(1)))?;
    let canvas_state_aj_2 = r.get(object_2, field::canvas::STATE_AJ)?.i();
    if canvas_state_aj_2 > 4 {
r.set(object_2, field::canvas::STATE_AJ, V::I(0))?;
}
    let graphics_get_translate_x_value_33 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_30 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_33.wrapping_neg(), graphics_get_translate_y_value_30.wrapping_neg())?;
    let canvas_get_width_value_11 = host::canvas_get_width(r, object_2)?;
    let quotient_value_16 = arithmetic::divide_int(r, canvas_get_width_value_11, 2)?;
    host::graphics_translate(r, object_3, quotient_value_16.wrapping_sub(72), 32)?;
    let canvas_get_width_value_12 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_10 = host::canvas_get_height(r, object_2)?;
    host::graphics_clip_rect(r, object_3, 0, 0, canvas_get_width_value_12, canvas_get_height_value_10)?;
    let canvas_get_width_value_13 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_11 = host::canvas_get_height(r, object_2)?;
    host::graphics_set_clip(r, object_3, 0, 0, canvas_get_width_value_13, canvas_get_height_value_11)?;
    let canvas_state_z = r.get(object_2, field::canvas::STATE_Z)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_z, 0, 0, 0)?;
    let canvas_player_7 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_bombs = r.get(canvas_player_7, field::player::HAS_BOMBS)?.i();
    if player_has_bombs != 0 {
let graphics_get_translate_x_value_36 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_33 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_36.wrapping_neg(), graphics_get_translate_y_value_33.wrapping_neg())?;
let canvas_get_width_value_18 = host::canvas_get_width(r, object_2)?;
let quotient_value_19 = arithmetic::divide_int(r, canvas_get_width_value_18, 2)?;
host::graphics_translate(r, object_3, quotient_value_19.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 30, 44)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper, 0, 0, 0)?;
}
    let canvas_state_ax = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax == 0 {
let graphics_get_translate_x_value_43 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_40 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_43.wrapping_neg(), graphics_get_translate_y_value_40.wrapping_neg())?;
let canvas_get_width_value_25 = host::canvas_get_width(r, object_2)?;
let quotient_value_24 = arithmetic::divide_int(r, canvas_get_width_value_25, 2)?;
host::graphics_translate(r, object_3, quotient_value_24.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 30, 44)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_10 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_7 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_25 = arithmetic::divide_int(r, canvas_state_aj_7.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_10, quotient_value_25.wrapping_mul(54), 0, 0)?;
}
    let canvas_player_18 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_boomerang = r.get(canvas_player_18, field::player::HAS_BOOMERANG)?.i();
    if player_has_boomerang != 0 {
let graphics_get_translate_x_value_47 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_44 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_47.wrapping_neg(), graphics_get_translate_y_value_44.wrapping_neg())?;
let canvas_get_width_value_29 = host::canvas_get_width(r, object_2)?;
let quotient_value_36 = arithmetic::divide_int(r, canvas_get_width_value_29, 2)?;
host::graphics_translate(r, object_3, quotient_value_36.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 52, 44)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_5 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_5, -16, 0, 0)?;
}
    let canvas_state_ax_4 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_4 == 1 {
let graphics_get_translate_x_value_51 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_48 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_51.wrapping_neg(), graphics_get_translate_y_value_48.wrapping_neg())?;
let canvas_get_width_value_36 = host::canvas_get_width(r, object_2)?;
let quotient_value_49 = arithmetic::divide_int(r, canvas_get_width_value_36, 2)?;
host::graphics_translate(r, object_3, quotient_value_49.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 52, 44)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_19 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_10 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_50 = arithmetic::divide_int(r, canvas_state_aj_10.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_19, quotient_value_50.wrapping_mul(54), 0, 0)?;
}
    let canvas_player_22 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_bow = r.get(canvas_player_22, field::player::HAS_BOW)?.i();
    if player_has_bow != 0 {
let graphics_get_translate_x_value_53 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_50 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_53.wrapping_neg(), graphics_get_translate_y_value_50.wrapping_neg())?;
let canvas_get_width_value_38 = host::canvas_get_width(r, object_2)?;
let quotient_value_53 = arithmetic::divide_int(r, canvas_get_width_value_38, 2)?;
host::graphics_translate(r, object_3, quotient_value_53.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 74, 44)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_6 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_6, -32, 0, 0)?;
}
    let canvas_state_ax_7 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_7 == 2 {
let graphics_get_translate_x_value_54 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_51 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_54.wrapping_neg(), graphics_get_translate_y_value_51.wrapping_neg())?;
let canvas_get_width_value_41 = host::canvas_get_width(r, object_2)?;
let quotient_value_58 = arithmetic::divide_int(r, canvas_get_width_value_41, 2)?;
host::graphics_translate(r, object_3, quotient_value_58.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 74, 44)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_26 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_13 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_59 = arithmetic::divide_int(r, canvas_state_aj_13.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_26, quotient_value_59.wrapping_mul(54), 0, 0)?;
}
    let canvas_player_42 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_candle = r.get(canvas_player_42, field::player::HAS_CANDLE)?.i();
    if player_has_candle != 0 {
let graphics_get_translate_x_value_56 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_53 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_56.wrapping_neg(), graphics_get_translate_y_value_53.wrapping_neg())?;
let canvas_get_width_value_42 = host::canvas_get_width(r, object_2)?;
let quotient_value_63 = arithmetic::divide_int(r, canvas_get_width_value_42, 2)?;
host::graphics_translate(r, object_3, quotient_value_63.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 96, 44)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_7 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_7, -48, 0, 0)?;
}
    let canvas_state_ax_8 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_8 == 3 {
let graphics_get_translate_x_value_57 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_54 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_57.wrapping_neg(), graphics_get_translate_y_value_54.wrapping_neg())?;
let canvas_get_width_value_43 = host::canvas_get_width(r, object_2)?;
let quotient_value_67 = arithmetic::divide_int(r, canvas_get_width_value_43, 2)?;
host::graphics_translate(r, object_3, quotient_value_67.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 96, 44)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_27 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_14 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_68 = arithmetic::divide_int(r, canvas_state_aj_14.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_27, quotient_value_68.wrapping_mul(54), 0, 0)?;
}
    let canvas_player_54 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_potion = r.get(canvas_player_54, field::player::HAS_POTION)?.i();
    if player_has_potion == 0 {
let graphics_get_translate_x_value_58 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_55 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_58.wrapping_neg(), graphics_get_translate_y_value_55.wrapping_neg())?;
let canvas_get_width_value_44 = host::canvas_get_width(r, object_2)?;
let quotient_value_72 = arithmetic::divide_int(r, canvas_get_width_value_44, 2)?;
host::graphics_translate(r, object_3, quotient_value_72.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 30, 66)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_8 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_8, -48, -16, 0)?;
let canvas_state_ax_9 = r.get(object_2, field::canvas::STATE_AX)?.i();
if canvas_state_ax_9 == 4 {
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_28 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_15 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_77 = arithmetic::divide_int(r, canvas_state_aj_15.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_28, quotient_value_77.wrapping_mul(54), 0, 0)?;
}
} else {
let graphics_get_translate_x_value_59 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_56 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_59.wrapping_neg(), graphics_get_translate_y_value_56.wrapping_neg())?;
let canvas_get_width_value_45 = host::canvas_get_width(r, object_2)?;
let quotient_value_73 = arithmetic::divide_int(r, canvas_get_width_value_45, 2)?;
host::graphics_translate(r, object_3, quotient_value_73.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 30, 66)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_9 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_9, -48, -32, 0)?;
let canvas_state_ax_10 = r.get(object_2, field::canvas::STATE_AX)?.i();
if canvas_state_ax_10 == 4 {
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_29 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_16 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_78 = arithmetic::divide_int(r, canvas_state_aj_16.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_29, quotient_value_78.wrapping_mul(54), 0, 0)?;
}
}
    let canvas_player_63 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_raft = r.get(canvas_player_63, field::player::HAS_RAFT)?.i();
    if player_has_raft != 0 {
let graphics_get_translate_x_value_61 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_58 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_61.wrapping_neg(), graphics_get_translate_y_value_58.wrapping_neg())?;
let canvas_get_width_value_49 = host::canvas_get_width(r, object_2)?;
let quotient_value_82 = arithmetic::divide_int(r, canvas_get_width_value_49, 2)?;
host::graphics_translate(r, object_3, quotient_value_82.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 54, 66)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_10 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_10, -64, 0, 0)?;
}
    let canvas_state_ax_11 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_11 == 5 {
let graphics_get_translate_x_value_62 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_59 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_62.wrapping_neg(), graphics_get_translate_y_value_59.wrapping_neg())?;
let canvas_get_width_value_50 = host::canvas_get_width(r, object_2)?;
let quotient_value_86 = arithmetic::divide_int(r, canvas_get_width_value_50, 2)?;
host::graphics_translate(r, object_3, quotient_value_86.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 54, 66)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_30 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_17 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_87 = arithmetic::divide_int(r, canvas_state_aj_17.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_30, quotient_value_87.wrapping_mul(54), 0, 0)?;
}
    let canvas_player_72 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_mirror_shield = r.get(canvas_player_72, field::player::HAS_MIRROR_SHIELD)?.i();
    if player_has_mirror_shield != 0 {
let graphics_get_translate_x_value_63 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_60 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_63.wrapping_neg(), graphics_get_translate_y_value_60.wrapping_neg())?;
let canvas_get_width_value_51 = host::canvas_get_width(r, object_2)?;
let quotient_value_91 = arithmetic::divide_int(r, canvas_get_width_value_51, 2)?;
host::graphics_translate(r, object_3, quotient_value_91.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 74, 66)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_11 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_11, -64, -16, 0)?;
}
    let canvas_state_ax_12 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_12 == 6 {
let graphics_get_translate_x_value_64 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_61 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_64.wrapping_neg(), graphics_get_translate_y_value_61.wrapping_neg())?;
let canvas_get_width_value_53 = host::canvas_get_width(r, object_2)?;
let quotient_value_98 = arithmetic::divide_int(r, canvas_get_width_value_53, 2)?;
host::graphics_translate(r, object_3, quotient_value_98.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 74, 66)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_31 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_18 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_99 = arithmetic::divide_int(r, canvas_state_aj_18.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_31, quotient_value_99.wrapping_mul(54), 0, 0)?;
}
    let canvas_player_84 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_shovel = r.get(canvas_player_84, field::player::HAS_SHOVEL)?.i();
    if player_has_shovel != 0 {
let graphics_get_translate_x_value_65 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_62 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_65.wrapping_neg(), graphics_get_translate_y_value_62.wrapping_neg())?;
let canvas_get_width_value_55 = host::canvas_get_width(r, object_2)?;
let quotient_value_102 = arithmetic::divide_int(r, canvas_get_width_value_55, 2)?;
host::graphics_translate(r, object_3, quotient_value_102.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 96, 66)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_12 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_12, -64, -32, 0)?;
}
    let canvas_state_ax_13 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_13 == 7 {
let graphics_get_translate_x_value_66 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_63 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_66.wrapping_neg(), graphics_get_translate_y_value_63.wrapping_neg())?;
let canvas_get_width_value_58 = host::canvas_get_width(r, object_2)?;
let quotient_value_105 = arithmetic::divide_int(r, canvas_get_width_value_58, 2)?;
host::graphics_translate(r, object_3, quotient_value_105.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 96, 66)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_32 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_19 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_106 = arithmetic::divide_int(r, canvas_state_aj_19.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_32, quotient_value_106.wrapping_mul(54), 0, 0)?;
}
    let canvas_state_ax_14 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_14 == 8 {
let graphics_get_translate_x_value_67 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_64 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_67.wrapping_neg(), graphics_get_translate_y_value_64.wrapping_neg())?;
let canvas_get_width_value_59 = host::canvas_get_width(r, object_2)?;
let quotient_value_109 = arithmetic::divide_int(r, canvas_get_width_value_59, 2)?;
host::graphics_translate(r, object_3, quotient_value_109.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 23, 89)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 54, 28)?;
host::graphics_set_clip(r, object_3, 0, 0, 54, 28)?;
let canvas_state_aa_33 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_20 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_110 = arithmetic::divide_int(r, canvas_state_aj_20.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_33, quotient_value_110.wrapping_mul(54), -30, 0)?;
}
    let canvas_state_ax_15 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_15 == 9 {
let graphics_get_translate_x_value_69 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_66 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_69.wrapping_neg(), graphics_get_translate_y_value_66.wrapping_neg())?;
let canvas_get_width_value_63 = host::canvas_get_width(r, object_2)?;
let quotient_value_112 = arithmetic::divide_int(r, canvas_get_width_value_63, 2)?;
host::graphics_translate(r, object_3, quotient_value_112.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 78, 89)?;
host::graphics_translate(r, object_3, -5, -5)?;
host::graphics_clip_rect(r, object_3, 0, 0, 54, 28)?;
host::graphics_set_clip(r, object_3, 0, 0, 54, 28)?;
let canvas_state_aa_34 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_22 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_113 = arithmetic::divide_int(r, canvas_state_aj_22.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_34, quotient_value_113.wrapping_mul(54), -30, 0)?;
}
    let graphics_get_font_value_3 = host::graphics_get_font(r, object_3)?;
    let font_get_font_value_9 = host::font_get_font(r, 64, 0, 8)?;
    object_5 = font_get_font_value_9;
    host::graphics_set_font(r, object_3, object_5)?;
    let font_get_height_value_6 = host::font_get_height(r, object_5)?;
    host::graphics_set_color(r, object_3, 3355443)?;
    let graphics_get_translate_x_value_68 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_65 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_68.wrapping_neg(), graphics_get_translate_y_value_65.wrapping_neg())?;
    let canvas_get_width_value_60 = host::canvas_get_width(r, object_2)?;
    let quotient_value_111 = arithmetic::divide_int(r, canvas_get_width_value_60, 2)?;
    host::graphics_translate(r, object_3, quotient_value_111.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, 32, 22)?;
    let canvas_get_width_value_61 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_30 = host::canvas_get_height(r, object_2)?;
    host::graphics_clip_rect(r, object_3, 0, 0, canvas_get_width_value_61, canvas_get_height_value_30)?;
    let canvas_get_width_value_62 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_31 = host::canvas_get_height(r, object_2)?;
    host::graphics_set_clip(r, object_3, 0, 0, canvas_get_width_value_62, canvas_get_height_value_31)?;
    let canvas_state_ax_16 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_16 == 0 {
let canvas_player_88 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_has_bombs_2 = r.get(canvas_player_88, field::player::HAS_BOMBS)?.i();
if player_has_bombs_2 != 0 {
let text_value_150 = r.intern("Bombs");
host::graphics_draw_string(r, object_3, text_value_150, 0, 0, 20)?;
}
}
    let canvas_state_ax_17 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_17 == 1 {
let canvas_player_90 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_has_boomerang_2 = r.get(canvas_player_90, field::player::HAS_BOOMERANG)?.i();
if player_has_boomerang_2 != 0 {
let text_value_151 = r.intern("Boomerang");
host::graphics_draw_string(r, object_3, text_value_151, 0, 0, 20)?;
}
}
    let canvas_state_ax_18 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_18 == 2 {
let canvas_player_95 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_has_bow_2 = r.get(canvas_player_95, field::player::HAS_BOW)?.i();
if player_has_bow_2 != 0 {
let text_value_152 = r.intern("Bow");
host::graphics_draw_string(r, object_3, text_value_152, 0, 0, 20)?;
}
}
    let canvas_state_ax_19 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_19 == 3 {
let canvas_player_102 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_has_candle_2 = r.get(canvas_player_102, field::player::HAS_CANDLE)?.i();
if player_has_candle_2 != 0 {
let text_value_153 = r.intern("Candle");
host::graphics_draw_string(r, object_3, text_value_153, 0, 0, 20)?;
}
}
    let canvas_state_ax_20 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_20 == 4 {
let canvas_player_112 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_has_potion_2 = r.get(canvas_player_112, field::player::HAS_POTION)?.i();
if player_has_potion_2 == 0 {
let text_value_154 = r.intern("Empty Bottle");
host::graphics_draw_string(r, object_3, text_value_154, 0, 0, 20)?;
} else {
let text_value_155 = r.intern("Red Potion");
host::graphics_draw_string(r, object_3, text_value_155, 0, 0, 20)?;
}
}
    let canvas_state_ax_21 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_21 == 5 {
let canvas_player_119 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_has_raft_2 = r.get(canvas_player_119, field::player::HAS_RAFT)?.i();
if player_has_raft_2 != 0 {
let text_value_156 = r.intern("HookShot");
host::graphics_draw_string(r, object_3, text_value_156, 0, 0, 20)?;
}
}
    let canvas_state_ax_22 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_22 == 6 {
let canvas_player_122 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_has_mirror_shield_2 = r.get(canvas_player_122, field::player::HAS_MIRROR_SHIELD)?.i();
if player_has_mirror_shield_2 != 0 {
let text_value_157 = r.intern("Shield");
host::graphics_draw_string(r, object_3, text_value_157, 0, 0, 20)?;
}
}
    let canvas_state_ax_23 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_23 == 7 {
let canvas_player_123 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_has_shovel_2 = r.get(canvas_player_123, field::player::HAS_SHOVEL)?.i();
if player_has_shovel_2 != 0 {
let text_value_161 = r.intern("Shovel");
host::graphics_draw_string(r, object_3, text_value_161, 0, 0, 20)?;
}
}
    let canvas_state_ax_24 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_24 == 8 {
let text_value_160 = r.intern("Save");
host::graphics_draw_string(r, object_3, text_value_160, 0, 0, 20)?;
}
    let canvas_state_ax_25 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_25 == 9 {
let text_value_162 = r.intern("Exit");
host::graphics_draw_string(r, object_3, text_value_162, 0, 0, 20)?;
}
    let canvas_state_ax_26 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_26 == 10 {
let text_value_163 = r.intern("Maps");
host::graphics_draw_string(r, object_3, text_value_163, 0, 0, 20)?;
}
    let canvas_state_ax_27 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_27 == 11 {
let text_value_164 = r.intern("Status");
host::graphics_draw_string(r, object_3, text_value_164, 0, 0, 20)?;
}
    let graphics_get_translate_x_value_80 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_77 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_80.wrapping_neg(), graphics_get_translate_y_value_77.wrapping_neg())?;
    let canvas_get_width_value_64 = host::canvas_get_width(r, object_2)?;
    let quotient_value_130 = arithmetic::divide_int(r, canvas_get_width_value_64, 2)?;
    host::graphics_translate(r, object_3, quotient_value_130.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, -8, 48)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 24, 24)?;
    host::graphics_set_clip(r, object_3, 0, 0, 24, 24)?;
    let canvas_state_ab_6 = r.get(object_2, field::canvas::STATE_AB)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_ab_6, 0, 0, 0)?;
    let canvas_state_ax_28 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_28 == 10 {
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_35 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_41 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_132 = arithmetic::divide_int(r, canvas_state_aj_41.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_35, quotient_value_132.wrapping_mul(54), 0, 0)?;
}
    let graphics_get_translate_x_value_81 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_78 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_81.wrapping_neg(), graphics_get_translate_y_value_78.wrapping_neg())?;
    let canvas_get_width_value_65 = host::canvas_get_width(r, object_2)?;
    let quotient_value_131 = arithmetic::divide_int(r, canvas_get_width_value_65, 2)?;
    host::graphics_translate(r, object_3, quotient_value_131.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, 132, 48)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 24, 24)?;
    host::graphics_set_clip(r, object_3, 0, 0, 24, 24)?;
    let canvas_state_ab_7 = r.get(object_2, field::canvas::STATE_AB)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_ab_7, -24, 0, 0)?;
    let canvas_state_ax_29 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_29 == 11 {
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_36 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_42 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_133 = arithmetic::divide_int(r, canvas_state_aj_42.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_36, quotient_value_133.wrapping_mul(54), 0, 0)?;
}

    break 'check_canvas_redraw_world;
    } // check_canvas_inventory_page_2
    let canvas_inventory_page_2 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
    if canvas_inventory_page_2 != 2 { break 'check_canvas_inventory_page_3; }
    } // next_step_5
    object_3 = graphics;
    object_2 = canvas;
    let canvas_state_aj_3 = r.get(canvas, field::canvas::STATE_AJ)?.i();
    r.set(canvas, field::canvas::STATE_AJ, V::I(canvas_state_aj_3.wrapping_add(1)))?;
    let canvas_state_aj_4 = r.get(object_2, field::canvas::STATE_AJ)?.i();
    if canvas_state_aj_4 > 4 {
r.set(object_2, field::canvas::STATE_AJ, V::I(0))?;
}
    let graphics_get_translate_x_value_34 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_31 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_34.wrapping_neg(), graphics_get_translate_y_value_31.wrapping_neg())?;
    let canvas_get_width_value_14 = host::canvas_get_width(r, object_2)?;
    let quotient_value_17 = arithmetic::divide_int(r, canvas_get_width_value_14, 2)?;
    host::graphics_translate(r, object_3, quotient_value_17.wrapping_sub(72), 32)?;
    let canvas_get_width_value_15 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_12 = host::canvas_get_height(r, object_2)?;
    host::graphics_clip_rect(r, object_3, 0, 0, canvas_get_width_value_15, canvas_get_height_value_12)?;
    let canvas_get_width_value_16 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_13 = host::canvas_get_height(r, object_2)?;
    host::graphics_set_clip(r, object_3, 0, 0, canvas_get_width_value_16, canvas_get_height_value_13)?;
    let canvas_state_z_2 = r.get(object_2, field::canvas::STATE_Z)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_z_2, 0, 0, 0)?;
    let graphics_get_translate_x_value_35 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_32 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_35.wrapping_neg(), graphics_get_translate_y_value_32.wrapping_neg())?;
    let canvas_get_width_value_17 = host::canvas_get_width(r, object_2)?;
    let quotient_value_18 = arithmetic::divide_int(r, canvas_get_width_value_17, 2)?;
    host::graphics_translate(r, object_3, quotient_value_18.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, 21, 37)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 24, 24)?;
    host::graphics_set_clip(r, object_3, 0, 0, 24, 24)?;
    let canvas_player_10 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_sprite = r.get(canvas_player_10, field::player::SPRITE)?.r();
    host::graphics_draw_image(r, object_3, player_sprite, -24, -48, 0)?;
    let canvas_player_11 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_has_sword = r.get(canvas_player_11, field::player::HAS_SWORD)?.i();
    if player_has_sword != 0 {
let graphics_get_translate_x_value_42 = host::graphics_get_translate_x(r, object_3)?;
let graphics_get_translate_y_value_39 = host::graphics_get_translate_y(r, object_3)?;
host::graphics_translate(r, object_3, graphics_get_translate_x_value_42.wrapping_neg(), graphics_get_translate_y_value_39.wrapping_neg())?;
let canvas_get_width_value_24 = host::canvas_get_width(r, object_2)?;
let quotient_value_23 = arithmetic::divide_int(r, canvas_get_width_value_24, 2)?;
host::graphics_translate(r, object_3, quotient_value_23.wrapping_sub(72), 32)?;
host::graphics_translate(r, object_3, 48, 43)?;
host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
let canvas_state_u_upper_4 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
let canvas_player_17 = r.get(object_2, field::canvas::PLAYER)?.r();
let player_sword_level = r.get(canvas_player_17, field::player::SWORD_LEVEL)?.i();
host::graphics_draw_image(r, object_3, canvas_state_u_upper_4, (-16_i32).wrapping_mul(player_sword_level), -16, 0)?;
}
    let graphics_get_translate_x_value_39 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_36 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_39.wrapping_neg(), graphics_get_translate_y_value_36.wrapping_neg())?;
    let canvas_get_width_value_21 = host::canvas_get_width(r, object_2)?;
    let quotient_value_20 = arithmetic::divide_int(r, canvas_get_width_value_21, 2)?;
    host::graphics_translate(r, object_3, quotient_value_20.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, 48, 65)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
    host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
    let canvas_state_u_upper_2 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
    let canvas_player_15 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_tunic_color = r.get(canvas_player_15, field::player::TUNIC_COLOR)?.i();
    host::graphics_draw_image(r, object_3, canvas_state_u_upper_2, (-16_i32).wrapping_mul(player_tunic_color), -32, 0)?;
    let graphics_get_translate_x_value_40 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_37 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_40.wrapping_neg(), graphics_get_translate_y_value_37.wrapping_neg())?;
    let canvas_get_width_value_22 = host::canvas_get_width(r, object_2)?;
    let quotient_value_21 = arithmetic::divide_int(r, canvas_get_width_value_22, 2)?;
    host::graphics_translate(r, object_3, quotient_value_21.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, 48, 87)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 16, 16)?;
    host::graphics_set_clip(r, object_3, 0, 0, 16, 16)?;
    let canvas_state_u_upper_3 = r.get(object_2, field::canvas::STATE_U_UPPER)?.r();
    let canvas_player_16 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_max_health = r.get(canvas_player_16, field::player::MAX_HEALTH)?.i();
    let remainder_value = arithmetic::remainder_int(r, player_max_health, 4)?;
    host::graphics_draw_image(r, object_3, canvas_state_u_upper_3, (-16_i32).wrapping_mul(remainder_value), -48, 0)?;
    let graphics_get_translate_x_value_41 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_38 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_41.wrapping_neg(), graphics_get_translate_y_value_38.wrapping_neg())?;
    let canvas_get_width_value_23 = host::canvas_get_width(r, object_2)?;
    let quotient_value_22 = arithmetic::divide_int(r, canvas_get_width_value_23, 2)?;
    host::graphics_translate(r, object_3, quotient_value_22.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, -8, 48)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 24, 24)?;
    host::graphics_set_clip(r, object_3, 0, 0, 24, 24)?;
    let canvas_state_ab_2 = r.get(object_2, field::canvas::STATE_AB)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_ab_2, 0, -24, 0)?;
    let canvas_state_ax_2 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_2 == 0 {
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_15 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_8 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_35 = arithmetic::divide_int(r, canvas_state_aj_8.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_15, quotient_value_35.wrapping_mul(54), 0, 0)?;
}
    let graphics_get_translate_x_value_46 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_43 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_46.wrapping_neg(), graphics_get_translate_y_value_43.wrapping_neg())?;
    let canvas_get_width_value_28 = host::canvas_get_width(r, object_2)?;
    let quotient_value_34 = arithmetic::divide_int(r, canvas_get_width_value_28, 2)?;
    host::graphics_translate(r, object_3, quotient_value_34.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, 132, 48)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 24, 24)?;
    host::graphics_set_clip(r, object_3, 0, 0, 24, 24)?;
    let canvas_state_ab_3 = r.get(object_2, field::canvas::STATE_AB)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_ab_3, -24, -24, 0)?;
    let canvas_state_ax_3 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_3 == 1 {
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_18 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_9 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_48 = arithmetic::divide_int(r, canvas_state_aj_9.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_18, quotient_value_48.wrapping_mul(54), 0, 0)?;
}
    let graphics_get_translate_x_value_50 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_47 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_50.wrapping_neg(), graphics_get_translate_y_value_47.wrapping_neg())?;
    let canvas_get_width_value_33 = host::canvas_get_width(r, object_2)?;
    let quotient_value_47 = arithmetic::divide_int(r, canvas_get_width_value_33, 2)?;
    host::graphics_translate(r, object_3, quotient_value_47.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, 65, 41)?;
    let canvas_get_width_value_34 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_20 = host::canvas_get_height(r, object_2)?;
    host::graphics_clip_rect(r, object_3, 0, 0, canvas_get_width_value_34, canvas_get_height_value_20)?;
    let canvas_get_width_value_35 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_21 = host::canvas_get_height(r, object_2)?;
    host::graphics_set_clip(r, object_3, 0, 0, canvas_get_width_value_35, canvas_get_height_value_21)?;
    let canvas_player_21 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_triforce_pieces = r.get(canvas_player_21, field::player::TRIFORCE_PIECES)?.r();
    let element_value = r.array_get(player_triforce_pieces, 0)?.i();
    if element_value != 0 {
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 32, 10, 25, 25, 39, 25)?;
}
    let canvas_player_27 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_triforce_pieces_2 = r.get(canvas_player_27, field::player::TRIFORCE_PIECES)?.r();
    let element_value_2 = r.array_get(player_triforce_pieces_2, 1)?.i();
    if element_value_2 != 0 {
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 25, 25, 18, 39, 32, 39)?;
}
    let canvas_player_41 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_triforce_pieces_3 = r.get(canvas_player_41, field::player::TRIFORCE_PIECES)?.r();
    let element_value_3 = r.array_get(player_triforce_pieces_3, 2)?.i();
    if element_value_3 != 0 {
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 39, 25, 32, 39, 46, 39)?;
}
    let canvas_player_48 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_triforce_pieces_4 = r.get(canvas_player_48, field::player::TRIFORCE_PIECES)?.r();
    let element_value_4 = r.array_get(player_triforce_pieces_4, 3)?.i();
    if element_value_4 != 0 {
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 18, 39, 11, 53, 25, 53)?;
}
    let canvas_player_53 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_triforce_pieces_5 = r.get(canvas_player_53, field::player::TRIFORCE_PIECES)?.r();
    let element_value_5 = r.array_get(player_triforce_pieces_5, 4)?.i();
    if element_value_5 != 0 {
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 32, 39, 25, 53, 39, 53)?;
}
    let canvas_player_58 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_triforce_pieces_6 = r.get(canvas_player_58, field::player::TRIFORCE_PIECES)?.r();
    let element_value_6 = r.array_get(player_triforce_pieces_6, 5)?.i();
    if element_value_6 != 0 {
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 46, 39, 39, 53, 53, 53)?;
}
    let canvas_player_62 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_triforce_pieces_7 = r.get(canvas_player_62, field::player::TRIFORCE_PIECES)?.r();
    let element_value_7 = r.array_get(player_triforce_pieces_7, 6)?.i();
    if element_value_7 != 0 {
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 25, 25, 39, 25, 32, 39)?;
}
    let canvas_player_68 = r.get(object_2, field::canvas::PLAYER)?.r();
    let player_triforce_pieces_8 = r.get(canvas_player_68, field::player::TRIFORCE_PIECES)?.r();
    let element_value_8 = r.array_get(player_triforce_pieces_8, 7)?.i();
    if element_value_8 != 0 {
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 18, 39, 32, 39, 25, 53)?;
host::graphics_set_color(r, object_3, 16316416)?;
host::graphics_fill_triangle(r, object_3, 32, 39, 46, 39, 39, 53)?;
}

    break 'check_canvas_redraw_world;
    } // check_canvas_inventory_page_3
    let canvas_inventory_page_3 = r.get(canvas, field::canvas::INVENTORY_PAGE)?.i();
    if canvas_inventory_page_3 != 3 { break 'check_canvas_redraw_world; }
    } // next_step_6
    object_3 = graphics;
    object_2 = canvas;
    let canvas_state_aj_5 = r.get(canvas, field::canvas::STATE_AJ)?.i();
    r.set(canvas, field::canvas::STATE_AJ, V::I(canvas_state_aj_5.wrapping_add(1)))?;
    let canvas_state_aj_6 = r.get(object_2, field::canvas::STATE_AJ)?.i();
    if canvas_state_aj_6 > 4 {
r.set(object_2, field::canvas::STATE_AJ, V::I(0))?;
}
    let graphics_get_translate_x_value_38 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_35 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_38.wrapping_neg(), graphics_get_translate_y_value_35.wrapping_neg())?;
    host::graphics_translate(r, object_3, 0, 32)?;
    let canvas_get_width_value_19 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_14 = host::canvas_get_height(r, object_2)?;
    host::graphics_clip_rect(r, object_3, 0, 0, canvas_get_width_value_19, canvas_get_height_value_14)?;
    let canvas_get_width_value_20 = host::canvas_get_width(r, object_2)?;
    let canvas_get_height_value_15 = host::canvas_get_height(r, object_2)?;
    host::graphics_set_clip(r, object_3, 0, 0, canvas_get_width_value_20, canvas_get_height_value_15)?;
    let canvas_state_z_3 = r.get(object_2, field::canvas::STATE_Z)?.r();
    if canvas_state_z_3 == 0 { break 'graphics_set_color_3; }
    } // next_step_7
    number_4 = 0;
    number_5 = 0;
    let canvas_get_width_value_27 = host::canvas_get_width(r, object_2)?;
    number_6 = (256_i32).wrapping_sub(canvas_get_width_value_27);
    let canvas_get_height_value_17 = host::canvas_get_height(r, object_2)?;
    number_7 = (176_i32).wrapping_sub(canvas_get_height_value_17.wrapping_sub(32));
    let canvas_zone = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone == 0 { break 'check_canvas_map; }
    } // check_canvas_zone
    let canvas_zone_3 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_3 != 1 { break 'check_canvas_zone_2; }
    } // check_canvas_map
    let canvas_map = r.get(object_2, field::canvas::MAP)?.i();
    let remainder_value_4 = arithmetic::remainder_int(r, canvas_map.wrapping_sub(1), 16)?;
    number_4 = remainder_value_4.wrapping_shl((4) as u32);
    let canvas_map_2 = r.get(object_2, field::canvas::MAP)?.i();
    let quotient_value_44 = arithmetic::divide_int(r, canvas_map_2.wrapping_sub(1), 16)?;
    number_5 = quotient_value_44.wrapping_mul(22);
    let canvas_player_19 = r.get(object_2, field::canvas::PLAYER)?.r();
    let drawable_x_3 = r.get(canvas_player_19, field::drawable::X)?.i();
    let quotient_value_45 = arithmetic::divide_int(r, drawable_x_3, 16)?;
    number_4 = number_4.wrapping_add(quotient_value_45);
    let canvas_player_20 = r.get(object_2, field::canvas::PLAYER)?.r();
    let drawable_y_3 = r.get(canvas_player_20, field::drawable::Y)?.i();
    let quotient_value_46 = arithmetic::divide_int(r, drawable_y_3, 8)?;
    number_5 = number_5.wrapping_add(quotient_value_46);
    } // check_canvas_zone_2
    let canvas_zone_4 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_4 == 3 {
let canvas_map_7 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_6 = arithmetic::remainder_int(r, canvas_map_7.wrapping_sub(1), 6)?;
number_4 = remainder_value_6.wrapping_mul(42);
let canvas_map_8 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_55 = arithmetic::divide_int(r, canvas_map_8.wrapping_sub(1), 6)?;
number_5 = quotient_value_55.wrapping_mul(29);
let canvas_player_39 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_7 = r.get(canvas_player_39, field::drawable::X)?.i();
let quotient_value_56 = arithmetic::divide_int(r, drawable_x_7, 6)?;
number_4 = number_4.wrapping_add(quotient_value_56);
let canvas_player_40 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_7 = r.get(canvas_player_40, field::drawable::Y)?.i();
let quotient_value_57 = arithmetic::divide_int(r, drawable_y_7, 6)?;
number_5 = number_5.wrapping_add(quotient_value_57);
}
    let canvas_zone_9 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_9 == 4 {
let canvas_map_9 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_7 = arithmetic::remainder_int(r, canvas_map_9.wrapping_sub(1), 4)?;
number_4 = remainder_value_7.wrapping_shl((5) as u32);
let canvas_map_10 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_60 = arithmetic::divide_int(r, canvas_map_10.wrapping_sub(1), 4)?;
number_5 = quotient_value_60.wrapping_mul(22);
let canvas_player_46 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_9 = r.get(canvas_player_46, field::drawable::X)?.i();
let quotient_value_61 = arithmetic::divide_int(r, drawable_x_9, 8)?;
number_4 = number_4.wrapping_add(quotient_value_61);
let canvas_player_47 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_9 = r.get(canvas_player_47, field::drawable::Y)?.i();
let quotient_value_62 = arithmetic::divide_int(r, drawable_y_9, 8)?;
number_5 = number_5.wrapping_add(quotient_value_62);
number_4 = number_4.wrapping_add(64);
}
    let canvas_zone_12 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_12 == 5 {
let canvas_map_11 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_8 = arithmetic::remainder_int(r, canvas_map_11.wrapping_sub(1), 5)?;
number_4 = remainder_value_8.wrapping_mul(42);
let canvas_map_12 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_64 = arithmetic::divide_int(r, canvas_map_12.wrapping_sub(1), 5)?;
number_5 = quotient_value_64.wrapping_mul(29);
let canvas_player_51 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_10 = r.get(canvas_player_51, field::drawable::X)?.i();
let quotient_value_65 = arithmetic::divide_int(r, drawable_x_10, 8)?;
number_4 = number_4.wrapping_add(quotient_value_65);
let canvas_player_52 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_10 = r.get(canvas_player_52, field::drawable::Y)?.i();
let quotient_value_66 = arithmetic::divide_int(r, drawable_y_10, 8)?;
number_5 = number_5.wrapping_add(quotient_value_66);
number_4 = number_4.wrapping_add(22);
}
    let canvas_zone_14 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_14 == 6 {
let canvas_map_13 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_12 = arithmetic::remainder_int(r, canvas_map_13.wrapping_sub(1), 4)?;
number_4 = remainder_value_12.wrapping_shl((5) as u32);
let canvas_map_14 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_69 = arithmetic::divide_int(r, canvas_map_14.wrapping_sub(1), 4)?;
number_5 = quotient_value_69.wrapping_mul(22);
let canvas_player_56 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_11 = r.get(canvas_player_56, field::drawable::X)?.i();
let quotient_value_70 = arithmetic::divide_int(r, drawable_x_11, 8)?;
number_4 = number_4.wrapping_add(quotient_value_70);
let canvas_player_57 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_11 = r.get(canvas_player_57, field::drawable::Y)?.i();
let quotient_value_71 = arithmetic::divide_int(r, drawable_y_11, 8)?;
number_5 = number_5.wrapping_add(quotient_value_71);
number_4 = number_4.wrapping_add(64);
}
    let canvas_zone_16 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_16 == 7 {
let canvas_map_15 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_15 = arithmetic::remainder_int(r, canvas_map_15.wrapping_sub(1), 4)?;
number_4 = remainder_value_15.wrapping_shl((5) as u32);
let canvas_map_16 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_74 = arithmetic::divide_int(r, canvas_map_16.wrapping_sub(1), 4)?;
number_5 = quotient_value_74.wrapping_mul(22);
let canvas_player_60 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_12 = r.get(canvas_player_60, field::drawable::X)?.i();
let quotient_value_75 = arithmetic::divide_int(r, drawable_x_12, 8)?;
number_4 = number_4.wrapping_add(quotient_value_75);
let canvas_player_61 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_12 = r.get(canvas_player_61, field::drawable::Y)?.i();
let quotient_value_76 = arithmetic::divide_int(r, drawable_y_12, 8)?;
number_5 = number_5.wrapping_add(quotient_value_76);
number_4 = number_4.wrapping_add(64);
}
    let canvas_zone_17 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_17 == 8 {
let canvas_map_17 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_16 = arithmetic::remainder_int(r, canvas_map_17.wrapping_sub(1), 6)?;
number_4 = remainder_value_16.wrapping_shl((5) as u32);
let canvas_map_18 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_79 = arithmetic::divide_int(r, canvas_map_18.wrapping_sub(1), 6)?;
number_5 = quotient_value_79.wrapping_mul(22);
let canvas_player_66 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_13 = r.get(canvas_player_66, field::drawable::X)?.i();
let quotient_value_80 = arithmetic::divide_int(r, drawable_x_13, 6)?;
number_4 = number_4.wrapping_add(quotient_value_80);
let canvas_player_67 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_13 = r.get(canvas_player_67, field::drawable::Y)?.i();
let quotient_value_81 = arithmetic::divide_int(r, drawable_y_13, 8)?;
number_5 = number_5.wrapping_add(quotient_value_81);
number_4 = number_4.wrapping_add(32);
}
    let canvas_zone_18 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_18 == 9 {
let canvas_map_19 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_17 = arithmetic::remainder_int(r, canvas_map_19.wrapping_sub(1), 6)?;
number_4 = remainder_value_17.wrapping_shl((5) as u32);
let canvas_map_20 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_83 = arithmetic::divide_int(r, canvas_map_20.wrapping_sub(1), 6)?;
number_5 = quotient_value_83.wrapping_mul(22);
let canvas_player_70 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_14 = r.get(canvas_player_70, field::drawable::X)?.i();
let quotient_value_84 = arithmetic::divide_int(r, drawable_x_14, 6)?;
number_4 = number_4.wrapping_add(quotient_value_84);
let canvas_player_71 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_14 = r.get(canvas_player_71, field::drawable::Y)?.i();
let quotient_value_85 = arithmetic::divide_int(r, drawable_y_14, 8)?;
number_5 = number_5.wrapping_add(quotient_value_85);
number_4 = number_4.wrapping_add(32);
}
    let canvas_zone_19 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_19 == 10 {
let canvas_map_21 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_18 = arithmetic::remainder_int(r, canvas_map_21.wrapping_sub(1), 5)?;
number_4 = remainder_value_18.wrapping_shl((5) as u32);
let canvas_map_22 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_88 = arithmetic::divide_int(r, canvas_map_22.wrapping_sub(1), 5)?;
number_5 = quotient_value_88.wrapping_mul(22);
let canvas_player_74 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_15 = r.get(canvas_player_74, field::drawable::X)?.i();
let quotient_value_89 = arithmetic::divide_int(r, drawable_x_15, 5)?;
number_4 = number_4.wrapping_add(quotient_value_89);
let canvas_player_75 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_15 = r.get(canvas_player_75, field::drawable::Y)?.i();
let quotient_value_90 = arithmetic::divide_int(r, drawable_y_15, 8)?;
number_5 = number_5.wrapping_add(quotient_value_90);
number_4 = number_4.wrapping_add(48);
}
    let canvas_zone_21 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_21 == 11 {
let canvas_map_24 = r.get(object_2, field::canvas::MAP)?.i();
let remainder_value_20 = arithmetic::remainder_int(r, canvas_map_24.wrapping_sub(1), 8)?;
number_4 = remainder_value_20.wrapping_shl((5) as u32);
let canvas_map_25 = r.get(object_2, field::canvas::MAP)?.i();
let quotient_value_95 = arithmetic::divide_int(r, canvas_map_25.wrapping_sub(1), 8)?;
number_5 = quotient_value_95.wrapping_mul(22);
let canvas_player_82 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_x_17 = r.get(canvas_player_82, field::drawable::X)?.i();
let quotient_value_96 = arithmetic::divide_int(r, drawable_x_17, 8)?;
number_4 = number_4.wrapping_add(quotient_value_96);
let canvas_player_83 = r.get(object_2, field::canvas::PLAYER)?.r();
let drawable_y_17 = r.get(canvas_player_83, field::drawable::Y)?.i();
let quotient_value_97 = arithmetic::divide_int(r, drawable_y_17, 8)?;
number_5 = number_5.wrapping_add(quotient_value_97);
}
    let canvas_get_width_value_52 = host::canvas_get_width(r, object_2)?;
    let quotient_value_94 = arithmetic::divide_int(r, canvas_get_width_value_52, 2)?;
    if number_4 > quotient_value_94 {
let canvas_get_width_value_54 = host::canvas_get_width(r, object_2)?;
let quotient_value_101 = arithmetic::divide_int(r, canvas_get_width_value_54, 2)?;
if number_4.wrapping_sub(quotient_value_101) <= number_6 {
let canvas_get_width_value_56 = host::canvas_get_width(r, object_2)?;
let quotient_value_103 = arithmetic::divide_int(r, canvas_get_width_value_56, 2)?;
host::graphics_translate(r, object_3, (number_4.wrapping_sub(quotient_value_103)).wrapping_neg(), 0)?;
let canvas_get_width_value_57 = host::canvas_get_width(r, object_2)?;
let quotient_value_104 = arithmetic::divide_int(r, canvas_get_width_value_57, 2)?;
number_4 = number_4.wrapping_sub(number_4.wrapping_sub(quotient_value_104));
} else {
host::graphics_translate(r, object_3, number_6.wrapping_neg(), 0)?;
number_4 = number_4.wrapping_sub(number_6);
}
}
    let canvas_get_height_value_27 = host::canvas_get_height(r, object_2)?;
    let quotient_value_100 = arithmetic::divide_int(r, canvas_get_height_value_27, 2)?;
    if !(number_5.wrapping_add(32) <= quotient_value_100) {

if number_5 <= number_7 {
let canvas_get_height_value_28 = host::canvas_get_height(r, object_2)?;
let quotient_value_107 = arithmetic::divide_int(r, canvas_get_height_value_28, 2)?;
host::graphics_translate(r, object_3, 0, (number_5.wrapping_sub(quotient_value_107)).wrapping_neg())?;
let canvas_get_height_value_29 = host::canvas_get_height(r, object_2)?;
let quotient_value_108 = arithmetic::divide_int(r, canvas_get_height_value_29, 2)?;
number_5 = number_5.wrapping_sub(number_5.wrapping_sub(quotient_value_108));
} else {
host::graphics_translate(r, object_3, 0, number_7.wrapping_neg())?;
number_5 = number_5.wrapping_sub(number_7);
}
}
    let canvas_state_z_5 = r.get(object_2, field::canvas::STATE_Z)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_z_5, 0, 0, 0)?;
    object_6 = object_3;
    object_8 = object_2;
    let canvas_zone_23 = r.get(object_2, field::canvas::ZONE)?.i();
    if canvas_zone_23 == 0 { break 'check_canvas_player_4; }
    } // check_canvas_zone_3
    let canvas_zone_24 = r.get(object_8, field::canvas::ZONE)?.i();
    if canvas_zone_24 != 1 { break 'check_canvas_zone_4; }
    } // check_canvas_player_4
    let canvas_player_85 = r.get(object_8, field::canvas::PLAYER)?.r();
    let player_triforce_pieces_9 = r.get(canvas_player_85, field::player::TRIFORCE_PIECES)?.r();
    let element_value_9 = r.array_get(player_triforce_pieces_9, 0)?.i();
    if element_value_9 != 0 {
let canvas_player_86 = r.get(object_8, field::canvas::PLAYER)?.r();
let player_triforce_pieces_10 = r.get(canvas_player_86, field::player::TRIFORCE_PIECES)?.r();
let element_value_10 = r.array_get(player_triforce_pieces_10, 1)?.i();
if element_value_10 != 0 {
let canvas_player_87 = r.get(object_8, field::canvas::PLAYER)?.r();
let player_triforce_pieces_11 = r.get(canvas_player_87, field::player::TRIFORCE_PIECES)?.r();
let element_value_11 = r.array_get(player_triforce_pieces_11, 2)?.i();
if element_value_11 != 0 {
let canvas_player_89 = r.get(object_8, field::canvas::PLAYER)?.r();
let player_triforce_pieces_12 = r.get(canvas_player_89, field::player::TRIFORCE_PIECES)?.r();
let element_value_12 = r.array_get(player_triforce_pieces_12, 3)?.i();
if element_value_12 != 0 {
let canvas_player_94 = r.get(object_8, field::canvas::PLAYER)?.r();
let player_triforce_pieces_13 = r.get(canvas_player_94, field::player::TRIFORCE_PIECES)?.r();
let element_value_13 = r.array_get(player_triforce_pieces_13, 4)?.i();
if element_value_13 != 0 {
let canvas_player_101 = r.get(object_8, field::canvas::PLAYER)?.r();
let player_triforce_pieces_14 = r.get(canvas_player_101, field::player::TRIFORCE_PIECES)?.r();
let element_value_14 = r.array_get(player_triforce_pieces_14, 5)?.i();
if element_value_14 != 0 {
let canvas_player_111 = r.get(object_8, field::canvas::PLAYER)?.r();
let player_triforce_pieces_15 = r.get(canvas_player_111, field::player::TRIFORCE_PIECES)?.r();
let element_value_15 = r.array_get(player_triforce_pieces_15, 6)?.i();
if element_value_15 != 0 {
let canvas_player_118 = r.get(object_8, field::canvas::PLAYER)?.r();
let player_triforce_pieces_16 = r.get(canvas_player_118, field::player::TRIFORCE_PIECES)?.r();
let element_value_16 = r.array_get(player_triforce_pieces_16, 7)?.i();
if element_value_16 != 0 {
host::graphics_translate(r, object_6, 84, 2)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_36 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_35 = arithmetic::remainder_int(r, canvas_state_aj_36, 4)?;
if remainder_value_35 < 1 {
let canvas_state_ac_15 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_15, -16, 0, 0)?;
}

} else {
host::graphics_translate(r, object_6, 212, 133)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_37 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_36 = arithmetic::remainder_int(r, canvas_state_aj_37, 3)?;
if remainder_value_36 <= 1 {
let canvas_state_ac_16 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_16, -8, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 36, 93)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_34 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_33 = arithmetic::remainder_int(r, canvas_state_aj_34, 3)?;
if remainder_value_33 <= 1 {
let canvas_state_ac_13 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_13, -8, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 36, 44)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_32 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_31 = arithmetic::remainder_int(r, canvas_state_aj_32, 3)?;
if remainder_value_31 <= 1 {
let canvas_state_ac_11 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_11, -8, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 180, 2)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_30 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_29 = arithmetic::remainder_int(r, canvas_state_aj_30, 3)?;
if remainder_value_29 <= 1 {
let canvas_state_ac_9 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_9, -8, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 84, 93)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_28 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_27 = arithmetic::remainder_int(r, canvas_state_aj_28, 3)?;
if remainder_value_27 <= 1 {
let canvas_state_ac_7 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_7, -8, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 74, 154)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_25 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_24 = arithmetic::remainder_int(r, canvas_state_aj_25, 3)?;
if remainder_value_24 <= 1 {
let canvas_state_ac_4 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_4, -8, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 196, 71)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_23 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_22 = arithmetic::remainder_int(r, canvas_state_aj_23, 3)?;
if remainder_value_22 <= 1 {
let canvas_state_ac_2 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_2, -8, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 116, 70)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_21 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_21 = arithmetic::remainder_int(r, canvas_state_aj_21, 3)?;
if remainder_value_21 <= 1 {
let canvas_state_ac = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac, -8, 0, 0)?;
}

}
    break 'graphics_get_translate_x_8;
    } // check_canvas_zone_4
    let canvas_zone_25 = r.get(object_8, field::canvas::ZONE)?.i();
    if canvas_zone_25 != 3 {
let canvas_zone_26 = r.get(object_8, field::canvas::ZONE)?.i();
if canvas_zone_26 != 4 {
let canvas_zone_28 = r.get(object_8, field::canvas::ZONE)?.i();
if canvas_zone_28 != 5 {
let canvas_zone_29 = r.get(object_8, field::canvas::ZONE)?.i();
if canvas_zone_29 != 6 {
let canvas_zone_30 = r.get(object_8, field::canvas::ZONE)?.i();
if canvas_zone_30 != 7 {
let canvas_zone_31 = r.get(object_8, field::canvas::ZONE)?.i();
if canvas_zone_31 != 8 {
let canvas_zone_32 = r.get(object_8, field::canvas::ZONE)?.i();
if canvas_zone_32 != 9 {
let canvas_zone_33 = r.get(object_8, field::canvas::ZONE)?.i();
if canvas_zone_33 != 10 {
let canvas_zone_34 = r.get(object_8, field::canvas::ZONE)?.i();
if canvas_zone_34 == 11 {
host::graphics_translate(r, object_6, 109, 73)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_40 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_39 = arithmetic::remainder_int(r, canvas_state_aj_40, 4)?;
if remainder_value_39 < 1 {
let canvas_state_ac_19 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_19, -16, 0, 0)?;
}
}
} else {
host::graphics_translate(r, object_6, 89, 70)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_39 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_38 = arithmetic::remainder_int(r, canvas_state_aj_39, 4)?;
if remainder_value_38 < 1 {
let canvas_state_ac_18 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_18, -16, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 108, 50)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_38 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_37 = arithmetic::remainder_int(r, canvas_state_aj_38, 4)?;
if remainder_value_37 < 1 {
let canvas_state_ac_17 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_17, -16, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 172, 28)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_35 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_34 = arithmetic::remainder_int(r, canvas_state_aj_35, 4)?;
if remainder_value_34 < 1 {
let canvas_state_ac_14 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_14, -16, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 76, 46)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_33 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_32 = arithmetic::remainder_int(r, canvas_state_aj_33, 4)?;
if remainder_value_32 < 1 {
let canvas_state_ac_12 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_12, -16, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 168, 28)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_31 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_30 = arithmetic::remainder_int(r, canvas_state_aj_31, 4)?;
if remainder_value_30 < 1 {
let canvas_state_ac_10 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_10, -16, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 208, 68)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_29 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_28 = arithmetic::remainder_int(r, canvas_state_aj_29, 4)?;
if remainder_value_28 < 1 {
let canvas_state_ac_8 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_8, -16, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 141, 4)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_27 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_26 = arithmetic::remainder_int(r, canvas_state_aj_27, 4)?;
if remainder_value_26 < 1 {
let canvas_state_ac_6 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_6, -16, 0, 0)?;
}

}
} else {
host::graphics_translate(r, object_6, 184, 37)?;
host::graphics_clip_rect(r, object_6, 0, 0, 8, 8)?;
host::graphics_set_clip(r, object_6, 0, 0, 8, 8)?;
let canvas_state_aj_24 = r.get(object_8, field::canvas::STATE_AJ)?.i();
let remainder_value_23 = arithmetic::remainder_int(r, canvas_state_aj_24, 4)?;
if remainder_value_23 < 1 {
let canvas_state_ac_3 = r.get(object_8, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_6, canvas_state_ac_3, -16, 0, 0)?;
}

}
    } // graphics_get_translate_x_8
    let graphics_get_translate_x_value_70 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_67 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_70.wrapping_neg(), graphics_get_translate_y_value_67.wrapping_neg())?;
    host::graphics_translate(r, object_3, 0, 32)?;
    host::graphics_translate(r, object_3, number_4.wrapping_sub(4), number_5.wrapping_sub(4))?;
    host::graphics_clip_rect(r, object_3, 0, 0, 8, 8)?;
    host::graphics_set_clip(r, object_3, 0, 0, 8, 8)?;
    let canvas_state_aj_26 = r.get(object_2, field::canvas::STATE_AJ)?.i();
    let remainder_value_25 = arithmetic::remainder_int(r, canvas_state_aj_26, 4)?;
    if remainder_value_25 > 1 {
let canvas_state_ac_5 = r.get(object_2, field::canvas::STATE_AC)?.r();
host::graphics_draw_image(r, object_3, canvas_state_ac_5, 0, 0, 0)?;
}
    break 'graphics_get_translate_x_7;
    } // graphics_set_color_3
    host::graphics_set_color(r, object_3, 11184810)?;
    let canvas_get_width_value_26 = host::canvas_get_width(r, object_2)?;
    let quotient_value_32 = arithmetic::divide_int(r, canvas_get_width_value_26, 2)?;
    let canvas_get_height_value_16 = host::canvas_get_height(r, object_2)?;
    let quotient_value_33 = arithmetic::divide_int(r, canvas_get_height_value_16, 2)?;
    host::graphics_translate(r, object_3, quotient_value_32.wrapping_sub(30), quotient_value_33.wrapping_sub(32))?;
    let text_value_125 = r.intern("No Map..");
    host::graphics_draw_string(r, object_3, text_value_125, 0, 0, 20)?;
    } // graphics_get_translate_x_7
    let graphics_get_translate_x_value_49 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_46 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_49.wrapping_neg(), graphics_get_translate_y_value_46.wrapping_neg())?;
    let canvas_get_width_value_32 = host::canvas_get_width(r, object_2)?;
    let quotient_value_43 = arithmetic::divide_int(r, canvas_get_width_value_32, 2)?;
    host::graphics_translate(r, object_3, quotient_value_43.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, -8, 48)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 24, 24)?;
    host::graphics_set_clip(r, object_3, 0, 0, 24, 24)?;
    let canvas_state_ab_4 = r.get(object_2, field::canvas::STATE_AB)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_ab_4, 0, -48, 0)?;
    let canvas_state_ax_5 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_5 == 0 {
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_22 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_11 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_52 = arithmetic::divide_int(r, canvas_state_aj_11.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_22, quotient_value_52.wrapping_mul(54), 0, 0)?;
}
    let graphics_get_translate_x_value_52 = host::graphics_get_translate_x(r, object_3)?;
    let graphics_get_translate_y_value_49 = host::graphics_get_translate_y(r, object_3)?;
    host::graphics_translate(r, object_3, graphics_get_translate_x_value_52.wrapping_neg(), graphics_get_translate_y_value_49.wrapping_neg())?;
    let canvas_get_width_value_37 = host::canvas_get_width(r, object_2)?;
    let quotient_value_51 = arithmetic::divide_int(r, canvas_get_width_value_37, 2)?;
    host::graphics_translate(r, object_3, quotient_value_51.wrapping_sub(72), 32)?;
    host::graphics_translate(r, object_3, 132, 48)?;
    host::graphics_clip_rect(r, object_3, 0, 0, 24, 24)?;
    host::graphics_set_clip(r, object_3, 0, 0, 24, 24)?;
    let canvas_state_ab_5 = r.get(object_2, field::canvas::STATE_AB)?.r();
    host::graphics_draw_image(r, object_3, canvas_state_ab_5, -24, -48, 0)?;
    let canvas_state_ax_6 = r.get(object_2, field::canvas::STATE_AX)?.i();
    if canvas_state_ax_6 == 1 {
host::graphics_clip_rect(r, object_3, 0, 0, 28, 26)?;
host::graphics_set_clip(r, object_3, 0, 0, 28, 26)?;
let canvas_state_aa_25 = r.get(object_2, field::canvas::STATE_AA)?.r();
let canvas_state_aj_12 = r.get(object_2, field::canvas::STATE_AJ)?.i();
let quotient_value_54 = arithmetic::divide_int(r, canvas_state_aj_12.wrapping_neg(), 2)?;
host::graphics_draw_image(r, object_3, canvas_state_aa_25, quotient_value_54.wrapping_mul(54), 0, 0)?;
}
    } // check_canvas_redraw_world
    let canvas_redraw_world_2 = r.get(canvas, field::canvas::REDRAW_WORLD)?.i();
    if canvas_redraw_world_2 != 0 {
let canvas_state_s_4 = r.get(canvas, field::canvas::STATE_S)?.r();
let image_get_graphics_value_3 = host::image_get_graphics(r, canvas_state_s_4)?;
super::canvas::draw_hud(r, canvas, image_get_graphics_value_3)?;
r.set(canvas, field::canvas::REDRAW_WORLD, V::I(0))?;
}
    let graphics_get_translate_x_value_37 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_34 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_37.wrapping_neg(), graphics_get_translate_y_value_34.wrapping_neg())?;
    let canvas_state_l_upper_44 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_44, 32)?;
    let canvas_state_l_upper_45 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_45, 32)?;
    let canvas_state_s_3 = r.get(canvas, field::canvas::STATE_S)?.r();
    host::graphics_draw_image(r, graphics, canvas_state_s_3, 0, 0, 0)?;
    let canvas_state_an_upper = r.get(canvas, field::canvas::STATE_AN_UPPER)?.i();
    if canvas_state_an_upper == 0 {
return Ok(());
}
    let graphics_get_translate_x_value_44 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_41 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_44.wrapping_neg(), graphics_get_translate_y_value_41.wrapping_neg())?;
    let canvas_state_l_upper_46 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_40 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_46, canvas_state_m_40)?;
    let canvas_state_l_upper_47 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_41 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_47, canvas_state_m_41)?;
    let canvas_state_l_upper_48 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_42 = r.get(canvas, field::canvas::STATE_M)?.i();
    let canvas_state_am_upper = r.get(canvas, field::canvas::STATE_AM_UPPER)?.i();
    number_5 = canvas_state_am_upper;
    number_4 = canvas_state_m_42;
    number_3 = canvas_state_l_upper_48;
    object_2 = graphics;
    let graphics_get_translate_x_value_45 = host::graphics_get_translate_x(r, object_2)?;
    let graphics_get_translate_y_value_42 = host::graphics_get_translate_y(r, object_2)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_45.wrapping_neg(), graphics_get_translate_y_value_42.wrapping_neg())?;
    host::graphics_set_color(r, object_2, 11184810)?;
    let quotient_value_26 = arithmetic::divide_int(r, number_3, 2)?;
    let quotient_value_27 = arithmetic::divide_int(r, number_4, 2)?;
    host::graphics_fill_rect(r, object_2, quotient_value_26.wrapping_sub(50), quotient_value_27.wrapping_sub(18), 98, 66)?;
    host::graphics_set_color(r, object_2, 0)?;
    let quotient_value_28 = arithmetic::divide_int(r, number_3, 2)?;
    let quotient_value_29 = arithmetic::divide_int(r, number_4, 2)?;
    host::graphics_fill_rect(r, object_2, quotient_value_28.wrapping_sub(48), quotient_value_29.wrapping_sub(16), 96, 64)?;
    let quotient_value_30 = arithmetic::divide_int(r, number_3, 2)?;
    let quotient_value_31 = arithmetic::divide_int(r, number_4, 2)?;
    host::graphics_translate(r, object_2, quotient_value_30.wrapping_sub(32), quotient_value_31.wrapping_add(8))?;
    host::graphics_set_color(r, object_2, 16777215)?;
    let remainder_value_2 = arithmetic::remainder_int(r, number_5.wrapping_shr((1) as u32), 3)?;
    if remainder_value_2 == 0 {
let text_value_129 = r.intern("Saving ");
host::graphics_draw_string(r, object_2, text_value_129, 0, 0, 20)?;
}
    let remainder_value_3 = arithmetic::remainder_int(r, number_5.wrapping_shr((1) as u32), 3)?;
    if remainder_value_3 == 1 {
let text_value_130 = r.intern("Saving.. ");
host::graphics_draw_string(r, object_2, text_value_130, 0, 0, 20)?;
}
    let remainder_value_5 = arithmetic::remainder_int(r, number_5.wrapping_shr((1) as u32), 3)?;
    if remainder_value_5 == 2 {
let text_value_139 = r.intern("Saving.... ");
host::graphics_draw_string(r, object_2, text_value_139, 0, 0, 20)?;
}
    let canvas_state_am_upper_2 = r.get(canvas, field::canvas::STATE_AM_UPPER)?.i();
    r.set(canvas, field::canvas::STATE_AM_UPPER, V::I(canvas_state_am_upper_2.wrapping_add(1)))?;
    let canvas_state_am_upper_3 = r.get(canvas, field::canvas::STATE_AM_UPPER)?.i();
    if canvas_state_am_upper_3 <= 20 {
return Ok(());
}
    r.set(canvas, field::canvas::STATE_AN_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AM_UPPER, V::I(0))?;
    return Ok(());
    } // graphics_get_clip_x
    let graphics_get_clip_x_value = host::graphics_get_clip_x(r, graphics)?;
    let graphics_get_clip_y_value = host::graphics_get_clip_y(r, graphics)?;
    let graphics_get_clip_height_value = host::graphics_get_clip_height(r, graphics)?;
    let graphics_get_clip_width_value = host::graphics_get_clip_width(r, graphics)?;
    let system_current_time_millis_value_2 = host::system_current_time_millis(r)?;
    r.set(canvas, field::canvas::STATE_AF, V::J(system_current_time_millis_value_2))?;
    let canvas_player_4 = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_digging = r.get(canvas_player_4, field::player::DIGGING)?.i();
    if player_digging != 0 {
let canvas_world_19 = r.get(canvas, field::canvas::WORLD)?.r();
let canvas_player_6 = r.get(canvas, field::canvas::PLAYER)?.r();
super::world::dig(r, canvas_world_19, canvas_player_6)?;
}
    let canvas_player_5 = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_health = r.get(canvas_player_5, field::player::HEALTH)?.i();
    if player_health == 0 {
let canvas_state_ak_2 = r.get(canvas, field::canvas::STATE_AK)?.i();
if canvas_state_ak_2 != 0 {
r.set(canvas, field::canvas::SCREEN_STATE, V::I(11))?;
r.set(canvas, field::canvas::STATE_AK_UPPER, V::I(-176))?;
r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
let canvas_player_9 = r.get(canvas, field::canvas::PLAYER)?.r();
r.set(canvas_player_9, field::player::STATE_AG, V::I(0))?;
return Ok(());
}
let canvas_player_8 = r.get(canvas, field::canvas::PLAYER)?.r();
let player_state_ag = r.get(canvas_player_8, field::player::STATE_AG)?.i();
if player_state_ag != 0 {
let canvas_player_14 = r.get(canvas, field::canvas::PLAYER)?.r();
r.set(canvas_player_14, field::player::STATE_AG, V::I(0))?;
}
}
    let canvas_redraw_world = r.get(canvas, field::canvas::REDRAW_WORLD)?.i();
    if canvas_redraw_world != 0 {
let canvas_state_s_2 = r.get(canvas, field::canvas::STATE_S)?.r();
let image_get_graphics_value_2 = host::image_get_graphics(r, canvas_state_s_2)?;
super::canvas::draw_hud(r, canvas, image_get_graphics_value_2)?;
r.set(canvas, field::canvas::REDRAW_WORLD, V::I(0))?;
}
    let canvas_state_i = r.get(canvas, field::canvas::STATE_I)?.i();
    if canvas_state_i != 0 { break 'check_canvas_world_18; }
    } // check_canvas_show_item_message
    let canvas_show_item_message = r.get(canvas, field::canvas::SHOW_ITEM_MESSAGE)?.i();
    if canvas_show_item_message != 0 { break 'check_canvas_world_18; }
    } // check_canvas_world_21
    let canvas_world_25 = r.get(canvas, field::canvas::WORLD)?.r();
    let world_state_m_39 = r.get(canvas_world_25, field::world::STATE_M)?.i();
    if world_state_m_39 != 0 { break 'check_canvas_world_18; }
    } // next_step_8
    number_2 =
        0;
    number_3 =
        0;
    let canvas_state_ad_upper = r.get(canvas, field::canvas::STATE_AD_UPPER)?.i();
    if canvas_state_ad_upper != 0 {
let canvas_world_31 = r.get(canvas, field::canvas::WORLD)?.r();
let canvas_player_23 = r.get(canvas, field::canvas::PLAYER)?.r();
let drawable_x_4 = r.get(canvas_player_23, field::drawable::X)?.i();
let canvas_state_ad_upper_2 = r.get(canvas, field::canvas::STATE_AD_UPPER)?.i();
let canvas_player_24 = r.get(canvas, field::canvas::PLAYER)?.r();
let drawable_y_4 = r.get(canvas_player_24, field::drawable::Y)?.i();
let canvas_player_25 = r.get(canvas, field::canvas::PLAYER)?.r();
let player_facing = r.get(canvas_player_25, field::player::FACING)?.i();
let canvas_player_26 = r.get(canvas, field::canvas::PLAYER)?.r();
let can_player_move_value = super::world::can_player_move(r, canvas_world_31, drawable_x_4.wrapping_add(canvas_state_ad_upper_2), drawable_y_4, player_facing, canvas_player_26, 0)?;
if can_player_move_value != 0 {
let canvas_state_ad_upper_4 = r.get(canvas, field::canvas::STATE_AD_UPPER)?.i();
number_2 = canvas_state_ad_upper_4;
}
}
    let canvas_state_ae = r.get(canvas, field::canvas::STATE_AE)?.i();
    if canvas_state_ae != 0 {
let canvas_world_36 = r.get(canvas, field::canvas::WORLD)?.r();
let canvas_player_35 = r.get(canvas, field::canvas::PLAYER)?.r();
let drawable_x_6 = r.get(canvas_player_35, field::drawable::X)?.i();
let canvas_player_36 = r.get(canvas, field::canvas::PLAYER)?.r();
let drawable_y_6 = r.get(canvas_player_36, field::drawable::Y)?.i();
let canvas_state_ae_3 = r.get(canvas, field::canvas::STATE_AE)?.i();
let canvas_player_37 = r.get(canvas, field::canvas::PLAYER)?.r();
let player_facing_3 = r.get(canvas_player_37, field::player::FACING)?.i();
let canvas_player_38 = r.get(canvas, field::canvas::PLAYER)?.r();
let can_player_move_value_3 = super::world::can_player_move(r, canvas_world_36, drawable_x_6, drawable_y_6.wrapping_add(canvas_state_ae_3), player_facing_3, canvas_player_38, 0)?;
if can_player_move_value_3 != 0 {
let canvas_state_ae_4 = r.get(canvas, field::canvas::STATE_AE)?.i();
number_3 = canvas_state_ae_4;
}
}
    let canvas_player_30 = r.get(canvas, field::canvas::PLAYER)?.r();
    let canvas_world_35 = r.get(canvas, field::canvas::WORLD)?.r();
    let canvas_player_31 = r.get(canvas, field::canvas::PLAYER)?.r();
    let drawable_x_5 = r.get(canvas_player_31, field::drawable::X)?.i();
    let canvas_player_32 = r.get(canvas, field::canvas::PLAYER)?.r();
    let drawable_y_5 = r.get(canvas_player_32, field::drawable::Y)?.i();
    let canvas_player_33 = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_facing_2 = r.get(canvas_player_33, field::player::FACING)?.i();
    let canvas_player_34 = r.get(canvas, field::canvas::PLAYER)?.r();
    let can_player_move_value_2 = super::world::can_player_move(r, canvas_world_35, drawable_x_5.wrapping_add(number_2), drawable_y_5.wrapping_add(number_3), player_facing_2, canvas_player_34, 1)?;
    let canvas_state_ad_upper_3 = r.get(canvas, field::canvas::STATE_AD_UPPER)?.i();
    let canvas_state_ae_2 = r.get(canvas, field::canvas::STATE_AE)?.i();
    number_7 = canvas_state_ae_2;
    number_6 = canvas_state_ad_upper_3;
    number_5 = can_player_move_value_2;
    number_4 = number_3;
    number_3 = number_2;
    object_2 = canvas_player_30;
    let player_state_y_number = r.get(canvas_player_30, field::player::STATE_Y_NUMBER)?.i();
    r.set(canvas_player_30, field::player::STATE_Y_NUMBER, V::I(player_state_y_number.wrapping_add(1)))?;
    number_8 =
        0;
    let player_attacking = r.get(object_2, field::player::ATTACKING)?.i();
    if player_attacking != 0 { break 'check_player_moving; }
    } // check_player_hookshot_extending
    let player_hookshot_extending = r.get(object_2, field::player::HOOKSHOT_EXTENDING)?.i();
    if player_hookshot_extending != 0 { break 'check_player_moving; }
    } // check_player_digging
    let player_digging_2 = r.get(object_2, field::player::DIGGING)?.i();
    if player_digging_2
        != 0
    {
        break 'check_player_moving;
    }
    } // next_step_10

    if number_6 < 0 {
        r.set(object_2, field::player::FACING, V::I(3))?;
        number_8 = number_8
            .wrapping_add(
                1,
            );
    }

    if number_6 > 0 {
        r.set(object_2, field::player::FACING, V::I(1))?;
        number_8 = number_8
            .wrapping_add(
                1,
            );
    }

    if number_7 < 0 {
        r.set(object_2, field::player::FACING, V::I(0))?;
        number_8 = number_8
            .wrapping_add(
                1,
            );
    }

    if number_7 > 0 {
        r.set(object_2, field::player::FACING, V::I(2))?;
        number_8 = number_8
            .wrapping_add(
                1,
            );
    }

    if (number_6 == 0)
        && (number_7 == 0)
    {
        r.set(object_2, field::player::ANIMATION, V::I(2))?;
    } else {
        let player_animation_5 = r.get(object_2, field::player::ANIMATION)?.i();
        r.set(object_2, field::player::ANIMATION, V::I(player_animation_5.wrapping_add(1)))?;
    }
    let player_animation_6 = r.get(object_2, field::player::ANIMATION)?.i();
    if player_animation_6 > 5
    {
        r.set(object_2, field::player::ANIMATION, V::I(0))?;
    }

    if (number_5 != 0)
        && (number_8 > 0)
    {
        let drawable_x_18 = r.get(object_2, field::drawable::X)?.i();
        r.set(object_2, field::drawable::X, V::I(drawable_x_18.wrapping_add(number_3)))?;
        let drawable_y_18 = r.get(object_2, field::drawable::Y)?.i();
        r.set(object_2, field::drawable::Y, V::I(drawable_y_18.wrapping_add(number_4)))?;
    }
    break 'check_canvas_world_18;
    } // check_player_moving
    let player_attacking_2 = r.get(object_2, field::player::ATTACKING)?.i();
    if player_attacking_2 != 0 {
        let player_animation = r.get(object_2, field::player::ANIMATION)?.i();
        r.set(object_2, field::player::ANIMATION, V::I(player_animation.wrapping_add(1)))?;
        r.set(object_2, field::player::HOOKSHOT_EXTENDING, V::I(0))?;
        let player_animation_2 = r.get(object_2, field::player::ANIMATION)?.i();
        if player_animation_2 > 5
        {
            r.set(object_2, field::player::ATTACKING, V::I(0))?;
            r.set(object_2, field::player::ANIMATION, V::I(0))?;
        }
    }
    let player_hookshot_extending_2 = r.get(object_2, field::player::HOOKSHOT_EXTENDING)?.i();
    if player_hookshot_extending_2 != 0 {
let player_animation_3 = r.get(object_2, field::player::ANIMATION)?.i();
r.set(object_2, field::player::ANIMATION, V::I(player_animation_3.wrapping_add(1)))?;
let player_action_timer = r.get(object_2, field::player::ACTION_TIMER)?.i();
r.set(object_2, field::player::ACTION_TIMER, V::I(player_action_timer.wrapping_add(1)))?;
r.set(object_2, field::player::ATTACKING, V::I(0))?;
let player_animation_4 = r.get(object_2, field::player::ANIMATION)?.i();
if player_animation_4 > 0 {
r.set(object_2, field::player::ANIMATION, V::I(0))?;
r.set(object_2, field::player::ACTION_TIMER, V::I(0))?;
}
}
    } // check_canvas_world_18
    let world_value_20 = r
        .get(
            canvas,
            field::canvas::WORLD,
        )?
        .r();
    let player_value_12 = r
        .get(
            canvas,
            field::canvas::PLAYER,
        )?
        .r();
    let x_value_2 = r
        .get(
            player_value_12,
            field::drawable::X,
        )?
        .i();
    let player_value_13 = r
        .get(
            canvas,
            field::canvas::PLAYER,
        )?
        .r();
    let y_value_2 = r
        .get(
            player_value_13,
            field::drawable::Y,
        )?
        .i();
    let find_portal_value =
        super::world::find_portal(
            r,
            world_value_20,
            x_value_2,
            y_value_2,
            0,
        )?;
    number_2 = find_portal_value;
    if find_portal_value > 0 {
        break 'check_canvas_world_19;
    }
    } // check_canvas_world_20
    let world_value_23 = r
        .get(
            canvas,
            field::canvas::WORLD,
        )?
        .r();
    let alternate_map_value_2 = r
        .get(
            world_value_23,
            field::world::ALTERNATE_MAP,
        )?
        .i();
    let world_value_24 = r
        .get(
            canvas,
            field::canvas::WORLD,
        )?
        .r();
    let world_alternate_room_2 = r.get(world_value_24, field::world::ALTERNATE_ROOM)?.i();
    if alternate_map_value_2
        == world_alternate_room_2
    {
        break 'check_canvas_state_i;
    }
    } // check_canvas_world_19
    let world_value_21 = r
        .get(canvas, field::canvas::WORLD)?
        .r();
    let alternate_map_value = r
        .get(
            world_value_21,
            field::world::ALTERNATE_MAP,
        )?
        .i();
    let world_value_22 = r
        .get(canvas, field::canvas::WORLD)?
        .r();
    let alternate_room_value = r
        .get(
            world_value_22,
            field::world::ALTERNATE_ROOM,
        )?
        .i();
    if alternate_map_value
        != alternate_room_value
    {
        let world_value_27 = r
            .get(
                canvas,
                field::canvas::WORLD,
            )?
            .r();
        let zone_value_2 = r
            .get(
                world_value_27,
                field::world::ZONE,
            )?
            .i();
        if zone_value_2 != 9 {
            let world_value_28 = r
                .get(
                    canvas,
                    field::canvas::WORLD,
                )?
                .r();
            let world_value_29 = r
                .get(
                    canvas,
                    field::canvas::WORLD,
                )?
                .r();
            let world_alternate_map_3 = r.get(world_value_29, field::world::ALTERNATE_MAP)?.i();
            r.set(world_value_28, field::world::ALTERNATE_ROOM, V::I(world_alternate_map_3))?;
            return Ok(());
        }
        let world_value_30 = r
            .get(
                canvas,
                field::canvas::WORLD,
            )?
            .r();
        let alternate_map_value_4 = r
            .get(
                world_value_30,
                field::world::ALTERNATE_MAP,
            )?
            .i();
        if alternate_map_value_4 != 0 {
            let world_value_32 = r
                .get(
                    canvas,
                    field::canvas::WORLD,
                )?
                .r();
            let object_value_16 = r.alloc(
                "java/lang/StringBuffer",
                Native::Empty,
            );
            host::string_buffer_initialize(
                r,
                object_value_16,
            )?;
            let text_value_133 =
                r.intern("/Maps/Map");
            let string_buffer_append_2_value_16 = host::string_buffer_append_2(r, object_value_16, text_value_133)?;
            let map_value_3 = r
                .get(
                    canvas,
                    field::canvas::MAP,
                )?
                .i();
            let string_buffer_append_value_9 = host::string_buffer_append(r, string_buffer_append_2_value_16, map_value_3)?;
            let text_value_134 =
                r.intern("-");
            let string_buffer_append_2_value_17 = host::string_buffer_append_2(r, string_buffer_append_value_9, text_value_134)?;
            let zone_value_5 = r
                .get(
                    canvas,
                    field::canvas::ZONE,
                )?
                .i();
            let string_buffer_append_value_10 = host::string_buffer_append(r, string_buffer_append_2_value_17, zone_value_5)?;
            let text_value_135 =
                r.intern("-1.xmap.xmob");
            let string_buffer_append_2_value_18 = host::string_buffer_append_2(r, string_buffer_append_value_10, text_value_135)?;
            let string_buffer_to_string_value_16 = host::string_buffer_to_string(r, string_buffer_append_2_value_18)?;
            let canvas_scene_image_10 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
            let image_get_graphics_value_4 =
                host::image_get_graphics(
                    r,
                    canvas_scene_image_10,
                )?;
            let player_value_28 = r
                .get(
                    canvas,
                    field::canvas::PLAYER,
                )?
                .r();
            let map_value_4 = r
                .get(
                    canvas,
                    field::canvas::MAP,
                )?
                .i();
            let zone_value_6 = r
                .get(
                    canvas,
                    field::canvas::ZONE,
                )?
                .i();
            let canvas_get_width_value_39 =
                host::canvas_get_width(
                    r, canvas,
                )?;
            let canvas_get_height_value_22 =
                host::canvas_get_height(
                    r, canvas,
                )?;
            let load_map_value = super::world::load_map(r, world_value_32, string_buffer_to_string_value_16, image_get_graphics_value_4, player_value_28, map_value_4, zone_value_6, canvas_get_width_value_39, canvas_get_height_value_22, 1)?;
            return Ok(());
        }
        let world_value_33 = r
            .get(
                canvas,
                field::canvas::WORLD,
            )?
            .r();
        let object_value_17 = r.alloc(
            "java/lang/StringBuffer",
            Native::Empty,
        );
        host::string_buffer_initialize(
            r,
            object_value_17,
        )?;
        let text_value_136 =
            r.intern("/Maps/Map");
        let string_buffer_append_2_value_19 =
            host::string_buffer_append_2(
                r,
                object_value_17,
                text_value_136,
            )?;
        let map_value_5 = r
            .get(
                canvas,
                field::canvas::MAP,
            )?
            .i();
        let string_buffer_append_value_11 = host::string_buffer_append(r, string_buffer_append_2_value_19, map_value_5)?;
        let text_value_137 = r.intern("-");
        let string_buffer_append_2_value_20 = host::string_buffer_append_2(r, string_buffer_append_value_11, text_value_137)?;
        let zone_value_7 = r
            .get(
                canvas,
                field::canvas::ZONE,
            )?
            .i();
        let string_buffer_append_value_12 = host::string_buffer_append(r, string_buffer_append_2_value_20, zone_value_7)?;
        let text_value_138 =
            r.intern(".xmap.xmob");
        let string_buffer_append_2_value_21 = host::string_buffer_append_2(r, string_buffer_append_value_12, text_value_138)?;
        let string_buffer_to_string_value_17 = host::string_buffer_to_string(r, string_buffer_append_2_value_21)?;
        let scene_image_value_11 = r
            .get(
                canvas,
                field::canvas::SCENE_IMAGE,
            )?
            .r();
        let image_get_graphics_value_5 =
            host::image_get_graphics(
                r,
                scene_image_value_11,
            )?;
        let player_value_29 = r
            .get(
                canvas,
                field::canvas::PLAYER,
            )?
            .r();
        let map_value_6 = r
            .get(
                canvas,
                field::canvas::MAP,
            )?
            .i();
        let zone_value_8 = r
            .get(
                canvas,
                field::canvas::ZONE,
            )?
            .i();
        let canvas_get_width_value_40 =
            host::canvas_get_width(
                r, canvas,
            )?;
        let canvas_get_height_value_23 =
            host::canvas_get_height(
                r, canvas,
            )?;
        let load_map_value_2 = super::world::load_map(r, world_value_33, string_buffer_to_string_value_17, image_get_graphics_value_5, player_value_29, map_value_6, zone_value_8, canvas_get_width_value_40, canvas_get_height_value_23, 1)?;
        return Ok(());
    }
    let world_value_26 = r
        .get(canvas, field::canvas::WORLD)?
        .r();
    let portals_value = r
        .get(
            world_value_26,
            field::world::PORTALS,
        )?
        .r();
    let vector_element_at_value_9 =
        host::vector_element_at(
            r,
            portals_value,
            number_2.wrapping_sub(1),
        )?;
    host::check_cast(
        r,
        vector_element_at_value_9,
        "e",
    )?;
    object_3 = vector_element_at_value_9;
    let graphics_get_translate_x_value_48 =
        host::graphics_get_translate_x(
            r, graphics,
        )?;
    let graphics_get_translate_y_value_45 =
        host::graphics_get_translate_y(
            r, graphics,
        )?;
    host::graphics_translate(
        r,
        graphics,
        (graphics_get_translate_x_value_48)
            .wrapping_neg(),
        (graphics_get_translate_y_value_45)
            .wrapping_neg(),
    )?;
    host::graphics_set_color(
        r, graphics, 0,
    )?;
    let canvas_get_width_value_30 =
        host::canvas_get_width(r, canvas)?;
    let quotient_value_39 =
        arithmetic::divide_int(
            r,
            canvas_get_width_value_30,
            2,
        )?;
    let canvas_get_height_value_18 =
        host::canvas_get_height(r, canvas)?;
    let quotient_value_40 =
        arithmetic::divide_int(
            r,
            canvas_get_height_value_18,
            2,
        )?;
    host::graphics_fill_rect(
        r,
        graphics,
        (quotient_value_39)
            .wrapping_sub(48),
        (quotient_value_40)
            .wrapping_sub(16),
        96,
        64,
    )?;
    let canvas_get_width_value_31 =
        host::canvas_get_width(r, canvas)?;
    let quotient_value_41 =
        arithmetic::divide_int(
            r,
            canvas_get_width_value_31,
            2,
        )?;
    let canvas_get_height_value_19 =
        host::canvas_get_height(r, canvas)?;
    let quotient_value_42 =
        arithmetic::divide_int(
            r,
            canvas_get_height_value_19,
            2,
        )?;
    host::graphics_translate(
        r,
        graphics,
        (quotient_value_41)
            .wrapping_sub(32),
        quotient_value_42.wrapping_add(8),
    )?;
    host::graphics_set_color(
        r, graphics, 16777215,
    )?;
    let text_value_128 =
        r.intern("Loading.. ");
    host::graphics_draw_string(
        r,
        graphics,
        text_value_128,
        0,
        0,
        20,
    )?;
    let state_av_value = r
        .get(
            canvas,
            field::canvas::STATE_AV,
        )?
        .i();
    if state_av_value == number_2 {
        let portal_destination_zone = r.get(object_3, field::portal::DESTINATION_ZONE)?.i();
        if portal_destination_zone != 20 {
            host::graphics_translate(
                r, graphics, -12, 12,
            )?;
            let text_value_132 = r.intern(
                "Error - go back!!",
            );
            host::graphics_draw_string(
                r,
                graphics,
                text_value_132,
                0,
                0,
                20,
            )?;
            return Ok(());
        }
        let portal_destination_map = r.get(object_3, field::portal::DESTINATION_MAP)?.i();
        if portal_destination_map != 3 {
            host::graphics_translate(
                r, graphics, -12, 12,
            )?;
            let text_value_132 = r.intern(
                "Error - go back!!",
            );
            host::graphics_draw_string(
                r,
                graphics,
                text_value_132,
                0,
                0,
                20,
            )?;
            return Ok(());
        }
        r.set(
            canvas,
            field::canvas::SCREEN_STATE,
            V::I(100),
        )?;
        let sound_value_2 = r
            .get(
                canvas,
                field::canvas::SOUND,
            )?
            .r();
        super::sound::select_map_music(
            r,
            sound_value_2,
            20,
            3,
        )?;
        r.set(
            canvas,
            field::canvas::REFRESH_COMMANDS,
            V::I(1),
        )?;
        return Ok(());
    }
    let state_ap_value = r
        .get(
            canvas,
            field::canvas::STATE_AP,
        )?
        .i();
    if state_ap_value == 0 {
        r.set(
            canvas,
            field::canvas::STATE_AP,
            V::I(1),
        )?;
        return Ok(());
    }
    let state_av_value_2 = r
        .get(
            canvas,
            field::canvas::STATE_AV,
        )?
        .i();
    if state_av_value_2 == number_2 {
        break 'check_canvas_state_i;
    }
    } // next_step_9
    number_4 = 0;
    let world_value_37 = r
        .get(canvas, field::canvas::WORLD)?
        .r();
    let zone_value_10 = r
        .get(
            world_value_37,
            field::world::ZONE,
        )?
        .i();
    if zone_value_10 != 9 {
        break 'check_canvas_player_2;
    }
    } // check_portal_destination_zone
    let destination_zone_value_2 = r
        .get(
            object_3,
            field::portal::DESTINATION_ZONE,
        )?
        .i();
    if destination_zone_value_2 != 9 {
        break 'check_canvas_player_2;
    }
    } // check_canvas_player_3
    let canvas_player_55 =
        r.get(canvas, field::canvas::PLAYER)?.r();
    let world_flags_value_2 = r
        .get(
            canvas_player_55,
            field::player::WORLD_FLAGS,
        )?
        .i();
    let canvas_zone_15 =
        r.get(canvas, field::canvas::ZONE)?.i();
    if !((world_flags_value_2
        & (1_i32).wrapping_shl((canvas_zone_15) as u32))
        <= 0)
    {
        let canvas_world_46 =
            r.get(canvas, field::canvas::WORLD)?.r();
        r.set(
            canvas_world_46,
            field::world::ALTERNATE_MAP,
            V::I(1),
        )?;
    }
    let canvas_world_45 =
        r.get(canvas, field::canvas::WORLD)?.r();
    let alternate_map_value_5 = r
        .get(
            canvas_world_45,
            field::world::ALTERNATE_MAP,
        )?
        .i();
    if alternate_map_value_5 != 0 {
        let canvas_world_47 =
            r.get(canvas, field::canvas::WORLD)?.r();
        let object_value_19 = r.alloc(
            "java/lang/StringBuffer",
            Native::Empty,
        );
        host::string_buffer_initialize(
            r,
            object_value_19,
        )?;
        let text_value_144 = r.intern("/Maps/Map");
        let string_buffer_append_2_value_27 =
            host::string_buffer_append_2(
                r,
                object_value_19,
                text_value_144,
            )?;
        let destination_map_value_4 = r
            .get(
                object_3,
                field::portal::DESTINATION_MAP,
            )?
            .i();
        let integer_to_string_value_3 =
            host::integer_to_string(
                r,
                destination_map_value_4,
            )?;
        let string_buffer_append_2_value_28 =
            host::string_buffer_append_2(
                r,
                string_buffer_append_2_value_27,
                integer_to_string_value_3,
            )?;
        let text_value_145 = r.intern("-");
        let string_buffer_append_2_value_29 =
            host::string_buffer_append_2(
                r,
                string_buffer_append_2_value_28,
                text_value_145,
            )?;
        let destination_zone_value_5 = r
            .get(
                object_3,
                field::portal::DESTINATION_ZONE,
            )?
            .i();
        let integer_to_string_value_4 =
            host::integer_to_string(
                r,
                destination_zone_value_5,
            )?;
        let string_buffer_append_2_value_30 =
            host::string_buffer_append_2(
                r,
                string_buffer_append_2_value_29,
                integer_to_string_value_4,
            )?;
        let text_value_146 = r.intern("-1.xmap.xmob");
        let string_buffer_append_2_value_31 =
            host::string_buffer_append_2(
                r,
                string_buffer_append_2_value_30,
                text_value_146,
            )?;
        let string_buffer_to_string_value_19 =
            host::string_buffer_to_string(
                r,
                string_buffer_append_2_value_31,
            )?;
        let scene_image_value_14 = r
            .get(canvas, field::canvas::SCENE_IMAGE)?
            .r();
        let image_get_graphics_value_7 =
            host::image_get_graphics(
                r,
                scene_image_value_14,
            )?;
        let canvas_player_64 =
            r.get(canvas, field::canvas::PLAYER)?.r();
        let destination_map_value_5 = r
            .get(
                object_3,
                field::portal::DESTINATION_MAP,
            )?
            .i();
        let destination_zone_value_6 = r
            .get(
                object_3,
                field::portal::DESTINATION_ZONE,
            )?
            .i();
        let canvas_get_width_value_47 =
            host::canvas_get_width(r, canvas)?;
        let canvas_get_height_value_25 =
            host::canvas_get_height(r, canvas)?;
        let load_map_value_4 = super::world::load_map(
            r,
            canvas_world_47,
            string_buffer_to_string_value_19,
            image_get_graphics_value_7,
            canvas_player_64,
            destination_map_value_5,
            destination_zone_value_6,
            canvas_get_width_value_47,
            canvas_get_height_value_25,
            1,
        )?;
        number_4 = load_map_value_4;
    } else {
        let canvas_world_48 =
            r.get(canvas, field::canvas::WORLD)?.r();
        let object_value_20 = r.alloc(
            "java/lang/StringBuffer",
            Native::Empty,
        );
        host::string_buffer_initialize(
            r,
            object_value_20,
        )?;
        let text_value_147 = r.intern("/Maps/Map");
        let string_buffer_append_2_value_32 =
            host::string_buffer_append_2(
                r,
                object_value_20,
                text_value_147,
            )?;
        let destination_map_value_6 = r
            .get(
                object_3,
                field::portal::DESTINATION_MAP,
            )?
            .i();
        let integer_to_string_value_5 =
            host::integer_to_string(
                r,
                destination_map_value_6,
            )?;
        let string_buffer_append_2_value_33 =
            host::string_buffer_append_2(
                r,
                string_buffer_append_2_value_32,
                integer_to_string_value_5,
            )?;
        let text_value_148 = r.intern("-");
        let string_buffer_append_2_value_34 =
            host::string_buffer_append_2(
                r,
                string_buffer_append_2_value_33,
                text_value_148,
            )?;
        let destination_zone_value_7 = r
            .get(
                object_3,
                field::portal::DESTINATION_ZONE,
            )?
            .i();
        let integer_to_string_value_6 =
            host::integer_to_string(
                r,
                destination_zone_value_7,
            )?;
        let string_buffer_append_2_value_35 =
            host::string_buffer_append_2(
                r,
                string_buffer_append_2_value_34,
                integer_to_string_value_6,
            )?;
        let text_value_149 = r.intern(".xmap.xmob");
        let string_buffer_append_2_value_36 =
            host::string_buffer_append_2(
                r,
                string_buffer_append_2_value_35,
                text_value_149,
            )?;
        let string_buffer_to_string_value_20 =
            host::string_buffer_to_string(
                r,
                string_buffer_append_2_value_36,
            )?;
        let scene_image_value_15 = r
            .get(canvas, field::canvas::SCENE_IMAGE)?
            .r();
        let image_get_graphics_value_8 =
            host::image_get_graphics(
                r,
                scene_image_value_15,
            )?;
        let canvas_player_65 =
            r.get(canvas, field::canvas::PLAYER)?.r();
        let destination_map_value_7 = r
            .get(
                object_3,
                field::portal::DESTINATION_MAP,
            )?
            .i();
        let destination_zone_value_8 = r
            .get(
                object_3,
                field::portal::DESTINATION_ZONE,
            )?
            .i();
        let canvas_get_width_value_48 =
            host::canvas_get_width(r, canvas)?;
        let canvas_get_height_value_26 =
            host::canvas_get_height(r, canvas)?;
        let load_map_value_5 = super::world::load_map(
            r,
            canvas_world_48,
            string_buffer_to_string_value_20,
            image_get_graphics_value_8,
            canvas_player_65,
            destination_map_value_7,
            destination_zone_value_8,
            canvas_get_width_value_48,
            canvas_get_height_value_26,
            1,
        )?;
        number_4 = load_map_value_5;
    }
    break 'next_step_11;
    } // check_canvas_player_2
    let canvas_player_49 =
        r.get(canvas, field::canvas::PLAYER)?.r();
    let world_flags_value = r
        .get(canvas_player_49, field::player::WORLD_FLAGS)?
        .i();
    let canvas_zone_13 =
        r.get(canvas, field::canvas::ZONE)?.i();
    if (world_flags_value
        & (1_i32).wrapping_shl((canvas_zone_13) as u32))
        <= 0
    {
        let canvas_world_42 =
            r.get(canvas, field::canvas::WORLD)?.r();
        r.set(
            canvas_world_42,
            field::world::ALTERNATE_MAP,
            V::I(0),
        )?;
    } else {
        let canvas_world_43 =
            r.get(canvas, field::canvas::WORLD)?.r();
        r.set(
            canvas_world_43,
            field::world::ALTERNATE_MAP,
            V::I(1),
        )?;
    }
    let canvas_world_44 =
        r.get(canvas, field::canvas::WORLD)?.r();
    let object_value_18 =
        r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value_18)?;
    let text_value_141 = r.intern("/Maps/Map");
    let string_buffer_append_2_value_22 =
        host::string_buffer_append_2(
            r,
            object_value_18,
            text_value_141,
        )?;
    let destination_map_value_2 = r
        .get(object_3, field::portal::DESTINATION_MAP)?
        .i();
    let integer_to_string_value = host::integer_to_string(
        r,
        destination_map_value_2,
    )?;
    let string_buffer_append_2_value_23 =
        host::string_buffer_append_2(
            r,
            string_buffer_append_2_value_22,
            integer_to_string_value,
        )?;
    let text_value_142 = r.intern("-");
    let string_buffer_append_2_value_24 =
        host::string_buffer_append_2(
            r,
            string_buffer_append_2_value_23,
            text_value_142,
        )?;
    let destination_zone_value_3 = r
        .get(object_3, field::portal::DESTINATION_ZONE)?
        .i();
    let integer_to_string_value_2 =
        host::integer_to_string(
            r,
            destination_zone_value_3,
        )?;
    let string_buffer_append_2_value_25 =
        host::string_buffer_append_2(
            r,
            string_buffer_append_2_value_24,
            integer_to_string_value_2,
        )?;
    let text_value_143 = r.intern(".xmap.xmob");
    let string_buffer_append_2_value_26 =
        host::string_buffer_append_2(
            r,
            string_buffer_append_2_value_25,
            text_value_143,
        )?;
    let string_buffer_to_string_value_18 =
        host::string_buffer_to_string(
            r,
            string_buffer_append_2_value_26,
        )?;
    let canvas_scene_image_12 =
        r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let image_get_graphics_value_6 =
        host::image_get_graphics(r, canvas_scene_image_12)?;
    let canvas_player_59 =
        r.get(canvas, field::canvas::PLAYER)?.r();
    let destination_map_value_3 = r
        .get(object_3, field::portal::DESTINATION_MAP)?
        .i();
    let destination_zone_value_4 = r
        .get(object_3, field::portal::DESTINATION_ZONE)?
        .i();
    let canvas_get_width_value_46 =
        host::canvas_get_width(r, canvas)?;
    let canvas_get_height_value_24 =
        host::canvas_get_height(r, canvas)?;
    let load_map_value_3 = super::world::load_map(
        r,
        canvas_world_44,
        string_buffer_to_string_value_18,
        image_get_graphics_value_6,
        canvas_player_59,
        destination_map_value_3,
        destination_zone_value_4,
        canvas_get_width_value_46,
        canvas_get_height_value_24,
        1,
    )?;
    number_4 = load_map_value_3;
    } // next_step_11

    if number_4 == 0 {
        r.set(canvas, field::canvas::STATE_AV, V::I(number_2))?;
    } else {
        let canvas_sound_3 =
            r.get(canvas, field::canvas::SOUND)?.r();
        let destination_zone_value_9 = r
            .get(object_3, field::portal::DESTINATION_ZONE)?
            .i();
        let destination_map_value_8 = r
            .get(object_3, field::portal::DESTINATION_MAP)?
            .i();
        super::sound::select_map_music(
            r,
            canvas_sound_3,
            destination_zone_value_9,
            destination_map_value_8,
        )?;
        let destination_zone_value_10 = r
            .get(object_3, field::portal::DESTINATION_ZONE)?
            .i();
        if destination_zone_value_10 != 0 {
            r.set(canvas, field::canvas::STATE_G, V::I(0))?;
            let destination_zone_value_11 = r
                .get(object_3, field::portal::DESTINATION_ZONE)?
                .i();
            if destination_zone_value_11 == 20 {
                let destination_map_value_10 = r
                    .get(
                        object_3,
                        field::portal::DESTINATION_MAP,
                    )?
                    .i();
                if destination_map_value_10 == 2 {
                    let use_phone_buttons_value_2 = r
                        .get(
                            canvas,
                            field::canvas::USE_PHONE_BUTTONS,
                        )?
                        .i();
                    if use_phone_buttons_value_2 != 0 {
                        r.set(
                            canvas,
                            field::canvas::STATE_G,
                            V::I(1),
                        )?;
                    }
                }
            }
        } else {
            let destination_zone_value_12 = r
                .get(object_3, field::portal::DESTINATION_ZONE)?
                .i();
            let canvas_zone_20 =
                r.get(canvas, field::canvas::ZONE)?.i();
            if destination_zone_value_12 != canvas_zone_20 {
                let object_value_21 =
                    r.alloc("java/util/Random", Native::Empty);
                host::random_initialize(r, object_value_21)?;
                object_2 = object_value_21;
                let destination_map_value_11 = r
                    .get(
                        object_3,
                        field::portal::DESTINATION_MAP,
                    )?
                    .i();
                let destination_zone_value_14 = r
                    .get(
                        object_3,
                        field::portal::DESTINATION_ZONE,
                    )?
                    .i();
                let random_next_int_value_6 =
                    host::random_next_int(r, object_2)?;
                host::random_set_seed(
                    r,
                    object_value_21,
                    ((destination_map_value_11.wrapping_mul(
                        destination_zone_value_14,
                    ))
                    .wrapping_add(random_next_int_value_6))
                        as i64,
                )?;
                let random_next_int_value_7 =
                    host::random_next_int(r, object_2)?;
                let remainder_value_19 =
                    arithmetic::remainder_int(
                        r,
                        random_next_int_value_7,
                        8,
                    )?;
                if remainder_value_19 == 1 {
                    r.set(
                        canvas,
                        field::canvas::STATE_G,
                        V::I(1),
                    )?;
                }
            }
        }
        let destination_map_value_9 = r
            .get(object_3, field::portal::DESTINATION_MAP)?
            .i();
        r.set(
            canvas,
            field::canvas::MAP,
            V::I(destination_map_value_9),
        )?;
        let destination_zone_value_13 = r
            .get(object_3, field::portal::DESTINATION_ZONE)?
            .i();
        r.set(
            canvas,
            field::canvas::ZONE,
            V::I(destination_zone_value_13),
        )?;
        let canvas_player_76 =
            r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_map_23 =
            r.get(canvas, field::canvas::MAP)?.i();
        r.set(
            canvas_player_76,
            field::player::MAP,
            V::I(canvas_map_23),
        )?;
        let canvas_player_77 =
            r.get(canvas, field::canvas::PLAYER)?.r();
        let canvas_zone_22 =
            r.get(canvas, field::canvas::ZONE)?.i();
        r.set(
            canvas_player_77,
            field::player::ZONE,
            V::I(canvas_zone_22),
        )?;
        let canvas_player_78 =
            r.get(canvas, field::canvas::PLAYER)?.r();
        let portal_destination_x =
            r.get(object_3, field::portal::DESTINATION_X)?.i();
        r.set(
            canvas_player_78,
            field::drawable::X,
            V::I(
                portal_destination_x.wrapping_shl((4) as u32),
            ),
        )?;
        let canvas_player_79 =
            r.get(canvas, field::canvas::PLAYER)?.r();
        let portal_destination_y =
            r.get(object_3, field::portal::DESTINATION_Y)?.i();
        r.set(
            canvas_player_79,
            field::drawable::Y,
            V::I(
                (((portal_destination_y)
                    .wrapping_shl((4) as u32))
                .wrapping_add(32))
                .wrapping_sub(8),
            ),
        )?;
        let canvas_player_80 =
            r.get(canvas, field::canvas::PLAYER)?.r();
        let drawable_x_16 =
            r.get(canvas_player_80, field::drawable::X)?.i();
        let canvas_state_l_upper_53 =
            r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        let quotient_value_92 = arithmetic::divide_int(
            r,
            canvas_state_l_upper_53,
            2,
        )?;
        r.set(
            canvas,
            field::canvas::STATE_P,
            V::I(drawable_x_16.wrapping_sub(quotient_value_92)),
        )?;
        let canvas_player_81 =
            r.get(canvas, field::canvas::PLAYER)?.r();
        let drawable_y_16 =
            r.get(canvas_player_81, field::drawable::Y)?.i();
        let canvas_state_m_48 =
            r.get(canvas, field::canvas::STATE_M)?.i();
        let quotient_value_93 = arithmetic::divide_int(
            r,
            canvas_state_m_48.wrapping_add(32),
            2,
        )?;
        r.set(
            canvas,
            field::canvas::STATE_Q,
            V::I(drawable_y_16.wrapping_sub(quotient_value_93)),
        )?;
        let canvas_state_p_4 =
            r.get(canvas, field::canvas::STATE_P)?.i();
        if canvas_state_p_4 < 0 {
            r.set(canvas, field::canvas::STATE_P, V::I(0))?;
        }
        let canvas_state_q_4 =
            r.get(canvas, field::canvas::STATE_Q)?.i();
        if canvas_state_q_4 < 0 {
            r.set(canvas, field::canvas::STATE_Q, V::I(0))?;
        }
        let canvas_state_p_5 =
            r.get(canvas, field::canvas::STATE_P)?.i();
        let canvas_state_al =
            r.get(canvas, field::canvas::STATE_AL)?.i();
        if canvas_state_p_5 > canvas_state_al {
            let canvas_state_al_2 =
                r.get(canvas, field::canvas::STATE_AL)?.i();
            r.set(
                canvas,
                field::canvas::STATE_P,
                V::I(canvas_state_al_2),
            )?;
        }
        let canvas_state_q_6 =
            r.get(canvas, field::canvas::STATE_Q)?.i();
        let canvas_state_am =
            r.get(canvas, field::canvas::STATE_AM)?.i();
        if canvas_state_q_6 > canvas_state_am {
            let canvas_state_am_2 =
                r.get(canvas, field::canvas::STATE_AM)?.i();
            r.set(
                canvas,
                field::canvas::STATE_Q,
                V::I(canvas_state_am_2),
            )?;
        }
        let canvas_state_m_50 =
            r.get(canvas, field::canvas::STATE_M)?.i();
        if canvas_state_m_50 > 208 {
            r.set(canvas, field::canvas::STATE_Q, V::I(0))?;
        }
        let canvas_state_l_upper_55 =
            r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
        if canvas_state_l_upper_55 > 256 {
            r.set(canvas, field::canvas::STATE_P, V::I(0))?;
        }
        r.set(canvas, field::canvas::REDRAW, V::I(1))?;
        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1))?;
        r.set(canvas, field::canvas::STATE_AP, V::I(0))?;
        r.set(canvas, field::canvas::STATE_AV, V::I(0))?;
        let canvas_world_60 =
            r.get(canvas, field::canvas::WORLD)?.r();
        let world_zone_27 =
            r.get(canvas_world_60, field::world::ZONE)?.i();
        if world_zone_27 == 9 {
            let canvas_world_61 =
                r.get(canvas, field::canvas::WORLD)?.r();
            let canvas_player_91 =
                r.get(canvas, field::canvas::PLAYER)?.r();
            let drawable_x_19 =
                r.get(canvas_player_91, field::drawable::X)?.i();
            let canvas_player_92 =
                r.get(canvas, field::canvas::PLAYER)?.r();
            let drawable_y_19 =
                r.get(canvas_player_92, field::drawable::Y)?.i();
            let canvas_player_93 =
                r.get(canvas, field::canvas::PLAYER)?.r();
            let can_player_move_value_4 =
                super::world::can_player_move(
                    r,
                    canvas_world_61,
                    drawable_x_19,
                    drawable_y_19,
                    0,
                    canvas_player_93,
                    0,
                )?;
            if can_player_move_value_4 == 0 {
                let canvas_world_64 =
                    r.get(canvas, field::canvas::WORLD)?.r();
                let canvas_player_96 =
                    r.get(canvas, field::canvas::PLAYER)?.r();
                let x_value_20 = r
                    .get(canvas_player_96, field::drawable::X)?
                    .i();
                let canvas_player_97 =
                    r.get(canvas, field::canvas::PLAYER)?.r();
                let y_value_20 = r
                    .get(canvas_player_97, field::drawable::Y)?
                    .i();
                let canvas_player_98 =
                    r.get(canvas, field::canvas::PLAYER)?.r();
                let can_player_move_value_5 =
                    super::world::can_player_move(
                        r,
                        canvas_world_64,
                        x_value_20,
                        y_value_20,
                        1,
                        canvas_player_98,
                        0,
                    )?;
                if can_player_move_value_5 == 0 {
                    let world_value_66 = r
                        .get(canvas, field::canvas::WORLD)?
                        .r();
                    let player_value_103 = r
                        .get(canvas, field::canvas::PLAYER)?
                        .r();
                    let x_value_21 = r
                        .get(
                            player_value_103,
                            field::drawable::X,
                        )?
                        .i();
                    let player_value_104 = r
                        .get(canvas, field::canvas::PLAYER)?
                        .r();
                    let y_value_21 = r
                        .get(
                            player_value_104,
                            field::drawable::Y,
                        )?
                        .i();
                    let player_value_105 = r
                        .get(canvas, field::canvas::PLAYER)?
                        .r();
                    let can_player_move_value_6 =
                        super::world::can_player_move(
                            r,
                            world_value_66,
                            x_value_21,
                            y_value_21,
                            2,
                            player_value_105,
                            0,
                        )?;
                    if can_player_move_value_6 == 0 {
                        let world_value_69 = r
                            .get(canvas, field::canvas::WORLD)?
                            .r();
                        let player_value_113 = r
                            .get(canvas, field::canvas::PLAYER)?
                            .r();
                        let x_value_22 = r
                            .get(
                                player_value_113,
                                field::drawable::X,
                            )?
                            .i();
                        let player_value_114 = r
                            .get(canvas, field::canvas::PLAYER)?
                            .r();
                        let y_value_22 = r
                            .get(
                                player_value_114,
                                field::drawable::Y,
                            )?
                            .i();
                        let player_value_115 = r
                            .get(canvas, field::canvas::PLAYER)?
                            .r();
                        let can_player_move_value_7 =
                            super::world::can_player_move(
                                r,
                                world_value_69,
                                x_value_22,
                                y_value_22,
                                3,
                                player_value_115,
                                0,
                            )?;
                        if can_player_move_value_7 == 0 {
                            let player_value_120 = r
                                .get(
                                    canvas,
                                    field::canvas::PLAYER,
                                )?
                                .r();
                            r.set(
                                player_value_120,
                                field::player::ON_RAFT,
                                V::I(1),
                            )?;
                        }
                    }
                }
            }
        }
    }
    } // check_canvas_state_i
    let canvas_state_i_2 =
        r.get(canvas, field::canvas::STATE_I)?.i();
    if canvas_state_i_2 == 0 {
        let canvas_show_item_message_2 =
            r.get(canvas, field::canvas::SHOW_ITEM_MESSAGE)?.i();
        if canvas_show_item_message_2 == 0 {
            let canvas_world_34 =
                r.get(canvas, field::canvas::WORLD)?.r();
            let world_state_m_43 =
                r.get(canvas_world_34, field::world::STATE_M)?.i();
            if world_state_m_43 == 0 {
                let canvas_player_44 =
                    r.get(canvas, field::canvas::PLAYER)?.r();
                let drawable_x_8 =
                    r.get(canvas_player_44, field::drawable::X)?.i();
                let canvas_player_45 =
                    r.get(canvas, field::canvas::PLAYER)?.r();
                let drawable_y_8 =
                    r.get(canvas_player_45, field::drawable::Y)?.i();
                super::canvas::center_camera(
                    r,
                    canvas,
                    drawable_x_8,
                    drawable_y_8.wrapping_sub(32),
                )?;
                let canvas_state_aq =
                    r.get(canvas, field::canvas::STATE_AQ)?.i();
                if canvas_state_aq != 0 {
                    let canvas_world_40 =
                        r.get(canvas, field::canvas::WORLD)?.r();
                    let hookshot_x_value = r
                        .get(
                            canvas_world_40,
                            field::world::HOOKSHOT_X,
                        )?
                        .i();
                    let canvas_world_41 =
                        r.get(canvas, field::canvas::WORLD)?.r();
                    let hookshot_y_value = r
                        .get(
                            canvas_world_41,
                            field::world::HOOKSHOT_Y,
                        )?
                        .i();
                    super::canvas::center_camera(
                        r,
                        canvas,
                        hookshot_x_value,
                        hookshot_y_value,
                    )?;
                }
                let canvas_world_39 =
                    r.get(canvas, field::canvas::WORLD)?.r();
                let canvas_player_50 =
                    r.get(canvas, field::canvas::PLAYER)?.r();
                super::world::update_enemies(
                    r,
                    canvas_world_39,
                    canvas_player_50,
                )?;
            }
        }
    }
    let canvas_state_af =
        r.get(canvas, field::canvas::STATE_AF)?.j();
    let canvas_state_ag_2 =
        r.get(canvas, field::canvas::STATE_AG)?.j();
    if (if canvas_state_af.wrapping_sub(canvas_state_ag_2) > 1000 {
        1
    } else if canvas_state_af.wrapping_sub(canvas_state_ag_2)
        == 1000
    {
        0
    } else if canvas_state_af.wrapping_sub(canvas_state_ag_2) < 1000
    {
        -1
    } else {
        -1
    }) >= 0
    {
        let system_current_time_millis_value_3 =
            host::system_current_time_millis(r)?;
        r.set(
            canvas,
            field::canvas::STATE_AG,
            V::J(system_current_time_millis_value_3),
        )?;
        let canvas_state_ai =
            r.get(canvas, field::canvas::STATE_AI)?.i();
        r.set(
            canvas,
            field::canvas::STATE_AH,
            V::I(canvas_state_ai),
        )?;
        r.set(canvas, field::canvas::STATE_AI, V::I(0))?;
    } else {
        let canvas_state_ai_2 =
            r.get(canvas, field::canvas::STATE_AI)?.i();
        r.set(
            canvas,
            field::canvas::STATE_AI,
            V::I(canvas_state_ai_2.wrapping_add(1)),
        )?;
    }
    let graphics_get_translate_x_value_55 =
        host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_52 =
        host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_55.wrapping_neg(),
        graphics_get_translate_y_value_52.wrapping_neg(),
    )?;
    host::graphics_translate(r, graphics, 0, 32)?;
    let canvas_state_l_upper_49 =
        r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_44 =
        r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(
        r,
        graphics,
        0,
        0,
        canvas_state_l_upper_49,
        canvas_state_m_44,
    )?;
    let canvas_state_l_upper_50 =
        r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_45 =
        r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(
        r,
        graphics,
        0,
        0,
        canvas_state_l_upper_50,
        canvas_state_m_45,
    )?;
    let canvas_world_38 = r.get(canvas, field::canvas::WORLD)?.r();
    let canvas_player_43 = r.get(canvas, field::canvas::PLAYER)?.r();
    super::world::update_triggers(
        r,
        canvas_world_38,
        canvas_player_43,
    )?;
    object_2 = canvas;
    let canvas_zone_11 = r.get(canvas, field::canvas::ZONE)?.i();
    if canvas_zone_11 != 12 {
        break 'check_canvas_state_f;
    }
    } // check_canvas_state_f_2
    let canvas_state_f_2 = r.get(object_2, field::canvas::STATE_F)?.i();
    let canvas_state_av_upper =
        r.get(object_2, field::canvas::STATE_AV_UPPER)?.i();
    if canvas_state_f_2 <= canvas_state_av_upper {
        break 'check_canvas_state_f_4;
    }
    } // check_canvas_random
    let canvas_random_3 = r.get(object_2, field::canvas::RANDOM)?.r();
    let random_next_int_value_3 = host::random_next_int(r, canvas_random_3)?;
    let remainder_value_11 =
        arithmetic::remainder_int(r, random_next_int_value_3, 200)?;
    let math_abs_value_7 = host::math_abs(r, remainder_value_11)?;
    r.set(
        object_2,
        field::canvas::STATE_AV_UPPER,
        V::I(math_abs_value_7.wrapping_add(50)),
    )?;
    r.set(object_2, field::canvas::STATE_F, V::I(0))?;
    break 'check_canvas_state_at_upper;
    } // check_canvas_state_f_4
    let canvas_state_f_4 = r.get(object_2, field::canvas::STATE_F)?.i();
    let canvas_state_av_upper_2 =
        r.get(object_2, field::canvas::STATE_AV_UPPER)?.i();
    if canvas_state_f_4.wrapping_add(30) <= canvas_state_av_upper_2 {
        r.set(object_2, field::canvas::STATE_AT_UPPER, V::I(0))?;
        r.set(object_2, field::canvas::STATE_AU_UPPER, V::I(0))?;
    } else {
        let canvas_random_4 = r.get(object_2, field::canvas::RANDOM)?.r();
        let random_next_int_value_4 = host::random_next_int(r, canvas_random_4)?;
        let remainder_value_13 =
            arithmetic::remainder_int(r, random_next_int_value_4, 5)?;
        r.set(
            object_2,
            field::canvas::STATE_AT_UPPER,
            V::I(remainder_value_13),
        )?;
        let canvas_random_5 = r.get(object_2, field::canvas::RANDOM)?.r();
        let random_next_int_value_5 = host::random_next_int(r, canvas_random_5)?;
        let remainder_value_14 =
            arithmetic::remainder_int(r, random_next_int_value_5, 5)?;
        r.set(
            object_2,
            field::canvas::STATE_AU_UPPER,
            V::I(remainder_value_14),
        )?;
    }
    let canvas_state_f_5 = r.get(object_2, field::canvas::STATE_F)?.i();
    r.set(
        object_2,
        field::canvas::STATE_F,
        V::I(canvas_state_f_5.wrapping_add(1)),
    )?;
    break 'check_canvas_scene_image_8;
    } // check_canvas_state_f
    let canvas_state_f = r.get(object_2, field::canvas::STATE_F)?.i();
    if canvas_state_f <= 0 {
        break 'check_canvas_state_at_upper;
    }
    } // check_canvas_state_f_3
    let canvas_state_f_3 = r.get(object_2, field::canvas::STATE_F)?.i();
    r.set(
        object_2,
        field::canvas::STATE_F,
        V::I(canvas_state_f_3.wrapping_sub(1)),
    )?;
    let canvas_random = r.get(object_2, field::canvas::RANDOM)?.r();
    let random_next_int_value = host::random_next_int(r, canvas_random)?;
    let remainder_value_9 = arithmetic::remainder_int(r, random_next_int_value, 5)?;
    r.set(
        object_2,
        field::canvas::STATE_AT_UPPER,
        V::I(remainder_value_9),
    )?;
    let canvas_random_2 = r.get(object_2, field::canvas::RANDOM)?.r();
    let random_next_int_value_2 = host::random_next_int(r, canvas_random_2)?;
    let remainder_value_10 = arithmetic::remainder_int(r, random_next_int_value_2, 5)?;
    r.set(
        object_2,
        field::canvas::STATE_AU_UPPER,
        V::I(remainder_value_10),
    )?;
    break 'check_canvas_scene_image_8;
    } // check_canvas_state_at_upper
    r.set(object_2, field::canvas::STATE_AT_UPPER, V::I(0))?;
    r.set(object_2, field::canvas::STATE_AU_UPPER, V::I(0))?;
    } // check_canvas_scene_image_8
    let canvas_scene_image_13 = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let canvas_state_p_3 = r.get(canvas, field::canvas::STATE_P)?.i();
    let canvas_state_at_upper = r.get(canvas, field::canvas::STATE_AT_UPPER)?.i();
    let canvas_state_q_3 = r.get(canvas, field::canvas::STATE_Q)?.i();
    let canvas_state_au_upper = r.get(canvas, field::canvas::STATE_AU_UPPER)?.i();
    host::graphics_draw_image(
        r,
        graphics,
        canvas_scene_image_13,
        (canvas_state_p_3.wrapping_neg()).wrapping_add(canvas_state_at_upper),
        (canvas_state_q_3.wrapping_neg()).wrapping_add(canvas_state_au_upper),
        0,
    )?;
    let graphics_get_translate_x_value_60 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_57 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_60.wrapping_neg(),
        graphics_get_translate_y_value_57.wrapping_neg(),
    )?;
    host::graphics_translate(r, graphics, 0, 32)?;
    let canvas_state_l_upper_51 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_46 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_51, canvas_state_m_46)?;
    let canvas_state_l_upper_52 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let canvas_state_m_47 = r.get(canvas, field::canvas::STATE_M)?.i();
    host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_52, canvas_state_m_47)?;
    let canvas_state_aw = r.get(canvas, field::canvas::STATE_AW)?.r();
    host::vector_remove_all_elements(r, canvas_state_aw)?;
    number_2 = 0;
    } // check_canvas_world_22
    'paint_loop_115: loop {
        'advance_index: {
        'check_canvas_world_23: {
        'check_canvas_world_26: {
        'check_canvas_world_25: {
        'check_canvas_world_24: {
        let canvas_world_49 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_15 =
            r.get(canvas_world_49, field::world::OBJECTS)?.r();
        let vector_size_value_9 = host::vector_size(r, world_objects_15)?;
        if number_2 >= vector_size_value_9 {
            let canvas_player_69 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_state_g =
                r.get(canvas_player_69, field::player::STATE_G)?.i();
            if player_state_g == 0 {
                let canvas_player_73 = r.get(canvas, field::canvas::PLAYER)?.r();
                super::canvas::remove_menu_entry(r, canvas, canvas_player_73)?;
            }
            number_2 = 0;
            loop {
                let canvas_world_53 = r.get(canvas, field::canvas::WORLD)?.r();
                let world_items_3 =
                    r.get(canvas_world_53, field::world::ITEMS)?.r();
                let vector_size_value_10 = host::vector_size(r, world_items_3)?;
                if number_2 >= vector_size_value_10 {
                    number_2 = 0;
                    loop {
                        let canvas_world_57 =
                            r.get(canvas, field::canvas::WORLD)?.r();
                        let world_enemies =
                            r.get(canvas_world_57, field::world::ENEMIES)?.r();
                        let vector_size_value_11 =
                            host::vector_size(r, world_enemies)?;
                        if number_2 >= vector_size_value_11 {
                            number_2 = 0;
                            loop {
                                let canvas_state_aw_2 =
                                    r.get(canvas, field::canvas::STATE_AW)?.r();
                                let vector_size_value_12 =
                                    host::vector_size(r, canvas_state_aw_2)?;
                                if number_2 >= vector_size_value_12 {
                                    let state_i_value_3 = r
                                        .get(canvas, field::canvas::STATE_I)?
                                        .i();
                                    if state_i_value_3 == 0 {
                                        let canvas_show_item_message_3 = r.get(canvas, field::canvas::SHOW_ITEM_MESSAGE)?.i();
                                        if canvas_show_item_message_3 == 0 {
                                            let world_value_63 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::WORLD,
                                                )?
                                                .r();
                                            let state_m_value_52 = r
                                                .get(
                                                    world_value_63,
                                                    field::world::STATE_M,
                                                )?
                                                .i();
                                            if state_m_value_52 == 0 {
                                                let player_value_100 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::PLAYER,
                                                    )?
                                                    .r();
                                                let player_attacking_3 = r.get(player_value_100, field::player::ATTACKING)?.i();
                                                if player_attacking_3 == 0 {
                                                    let canvas_player_108 = r.get(canvas, field::canvas::PLAYER)?.r();
                                                    let player_hookshot_extending_3 = r.get(canvas_player_108, field::player::HOOKSHOT_EXTENDING)?.i();
                                                    if player_hookshot_extending_3 == 0 {
let canvas_world_71 = r.get(canvas, field::canvas::WORLD)?.r();
let canvas_player_116 = r.get(canvas, field::canvas::PLAYER)?.r();
let canvas_sound_5 = r.get(canvas, field::canvas::SOUND)?.r();
let update_contacts_value = super::world::update_contacts(r, canvas_world_71, canvas_player_116, canvas_sound_5)?;
if update_contacts_value > 0 {
r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1))?;
}
} else {
let canvas_world_72 = r.get(canvas, field::canvas::WORLD)?.r();
let canvas_player_117 = r.get(canvas, field::canvas::PLAYER)?.r();
super::world::update_hookshot(r, canvas_world_72, canvas_player_117)?;
}
                                                } else {
                                                    let canvas_world_68 = r.get(canvas, field::canvas::WORLD)?.r();
                                                    let canvas_player_109 = r.get(canvas, field::canvas::PLAYER)?.r();
                                                    let canvas_sound_4 = r.get(canvas, field::canvas::SOUND)?.r();
                                                    let canvas_player_110 = r.get(canvas, field::canvas::PLAYER)?.r();
                                                    let player_has_sword_2 = r.get(canvas_player_110, field::player::HAS_SWORD)?.i();
                                                    let interact_value =
                                                        super::world::interact(
                                                            r,
                                                            canvas_world_68,
                                                            canvas_player_109,
                                                            canvas_sound_4,
                                                            player_has_sword_2,
                                                        )?;
                                                    number_2 = interact_value;
                                                    if interact_value > 0 {
                                                        r.set(canvas, field::canvas::REDRAW_WORLD, V::I(1))?;
                                                    }
                                                }
                                                let state_aq_value_2 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_AQ,
                                                    )?
                                                    .i();
                                                if state_aq_value_2 != 0 {
                                                    let canvas_world_76 = r.get(canvas, field::canvas::WORLD)?.r();
                                                    let canvas_state_ab_upper = r.get(canvas, field::canvas::STATE_AB_UPPER)?.i();
                                                    let canvas_state_ac_upper = r.get(canvas, field::canvas::STATE_AC_UPPER)?.i();
                                                    let update_hookshot_target_value = super::world::update_hookshot_target(r, canvas_world_76, canvas_state_ab_upper, canvas_state_ac_upper)?;
                                                }
                                                let canvas_redraw_world_3 = r.get(canvas, field::canvas::REDRAW_WORLD)?.i();
                                                let world_value_75 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::WORLD,
                                                    )?
                                                    .r();
                                                let state_p_value_12 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_P,
                                                    )?
                                                    .i();
                                                let canvas_state_at_upper_7 = r.get(canvas, field::canvas::STATE_AT_UPPER)?.i();
                                                let state_q_value_12 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_Q,
                                                    )?
                                                    .i();
                                                let canvas_state_au_upper_7 = r.get(canvas, field::canvas::STATE_AU_UPPER)?.i();
                                                let canvas_state_l_upper_64 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
                                                let state_m_value_59 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_M,
                                                    )?
                                                    .i();
                                                let player_value_121 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::PLAYER,
                                                    )?
                                                    .r();
                                                let sound_value_6 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::SOUND,
                                                    )?
                                                    .r();
                                                let update_projectiles_value = super::world::update_projectiles(r, world_value_75, graphics, state_p_value_12.wrapping_add(canvas_state_at_upper_7), state_q_value_12.wrapping_add(canvas_state_au_upper_7), canvas_state_l_upper_64, state_m_value_59, player_value_121, sound_value_6)?;
                                                r.set(canvas, field::canvas::REDRAW_WORLD, V::I((canvas_redraw_world_3 | update_projectiles_value)))?;
                                            }
                                        }
                                    }
                                    number_2 = 0;
                                    loop {
                                        let world_value_62 = r
                                            .get(canvas, field::canvas::WORLD)?
                                            .r();
                                        let overlay_objects_value = r
                                            .get(
                                                world_value_62,
                                                field::world::OVERLAY_OBJECTS,
                                            )?
                                            .r();
                                        let vector_size_value_13 =
                                            host::vector_size(
                                                r,
                                                overlay_objects_value,
                                            )?;
                                        if number_2 >= vector_size_value_13 {
                                            let player_value_99 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::PLAYER,
                                                )?
                                                .r();
                                            let state_g_value_2 = r
                                                .get(
                                                    player_value_99,
                                                    field::player::STATE_G,
                                                )?
                                                .i();
                                            if state_g_value_2 == 1 {
                                                let player_value_107 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::PLAYER,
                                                    )?
                                                    .r();
                                                let state_p_value_11 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_P,
                                                    )?
                                                    .i();
                                                let canvas_state_at_upper_6 = r.get(canvas, field::canvas::STATE_AT_UPPER)?.i();
                                                let state_q_value_11 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_Q,
                                                    )?
                                                    .i();
                                                let canvas_state_au_upper_6 = r.get(canvas, field::canvas::STATE_AU_UPPER)?.i();
                                                let canvas_state_l_upper_61 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
                                                let state_m_value_55 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_M,
                                                    )?
                                                    .i();
                                                super::player::draw(
                                                    r,
                                                    player_value_107,
                                                    graphics,
                                                    (state_p_value_11)
                                                        .wrapping_add(
                                                        canvas_state_at_upper_6,
                                                    ),
                                                    (state_q_value_11)
                                                        .wrapping_add(
                                                        canvas_state_au_upper_6,
                                                    ),
                                                    canvas_state_l_upper_61,
                                                    state_m_value_55,
                                                )?;
                                            }
                                            let world_value_67 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::WORLD,
                                                )?
                                                .r();
                                            let state_p_value_9 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::STATE_P,
                                                )?
                                                .i();
                                            let canvas_state_at_upper_5 = r.get(canvas, field::canvas::STATE_AT_UPPER)?.i();
                                            let state_q_value_9 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::STATE_Q,
                                                )?
                                                .i();
                                            let canvas_state_au_upper_5 = r.get(canvas, field::canvas::STATE_AU_UPPER)?.i();
                                            let canvas_state_l_upper_58 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
                                            let state_m_value_54 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::STATE_M,
                                                )?
                                                .i();
                                            let player_value_106 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::PLAYER,
                                                )?
                                                .r();
                                            super::world::draw(
                                                r,
                                                world_value_67,
                                                graphics,
                                                state_p_value_9.wrapping_add(
                                                    canvas_state_at_upper_5,
                                                ),
                                                state_q_value_9.wrapping_add(
                                                    canvas_state_au_upper_5,
                                                ),
                                                canvas_state_l_upper_58,
                                                state_m_value_54,
                                                player_value_106,
                                            )?;
                                            super::canvas::draw_inventory(
                                                r, canvas, graphics, 1,
                                            )?;
                                            let state_p_value_10 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::STATE_P,
                                                )?
                                                .i();
                                            let state_q_value_10 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::STATE_Q,
                                                )?
                                                .i();
                                            super::canvas::draw_title(
                                                r,
                                                canvas,
                                                graphics,
                                                state_p_value_10,
                                                state_q_value_10,
                                            )?;
                                            let graphics_get_translate_x_value_71 =
                                                host::graphics_get_translate_x(
                                                    r, graphics,
                                                )?;
                                            let graphics_get_translate_y_value_68 =
                                                host::graphics_get_translate_y(
                                                    r, graphics,
                                                )?;
                                            host::graphics_translate(r, graphics, graphics_get_translate_x_value_71.wrapping_neg(), graphics_get_translate_y_value_68.wrapping_neg())?;
                                            let canvas_state_l_upper_59 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
                                            host::graphics_set_clip(
                                                r,
                                                graphics,
                                                0,
                                                0,
                                                canvas_state_l_upper_59,
                                                32,
                                            )?;
                                            let canvas_state_l_upper_60 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
                                            host::graphics_clip_rect(
                                                r,
                                                graphics,
                                                0,
                                                0,
                                                canvas_state_l_upper_60,
                                                32,
                                            )?;
                                            let state_s_value_5 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::STATE_S,
                                                )?
                                                .r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                state_s_value_5,
                                                0,
                                                0,
                                                0,
                                            )?;
                                            super::canvas::draw_settings(
                                                r, canvas, graphics,
                                            )?;
                                            let state_i_value_4 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::STATE_I,
                                                )?
                                                .i();
                                            if state_i_value_4 != 0 {
                                                let world_value_70 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::WORLD,
                                                    )?
                                                    .r();
                                                let state_l_value = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_L,
                                                    )?
                                                    .i();
                                                let canvas_state_l_upper_62 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
                                                let state_m_value_56 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_M,
                                                    )?
                                                    .i();
                                                let draw_dialogue_value = super::world::draw_dialogue(r, world_value_70, state_l_value, graphics, canvas_state_l_upper_62, state_m_value_56)?;
                                            }
                                            let canvas_show_item_message_4 = r.get(canvas, field::canvas::SHOW_ITEM_MESSAGE)?.i();
                                            if canvas_show_item_message_4 != 0 {
                                                let world_value_74 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::WORLD,
                                                    )?
                                                    .r();
                                                let canvas_state_l_upper_63 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
                                                let state_m_value_58 = r
                                                    .get(
                                                        canvas,
                                                        field::canvas::STATE_M,
                                                    )?
                                                    .i();
                                                let canvas_item_message = r.get(canvas, field::canvas::ITEM_MESSAGE)?.i();
                                                let draw_item_message_value = super::world::draw_item_message(r, world_value_74, graphics, canvas_state_l_upper_63, state_m_value_58, canvas_item_message)?;
                                            }
                                            let world_value_73 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::WORLD,
                                                )?
                                                .r();
                                            let state_m_value_57 = r
                                                .get(
                                                    world_value_73,
                                                    field::world::STATE_M,
                                                )?
                                                .i();
                                            if state_m_value_57 == 0 {
                                                return Ok(());
                                            }
                                            let world_value_77 = r
                                                .get(
                                                    canvas,
                                                    field::canvas::WORLD,
                                                )?
                                                .r();
                                            object_3 = graphics;
                                            object_2 = world_value_77;
                                            let state_d_value = r
                                                .get(
                                                    world_value_77,
                                                    field::world::STATE_D,
                                                )?
                                                .r();
                                            if state_d_value == 0 {
                                                let object_value_22 = r.alloc(
                                                    "java/lang/StringBuffer",
                                                    Native::Empty,
                                                );
                                                outcome = (|| -> JResult {
                                                    host::string_buffer_initialize(r, object_value_22)?;
                                                    Ok(V::I(0))
                                                })(
                                                );
                                                (carry_0_r, carry_1_r) =
                                                    (object_2, object_value_22);
                                                if outcome.is_err() {
                                                    carry_0_r =
                                                        outcome.unwrap_err();
                                                    if !(r.is_instance(
                                                        carry_0_r,
                                                        "java/io/IOException",
                                                    )) {
                                                        return Err(outcome
                                                            .unwrap_err());
                                                    }
                                                    let state_out_value = r.static_get(field::system::STATE_OUT).r();
                                                    let text_value = r.intern(
                                                        "error loadin Map img",
                                                    );
                                                    host::print_stream_println(
                                                        r,
                                                        state_out_value,
                                                        text_value,
                                                    )?;
                                                    return Ok(());
                                                }
                                                let text_value_158 = r
                                                    .intern("/Images/BossName");
                                                outcome = (|| -> JResult {
                                                    Ok(V::R(host::string_buffer_append_2(r, carry_1_r, text_value_158)?))
                                                })(
                                                );
                                                (carry_0_r,) = (carry_0_r,);
                                                if outcome.is_err() {
                                                    carry_0_r =
                                                        outcome.unwrap_err();
                                                    if !(r.is_instance(
                                                        carry_0_r,
                                                        "java/io/IOException",
                                                    )) {
                                                        return Err(outcome
                                                            .unwrap_err());
                                                    }
                                                    let state_out_value = r.static_get(field::system::STATE_OUT).r();
                                                    let text_value = r.intern(
                                                        "error loadin Map img",
                                                    );
                                                    host::print_stream_println(
                                                        r,
                                                        state_out_value,
                                                        text_value,
                                                    )?;
                                                    return Ok(());
                                                }
                                                let string_buffer_append_2_value_37 =
                                                    outcome.unwrap().r();
                                                let zone_value_36 = r
                                                    .get(
                                                        object_2,
                                                        field::world::ZONE,
                                                    )?
                                                    .i();
                                                outcome = (|| -> JResult {
                                                    Ok(V::R(host::string_buffer_append(r, string_buffer_append_2_value_37, zone_value_36)?))
                                                })(
                                                );
                                                (carry_0_r,) = (carry_0_r,);
                                                if outcome.is_err() {
                                                    carry_0_r =
                                                        outcome.unwrap_err();
                                                    if !(r.is_instance(
                                                        carry_0_r,
                                                        "java/io/IOException",
                                                    )) {
                                                        return Err(outcome
                                                            .unwrap_err());
                                                    }
                                                    let state_out_value = r.static_get(field::system::STATE_OUT).r();
                                                    let text_value = r.intern(
                                                        "error loadin Map img",
                                                    );
                                                    host::print_stream_println(
                                                        r,
                                                        state_out_value,
                                                        text_value,
                                                    )?;
                                                    return Ok(());
                                                }
                                                let string_buffer_append_value_13 =
                                                    outcome.unwrap().r();
                                                let text_value_159 =
                                                    r.intern(".png");
                                                outcome = (|| -> JResult {
                                                    Ok(V::R(host::string_buffer_append_2(r, string_buffer_append_value_13, text_value_159)?))
                                                })(
                                                );
                                                (carry_0_r,) = (carry_0_r,);
                                                if outcome.is_err() {
                                                    carry_0_r =
                                                        outcome.unwrap_err();
                                                    if !(r.is_instance(
                                                        carry_0_r,
                                                        "java/io/IOException",
                                                    )) {
                                                        return Err(outcome
                                                            .unwrap_err());
                                                    }
                                                    let state_out_value = r.static_get(field::system::STATE_OUT).r();
                                                    let text_value = r.intern(
                                                        "error loadin Map img",
                                                    );
                                                    host::print_stream_println(
                                                        r,
                                                        state_out_value,
                                                        text_value,
                                                    )?;
                                                    return Ok(());
                                                }
                                                let string_buffer_append_2_value_38 =
                                                    outcome.unwrap().r();
                                                outcome = (|| -> JResult {
                                                    Ok(V::R(host::string_buffer_to_string(r, string_buffer_append_2_value_38)?))
                                                })(
                                                );
                                                (carry_0_r,) = (carry_0_r,);
                                                if outcome.is_err() {
                                                    carry_0_r =
                                                        outcome.unwrap_err();
                                                    if !(r.is_instance(
                                                        carry_0_r,
                                                        "java/io/IOException",
                                                    )) {
                                                        return Err(outcome
                                                            .unwrap_err());
                                                    }
                                                    let state_out_value = r.static_get(field::system::STATE_OUT).r();
                                                    let text_value = r.intern(
                                                        "error loadin Map img",
                                                    );
                                                    host::print_stream_println(
                                                        r,
                                                        state_out_value,
                                                        text_value,
                                                    )?;
                                                    return Ok(());
                                                }
                                                let string_buffer_to_string_value_21 =
                                                    outcome.unwrap().r();
                                                outcome = (|| -> JResult {
                                                    Ok(V::R(host::image_create_image(r, string_buffer_to_string_value_21)?))
                                                })(
                                                );
                                                (carry_0_r,) = (carry_0_r,);
                                                if outcome.is_err() {
                                                    carry_0_r =
                                                        outcome.unwrap_err();
                                                    if !(r.is_instance(
                                                        carry_0_r,
                                                        "java/io/IOException",
                                                    )) {
                                                        return Err(outcome
                                                            .unwrap_err());
                                                    }
                                                    let state_out_value = r.static_get(field::system::STATE_OUT).r();
                                                    let text_value = r.intern(
                                                        "error loadin Map img",
                                                    );
                                                    host::print_stream_println(
                                                        r,
                                                        state_out_value,
                                                        text_value,
                                                    )?;
                                                    return Ok(());
                                                }
                                                let image_create_image_value_2 =
                                                    outcome.unwrap().r();
                                                r.set(carry_0_r, field::world::STATE_D, V::R(image_create_image_value_2))?;
                                            }
                                            let zone_value_35 = r
                                                .get(
                                                    object_2,
                                                    field::world::ZONE,
                                                )?
                                                .i();
                                            match zone_value_35 {
                                                3 => {
                                                    let graphics_get_translate_x_value_72 = host::graphics_get_translate_x(r, object_3)?;
                                                    let graphics_get_translate_y_value_69 = host::graphics_get_translate_y(r, object_3)?;
                                                    host::graphics_translate(r, object_3, graphics_get_translate_x_value_72.wrapping_neg(), graphics_get_translate_y_value_69.wrapping_neg())?;
                                                    let world_state_i_5 = r.get(object_2, field::world::STATE_I)?.i();
                                                    let quotient_value_114 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_i_5,
                                                            2,
                                                        )?;
                                                    let world_state_j = r.get(object_2, field::world::STATE_J)?.i();
                                                    let quotient_value_115 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_j,
                                                            2,
                                                        )?;
                                                    host::graphics_translate(
                                                        r,
                                                        object_3,
                                                        (quotient_value_114)
                                                            .wrapping_sub(65),
                                                        (quotient_value_115)
                                                            .wrapping_sub(23),
                                                    )?;
                                                    host::graphics_set_clip(
                                                        r, object_3, 0, 0, 131,
                                                        47,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, object_3, 0, 0, 131,
                                                        47,
                                                    )?;
                                                    let world_state_d_2 = r.get(object_2, field::world::STATE_D)?.r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        object_3,
                                                        world_state_d_2,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                    return Ok(());
                                                }
                                                4 => {
                                                    let graphics_get_translate_x_value_73 = host::graphics_get_translate_x(r, object_3)?;
                                                    let graphics_get_translate_y_value_70 = host::graphics_get_translate_y(r, object_3)?;
                                                    host::graphics_translate(r, object_3, graphics_get_translate_x_value_73.wrapping_neg(), graphics_get_translate_y_value_70.wrapping_neg())?;
                                                    let world_state_i_6 = r.get(object_2, field::world::STATE_I)?.i();
                                                    let quotient_value_116 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_i_6,
                                                            2,
                                                        )?;
                                                    let world_state_j_2 = r.get(object_2, field::world::STATE_J)?.i();
                                                    let quotient_value_117 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_j_2,
                                                            2,
                                                        )?;
                                                    host::graphics_translate(
                                                        r,
                                                        object_3,
                                                        (quotient_value_116)
                                                            .wrapping_sub(54),
                                                        (quotient_value_117)
                                                            .wrapping_sub(23),
                                                    )?;
                                                    host::graphics_set_clip(
                                                        r, object_3, 0, 0, 109,
                                                        47,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, object_3, 0, 0, 109,
                                                        47,
                                                    )?;
                                                    let world_state_d_3 = r.get(object_2, field::world::STATE_D)?.r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        object_3,
                                                        world_state_d_3,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                    return Ok(());
                                                }
                                                5 => {
                                                    let graphics_get_translate_x_value_74 = host::graphics_get_translate_x(r, object_3)?;
                                                    let graphics_get_translate_y_value_71 = host::graphics_get_translate_y(r, object_3)?;
                                                    host::graphics_translate(r, object_3, graphics_get_translate_x_value_74.wrapping_neg(), graphics_get_translate_y_value_71.wrapping_neg())?;
                                                    let world_state_i_7 = r.get(object_2, field::world::STATE_I)?.i();
                                                    let quotient_value_118 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_i_7,
                                                            2,
                                                        )?;
                                                    let world_state_j_3 = r.get(object_2, field::world::STATE_J)?.i();
                                                    let quotient_value_119 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_j_3,
                                                            2,
                                                        )?;
                                                    host::graphics_translate(
                                                        r,
                                                        object_3,
                                                        (quotient_value_118)
                                                            .wrapping_sub(63),
                                                        (quotient_value_119)
                                                            .wrapping_sub(22),
                                                    )?;
                                                    host::graphics_set_clip(
                                                        r, object_3, 0, 0, 127,
                                                        45,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, object_3, 0, 0, 127,
                                                        45,
                                                    )?;
                                                    let world_state_d_4 = r.get(object_2, field::world::STATE_D)?.r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        object_3,
                                                        world_state_d_4,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                    return Ok(());
                                                }
                                                6 => {
                                                    let graphics_get_translate_x_value_75 = host::graphics_get_translate_x(r, object_3)?;
                                                    let graphics_get_translate_y_value_72 = host::graphics_get_translate_y(r, object_3)?;
                                                    host::graphics_translate(r, object_3, graphics_get_translate_x_value_75.wrapping_neg(), graphics_get_translate_y_value_72.wrapping_neg())?;
                                                    let world_state_i_8 = r.get(object_2, field::world::STATE_I)?.i();
                                                    let quotient_value_120 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_i_8,
                                                            2,
                                                        )?;
                                                    let world_state_j_4 = r.get(object_2, field::world::STATE_J)?.i();
                                                    let quotient_value_121 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_j_4,
                                                            2,
                                                        )?;
                                                    host::graphics_translate(
                                                        r,
                                                        object_3,
                                                        (quotient_value_120)
                                                            .wrapping_sub(53),
                                                        (quotient_value_121)
                                                            .wrapping_sub(24),
                                                    )?;
                                                    host::graphics_set_clip(
                                                        r, object_3, 0, 0, 106,
                                                        48,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, object_3, 0, 0, 106,
                                                        48,
                                                    )?;
                                                    let world_state_d_5 = r.get(object_2, field::world::STATE_D)?.r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        object_3,
                                                        world_state_d_5,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                    return Ok(());
                                                }
                                                7 => {
                                                    let graphics_get_translate_x_value_76 = host::graphics_get_translate_x(r, object_3)?;
                                                    let graphics_get_translate_y_value_73 = host::graphics_get_translate_y(r, object_3)?;
                                                    host::graphics_translate(r, object_3, graphics_get_translate_x_value_76.wrapping_neg(), graphics_get_translate_y_value_73.wrapping_neg())?;
                                                    let world_state_i_9 = r.get(object_2, field::world::STATE_I)?.i();
                                                    let quotient_value_122 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_i_9,
                                                            2,
                                                        )?;
                                                    let world_state_j_5 = r.get(object_2, field::world::STATE_J)?.i();
                                                    let quotient_value_123 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_j_5,
                                                            2,
                                                        )?;
                                                    host::graphics_translate(
                                                        r,
                                                        object_3,
                                                        (quotient_value_122)
                                                            .wrapping_sub(50),
                                                        (quotient_value_123)
                                                            .wrapping_sub(40),
                                                    )?;
                                                    host::graphics_set_clip(
                                                        r, object_3, 0, 0, 100,
                                                        80,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, object_3, 0, 0, 100,
                                                        80,
                                                    )?;
                                                    let world_state_d_6 = r.get(object_2, field::world::STATE_D)?.r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        object_3,
                                                        world_state_d_6,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                    return Ok(());
                                                }
                                                8 => {
                                                    let graphics_get_translate_x_value_77 = host::graphics_get_translate_x(r, object_3)?;
                                                    let graphics_get_translate_y_value_74 = host::graphics_get_translate_y(r, object_3)?;
                                                    host::graphics_translate(r, object_3, graphics_get_translate_x_value_77.wrapping_neg(), graphics_get_translate_y_value_74.wrapping_neg())?;
                                                    let world_state_i_10 = r.get(object_2, field::world::STATE_I)?.i();
                                                    let quotient_value_124 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_i_10,
                                                            2,
                                                        )?;
                                                    let world_state_j_6 = r.get(object_2, field::world::STATE_J)?.i();
                                                    let quotient_value_125 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_j_6,
                                                            2,
                                                        )?;
                                                    host::graphics_translate(
                                                        r,
                                                        object_3,
                                                        (quotient_value_124)
                                                            .wrapping_sub(64),
                                                        (quotient_value_125)
                                                            .wrapping_sub(22),
                                                    )?;
                                                    host::graphics_set_clip(
                                                        r, object_3, 0, 0, 128,
                                                        44,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, object_3, 0, 0, 128,
                                                        44,
                                                    )?;
                                                    let world_state_d_7 = r.get(object_2, field::world::STATE_D)?.r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        object_3,
                                                        world_state_d_7,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                    return Ok(());
                                                }
                                                9 => {
                                                    let graphics_get_translate_x_value_78 = host::graphics_get_translate_x(r, object_3)?;
                                                    let graphics_get_translate_y_value_75 = host::graphics_get_translate_y(r, object_3)?;
                                                    host::graphics_translate(r, object_3, graphics_get_translate_x_value_78.wrapping_neg(), graphics_get_translate_y_value_75.wrapping_neg())?;
                                                    let world_state_i_11 = r.get(object_2, field::world::STATE_I)?.i();
                                                    let quotient_value_126 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_i_11,
                                                            2,
                                                        )?;
                                                    let world_state_j_7 = r.get(object_2, field::world::STATE_J)?.i();
                                                    let quotient_value_127 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_j_7,
                                                            2,
                                                        )?;
                                                    host::graphics_translate(
                                                        r,
                                                        object_3,
                                                        (quotient_value_126)
                                                            .wrapping_sub(66),
                                                        (quotient_value_127)
                                                            .wrapping_sub(23),
                                                    )?;
                                                    host::graphics_set_clip(
                                                        r, object_3, 0, 0, 132,
                                                        46,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, object_3, 0, 0, 132,
                                                        46,
                                                    )?;
                                                    let world_state_d_8 = r.get(object_2, field::world::STATE_D)?.r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        object_3,
                                                        world_state_d_8,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                    return Ok(());
                                                }
                                                10 => {
                                                    let graphics_get_translate_x_value_79 = host::graphics_get_translate_x(r, object_3)?;
                                                    let graphics_get_translate_y_value_76 = host::graphics_get_translate_y(r, object_3)?;
                                                    host::graphics_translate(r, object_3, graphics_get_translate_x_value_79.wrapping_neg(), graphics_get_translate_y_value_76.wrapping_neg())?;
                                                    let world_state_i_12 = r.get(object_2, field::world::STATE_I)?.i();
                                                    let quotient_value_128 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_i_12,
                                                            2,
                                                        )?;
                                                    let world_state_j_8 = r.get(object_2, field::world::STATE_J)?.i();
                                                    let quotient_value_129 =
                                                        arithmetic::divide_int(
                                                            r,
                                                            world_state_j_8,
                                                            2,
                                                        )?;
                                                    host::graphics_translate(
                                                        r,
                                                        object_3,
                                                        (quotient_value_128)
                                                            .wrapping_sub(66),
                                                        (quotient_value_129)
                                                            .wrapping_sub(23),
                                                    )?;
                                                    host::graphics_set_clip(
                                                        r, object_3, 0, 0, 132,
                                                        46,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, object_3, 0, 0, 132,
                                                        46,
                                                    )?;
                                                    let world_state_d_9 = r.get(object_2, field::world::STATE_D)?.r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        object_3,
                                                        world_state_d_9,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                _ => {}
                                            }
                                            return Ok(());
                                        }
                                        let world_value_65 = r
                                            .get(canvas, field::canvas::WORLD)?
                                            .r();
                                        let overlay_objects_value_2 = r
                                            .get(
                                                world_value_65,
                                                field::world::OVERLAY_OBJECTS,
                                            )?
                                            .r();
                                        let vector_element_at_value_19 =
                                            host::vector_element_at(
                                                r,
                                                overlay_objects_value_2,
                                                number_2,
                                            )?;
                                        host::check_cast(
                                            r,
                                            vector_element_at_value_19,
                                            "n",
                                        )?;
                                        let state_p_value_8 = r
                                            .get(
                                                canvas,
                                                field::canvas::STATE_P,
                                            )?
                                            .i();
                                        let state_at_upper_value_4 = r
                                            .get(
                                                canvas,
                                                field::canvas::STATE_AT_UPPER,
                                            )?
                                            .i();
                                        let state_q_value_8 = r
                                            .get(
                                                canvas,
                                                field::canvas::STATE_Q,
                                            )?
                                            .i();
                                        let state_au_upper_value_4 = r
                                            .get(
                                                canvas,
                                                field::canvas::STATE_AU_UPPER,
                                            )?
                                            .i();
                                        let state_l_upper_value_57 = r
                                            .get(
                                                canvas,
                                                field::canvas::STATE_L_UPPER,
                                            )?
                                            .i();
                                        let state_m_value_53 = r
                                            .get(
                                                canvas,
                                                field::canvas::STATE_M,
                                            )?
                                            .i();
                                        super::map_object::draw(
                                            r,
                                            vector_element_at_value_19,
                                            graphics,
                                            state_p_value_8.wrapping_add(
                                                state_at_upper_value_4,
                                            ),
                                            state_q_value_8.wrapping_add(
                                                state_au_upper_value_4,
                                            ),
                                            state_l_upper_value_57,
                                            state_m_value_53,
                                        )?;
                                        number_2 = number_2.wrapping_add(1);
                                    }
                                }
                                let canvas_state_aw_3 =
                                    r.get(canvas, field::canvas::STATE_AW)?.r();
                                let vector_element_at_value_18 =
                                    host::vector_element_at(
                                        r,
                                        canvas_state_aw_3,
                                        number_2,
                                    )?;
                                host::check_cast(
                                    r,
                                    vector_element_at_value_18,
                                    "f",
                                )?;
                                object_3 = vector_element_at_value_18;
                                let canvas_state_p_7 =
                                    r.get(canvas, field::canvas::STATE_P)?.i();
                                let state_at_upper_value_3 = r
                                    .get(canvas, field::canvas::STATE_AT_UPPER)?
                                    .i();
                                let canvas_state_q_7 =
                                    r.get(canvas, field::canvas::STATE_Q)?.i();
                                let state_au_upper_value_3 = r
                                    .get(canvas, field::canvas::STATE_AU_UPPER)?
                                    .i();
                                let state_l_upper_value_56 = r
                                    .get(canvas, field::canvas::STATE_L_UPPER)?
                                    .i();
                                let canvas_state_m_51 =
                                    r.get(canvas, field::canvas::STATE_M)?.i();
                                host::draw_entity(
                                    r,
                                    vector_element_at_value_18,
                                    graphics,
                                    (canvas_state_p_7)
                                        .wrapping_add(state_at_upper_value_3),
                                    (canvas_state_q_7)
                                        .wrapping_add(state_au_upper_value_3),
                                    state_l_upper_value_56,
                                    canvas_state_m_51,
                                )?;
                                number_2 = number_2.wrapping_add(1);
                            }
                        }
                        let canvas_world_59 =
                            r.get(canvas, field::canvas::WORLD)?.r();
                        let world_enemies_2 =
                            r.get(canvas_world_59, field::world::ENEMIES)?.r();
                        let vector_element_at_value_17 =
                            host::vector_element_at(
                                r,
                                world_enemies_2,
                                number_2,
                            )?;
                        super::canvas::remove_menu_entry(
                            r,
                            canvas,
                            vector_element_at_value_17,
                        )?;
                        number_2 = number_2.wrapping_add(1);
                    }
                }
                let canvas_world_55 = r.get(canvas, field::canvas::WORLD)?.r();
                let world_items_4 =
                    r.get(canvas_world_55, field::world::ITEMS)?.r();
                let vector_element_at_value_14 =
                    host::vector_element_at(r, world_items_4, number_2)?;
                super::canvas::remove_menu_entry(
                    r,
                    canvas,
                    vector_element_at_value_14,
                )?;
                number_2 = number_2.wrapping_add(1);
            }
        }
        let canvas_world_50 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_16 =
            r.get(canvas_world_50, field::world::OBJECTS)?.r();
        let vector_element_at_value_10 =
            host::vector_element_at(r, world_objects_16, number_2)?;
        host::check_cast(r, vector_element_at_value_10, "n")?;
        let kind_value = r
            .get(vector_element_at_value_10, field::map_object::KIND)?
            .i();
        if kind_value == 12 {
            break 'check_canvas_world_23;
        }
        } // check_canvas_world_24
        let canvas_world_52 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_18 = r.get(canvas_world_52, field::world::OBJECTS)?.r();
        let vector_element_at_value_12 =
            host::vector_element_at(r, world_objects_18, number_2)?;
        host::check_cast(r, vector_element_at_value_12, "n")?;
        let kind_value_2 = r
            .get(vector_element_at_value_12, field::map_object::KIND)?
            .i();
        if kind_value_2 == 92 {
            break 'check_canvas_world_23;
        }
        } // check_canvas_world_25
        let canvas_world_54 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_19 = r.get(canvas_world_54, field::world::OBJECTS)?.r();
        let vector_element_at_value_13 =
            host::vector_element_at(r, world_objects_19, number_2)?;
        host::check_cast(r, vector_element_at_value_13, "n")?;
        let kind_value_3 = r
            .get(vector_element_at_value_13, field::map_object::KIND)?
            .i();
        if kind_value_3 == 62 {
            break 'check_canvas_world_23;
        }
        } // check_canvas_world_26
        let canvas_world_56 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_20 = r.get(canvas_world_56, field::world::OBJECTS)?.r();
        let vector_element_at_value_15 =
            host::vector_element_at(r, world_objects_20, number_2)?;
        host::check_cast(r, vector_element_at_value_15, "n")?;
        let state_z_value_4 = r
            .get(vector_element_at_value_15, field::map_object::STATE_Z)?
            .i();
        if state_z_value_4 == 0 {
            let canvas_world_58 = r.get(canvas, field::canvas::WORLD)?.r();
            let world_objects_21 = r.get(canvas_world_58, field::world::OBJECTS)?.r();
            let vector_element_at_value_16 =
                host::vector_element_at(r, world_objects_21, number_2)?;
            host::check_cast(r, vector_element_at_value_16, "n")?;
            let canvas_state_p_6 = r.get(canvas, field::canvas::STATE_P)?.i();
            let canvas_state_at_upper_2 = r.get(canvas, field::canvas::STATE_AT_UPPER)?.i();
            let canvas_state_q_5 = r.get(canvas, field::canvas::STATE_Q)?.i();
            let canvas_state_au_upper_2 = r.get(canvas, field::canvas::STATE_AU_UPPER)?.i();
            let canvas_state_l_upper_54 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            let canvas_state_m_49 = r.get(canvas, field::canvas::STATE_M)?.i();
            super::map_object::draw(
                r,
                vector_element_at_value_16,
                graphics,
                canvas_state_p_6.wrapping_add(canvas_state_at_upper_2),
                canvas_state_q_5.wrapping_add(canvas_state_au_upper_2),
                canvas_state_l_upper_54,
                canvas_state_m_49,
            )?;
        }
        break 'advance_index;
        } // check_canvas_world_23
        let canvas_world_51 = r.get(canvas, field::canvas::WORLD)?.r();
        let world_objects_17 = r.get(canvas_world_51, field::world::OBJECTS)?.r();
        let vector_element_at_value_11 =
            host::vector_element_at(r, world_objects_17, number_2)?;
        super::canvas::remove_menu_entry(r, canvas, vector_element_at_value_11)?;
        } // advance_index
        number_2 = number_2.wrapping_add(1);
        continue 'paint_loop_115;
    }
}

/// Draw hud. Original reference: `k.a(Ljavax/microedition/lcdui/Graphics;)V`.
pub(in crate::game) fn draw_hud(r: &mut Runtime, canvas: u32, graphics: u32) -> Result<(), u32> {
    let mut object_2: u32 = 0;
    let mut number_3: i32 = 0;
    let mut object_3: u32 = 0;
    let mut number_4: i32 = 0;
    let graphics_get_font_value = host::graphics_get_font(r, graphics)?;
    object_2 = graphics_get_font_value;
    let font_get_font_value = host::font_get_font(r, 0, 0, 8)?;
    object_3 = font_get_font_value;
    host::graphics_set_font(r, graphics, object_3)?;
    host::graphics_set_color(r, graphics, 0)?;
    let canvas_get_width_value = host::canvas_get_width(r, canvas)?;
    let canvas_get_height_value = host::canvas_get_height(r, canvas)?;
    host::graphics_fill_rect(
        r,
        graphics,
        0,
        0,
        canvas_get_width_value,
        canvas_get_height_value,
    )?;
    host::graphics_set_color(r, graphics, 16711680)?;
    host::graphics_translate(r, graphics, 13, 0)?;
    host::graphics_translate(r, graphics, -11, 15)?;
    let canvas_player = r.get(canvas, field::canvas::PLAYER)?.r();
    let player_health = r.get(canvas_player, field::player::HEALTH)?.i();
    number_3 = player_health;
    number_4 = 0;
    loop {
        let canvas_player_2 = r.get(canvas, field::canvas::PLAYER)?.r();
        let player_max_health = r.get(canvas_player_2, field::player::MAX_HEALTH)?.i();
        let quotient_value = arithmetic::divide_int(r, player_max_health, 4)?;
        if number_4 >= quotient_value {
            let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value.wrapping_neg(),
                graphics_get_translate_y_value.wrapping_neg(),
            )?;
            host::graphics_translate(r, graphics, 2, 1)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
            host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
            let canvas_player_3 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_wallet_capacity =
                r.get(canvas_player_3, field::player::WALLET_CAPACITY)?.i();
            if player_wallet_capacity == 99 {
                let canvas_state_t = r.get(canvas, field::canvas::STATE_T)?.r();
                host::graphics_draw_image(r, graphics, canvas_state_t, -35, 0, 0)?;
            }
            let canvas_player_4 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_wallet_capacity_2 =
                r.get(canvas_player_4, field::player::WALLET_CAPACITY)?.i();
            if player_wallet_capacity_2 == 500 {
                let canvas_state_t_3 = r.get(canvas, field::canvas::STATE_T)?.r();
                host::graphics_draw_image(r, graphics, canvas_state_t_3, -43, 0, 0)?;
            }
            let canvas_player_5 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_wallet_capacity_3 =
                r.get(canvas_player_5, field::player::WALLET_CAPACITY)?.i();
            if player_wallet_capacity_3 == 999 {
                let canvas_state_t_8 = r.get(canvas, field::canvas::STATE_T)?.r();
                host::graphics_draw_image(r, graphics, canvas_state_t_8, -51, 0, 0)?;
            }
            host::graphics_translate(r, graphics, 9, 0)?;
            host::graphics_set_color(r, graphics, 16777215)?;
            let canvas_state_l_upper = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper, 32)?;
            let canvas_state_l_upper_2 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_2, 32)?;
            host::graphics_translate(r, graphics, 0, 2)?;
            let canvas_player_6 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_rupees = r.get(canvas_player_6, field::player::RUPEES)?.i();
            super::canvas::draw_map_marker(r, canvas, graphics, player_rupees, 0)?;
            host::graphics_translate(r, graphics, 0, -2)?;
            host::graphics_translate(r, graphics, 28, 0)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
            host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
            let canvas_state_t_5 = r.get(canvas, field::canvas::STATE_T)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_t_5, -59, 0, 0)?;
            host::graphics_translate(r, graphics, 9, 0)?;
            let canvas_state_l_upper_3 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_3, 32)?;
            let canvas_state_l_upper_4 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_4, 32)?;
            host::graphics_translate(r, graphics, 0, 2)?;
            let canvas_player_7 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_keys = r.get(canvas_player_7, field::player::KEYS)?.i();
            super::canvas::draw_map_marker(r, canvas, graphics, player_keys, 0)?;
            host::graphics_translate(r, graphics, 0, -2)?;
            host::graphics_translate(r, graphics, 17, 0)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
            host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
            let canvas_state_t_6 = r.get(canvas, field::canvas::STATE_T)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_t_6, -67, 0, 0)?;
            host::graphics_translate(r, graphics, 8, 0)?;
            let canvas_state_l_upper_5 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_5, 32)?;
            let canvas_state_l_upper_6 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_6, 32)?;
            host::graphics_translate(r, graphics, 0, 2)?;
            let canvas_player_8 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_bombs = r.get(canvas_player_8, field::player::BOMBS)?.i();
            super::canvas::draw_map_marker(r, canvas, graphics, player_bombs, 0)?;
            host::graphics_translate(r, graphics, 0, -2)?;
            host::graphics_translate(r, graphics, 20, 0)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 14, 8)?;
            host::graphics_set_clip(r, graphics, 0, 0, 14, 8)?;
            let canvas_state_t_7 = r.get(canvas, field::canvas::STATE_T)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_t_7, -83, 0, 0)?;
            host::graphics_translate(r, graphics, 14, 0)?;
            let canvas_state_l_upper_7 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_7, 32)?;
            let canvas_state_l_upper_8 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_8, 32)?;
            host::graphics_translate(r, graphics, 0, 2)?;
            let canvas_player_9 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_arrows = r.get(canvas_player_9, field::player::ARROWS)?.i();
            super::canvas::draw_map_marker(r, canvas, graphics, player_arrows, 0)?;
            host::graphics_translate(r, graphics, 0, -2)?;
            let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_2 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_2.wrapping_neg(),
                graphics_get_translate_y_value_2.wrapping_neg(),
            )?;
            host::graphics_translate(r, graphics, 80, 20)?;
            let canvas_state_l_upper_9 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_9, 32)?;
            let canvas_state_l_upper_10 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_10, 32)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 7, 7)?;
            host::graphics_set_clip(r, graphics, 0, 0, 7, 7)?;
            let canvas_state_x_upper = r.get(canvas, field::canvas::STATE_X_UPPER)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_x_upper, -73, 0, 0)?;
            host::graphics_translate(r, graphics, 8, -5)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 7, 16)?;
            host::graphics_set_clip(r, graphics, 0, 0, 7, 16)?;
            let canvas_player_10 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_sword = r.get(canvas_player_10, field::player::HAS_SWORD)?.i();
            if player_has_sword != 0 {
                let canvas_player_11 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_equipment_sprite = r
                    .get(canvas_player_11, field::player::EQUIPMENT_SPRITE)?
                    .r();
                host::graphics_draw_image(r, graphics, player_equipment_sprite, 0, 0, 0)?;
            }
            host::graphics_translate(r, graphics, 10, 5)?;
            let canvas_state_l_upper_11 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_clip_rect(r, graphics, 0, 0, canvas_state_l_upper_11, 32)?;
            let canvas_state_l_upper_12 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
            host::graphics_set_clip(r, graphics, 0, 0, canvas_state_l_upper_12, 32)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 7, 7)?;
            host::graphics_set_clip(r, graphics, 0, 0, 7, 7)?;
            let canvas_state_x_upper_2 = r.get(canvas, field::canvas::STATE_X_UPPER)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_x_upper_2, -80, 0, 0)?;
            host::graphics_translate(r, graphics, 8, 0)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
            host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
            let canvas_state_ax = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax == 0 {
                let canvas_player_12 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_has_bombs = r.get(canvas_player_12, field::player::HAS_BOMBS)?.i();
                if player_has_bombs != 0 {
                    let canvas_state_t_12 = r.get(canvas, field::canvas::STATE_T)?.r();
                    host::graphics_draw_image(r, graphics, canvas_state_t_12, -67, 0, 0)?;
                }
            }
            let canvas_state_ax_2 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_2 == 1 {
                let canvas_player_13 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_has_boomerang =
                    r.get(canvas_player_13, field::player::HAS_BOOMERANG)?.i();
                if player_has_boomerang != 0 {
                    let canvas_state_t_13 = r.get(canvas, field::canvas::STATE_T)?.r();
                    host::graphics_draw_image(r, graphics, canvas_state_t_13, -75, 0, 0)?;
                }
            }
            let canvas_state_ax_3 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_3 == 2 {
                let canvas_player_14 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_has_bow = r.get(canvas_player_14, field::player::HAS_BOW)?.i();
                if player_has_bow != 0 {
                    host::graphics_clip_rect(r, graphics, 0, 0, 14, 8)?;
                    host::graphics_set_clip(r, graphics, 0, 0, 14, 8)?;
                    let canvas_state_t_14 = r.get(canvas, field::canvas::STATE_T)?.r();
                    host::graphics_draw_image(r, graphics, canvas_state_t_14, -83, 0, 0)?;
                }
            }
            let canvas_state_ax_4 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_4 == 3 {
                let canvas_player_15 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_has_candle = r.get(canvas_player_15, field::player::HAS_CANDLE)?.i();
                if player_has_candle != 0 {
                    let canvas_state_t_15 = r.get(canvas, field::canvas::STATE_T)?.r();
                    let canvas_state_ax_7 = r.get(canvas, field::canvas::STATE_AX)?.i();
                    host::graphics_draw_image(
                        r,
                        graphics,
                        canvas_state_t_15,
                        (((67_i32).wrapping_add((8_i32).wrapping_mul(canvas_state_ax_7)))
                            .wrapping_add(6))
                        .wrapping_neg(),
                        0,
                        0,
                    )?;
                }
            }
            let canvas_state_ax_5 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_5 == 4 {
                let canvas_player_16 = r.get(canvas, field::canvas::PLAYER)?.r();
                let player_has_potion = r.get(canvas_player_16, field::player::HAS_POTION)?.i();
                if player_has_potion != 0 {
                    let canvas_state_t_16 = r.get(canvas, field::canvas::STATE_T)?.r();
                    let canvas_state_ax_9 = r.get(canvas, field::canvas::STATE_AX)?.i();
                    host::graphics_draw_image(
                        r,
                        graphics,
                        canvas_state_t_16,
                        (((67_i32).wrapping_add((8_i32).wrapping_mul(canvas_state_ax_9)))
                            .wrapping_add(6))
                        .wrapping_neg(),
                        0,
                        0,
                    )?;
                }
            }
            let canvas_state_ax_6 = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_6 != 5 {
                let canvas_state_ax_8_read = r.get(canvas, field::canvas::STATE_AX)?.i();
                if canvas_state_ax_8_read != 6 {
                    host::graphics_set_font(r, graphics, object_2)?;
                    return Ok(());
                }
                let canvas_player_18_read = r.get(canvas, field::canvas::PLAYER)?.r();
                let has_mirror_shield_value = r
                    .get(canvas_player_18_read, field::player::HAS_MIRROR_SHIELD)?
                    .i();
                if has_mirror_shield_value == 0 {
                    host::graphics_set_font(r, graphics, object_2)?;
                    return Ok(());
                }
                let canvas_state_t_18_read = r.get(canvas, field::canvas::STATE_T)?.r();
                let canvas_state_ax_11_read = r.get(canvas, field::canvas::STATE_AX)?.i();
                host::graphics_draw_image(
                    r,
                    graphics,
                    canvas_state_t_18_read,
                    (((67_i32).wrapping_add((8_i32).wrapping_mul(canvas_state_ax_11_read)))
                        .wrapping_add(6))
                    .wrapping_neg(),
                    0,
                    0,
                )?;
                host::graphics_set_font(r, graphics, object_2)?;
                return Ok(());
            }
            let canvas_player_17 = r.get(canvas, field::canvas::PLAYER)?.r();
            let player_has_raft = r.get(canvas_player_17, field::player::HAS_RAFT)?.i();
            if player_has_raft != 0 {
                let canvas_state_t_17 = r.get(canvas, field::canvas::STATE_T)?.r();
                let canvas_state_ax_10 = r.get(canvas, field::canvas::STATE_AX)?.i();
                host::graphics_draw_image(
                    r,
                    graphics,
                    canvas_state_t_17,
                    (((67_i32).wrapping_add((8_i32).wrapping_mul(canvas_state_ax_10)))
                        .wrapping_add(6))
                    .wrapping_neg(),
                    0,
                    0,
                )?;
            }
            let canvas_state_ax_8_read = r.get(canvas, field::canvas::STATE_AX)?.i();
            if canvas_state_ax_8_read != 6 {
                host::graphics_set_font(r, graphics, object_2)?;
                return Ok(());
            }
            let canvas_player_18_read = r.get(canvas, field::canvas::PLAYER)?.r();
            let has_mirror_shield_value = r
                .get(canvas_player_18_read, field::player::HAS_MIRROR_SHIELD)?
                .i();
            if has_mirror_shield_value == 0 {
                host::graphics_set_font(r, graphics, object_2)?;
                return Ok(());
            }
            let canvas_state_t_18_read = r.get(canvas, field::canvas::STATE_T)?.r();
            let canvas_state_ax_11_read = r.get(canvas, field::canvas::STATE_AX)?.i();
            host::graphics_draw_image(
                r,
                graphics,
                canvas_state_t_18_read,
                (((67_i32).wrapping_add((8_i32).wrapping_mul(canvas_state_ax_11_read)))
                    .wrapping_add(6))
                .wrapping_neg(),
                0,
                0,
            )?;
            host::graphics_set_font(r, graphics, object_2)?;
            return Ok(());
        }
        host::graphics_clip_rect(r, graphics, 0, 0, 7, 7)?;
        host::graphics_set_clip(r, graphics, 0, 0, 7, 7)?;
        if number_3 >= 4 {
            let canvas_state_t_2 = r.get(canvas, field::canvas::STATE_T)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_t_2, 0, 0, 0)?;
        }

        if number_3 == 3 {
            let canvas_state_t_4 = r.get(canvas, field::canvas::STATE_T)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_t_4, -7, 0, 0)?;
        }

        if number_3 == 2 {
            let canvas_state_t_9 = r.get(canvas, field::canvas::STATE_T)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_t_9, -14, 0, 0)?;
        }

        if number_3 == 1 {
            let canvas_state_t_10 = r.get(canvas, field::canvas::STATE_T)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_t_10, -21, 0, 0)?;
        }

        if number_3 < 1 {
            let canvas_state_t_11 = r.get(canvas, field::canvas::STATE_T)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_t_11, -28, 0, 0)?;
        }

        if number_4 == 7 {
            host::graphics_translate(r, graphics, -64, 8)?;
        }
        host::graphics_translate(r, graphics, 8, 0)?;
        number_3 = number_3.wrapping_add(-4);
        number_4 = number_4.wrapping_add(1);
    }
}

/// Draw title. Original reference: `k.b(Ljavax/microedition/lcdui/Graphics;II)V`.
pub(in crate::game) fn draw_title(
    r: &mut Runtime,
    canvas: u32,
    graphics: u32,
    argument_2: i32,
    argument_3: i32,
) -> Result<(), u32> {
    let mut number_4: i32 = 0;
    let mut object_5: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let canvas_state_g = r.get(canvas, field::canvas::STATE_G)?.i();
    if !(canvas_state_g != 0) {
        return Ok(());
    }
    let canvas_use_phone_buttons = r.get(canvas, field::canvas::USE_PHONE_BUTTONS)?.i();
    if !(canvas_use_phone_buttons != 0) {
        return Ok(());
    }
    let canvas_state_y = r.get(canvas, field::canvas::STATE_Y)?.r();
    if !(canvas_state_y != 0) {
        let canvas_visual_effects = r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i();
        if canvas_visual_effects == 0 {
            let text_value = r.intern("/Images/RainEffect2.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value)?)) })();
            (carry_0_r,) = (canvas,);
            if !(outcome.is_ok()) {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                r.set(canvas, field::canvas::STATE_Y, V::R(0))?;
                return Ok(());
            }
            let image_create_image_value = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Y,
                V::R(image_create_image_value),
            )?;
        } else {
            let text_value_2 = r.intern("/Images/RainEffect.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_2)?)) })();
            (carry_0_r,) = (canvas,);
            if !(outcome.is_ok()) {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                r.set(canvas, field::canvas::STATE_Y, V::R(0))?;
                return Ok(());
            }
            let image_create_image_value_2 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_Y,
                V::R(image_create_image_value_2),
            )?;
        }
        let canvas_state_aw_upper_2 = r.get(canvas, field::canvas::STATE_AW_UPPER)?.r();
        host::vector_remove_all_elements(r, canvas_state_aw_upper_2)?;
        number_4 = 0_i32;
        loop {
            if number_4 >= 500_i32 {
                return Ok(());
            }
            let trail_point_value = r.alloc("g", Native::Empty);
            outcome = (|| -> JResult {
                super::trail_point::initialize(r, trail_point_value)?;
                Ok(V::I(0))
            })();
            (carry_0_r,) = (trail_point_value,);
            if !(outcome.is_ok()) {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                r.set(canvas, field::canvas::STATE_Y, V::R(0))?;
                return Ok(());
            }
            object_5 = carry_0_r;
            let canvas_random_3 = r.get(canvas, field::canvas::RANDOM)?.r();
            outcome = (|| -> JResult { Ok(V::I(host::random_next_int(r, canvas_random_3)?)) })();
            (carry_0_r,) = (carry_0_r,);
            if !(outcome.is_ok()) {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                r.set(canvas, field::canvas::STATE_Y, V::R(0))?;
                return Ok(());
            }
            let random_next_int_value_3 = outcome.unwrap().i();
            let remainder_value_3 = arithmetic::remainder_int(r, random_next_int_value_3, 256_i32)?;
            r.set(
                carry_0_r,
                field::trail_point::X,
                V::I(remainder_value_3.wrapping_add(50_i32)),
            )?;
            let canvas_random_4 = r.get(canvas, field::canvas::RANDOM)?.r();
            outcome = (|| -> JResult { Ok(V::I(host::random_next_int(r, canvas_random_4)?)) })();
            (carry_0_r,) = (object_5,);
            if !(outcome.is_ok()) {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                r.set(canvas, field::canvas::STATE_Y, V::R(0))?;
                return Ok(());
            }
            let random_next_int_value_4 = outcome.unwrap().i();
            let remainder_value_4 = arithmetic::remainder_int(r, random_next_int_value_4, 176_i32)?;
            r.set(carry_0_r, field::trail_point::Y, V::I(remainder_value_4))?;
            let canvas_random_5 = r.get(canvas, field::canvas::RANDOM)?.r();
            outcome = (|| -> JResult { Ok(V::I(host::random_next_int(r, canvas_random_5)?)) })();
            (carry_0_r,) = (object_5,);
            if !(outcome.is_ok()) {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                r.set(canvas, field::canvas::STATE_Y, V::R(0))?;
                return Ok(());
            }
            let random_next_int_value_5 = outcome.unwrap().i();
            let remainder_value_5 = arithmetic::remainder_int(r, random_next_int_value_5, 176_i32)?;
            r.set(
                carry_0_r,
                field::trail_point::STATE_D,
                V::I(remainder_value_5),
            )?;
            let canvas_state_aw_upper_3 = r.get(canvas, field::canvas::STATE_AW_UPPER)?.r();
            host::vector_add_element(r, canvas_state_aw_upper_3, object_5)?;
            number_4 = number_4.wrapping_add(1);
        }
    }
    number_4 = 0_i32;
    loop {
        if number_4 >= 500_i32 {
            return Ok(());
        }
        let canvas_state_aw_upper = r.get(canvas, field::canvas::STATE_AW_UPPER)?.r();
        let vector_element_at_value = host::vector_element_at(r, canvas_state_aw_upper, number_4)?;
        host::check_cast(r, vector_element_at_value, "g")?;
        object_5 = vector_element_at_value;
        let trail_point_y = r.get(vector_element_at_value, field::trail_point::Y)?.i();
        let trail_point_state_d = r.get(object_5, field::trail_point::STATE_D)?.i();
        if !(trail_point_y < trail_point_state_d) {
            let trail_point_state_c = r.get(object_5, field::trail_point::STATE_C)?.i();
            if !(trail_point_state_c > 0) {
                let canvas_random = r.get(canvas, field::canvas::RANDOM)?.r();
                let random_next_int_value = host::random_next_int(r, canvas_random)?;
                let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 256_i32)?;
                r.set(
                    object_5,
                    field::trail_point::X,
                    V::I(remainder_value.wrapping_add(50_i32)),
                )?;
                r.set(object_5, field::trail_point::Y, V::I(0_i32))?;
                let canvas_random_2 = r.get(canvas, field::canvas::RANDOM)?.r();
                let random_next_int_value_2 = host::random_next_int(r, canvas_random_2)?;
                let remainder_value_2 =
                    arithmetic::remainder_int(r, random_next_int_value_2, 100_i32)?;
                r.set(
                    object_5,
                    field::trail_point::STATE_D,
                    V::I(remainder_value_2.wrapping_add(76_i32)),
                )?;
                r.set(object_5, field::trail_point::STATE_C, V::I(6_i32))?;
            }
        }
        let trail_point_y_2 = r.get(object_5, field::trail_point::Y)?.i();
        let trail_point_state_d_2 = r.get(object_5, field::trail_point::STATE_D)?.i();
        if trail_point_y_2 >= trail_point_state_d_2 {
            let trail_point_state_c_2 = r.get(object_5, field::trail_point::STATE_C)?.i();
            r.set(
                object_5,
                field::trail_point::STATE_C,
                V::I(trail_point_state_c_2.wrapping_sub(1_i32)),
            )?;
        } else {
            let trail_point_x = r.get(object_5, field::trail_point::X)?.i();
            r.set(
                object_5,
                field::trail_point::X,
                V::I(trail_point_x.wrapping_sub(3_i32)),
            )?;
            let trail_point_y_3 = r.get(object_5, field::trail_point::Y)?.i();
            let trail_point_state_d_3 = r.get(object_5, field::trail_point::STATE_D)?.i();
            let quotient_value = arithmetic::divide_int(r, trail_point_state_d_3, 44_i32)?;
            r.set(
                object_5,
                field::trail_point::Y,
                V::I(trail_point_y_3.wrapping_add((8_i32).wrapping_sub(quotient_value))),
            )?;
        }
        let trail_point_x_2 = r.get(object_5, field::trail_point::X)?.i();
        if !(trail_point_x_2 >= 0) {
            r.set(object_5, field::trail_point::Y, V::I(176_i32))?;
        }
        let trail_point_y_4 = r.get(object_5, field::trail_point::Y)?.i();
        if !(((trail_point_y_4.wrapping_sub(argument_3)).wrapping_add(32_i32)).wrapping_add(8_i32)
            > 208_i32)
        {
            let trail_point_y_5 = r.get(object_5, field::trail_point::Y)?.i();
            if !(((trail_point_y_5.wrapping_sub(argument_3)).wrapping_add(32_i32))
                .wrapping_add(8_i32)
                < 0)
            {
                let trail_point_x_3 = r.get(object_5, field::trail_point::X)?.i();
                if !((trail_point_x_3.wrapping_sub(argument_2)).wrapping_add(8_i32) > 256_i32) {
                    let trail_point_x_4 = r.get(object_5, field::trail_point::X)?.i();
                    if !((trail_point_x_4.wrapping_sub(argument_2)).wrapping_add(8_i32) < 0) {
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
                        let trail_point_x_5 = r.get(object_5, field::trail_point::X)?.i();
                        let trail_point_y_6 = r.get(object_5, field::trail_point::Y)?.i();
                        host::graphics_translate(
                            r,
                            graphics,
                            trail_point_x_5.wrapping_sub(argument_2),
                            (trail_point_y_6.wrapping_sub(argument_3)).wrapping_add(32_i32),
                        )?;
                        host::graphics_set_clip(r, graphics, 0_i32, 0_i32, 8_i32, 8_i32)?;
                        host::graphics_clip_rect(r, graphics, 0_i32, 0_i32, 8_i32, 8_i32)?;
                        let trail_point_state_c_3 =
                            r.get(object_5, field::trail_point::STATE_C)?.i();
                        if trail_point_state_c_3 != 6_i32 {
                            let trail_point_state_c_4 =
                                r.get(object_5, field::trail_point::STATE_C)?.i();
                            if trail_point_state_c_4 <= 4_i32 {
                                let trail_point_state_c_5 =
                                    r.get(object_5, field::trail_point::STATE_C)?.i();
                                if trail_point_state_c_5 <= 2_i32 {
                                    let trail_point_state_c_6 =
                                        r.get(object_5, field::trail_point::STATE_C)?.i();
                                    if !(trail_point_state_c_6 < 0) {
                                        let canvas_state_y_5 =
                                            r.get(canvas, field::canvas::STATE_Y)?.r();
                                        host::graphics_draw_image(
                                            r,
                                            graphics,
                                            canvas_state_y_5,
                                            -24_i32,
                                            0_i32,
                                            0_i32,
                                        )?;
                                    }
                                } else {
                                    let canvas_state_y_4 =
                                        r.get(canvas, field::canvas::STATE_Y)?.r();
                                    host::graphics_draw_image(
                                        r,
                                        graphics,
                                        canvas_state_y_4,
                                        -16_i32,
                                        0_i32,
                                        0_i32,
                                    )?;
                                }
                            } else {
                                let canvas_state_y_3 = r.get(canvas, field::canvas::STATE_Y)?.r();
                                host::graphics_draw_image(
                                    r,
                                    graphics,
                                    canvas_state_y_3,
                                    -8_i32,
                                    0_i32,
                                    0_i32,
                                )?;
                            }
                        } else {
                            let canvas_state_y_2 = r.get(canvas, field::canvas::STATE_Y)?.r();
                            host::graphics_draw_image(
                                r,
                                graphics,
                                canvas_state_y_2,
                                0_i32,
                                0_i32,
                                0_i32,
                            )?;
                        }
                    }
                }
            }
        }
        number_4 = number_4.wrapping_add(1);
    }
}

/// Draw key prompt. Original reference: `k.a(Ljavax/microedition/lcdui/Graphics;I)V`.
pub(in crate::game) fn draw_key_prompt(
    r: &mut Runtime,
    canvas: u32,
    graphics: u32,
    argument_2: i32,
) -> Result<(), u32> {
    let mut number_3: i32 = 0;
    let mut number_4: i32 = 0;
    let mut number_5: i32 = 0;
    let mut number_6: i32 = 0;
    let canvas_save_slots = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let element_value = r.array_get(canvas_save_slots, argument_2)?.r();
    if element_value == 0 {
        host::graphics_translate(r, graphics, 40_i32, 7_i32)?;
        let canvas_get_width_value = host::canvas_get_width(r, canvas)?;
        let canvas_get_height_value = host::canvas_get_height(r, canvas)?;
        host::graphics_clip_rect(
            r,
            graphics,
            0_i32,
            0_i32,
            canvas_get_width_value,
            canvas_get_height_value,
        )?;
        let canvas_get_width_value_2 = host::canvas_get_width(r, canvas)?;
        let canvas_get_height_value_2 = host::canvas_get_height(r, canvas)?;
        host::graphics_set_clip(
            r,
            graphics,
            0_i32,
            0_i32,
            canvas_get_width_value_2,
            canvas_get_height_value_2,
        )?;
        let text_value = r.intern("New Game..");
        host::graphics_draw_string(r, graphics, text_value, 0_i32, 0_i32, 0_i32)?;
        host::graphics_translate(r, graphics, -40_i32, -7_i32)?;
        return Ok(());
    }
    let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
    number_3 = graphics_get_translate_x_value;
    let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
    number_4 = graphics_get_translate_y_value;
    host::graphics_translate(r, graphics, 18_i32, 7_i32)?;
    host::graphics_clip_rect(r, graphics, 0_i32, 0_i32, 18_i32, 25_i32)?;
    host::graphics_set_clip(r, graphics, 0_i32, 0_i32, 18_i32, 25_i32)?;
    let canvas_state_ab = r.get(canvas, field::canvas::STATE_AB)?.r();
    let canvas_save_slots_2 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let element_value_2 = r.array_get(canvas_save_slots_2, argument_2)?.r();
    let player_tunic_color = r.get(element_value_2, field::player::TUNIC_COLOR)?.i();
    host::graphics_draw_image(
        r,
        graphics,
        canvas_state_ab,
        (-18_i32).wrapping_mul(player_tunic_color),
        0_i32,
        0_i32,
    )?;
    let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_2 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_2.wrapping_neg(),
        graphics_get_translate_y_value_2.wrapping_neg(),
    )?;
    host::graphics_translate(r, graphics, number_3, number_4)?;
    host::graphics_translate(r, graphics, 40_i32, 6_i32)?;
    let canvas_save_slots_3 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let element_value_3 = r.array_get(canvas_save_slots_3, argument_2)?.r();
    let player_health = r.get(element_value_3, field::player::HEALTH)?.i();
    number_5 = player_health;
    number_6 = 0_i32;
    loop {
        let canvas_save_slots_4 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
        let element_value_4 = r.array_get(canvas_save_slots_4, argument_2)?.r();
        let player_max_health = r.get(element_value_4, field::player::MAX_HEALTH)?.i();
        let quotient_value = arithmetic::divide_int(r, player_max_health, 4_i32)?;
        if number_6 >= quotient_value {
            let graphics_get_translate_x_value_3 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_3 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_3.wrapping_neg(),
                graphics_get_translate_y_value_3.wrapping_neg(),
            )?;
            host::graphics_translate(r, graphics, number_3, number_4)?;
            host::graphics_translate(r, graphics, 40_i32, 23_i32)?;
            host::graphics_clip_rect(r, graphics, 0_i32, 0_i32, 8_i32, 8_i32)?;
            host::graphics_set_clip(r, graphics, 0_i32, 0_i32, 8_i32, 8_i32)?;
            let canvas_save_slots_5 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            let element_value_5 = r.array_get(canvas_save_slots_5, argument_2)?.r();
            let player_wallet_capacity =
                r.get(element_value_5, field::player::WALLET_CAPACITY)?.i();
            if !(player_wallet_capacity != 99_i32) {
                let canvas_state_ad = r.get(canvas, field::canvas::STATE_AD)?.r();
                host::graphics_draw_image(r, graphics, canvas_state_ad, -35_i32, 0_i32, 0_i32)?;
            }
            let canvas_save_slots_6 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            let element_value_6 = r.array_get(canvas_save_slots_6, argument_2)?.r();
            let player_wallet_capacity_2 =
                r.get(element_value_6, field::player::WALLET_CAPACITY)?.i();
            if !(player_wallet_capacity_2 != 500_i32) {
                let canvas_state_ad_3 = r.get(canvas, field::canvas::STATE_AD)?.r();
                host::graphics_draw_image(r, graphics, canvas_state_ad_3, -43_i32, 0_i32, 0_i32)?;
            }
            let canvas_save_slots_7 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            let element_value_7 = r.array_get(canvas_save_slots_7, argument_2)?.r();
            let player_wallet_capacity_3 =
                r.get(element_value_7, field::player::WALLET_CAPACITY)?.i();
            if !(player_wallet_capacity_3 != 999_i32) {
                let canvas_state_ad_5 = r.get(canvas, field::canvas::STATE_AD)?.r();
                host::graphics_draw_image(r, graphics, canvas_state_ad_5, -51_i32, 0_i32, 0_i32)?;
            }
            host::graphics_translate(r, graphics, 9_i32, 0_i32)?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            let canvas_get_width_value_3 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_3 = host::canvas_get_height(r, canvas)?;
            host::graphics_clip_rect(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_3,
                canvas_get_height_value_3,
            )?;
            let canvas_get_width_value_4 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_4 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_4,
                canvas_get_height_value_4,
            )?;
            host::graphics_translate(r, graphics, 0_i32, 2_i32)?;
            let canvas_save_slots_8 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            let element_value_8 = r.array_get(canvas_save_slots_8, argument_2)?.r();
            let player_rupees = r.get(element_value_8, field::player::RUPEES)?.i();
            super::canvas::draw_map_marker(r, canvas, graphics, player_rupees, 0_i32)?;
            host::graphics_translate(r, graphics, 0_i32, -2_i32)?;
            let graphics_get_translate_x_value_4 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_4 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_4.wrapping_neg(),
                graphics_get_translate_y_value_4.wrapping_neg(),
            )?;
            host::graphics_translate(r, graphics, number_3, number_4)?;
            host::graphics_translate(r, graphics, 100_i32, 6_i32)?;
            host::graphics_clip_rect(r, graphics, 0_i32, 0_i32, 18_i32, 25_i32)?;
            host::graphics_set_clip(r, graphics, 0_i32, 0_i32, 18_i32, 25_i32)?;
            let canvas_state_ab_2 = r.get(canvas, field::canvas::STATE_AB)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_ab_2, -72_i32, 0_i32, 0_i32)?;
            host::graphics_translate(r, graphics, -1_i32, 18_i32)?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            let canvas_get_width_value_5 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_5 = host::canvas_get_height(r, canvas)?;
            host::graphics_clip_rect(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_5,
                canvas_get_height_value_5,
            )?;
            let canvas_get_width_value_6 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_6 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_6,
                canvas_get_height_value_6,
            )?;
            let canvas_save_slots_9 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            let element_value_9 = r.array_get(canvas_save_slots_9, argument_2)?.r();
            let player_triforce_count = r.get(element_value_9, field::player::TRIFORCE_COUNT)?.i();
            super::canvas::draw_map_marker(r, canvas, graphics, player_triforce_count, 2_i32)?;
            host::graphics_translate(r, graphics, 7_i32, 0_i32)?;
            host::graphics_set_color(r, graphics, 16777215_i32)?;
            let canvas_get_width_value_7 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_7 = host::canvas_get_height(r, canvas)?;
            host::graphics_clip_rect(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_7,
                canvas_get_height_value_7,
            )?;
            let canvas_get_width_value_8 = host::canvas_get_width(r, canvas)?;
            let canvas_get_height_value_8 = host::canvas_get_height(r, canvas)?;
            host::graphics_set_clip(
                r,
                graphics,
                0_i32,
                0_i32,
                canvas_get_width_value_8,
                canvas_get_height_value_8,
            )?;
            host::graphics_draw_line(r, graphics, 4_i32, 0_i32, 0_i32, 6_i32)?;
            host::graphics_translate(r, graphics, 4_i32, 0_i32)?;
            super::canvas::draw_map_marker(r, canvas, graphics, 8_i32, 2_i32)?;
            let graphics_get_translate_x_value_5 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_5 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_5.wrapping_neg(),
                graphics_get_translate_y_value_5.wrapping_neg(),
            )?;
            host::graphics_translate(r, graphics, number_3, number_4)?;
            return Ok(());
        }
        host::graphics_clip_rect(r, graphics, 0_i32, 0_i32, 7_i32, 7_i32)?;
        host::graphics_set_clip(r, graphics, 0_i32, 0_i32, 7_i32, 7_i32)?;
        if !(number_5 < 4_i32) {
            let canvas_state_ad_2 = r.get(canvas, field::canvas::STATE_AD)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_ad_2, 0_i32, 0_i32, 0_i32)?;
        }

        if !(number_5 != 3_i32) {
            let canvas_state_ad_4 = r.get(canvas, field::canvas::STATE_AD)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_ad_4, -7_i32, 0_i32, 0_i32)?;
        }

        if !(number_5 != 2_i32) {
            let canvas_state_ad_6 = r.get(canvas, field::canvas::STATE_AD)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_ad_6, -14_i32, 0_i32, 0_i32)?;
        }

        if !(number_5 != 1_i32) {
            let canvas_state_ad_7 = r.get(canvas, field::canvas::STATE_AD)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_ad_7, -21_i32, 0_i32, 0_i32)?;
        }

        if !(number_5 >= 1_i32) {
            let canvas_state_ad_8 = r.get(canvas, field::canvas::STATE_AD)?.r();
            host::graphics_draw_image(r, graphics, canvas_state_ad_8, -28_i32, 0_i32, 0_i32)?;
        }

        if !(number_6 != 7_i32) {
            host::graphics_translate(r, graphics, -56_i32, 8_i32)?;
        }
        host::graphics_translate(r, graphics, 7_i32, 0_i32)?;
        number_5 = number_5.wrapping_add(-4);
        number_6 = number_6.wrapping_add(1);
    }
}
