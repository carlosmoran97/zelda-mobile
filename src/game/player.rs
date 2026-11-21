//! Player state, inventory, progression and original save encoding.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `h.<init>()V`.
pub(super) fn initialize(r: &mut Runtime, player: u32) -> Result<(), u32> {
    super::drawable::initialize(r, player)?;
    r.set(player, field::player::STATE_Y_NUMBER, V::I(0))?;
    r.set(player, field::player::COLLISION_HEIGHT, V::I(24))?;
    r.set(player, field::player::STATE_AA, V::I(100))?;
    r.set(player, field::player::STATE_AB_LOWER, V::I(75))?;
    r.set(player, field::player::STATE_AC_LOWER, V::I(2))?;
    r.set(player, field::player::DIG_TIMER, V::I(0))?;
    r.set(player, field::player::TRIFORCE_COUNT, V::I(0))?;
    r.set(player, field::player::PROJECTILE_COOLDOWN, V::I(0))?;
    let array_value = r.new_array(10, 4)?;
    r.set(player, field::player::OVERWORLD_EVENTS, V::R(array_value))?;
    let array_value_2 = r.new_array(10, 4)?;
    r.set(player, field::player::CAVE_EVENTS, V::R(array_value_2))?;
    let array_value_3 = r.new_array(10, 2)?;
    r.set(player, field::player::DUNGEON_1_EVENTS, V::R(array_value_3))?;
    let array_value_4 = r.new_array(10, 2)?;
    r.set(player, field::player::DUNGEON_2_EVENTS, V::R(array_value_4))?;
    let array_value_5 = r.new_array(10, 2)?;
    r.set(player, field::player::DUNGEON_3_EVENTS, V::R(array_value_5))?;
    let array_value_6 = r.new_array(10, 2)?;
    r.set(player, field::player::DUNGEON_4_EVENTS, V::R(array_value_6))?;
    let array_value_7 = r.new_array(10, 2)?;
    r.set(player, field::player::DUNGEON_5_EVENTS, V::R(array_value_7))?;
    let array_value_8 = r.new_array(10, 2)?;
    r.set(player, field::player::DUNGEON_6_EVENTS, V::R(array_value_8))?;
    let array_value_9 = r.new_array(10, 2)?;
    r.set(player, field::player::DUNGEON_7_EVENTS, V::R(array_value_9))?;
    let array_value_10 = r.new_array(10, 2)?;
    r.set(
        player,
        field::player::DUNGEON_8_EVENTS,
        V::R(array_value_10),
    )?;
    let array_value_11 = r.new_array(10, 2)?;
    r.set(
        player,
        field::player::DUNGEON_9_EVENTS,
        V::R(array_value_11),
    )?;
    let array_value_12 = r.new_array(4, 8)?;
    r.set(player, field::player::TRIFORCE_PIECES, V::R(array_value_12))?;
    r.set(player, field::player::WORLD_FLAGS, V::I(0))?;
    r.set(player, field::player::TUNIC_COLOR, V::I(0))?;
    r.set(player, field::player::SWORD_LEVEL, V::I(0))?;
    r.set(player, field::player::ZONE, V::I(0))?;
    r.set(player, field::player::MAP, V::I(120))?;
    r.set(player, field::player::COLLISION_HEIGHT, V::I(24))?;
    r.set(player, field::player::STATE_G, V::I(0))?;
    r.set(player, field::player::KEYS, V::I(0))?;
    r.set(player, field::player::PROGRESSION_FLAGS, V::I(0))?;
    r.set(player, field::drawable::X, V::I(75))?;
    r.set(player, field::player::SWORD_DAMAGE, V::I(1))?;
    r.set(player, field::drawable::Y, V::I(85))?;
    r.set(player, field::player::FACING, V::I(0))?;
    r.set(player, field::player::ANIMATION, V::I(0))?;
    r.set(player, field::player::HEALTH, V::I(12))?;
    r.set(player, field::player::MAX_HEALTH, V::I(12))?;
    r.set(player, field::player::WALLET_CAPACITY, V::I(99))?;
    r.set(player, field::player::BOMB_CAPACITY, V::I(0))?;
    r.set(player, field::player::BOMBS, V::I(0))?;
    r.set(player, field::player::ARROWS, V::I(0))?;
    r.set(player, field::player::ARROW_CAPACITY, V::I(0))?;
    r.set(player, field::player::RUPEES, V::I(0))?;
    r.set(player, field::drawable::HALF_WIDTH, V::I(12))?;
    r.set(player, field::drawable::HALF_HEIGHT, V::I(16))?;
    r.set(player, field::player::RUPEE_PICKUPS, V::I(0))?;
    r.set(player, field::player::TRIFORCE_COUNT, V::I(0))?;
    r.set(player, field::player::FISHING, V::I(0))?;
    r.set(player, field::player::ON_RAFT, V::I(0))?;
    r.set(player, field::player::ATTACKING, V::I(0))?;
    r.set(player, field::player::STATE_AH_UPPER, V::I(0))?;
    r.set(player, field::player::HAS_SWORD, V::I(1))?;
    r.set(player, field::player::HAS_BOMBS, V::I(1))?;
    r.set(player, field::player::HAS_BOOMERANG, V::I(1))?;
    r.set(player, field::player::HAS_BOW, V::I(1))?;
    r.set(player, field::player::HAS_CANDLE, V::I(1))?;
    r.set(player, field::player::HAS_RAFT, V::I(0))?;
    r.set(player, field::player::HAS_MIRROR_SHIELD, V::I(0))?;
    r.set(player, field::player::SHIELD_RAISED, V::I(0))?;
    r.set(player, field::player::HAS_SHOVEL, V::I(0))?;
    r.set(player, field::player::HAS_SWORD, V::I(0))?;
    r.set(player, field::player::HAS_BOMBS, V::I(0))?;
    r.set(player, field::player::HAS_BOOMERANG, V::I(0))?;
    r.set(player, field::player::HAS_BOW, V::I(0))?;
    r.set(player, field::player::HAS_CANDLE, V::I(0))?;
    r.set(player, field::player::HAS_POTION, V::I(0))?;
    r.set(player, field::player::HAS_HOOKSHOT, V::I(0))?;
    r.set(player, field::player::HAS_FISH, V::I(0))?;
    r.set(player, field::player::STATE_A, V::I(0))?;
    r.set(player, field::player::STATE_AG, V::I(0))?;
    r.set(player, field::player::HURT_TIMER, V::I(0))?;
    r.set(player, field::player::DEFENSE, V::I(1))?;
    let object_value = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value)?;
    r.set(player, field::player::HOOKSHOT_TRAIL, V::R(object_value))?;
    r.set(player, field::player::STATE_AH, V::I(0))?;
    return Ok(());
}

pub(super) use super::player_save::{encode_save, load_save};

/// Update the eight Triforce indicators and unlock the final overworld event.
pub(super) fn count_triforce_pieces(r: &mut Runtime, player: u32) -> Result<i32, u32> {
    let flags = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
    let pieces = r.get(player, field::player::TRIFORCE_PIECES)?.r();
    let mut count = 0;
    for (index, mask) in [
        0x200, 0x400, 0x800, 0x4000, 0x8000, 0x40000, 0x400000, 0x800000,
    ]
    .into_iter()
    .enumerate()
    {
        if flags & mask > 0 {
            r.array_set(pieces, index as i32, V::I(1))?;
        }
        if r.array_get(pieces, index as i32)?.i() != 0 {
            count += 1;
        }
    }
    if count == 8 {
        let events = r.get(player, field::player::OVERWORLD_EVENTS)?.r();
        let unlocked = r.array_get(events, 0)?.i() | 0x40;
        r.array_set(events, 0, V::I(unlocked))?;
    }
    Ok(count)
}

/// Draw. Original reference: `h.a(Ljavax/microedition/lcdui/Graphics;IIII)V`.
#[rustfmt::skip]
pub(super) fn draw(
    r: &mut Runtime,
    player: u32,
    graphics: u32,
    x: i32,
    y: i32,
    mut width: i32,
    mut height: i32,
) -> Result<(), u32> {
    let mut object_5: u32 = 0;
    let mut object_6: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    'check_player_facing_4: {
    'graphics_get_translate_x_4: {
    'check_player_hookshot_trail_4: {
    'next_step_3: {
    'check_player_on_raft_4: {
    'check_player_facing_3: {
    'graphics_get_translate_x_3: {
    'check_player_hookshot_trail_2: {
    'next_step_2: {
    'check_player_on_raft_3: {
    'check_player_facing_2: {
    'graphics_get_translate_x_2: {
    'check_player_hookshot_trail: {
    'next_step: {
    'check_player_on_raft: {
    'check_player_facing: {
    'check_player_state_p_2: {
    'graphics_get_translate_x_6: {
    'next_step_10: {
    'next_step_11: {
    'next_step_8: {
    'check_player_facing_11: {
    'graphics_translate_4: {
    'check_player_state_ac_lower_6: {
    'next_step_9: {
    'check_player_state_ad_12: {
    'check_player_state_ad_10: {
    'check_player_facing_10: {
    'graphics_translate_3: {
    'check_player_state_ac_lower_5: {
    'next_step_7: {
    'check_player_state_ad_11: {
    'check_player_state_ad_8: {
    'check_player_facing_9: {
    'graphics_set_clip_4: {
    'check_player_state_ac_lower_4: {
    'next_step_6: {
    'check_player_state_ad_9: {
    'check_player_state_ad_6: {
    'check_player_facing_8: {
    'graphics_set_clip_3: {
    'check_player_state_ac_lower_3: {
    'next_step_5: {
    'check_player_state_ad_7: {
    'check_player_state_ad_4: {
    'check_player_facing_7: {
    'graphics_set_clip_2: {
    'check_player_state_ac_lower_2: {
    'next_step_4: {
    'check_player_state_ad_5: {
    'check_player_state_ad_3: {
    'check_player_facing_6: {
    'graphics_set_clip: {
    'check_player_state_ac_lower: {
    'graphics_translate_2: {
    'graphics_translate: {
    'check_player_facing_5: {
    'graphics_get_translate_x_5: {
    'check_player_animation: {
    'check_player_state_ab: {
    'check_player_on_raft_2: {
    'check_player_hookshot_extending: {
    'check_player_state_p: {
    'check_player_state_ab_2: {
    'check_player_state_ad: {
    'handle_result: {
    'handle_result_7: {
    'handle_result_6: {
    'handle_result_9: {
    'handle_result_8: {
    'check_player_tunic_color_3: {
    'handle_result_5: {
    'handle_result_4: {
    'check_player_tunic_color_2: {
    'handle_result_3: {
    'handle_result_2: {
    'check_player_tunic_color: {
    'check_player_state_ae: {
    'check_player_state_ad_2: {
    'check_player_state_ab_3: {
    let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value.wrapping_neg(), graphics_get_translate_y_value.wrapping_neg())?;
    let drawable_x = r.get(player, field::drawable::X)?.i();
    let drawable_y = r.get(player, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x.wrapping_sub(x), drawable_y.wrapping_sub(y))?;
    host::graphics_set_clip(r, graphics, 0, 0, 24, 24)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 24, 24)?;
    let player_hurt_timer = r.get(player, field::player::HURT_TIMER)?.i();
    let remainder_value = arithmetic::remainder_int(r, player_hurt_timer, 2)?;
    if remainder_value == 1 {
return Ok(());
}
    let player_state_ag = r.get(player, field::player::STATE_AG)?.i();
    if player_state_ag != 0 {
return Ok(());
}
    let player_holding_item = r.get(player, field::player::HOLDING_ITEM)?.i();
    if player_holding_item != 0 {
let player_state_af = r.get(player, field::player::STATE_AF)?.r();
if player_state_af == 0 {
let text_value_7 = r.intern("/Images/ItemGet.png");
outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_7)?)) })();
(carry_0_r,) = (player,);
if outcome.is_err() {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
let state_out_value = r.static_get(field::system::STATE_OUT).r();
let text_value = r.intern("error loadin raft image");
host::print_stream_println(r, state_out_value, text_value)?;
return Ok(());
}
let image_create_image_value_2 = outcome.unwrap().r();
r.set(carry_0_r, field::player::STATE_AF, V::R(image_create_image_value_2))?;

}
let player_item_pose_timer = r.get(player, field::player::ITEM_POSE_TIMER)?.i();
r.set(player, field::player::ITEM_POSE_TIMER, V::I(player_item_pose_timer.wrapping_sub(1)))?;
let player_item_pose_timer_2 = r.get(player, field::player::ITEM_POSE_TIMER)?.i();
if player_item_pose_timer_2 < 0 {
r.set(player, field::player::HOLDING_ITEM, V::I(0))?;
}
host::graphics_translate(r, graphics, 2, -3)?;
host::graphics_set_clip(r, graphics, 0, 0, 18, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 27)?;
let player_item_pose_timer_3 = r.get(player, field::player::ITEM_POSE_TIMER)?.i();
if player_item_pose_timer_3 > 10 {
let player_state_af_3 = r.get(player, field::player::STATE_AF)?.r();
let player_tunic_color_7 = r.get(player, field::player::TUNIC_COLOR)?.i();
host::graphics_draw_image(r, graphics, player_state_af_3, 0, (-27_i32).wrapping_mul(player_tunic_color_7), 0)?;
return Ok(());
}
let player_state_af_2 = r.get(player, field::player::STATE_AF)?.r();
let player_tunic_color_6 = r.get(player, field::player::TUNIC_COLOR)?.i();
host::graphics_draw_image(r, graphics, player_state_af_2, -18, (-27_i32).wrapping_mul(player_tunic_color_6), 0)?;
return Ok(());
}
    let player_digging = r.get(player, field::player::DIGGING)?.i();
    if player_digging != 0 {
let player_state_ag_upper = r.get(player, field::player::STATE_AG_UPPER)?.r();
if player_state_ag_upper == 0 {
let player_tunic_color_3 = r.get(player, field::player::TUNIC_COLOR)?.i();
if player_tunic_color_3 == 0 {
let text_value_11 = r.intern("/Images/Dig_Green.png");
outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_11)?)) })();
(carry_0_r,) = (player,);
if outcome.is_err() {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
let text_value_2 = r.intern("error loadin dig image");
host::print_stream_println(r, state_out_value_2, text_value_2)?;
r.set(player, field::player::STATE_AF, V::R(0))?;
r.set(player, field::player::STATE_AB, V::R(0))?;
return Ok(());
}
let image_create_image_value_6 = outcome.unwrap().r();
r.set(carry_0_r, field::player::STATE_AG_UPPER, V::R(image_create_image_value_6))?;
}
let player_tunic_color_5 = r.get(player, field::player::TUNIC_COLOR)?.i();
if player_tunic_color_5 == 1 {
let text_value_14 = r.intern("/Images/Dig_Blue.png");
outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_14)?)) })();
(carry_0_r,) = (player,);
if outcome.is_err() {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
let text_value_2 = r.intern("error loadin dig image");
host::print_stream_println(r, state_out_value_2, text_value_2)?;
r.set(player, field::player::STATE_AF, V::R(0))?;
r.set(player, field::player::STATE_AB, V::R(0))?;
return Ok(());
}
let image_create_image_value_9 = outcome.unwrap().r();
r.set(carry_0_r, field::player::STATE_AG_UPPER, V::R(image_create_image_value_9))?;
}
let player_tunic_color_8 = r.get(player, field::player::TUNIC_COLOR)?.i();
if player_tunic_color_8 == 2 {
let text_value_15 = r.intern("/Images/Dig_Red.png");
outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_15)?)) })();
(carry_0_r,) = (player,);
if outcome.is_err() {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
let text_value_2 = r.intern("error loadin dig image");
host::print_stream_println(r, state_out_value_2, text_value_2)?;
r.set(player, field::player::STATE_AF, V::R(0))?;
r.set(player, field::player::STATE_AB, V::R(0))?;
return Ok(());
}
let image_create_image_value_10 = outcome.unwrap().r();
r.set(carry_0_r, field::player::STATE_AG_UPPER, V::R(image_create_image_value_10))?;
}

}
let player_dig_timer = r.get(player, field::player::DIG_TIMER)?.i();
r.set(player, field::player::DIG_TIMER, V::I(player_dig_timer.wrapping_sub(1)))?;
let player_dig_timer_2 = r.get(player, field::player::DIG_TIMER)?.i();
if player_dig_timer_2 <= 0 {
r.set(player, field::player::DIGGING, V::I(0))?;
}
let player_facing_8 = r.get(player, field::player::FACING)?.i();
if player_facing_8 == 0 {
let player_dig_timer_3 = r.get(player, field::player::DIG_TIMER)?.i();
if player_dig_timer_3 <= 5 {
host::graphics_set_clip(r, graphics, 0, 0, 18, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 24)?;
let player_state_ag_upper_2 = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_2, -21, 0, 0)?;
} else {
host::graphics_set_clip(r, graphics, 0, 0, 21, 24)?;
host::graphics_clip_rect(r, graphics, 0, 0, 21, 24)?;
let player_state_ag_upper_3 = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_3, 0, 0, 0)?;
}
}
let player_facing_12 = r.get(player, field::player::FACING)?.i();
if player_facing_12 == 1 {
let player_dig_timer_4 = r.get(player, field::player::DIG_TIMER)?.i();
if player_dig_timer_4 <= 5 {
host::graphics_set_clip(r, graphics, 0, 0, 19, 25)?;
host::graphics_clip_rect(r, graphics, 0, 0, 19, 25)?;
let player_state_ag_upper_4 = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_4, -20, -24, 0)?;
} else {
host::graphics_set_clip(r, graphics, 0, 0, 20, 25)?;
host::graphics_clip_rect(r, graphics, 0, 0, 20, 25)?;
let player_state_ag_upper_5 = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_5, 0, -24, 0)?;
}
}
let player_facing_17 = r.get(player, field::player::FACING)?.i();
if player_facing_17 != 2 {
let player_facing_21_read = r.get(player, field::player::FACING)?.i();
if player_facing_21_read != 3 {
return Ok(());
}
let player_dig_timer_6_read = r.get(player, field::player::DIG_TIMER)?.i();
if player_dig_timer_6_read > 5 {
host::graphics_set_clip(r, graphics, 0, 0, 20, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 20, 26)?;
let player_state_ag_upper_9_read = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_9_read, 0, -76, 0)?;
return Ok(());
}
host::graphics_set_clip(r, graphics, 0, 0, 19, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 19, 26)?;
let player_state_ag_upper_8_read = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_8_read, -20, -76, 0)?;
return Ok(());
}
let player_dig_timer_5 = r.get(player, field::player::DIG_TIMER)?.i();
if player_dig_timer_5 <= 5 {
host::graphics_set_clip(r, graphics, 0, 0, 21, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 21, 27)?;
let player_state_ag_upper_6 = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_6, -18, -49, 0)?;
} else {
host::graphics_set_clip(r, graphics, 0, 0, 18, 27)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 27)?;
let player_state_ag_upper_7 = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_7, 0, -49, 0)?;
}
let player_facing_21_read = r.get(player, field::player::FACING)?.i();
if player_facing_21_read != 3 {
return Ok(());
}
let player_dig_timer_6_read = r.get(player, field::player::DIG_TIMER)?.i();
if player_dig_timer_6_read > 5 {
host::graphics_set_clip(r, graphics, 0, 0, 20, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 20, 26)?;
let player_state_ag_upper_9_read = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_9_read, 0, -76, 0)?;
return Ok(());
}
host::graphics_set_clip(r, graphics, 0, 0, 19, 26)?;
host::graphics_clip_rect(r, graphics, 0, 0, 19, 26)?;
let player_state_ag_upper_8_read = r.get(player, field::player::STATE_AG_UPPER)?.r();
host::graphics_draw_image(r, graphics, player_state_ag_upper_8_read, -20, -76, 0)?;
return Ok(());
}
    let player_on_raft_3 = r.get(player, field::player::ON_RAFT)?.i();
    if player_on_raft_3 == 0 { break 'check_player_state_ab_2; }
    } // check_player_state_ab_3
    let player_state_ab_3 = r.get(player, field::player::STATE_AB)?.r();
    if player_state_ab_3 == 0 {
let text_value_10 = r.intern("/Images/raft.png");
outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_10)?)) })();
(carry_0_r,) = (player,);
if outcome.is_ok() {
let image_create_image_value_5 = outcome.unwrap().r();
r.set(carry_0_r, field::player::STATE_AB, V::R(image_create_image_value_5))?;

} else {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
let text_value_3 = r.intern("error loadin raft image");
host::print_stream_println(r, state_out_value_3, text_value_3)?;
}
}
    let player_fishing = r.get(player, field::player::FISHING)?.i();
    if player_fishing == 0 { break 'check_player_state_ad; }
    } // check_player_state_ad_2
    let player_state_ad = r.get(player, field::player::STATE_AD)?.r();
    if player_state_ad == 0 { break 'check_player_tunic_color; }
    } // check_player_state_ae
    let player_state_ae = r.get(player, field::player::STATE_AE)?.r();
    if player_state_ae != 0 { break 'check_player_state_p; }
    } // check_player_tunic_color
    let player_tunic_color = r.get(player, field::player::TUNIC_COLOR)?.i();
    if player_tunic_color != 0 { break 'check_player_tunic_color_2; }
    } // handle_result_2
    let text_value_8 = r.intern("/Images/GreenLinkFishing.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_8)?)) })();
    (carry_0_r,) = (player,);
    if outcome.is_err() { break 'handle_result; }
    } // handle_result_3
    let image_create_image_value_3 = outcome.unwrap().r();
    r.set(carry_0_r, field::player::STATE_AD, V::R(image_create_image_value_3))?;
    } // check_player_tunic_color_2
    let player_tunic_color_2 = r.get(player, field::player::TUNIC_COLOR)?.i();
    if player_tunic_color_2 != 1 { break 'check_player_tunic_color_3; }
    } // handle_result_4
    let text_value_9 = r.intern("/Images/BlueLinkFishing.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_9)?)) })();
    (carry_0_r,) = (player,);
    if outcome.is_err() { break 'handle_result; }
    } // handle_result_5
    let image_create_image_value_4 = outcome.unwrap().r();
    r.set(carry_0_r, field::player::STATE_AD, V::R(image_create_image_value_4))?;
    } // check_player_tunic_color_3
    let player_tunic_color_4 = r.get(player, field::player::TUNIC_COLOR)?.i();
    if player_tunic_color_4 != 2 { break 'handle_result_6; }
    } // handle_result_8
    let text_value_13 = r.intern("/Images/RedLinkFishing.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_13)?)) })();
    (carry_0_r,) = (player,);
    if outcome.is_err() { break 'handle_result; }
    } // handle_result_9
    let image_create_image_value_8 = outcome.unwrap().r();
    r.set(carry_0_r, field::player::STATE_AD, V::R(image_create_image_value_8))?;
    } // handle_result_6
    let text_value_12 = r.intern("/Images/RingsOnWater.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_12)?)) })();
    (carry_0_r,) = (player,);
    if outcome.is_err() { break 'handle_result; }
    } // handle_result_7
    let image_create_image_value_7 = outcome.unwrap().r();
    r.set(carry_0_r, field::player::STATE_AE, V::R(image_create_image_value_7))?;

    break 'check_player_state_p;
    } // handle_result
    carry_0_r = outcome.unwrap_err();
    if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
    let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
    let text_value_4 = r.intern("error loadin fishing image image");
    host::print_stream_println(r, state_out_value_4, text_value_4)?;
    break 'check_player_state_p;
    } // check_player_state_ad
    r.set(player, field::player::STATE_AD, V::R(0))?;
    r.set(player, field::player::STATE_AE, V::R(0))?;
    break 'check_player_state_p;
    } // check_player_state_ab_2
    r.set(player, field::player::STATE_AB, V::R(0))?;
    } // check_player_state_p
    let player_shield_raised = r.get(player, field::player::SHIELD_RAISED)?.i();
    if player_shield_raised == 0 {
r.set(player, field::player::STATE_AC, V::R(0))?;
} else {
let player_state_ac = r.get(player, field::player::STATE_AC)?.r();
if player_state_ac == 0 {
let text_value_6 = r.intern("/Images/Shield.png");
outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_6)?)) })();
(carry_0_r,) = (player,);
if outcome.is_ok() {
let image_create_image_value = outcome.unwrap().r();
r.set(carry_0_r, field::player::STATE_AC, V::R(image_create_image_value))?;

} else {
carry_0_r = outcome.unwrap_err();
if !(r.is_instance(carry_0_r, "java/io/IOException")) {
return Err(outcome.unwrap_err());
}
let state_out_value_5 = r.static_get(field::system::STATE_OUT).r();
let text_value_5 = r.intern("error loadin raft image");
host::print_stream_println(r, state_out_value_5, text_value_5)?;
}
}
}
    let player_attacking = r.get(player, field::player::ATTACKING)?.i();
    if player_attacking != 0 { break 'check_player_facing; }
    } // check_player_hookshot_extending
    let player_hookshot_extending = r.get(player, field::player::HOOKSHOT_EXTENDING)?.i();
    if player_hookshot_extending != 0 { break 'check_player_facing; }
    } // check_player_on_raft_2
    let player_on_raft_2 = r.get(player, field::player::ON_RAFT)?.i();
    if player_on_raft_2 == 0 { break 'check_player_state_p_2; }
    } // check_player_state_ab
    let player_state_ab_2 = r.get(player, field::player::STATE_AB)?.r();
    if player_state_ab_2 == 0 { break 'check_player_state_p_2; }
    } // check_player_animation
    r.set(player, field::player::ANIMATION, V::I(2))?;
    let player_facing_7 = r.get(player, field::player::FACING)?.i();
    if player_facing_7 == 0 {
host::graphics_translate(r, graphics, 3, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 18, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 16)?;
let player_state_ab_10 = r.get(player, field::player::STATE_AB)?.r();
host::graphics_draw_image(r, graphics, player_state_ab_10, -32, 0, 0)?;
host::graphics_translate(r, graphics, 0, 16)?;
host::graphics_set_clip(r, graphics, 0, 0, 18, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 16)?;
let player_state_ab_11 = r.get(player, field::player::STATE_AB)?.r();
host::graphics_draw_image(r, graphics, player_state_ab_11, -50, 0, 0)?;
}
    let player_facing_11 = r.get(player, field::player::FACING)?.i();
    if player_facing_11 == 1 {
host::graphics_translate(r, graphics, -3, 12)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let player_state_ab_15 = r.get(player, field::player::STATE_AB)?.r();
host::graphics_draw_image(r, graphics, player_state_ab_15, 0, 0, 0)?;
host::graphics_translate(r, graphics, 16, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let player_state_ab_16 = r.get(player, field::player::STATE_AB)?.r();
host::graphics_draw_image(r, graphics, player_state_ab_16, -16, 0, 0)?;
}
    let player_facing_16 = r.get(player, field::player::FACING)?.i();
    if player_facing_16 == 2 {
host::graphics_translate(r, graphics, 3, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 18, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 16)?;
let player_state_ab_19 = r.get(player, field::player::STATE_AB)?.r();
host::graphics_draw_image(r, graphics, player_state_ab_19, -32, 0, 0)?;
host::graphics_translate(r, graphics, 0, 16)?;
host::graphics_set_clip(r, graphics, 0, 0, 18, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 18, 16)?;
let player_state_ab_20 = r.get(player, field::player::STATE_AB)?.r();
host::graphics_draw_image(r, graphics, player_state_ab_20, -50, 0, 0)?;
}
    let player_facing_20 = r.get(player, field::player::FACING)?.i();
    if player_facing_20 == 3 {
host::graphics_translate(r, graphics, -3, 12)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let player_state_ab_21 = r.get(player, field::player::STATE_AB)?.r();
host::graphics_draw_image(r, graphics, player_state_ab_21, 0, 0, 0)?;
host::graphics_translate(r, graphics, 16, 0)?;
host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
let player_state_ab_22 = r.get(player, field::player::STATE_AB)?.r();
host::graphics_draw_image(r, graphics, player_state_ab_22, -16, 0, 0)?;
}
    let player_fishing_2 = r.get(player, field::player::FISHING)?.i();
    if player_fishing_2 == 0 { break 'check_player_state_p_2; }
    } // graphics_get_translate_x_5
    let graphics_get_translate_x_value_17 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_17 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_17.wrapping_neg(), graphics_get_translate_y_value_17.wrapping_neg())?;
    let drawable_x_17 = r.get(player, field::drawable::X)?.i();
    let drawable_y_17 = r.get(player, field::drawable::Y)?.i();
    host::graphics_translate(r, graphics, drawable_x_17.wrapping_sub(x), drawable_y_17.wrapping_sub(y))?;
    width = 0;
    height = 0;
    let player_facing_22 = r.get(player, field::player::FACING)?.i();
    if player_facing_22 == 2 { break 'graphics_translate; }
    } // check_player_facing_5
    let player_facing_23 = r.get(player, field::player::FACING)?.i();
    if player_facing_23 != 3 { break 'graphics_translate_2; }
    } // graphics_translate
    host::graphics_translate(r, graphics, -4, 0)?;
    width = -4;
    break 'check_player_state_ac_lower;
    } // graphics_translate_2
    host::graphics_translate(r, graphics, -4, 0)?;
    width = 20;
    } // check_player_state_ac_lower
    let player_state_ac_lower = r.get(player, field::player::STATE_AC_LOWER)?.i();
    if player_state_ac_lower != 1 { break 'check_player_state_ac_lower_2; }
    } // graphics_set_clip
    host::graphics_set_clip(r, graphics, 0, 0, 28, 23)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 28, 23)?;
    let player_facing_24 = r.get(player, field::player::FACING)?.i();
    if player_facing_24 == 0 { break 'check_player_state_ad_3; }
    } // check_player_facing_6
    let player_facing_26 = r.get(player, field::player::FACING)?.i();
    if player_facing_26 != 1 { break 'check_player_state_ad_5; }
    } // check_player_state_ad_3
    let player_state_ad_2 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_2, 27, 0, 28, 23, 2, 0, 0, 0)?;
    break 'next_step_4;
    } // check_player_state_ad_5
    let player_state_ad_4 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_4, 27, 0, 28, 23, 0, 0, 0, 0)?;
    } // next_step_4
    height = 3;
    } // check_player_state_ac_lower_2
    let player_state_ac_lower_2 = r.get(player, field::player::STATE_AC_LOWER)?.i();
    if player_state_ac_lower_2 != 2 { break 'check_player_state_ac_lower_3; }
    } // graphics_set_clip_2
    host::graphics_set_clip(r, graphics, 0, 0, 28, 23)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 28, 23)?;
    let player_facing_25 = r.get(player, field::player::FACING)?.i();
    if player_facing_25 == 0 { break 'check_player_state_ad_4; }
    } // check_player_facing_7
    let player_facing_28 = r.get(player, field::player::FACING)?.i();
    if player_facing_28 != 1 { break 'check_player_state_ad_7; }
    } // check_player_state_ad_4
    let player_state_ad_3 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_3, 55, 0, 28, 23, 2, 0, 0, 0)?;
    break 'next_step_5;
    } // check_player_state_ad_7
    let player_state_ad_6 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_6, 55, 0, 28, 23, 0, 0, 0, 0)?;
    } // next_step_5
    height = 4;
    } // check_player_state_ac_lower_3
    let player_state_ac_lower_3 = r.get(player, field::player::STATE_AC_LOWER)?.i();
    if player_state_ac_lower_3 != 3 { break 'check_player_state_ac_lower_4; }
    } // graphics_set_clip_3
    host::graphics_set_clip(r, graphics, 0, 0, 28, 23)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 28, 23)?;
    let player_facing_27 = r.get(player, field::player::FACING)?.i();
    if player_facing_27 == 0 { break 'check_player_state_ad_6; }
    } // check_player_facing_8
    let player_facing_30 = r.get(player, field::player::FACING)?.i();
    if player_facing_30 != 1 { break 'check_player_state_ad_9; }
    } // check_player_state_ad_6
    let player_state_ad_5 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_5, 83, 0, 28, 23, 2, 0, 0, 0)?;
    break 'next_step_6;
    } // check_player_state_ad_9
    let player_state_ad_8 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_8, 83, 0, 28, 23, 0, 0, 0, 0)?;
    } // next_step_6
    height = 5;
    } // check_player_state_ac_lower_4
    let player_state_ac_lower_4 = r.get(player, field::player::STATE_AC_LOWER)?.i();
    if player_state_ac_lower_4 != 4 { break 'check_player_state_ac_lower_5; }
    } // graphics_set_clip_4
    host::graphics_set_clip(r, graphics, 0, 0, 29, 23)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 29, 23)?;
    let player_facing_29 = r.get(player, field::player::FACING)?.i();
    if player_facing_29 == 0 { break 'check_player_state_ad_8; }
    } // check_player_facing_9
    let player_facing_32 = r.get(player, field::player::FACING)?.i();
    if player_facing_32 != 1 { break 'check_player_state_ad_11; }
    } // check_player_state_ad_8
    let player_state_ad_7 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_7, 111, 0, 29, 23, 2, 0, 0, 0)?;
    break 'next_step_7;
    } // check_player_state_ad_11
    let player_state_ad_10 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_10, 111, 0, 29, 23, 0, 0, 0, 0)?;
    } // next_step_7
    height = 5;
    } // check_player_state_ac_lower_5
    let player_state_ac_lower_5 = r.get(player, field::player::STATE_AC_LOWER)?.i();
    if player_state_ac_lower_5 != 5 { break 'check_player_state_ac_lower_6; }
    } // graphics_translate_3
    host::graphics_translate(r, graphics, 3, 0)?;
    host::graphics_set_clip(r, graphics, 0, 0, 25, 23)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 25, 23)?;
    let player_facing_31 = r.get(player, field::player::FACING)?.i();
    if player_facing_31 == 0 { break 'check_player_state_ad_10; }
    } // check_player_facing_10
    let player_facing_34 = r.get(player, field::player::FACING)?.i();
    if player_facing_34 != 1 { break 'check_player_state_ad_12; }
    } // check_player_state_ad_10
    let player_state_ad_9 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_9, 140, 0, 25, 23, 2, 0, 0, 0)?;
    width = 16;
    break 'next_step_9;
    } // check_player_state_ad_12
    let player_state_ad_12 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_12, 140, 0, 25, 23, 0, 0, 0, 0)?;
    width = -1;
    } // next_step_9
    height = 6;
    } // check_player_state_ac_lower_6
    let player_state_ac_lower_6 = r.get(player, field::player::STATE_AC_LOWER)?.i();
    if player_state_ac_lower_6 != 6 { break 'graphics_get_translate_x_6; }
    } // graphics_translate_4
    host::graphics_translate(r, graphics, 3, 0)?;
    host::graphics_set_clip(r, graphics, 0, 0, 25, 23)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 25, 23)?;
    let player_facing_33 = r.get(player, field::player::FACING)?.i();
    if player_facing_33 == 0 { break 'next_step_8; }
    } // check_player_facing_11
    let player_facing_35 = r.get(player, field::player::FACING)?.i();
    if player_facing_35 != 1 { break 'next_step_11; }
    } // next_step_8
    width = 16;
    let player_state_ad_11 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_11, 165, 0, 25, 23, 2, 0, 0, 0)?;
    break 'next_step_10;
    } // next_step_11
    width = 1;
    let player_state_ad_13 = r.get(player, field::player::STATE_AD)?.r();
    host::graphics_draw_region(r, graphics, player_state_ad_13, 165, 0, 25, 23, 0, 0, 0, 0)?;
    } // next_step_10
    height = 6;
    } // graphics_get_translate_x_6
    let graphics_get_translate_x_value_19 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_19 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_19.wrapping_neg(), graphics_get_translate_y_value_19.wrapping_neg())?;
    let player_state_aa = r.get(player, field::player::STATE_AA)?.i();
    let player_state_ab_lower = r.get(player, field::player::STATE_AB_LOWER)?.i();
    host::graphics_translate(
        r,
        graphics,
        (player_state_aa)
            .wrapping_sub(x),
        (player_state_ab_lower)
            .wrapping_sub(y),
    )?;
    let object_value = r.alloc(
        "java/util/Random",
        Native::Empty,
    );
    host::random_initialize(
        r,
        object_value,
    )?;
    object_6 = object_value;
    let random_next_int_value =
        host::random_next_int(
            r,
            object_value,
        )?;
    let remainder_value_6 = arithmetic::remainder_int(r, random_next_int_value, 2)?;
    if remainder_value_6 != 1 {
        host::graphics_translate(r, graphics, -6, -4)?;
        host::graphics_set_clip(r, graphics, 0, 0, 12, 8)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 12, 8)?;
        let player_state_ae_2 = r.get(player, field::player::STATE_AE)?.r();
        host::graphics_draw_image(r, graphics, player_state_ae_2, -14, 0, 0)?;
    } else {
        host::graphics_translate(r, graphics, -7, -4)?;
        host::graphics_set_clip(r, graphics, 0, 0, 14, 8)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 14, 8)?;
        let player_state_ae_3 = r.get(player, field::player::STATE_AE)?.r();
        host::graphics_draw_image(r, graphics, player_state_ae_3, 0, 0, 0)?;
    }
    let graphics_get_translate_x_value_20 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_20 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_20.wrapping_neg(), graphics_get_translate_y_value_20.wrapping_neg())?;
    host::graphics_set_clip(
        r, graphics, 0, 0, 256,
        176,
    )?;
    host::graphics_clip_rect(
        r, graphics, 0, 0, 256,
        176,
    )?;
    host::graphics_set_color_2(
        r, graphics, 0, 0, 0,
    )?;
    let player_state_aa_2 = r.get(player, field::player::STATE_AA)?.i();
    let player_state_ab_lower_2 = r.get(player, field::player::STATE_AB_LOWER)?.i();
    let x_value_19 = r
        .get(
            player,
            field::drawable::X,
        )?
        .i();
    let y_value_19 = r
        .get(
            player,
            field::drawable::Y,
        )?
        .i();
    host::graphics_draw_line(r, graphics, player_state_aa_2.wrapping_sub(x), player_state_ab_lower_2.wrapping_sub(y), (x_value_19.wrapping_sub(x)).wrapping_add(width), (y_value_19.wrapping_sub(y)).wrapping_add(height))?;
    let previous_player_value =
        player;

    let player_canvas = r.get(previous_player_value, field::player::CANVAS)?.r();
    let canvas_world = r.get(player_canvas, field::canvas::WORLD)?.r();
    let player_state_aa_3 = r.get(player, field::player::STATE_AA)?.i();
    let player_state_ab_lower_3 = r.get(player, field::player::STATE_AB_LOWER)?.i();
    let can_entity_move_value = super::world::can_entity_move(r, canvas_world, player_state_aa_3.wrapping_add(1), player_state_ab_lower_3, 1, 26, 0)?;
    if can_entity_move_value
        == 0
    {
        return Ok(());
    }
    let player_state_aa_4 = r.get(player, field::player::STATE_AA)?.i();
    r.set(
        player,
        field::player::STATE_AA,
        V::I(
            (player_state_aa_4)
                .wrapping_add(
                    1,
                ),
        ),
    )?;
    return Ok(());
    } // check_player_state_p_2
    let player_shield_raised_2 = r.get(player, field::player::SHIELD_RAISED)?.i();
    if player_shield_raised_2 != 0 {
        let player_facing_6 = r.get(player, field::player::FACING)?.i();
        if player_facing_6 == 0 {
            let graphics_get_translate_x_value_6 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_6 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(r, graphics, graphics_get_translate_x_value_6.wrapping_neg(), graphics_get_translate_y_value_6.wrapping_neg())?;
            let drawable_x_6 = r.get(player, field::drawable::X)?.i();
            let drawable_y_6 = r.get(player, field::drawable::Y)?.i();
            host::graphics_translate(r, graphics, (drawable_x_6.wrapping_sub(x)).wrapping_add(14), (drawable_y_6.wrapping_sub(y)).wrapping_add(8))?;
            host::graphics_set_clip(r, graphics, 0, 0, 10, 12)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 10, 12)?;
            let player_state_ac_2 = r.get(player, field::player::STATE_AC)?.r();
            host::graphics_draw_image(r, graphics, player_state_ac_2, -33, 0, 0)?;
            let player_animation_2 = r.get(player, field::player::ANIMATION)?.i();
            if player_animation_2 > 3
            {
                r.set(player, field::player::ANIMATION, V::I(0))?;
            }
        }
    }
    let graphics_get_translate_x_value_3 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_3 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_3.wrapping_neg(), graphics_get_translate_y_value_3.wrapping_neg())?;
    let x_value_3 = r
        .get(
            player,
            field::drawable::X,
        )?
        .i();
    let y_value_3 = r
        .get(
            player,
            field::drawable::Y,
        )?
        .i();
    host::graphics_translate(
        r,
        graphics,
        x_value_3.wrapping_sub(x),
        y_value_3.wrapping_sub(y),
    )?;
    host::graphics_set_clip(
        r, graphics, 0, 0, 24, 24,
    )?;
    host::graphics_clip_rect(
        r, graphics, 0, 0, 24, 24,
    )?;
    let sprite_value = r
        .get(
            player,
            field::player::SPRITE,
        )?
        .r();
    let player_animation = r.get(player, field::player::ANIMATION)?.i();
    let facing_value_5 = r
        .get(
            player,
            field::player::FACING,
        )?
        .i();
    host::graphics_draw_image(
        r,
        graphics,
        sprite_value,
        (((player_animation)
            .wrapping_shr(
                (1) as u32,
            ))
        .wrapping_neg())
        .wrapping_mul(24),
        ((facing_value_5)
            .wrapping_neg())
        .wrapping_mul(24),
        0,
    )?;
    let player_shield_raised_3 = r.get(player, field::player::SHIELD_RAISED)?.i();
    if player_shield_raised_3 == 0 {
        return Ok(());
    }
    let facing_value_10 = r
        .get(
            player,
            field::player::FACING,
        )?
        .i();
    if facing_value_10 == 1 {
        let graphics_get_translate_x_value_9 = host::graphics_get_translate_x(r, graphics)?;
        let graphics_get_translate_y_value_9 = host::graphics_get_translate_y(r, graphics)?;
        host::graphics_translate(r, graphics, graphics_get_translate_x_value_9.wrapping_neg(), graphics_get_translate_y_value_9.wrapping_neg())?;
        let x_value_9 = r
            .get(
                player,
                field::drawable::X,
            )?
            .i();
        let y_value_9 = r
            .get(
                player,
                field::drawable::Y,
            )?
            .i();
        let player_animation_3 = r.get(player, field::player::ANIMATION)?.i();
        host::graphics_translate(
            r,
            graphics,
            ((x_value_9)
                .wrapping_sub(x))
            .wrapping_add(17),
            (((y_value_9)
                .wrapping_sub(y))
            .wrapping_add(7))
            .wrapping_sub(
                (player_animation_3)
                    .wrapping_shr(
                        (1) as u32,
                    ),
            ),
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 6,
            12,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 6,
            12,
        )?;
        let player_state_ac_3 = r.get(player, field::player::STATE_AC)?.r();
        let player_animation_4 = r.get(player, field::player::ANIMATION)?.i();
        host::graphics_draw_image(r, graphics, player_state_ac_3, (-18_i32).wrapping_sub((player_animation_4.wrapping_shr((1) as u32)).wrapping_mul(6)), -12, 0)?;
    }
    let facing_value_15 = r
        .get(
            player,
            field::player::FACING,
        )?
        .i();
    if facing_value_15 == 2 {
        let graphics_get_translate_x_value_13 = host::graphics_get_translate_x(r, graphics)?;
        let graphics_get_translate_y_value_13 = host::graphics_get_translate_y(r, graphics)?;
        host::graphics_translate(r, graphics, graphics_get_translate_x_value_13.wrapping_neg(), graphics_get_translate_y_value_13.wrapping_neg())?;
        let x_value_13 = r
            .get(
                player,
                field::drawable::X,
            )?
            .i();
        let y_value_13 = r
            .get(
                player,
                field::drawable::Y,
            )?
            .i();
        host::graphics_translate(
            r,
            graphics,
            ((x_value_13)
                .wrapping_sub(x))
            .wrapping_add(3),
            ((y_value_13)
                .wrapping_sub(y))
            .wrapping_add(12),
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 10,
            12,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 10,
            12,
        )?;
        let player_state_ac_4 = r.get(player, field::player::STATE_AC)?.r();
        let player_animation_5 = r.get(player, field::player::ANIMATION)?.i();
        host::graphics_draw_image(
            r,
            graphics,
            player_state_ac_4,
            (((player_animation_5)
                .wrapping_shr(
                    (1) as u32,
                ))
            .wrapping_neg())
            .wrapping_mul(12),
            0,
            0,
        )?;
    }
    let facing_value_19 = r
        .get(
            player,
            field::player::FACING,
        )?
        .i();
    if facing_value_19 != 3 {
        return Ok(());
    }
    let graphics_get_translate_x_value_15 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_15 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(r, graphics, graphics_get_translate_x_value_15.wrapping_neg(), graphics_get_translate_y_value_15.wrapping_neg())?;
    let x_value_15 = r
        .get(
            player,
            field::drawable::X,
        )?
        .i();
    let y_value_15 = r
        .get(
            player,
            field::drawable::Y,
        )?
        .i();
    let player_animation_6 = r.get(player, field::player::ANIMATION)?.i();
    host::graphics_translate(
        r,
        graphics,
        ((x_value_15)
            .wrapping_sub(x))
        .wrapping_add(1),
        (((y_value_15)
            .wrapping_sub(y))
        .wrapping_add(7))
        .wrapping_sub(
            (player_animation_6)
                .wrapping_shr(
                    (1) as u32,
                ),
        ),
    )?;
    host::graphics_set_clip(
        r, graphics, 0, 0, 6, 12,
    )?;
    host::graphics_clip_rect(
        r, graphics, 0, 0, 6, 12,
    )?;
    let state_ac_value_5 = r
        .get(
            player,
            field::player::STATE_AC,
        )?
        .r();
    let player_animation_7 = r.get(player, field::player::ANIMATION)?.i();
    host::graphics_draw_image(
        r,
        graphics,
        state_ac_value_5,
        (((player_animation_7)
            .wrapping_shr(
                (1) as u32,
            ))
        .wrapping_neg())
        .wrapping_mul(6),
        -12,
        0,
    )?;
    return Ok(());
    } // check_player_facing
    let facing_value = r
        .get(
            player,
            field::player::FACING,
        )?
        .i();
    if facing_value != 0 {
        break 'check_player_facing_2;
    }
    } // check_player_on_raft
    let on_raft_value = r
        .get(
            player,
            field::player::ON_RAFT,
        )?
        .i();
    if on_raft_value != 0 {
        let state_ab_value = r
            .get(
                player,
                field::player::STATE_AB,
            )?
            .r();
        if state_ab_value != 0 {
            host::graphics_translate(
                r, graphics, 3, -4,
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 18, 16,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 18, 16,
            )?;
            let state_ab_value_5 = r
                .get(
                    player,
                    field::player::STATE_AB,
                )?
                .r();
            host::graphics_draw_image(
                r,
                graphics,
                state_ab_value_5,
                -32,
                0,
                0,
            )?;
            host::graphics_translate(
                r, graphics, 0, 16,
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 18, 16,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 18, 16,
            )?;
            let state_ab_value_6 = r
                .get(
                    player,
                    field::player::STATE_AB,
                )?
                .r();
            host::graphics_draw_image(
                r,
                graphics,
                state_ab_value_6,
                -50,
                0,
                0,
            )?;
            let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_2 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(r, graphics, graphics_get_translate_x_value_2.wrapping_neg(), graphics_get_translate_y_value_2.wrapping_neg())?;
            let x_value_2 = r
                .get(
                    player,
                    field::drawable::X,
                )?
                .i();
            let y_value_2 = r
                .get(
                    player,
                    field::drawable::Y,
                )?
                .i();
            host::graphics_translate(
                r,
                graphics,
                x_value_2.wrapping_sub(x),
                y_value_2.wrapping_sub(y),
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 24, 24,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 24, 24,
            )?;
        }
    }
    let moving_value_2 = r
        .get(
            player,
            field::player::ATTACKING,
        )?
        .i();
    if moving_value_2 != 0 {
        host::graphics_translate(
            r, graphics, 6, -9,
        )?;
        host::graphics_set_clip(
            r, graphics, 0, 0, 9, 18,
        )?;
        host::graphics_clip_rect(
            r, graphics, 0, 0, 9, 18,
        )?;
        let player_equipment_sprite = r.get(player, field::player::EQUIPMENT_SPRITE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            player_equipment_sprite,
            0,
            0,
            0,
        )?;
    }
    let player_hookshot_extending_2 = r.get(player, field::player::HOOKSHOT_EXTENDING)?.i();
    if player_hookshot_extending_2 == 0 {
        break 'graphics_get_translate_x_2;
    }
    } // next_step
    width = 0;
    } // check_player_hookshot_trail
    'draw_loop_69: loop {
        'check_player_hookshot_trail_3: {
        let hookshot_trail_value = r
            .get(
                player,
                field::player::HOOKSHOT_TRAIL,
            )?
            .r();
        let vector_size_value =
            host::vector_size(
                r,
                hookshot_trail_value,
            )?;
        if width >= vector_size_value {
            break 'graphics_get_translate_x_2;
        }
        } // check_player_hookshot_trail_3
        let hookshot_trail_value_3 = r
            .get(
                player,
                field::player::HOOKSHOT_TRAIL,
            )?
            .r();
        let vector_element_at_value =
            host::vector_element_at(
                r,
                hookshot_trail_value_3,
                width,
            )?;
        host::check_cast(
            r,
            vector_element_at_value,
            "g",
        )?;
        object_5 = vector_element_at_value;
        let graphics_get_translate_x_value_12 =
            host::graphics_get_translate_x(
                r, graphics,
            )?;
        let graphics_get_translate_y_value_12 =
            host::graphics_get_translate_y(
                r, graphics,
            )?;
        host::graphics_translate(
            r,
            graphics,
            (graphics_get_translate_x_value_12)
                .wrapping_neg(),
            (graphics_get_translate_y_value_12)
                .wrapping_neg(),
        )?;
        let x_value_12 = r
            .get(object_5, field::trail_point::X)?
            .i();
        let y_value_12 = r
            .get(object_5, field::trail_point::Y)?
            .i();
        host::graphics_translate(
            r,
            graphics,
            x_value_12.wrapping_sub(x),
            y_value_12.wrapping_sub(y),
        )?;
        let hookshot_trail_value_4 = r
            .get(
                player,
                field::player::HOOKSHOT_TRAIL,
            )?
            .r();
        let vector_size_value_3 =
            host::vector_size(
                r,
                hookshot_trail_value_4,
            )?;
        if width
            != vector_size_value_3.wrapping_sub(1)
        {
            host::graphics_set_clip(
                r, graphics, 0, 0, 8, 8,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 8, 8,
            )?;
            let action_sprite_value = r
                .get(
                    player,
                    field::player::ACTION_SPRITE,
                )?
                .r();
            let remainder_value_2 =
                arithmetic::remainder_int(
                    r, width, 2,
                )?;
            host::graphics_draw_image(
                r,
                graphics,
                action_sprite_value,
                0,
                ((remainder_value_2)
                    .wrapping_neg())
                .wrapping_shl((3) as u32),
                0,
            )?;
        } else {
            host::graphics_translate(
                r, graphics, -2, 0,
            )?;
            host::graphics_set_clip(
                r, graphics, 0, 0, 12, 8,
            )?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 12, 8,
            )?;
            let action_sprite_value_2 = r
                .get(
                    player,
                    field::player::ACTION_SPRITE,
                )?
                .r();
            host::graphics_draw_image(
                r,
                graphics,
                action_sprite_value_2,
                0,
                -16,
                0,
            )?;
        }
        width = width.wrapping_add(1);
        continue 'draw_loop_69;
    }
    } // graphics_get_translate_x_2
    let graphics_get_translate_x_value_5 =
        host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_5 =
        host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        (graphics_get_translate_x_value_5)
            .wrapping_neg(),
        (graphics_get_translate_y_value_5)
            .wrapping_neg(),
    )?;
    let drawable_x_5 =
        r.get(player, field::drawable::X)?.i();
    let drawable_y_5 =
        r.get(player, field::drawable::Y)?.i();
    host::graphics_translate(
        r,
        graphics,
        drawable_x_5.wrapping_sub(x),
        drawable_y_5.wrapping_sub(y),
    )?;
    host::graphics_set_clip(r, graphics, 0, 0, 24, 24)?;
    host::graphics_clip_rect(
        r, graphics, 0, 0, 24, 24,
    )?;
    let player_sprite_2 =
        r.get(player, field::player::SPRITE)?.r();
    let player_facing_9 =
        r.get(player, field::player::FACING)?.i();
    host::graphics_draw_image(
        r,
        graphics,
        player_sprite_2,
        -72,
        (player_facing_9.wrapping_neg())
            .wrapping_mul(24),
        0,
    )?;
    } // check_player_facing_2
    let player_facing_2 =
        r.get(player, field::player::FACING)?.i();
    if player_facing_2 != 1 {
        break 'check_player_facing_3;
    }
    } // check_player_on_raft_3
    let player_on_raft_4 =
        r.get(player, field::player::ON_RAFT)?.i();
    if player_on_raft_4 != 0 {
        let player_state_ab_4 =
            r.get(player, field::player::STATE_AB)?.r();
        if player_state_ab_4 != 0 {
            host::graphics_translate(r, graphics, -3, 12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 16, 16,
            )?;
            let player_state_ab_8 =
                r.get(player, field::player::STATE_AB)?.r();
            host::graphics_draw_image(
                r,
                graphics,
                player_state_ab_8,
                0,
                0,
                0,
            )?;
            host::graphics_translate(r, graphics, 16, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 16, 16,
            )?;
            let player_state_ab_9 =
                r.get(player, field::player::STATE_AB)?.r();
            host::graphics_draw_image(
                r,
                graphics,
                player_state_ab_9,
                -16,
                0,
                0,
            )?;
            let graphics_get_translate_x_value_4 =
                host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_4 =
                host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                (graphics_get_translate_x_value_4)
                    .wrapping_neg(),
                (graphics_get_translate_y_value_4)
                    .wrapping_neg(),
            )?;
            let drawable_x_4 =
                r.get(player, field::drawable::X)?.i();
            let drawable_y_4 =
                r.get(player, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                drawable_x_4.wrapping_sub(x),
                drawable_y_4.wrapping_sub(y),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 24, 24)?;
            host::graphics_clip_rect(
                r, graphics, 0, 0, 24, 24,
            )?;
        }
    }
    let player_attacking_3 =
        r.get(player, field::player::ATTACKING)?.i();
    if player_attacking_3 != 0 {
        host::graphics_translate(r, graphics, 17, 12)?;
        host::graphics_set_clip(r, graphics, 0, 0, 18, 8)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 18, 8)?;
        let player_equipment_sprite_2 =
            r.get(player, field::player::EQUIPMENT_SPRITE)?.r();
        host::graphics_draw_image(
            r,
            graphics,
            player_equipment_sprite_2,
            -18,
            0,
            0,
        )?;
    }
    let player_hookshot_extending_3 =
        r.get(player, field::player::HOOKSHOT_EXTENDING)?.i();
    if player_hookshot_extending_3 == 0 {
        break 'graphics_get_translate_x_3;
    }
    } // next_step_2
    width = 0;
    } // check_player_hookshot_trail_2
    'draw_loop_75: loop {
        'check_player_hookshot_trail_5: {
        let player_hookshot_trail_2 =
            r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_2 =
            host::vector_size(r, player_hookshot_trail_2)?;
        if width >= vector_size_value_2 {
            break 'graphics_get_translate_x_3;
        }
        } // check_player_hookshot_trail_5
        let player_hookshot_trail_6 =
            r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_element_at_value_2 =
            host::vector_element_at(r, player_hookshot_trail_6, width)?;
        host::check_cast(r, vector_element_at_value_2, "g")?;
        object_5 = vector_element_at_value_2;
        let graphics_get_translate_x_value_14 =
            host::graphics_get_translate_x(r, graphics)?;
        let graphics_get_translate_y_value_14 =
            host::graphics_get_translate_y(r, graphics)?;
        host::graphics_translate(
            r,
            graphics,
            graphics_get_translate_x_value_14.wrapping_neg(),
            graphics_get_translate_y_value_14.wrapping_neg(),
        )?;
        let trail_point_x_14 = r.get(object_5, field::trail_point::X)?.i();
        let trail_point_y_14 = r.get(object_5, field::trail_point::Y)?.i();
        host::graphics_translate(
            r,
            graphics,
            trail_point_x_14.wrapping_sub(x),
            trail_point_y_14.wrapping_sub(y),
        )?;
        let player_hookshot_trail_7 =
            r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_5 =
            host::vector_size(r, player_hookshot_trail_7)?;
        if width != vector_size_value_5.wrapping_sub(1) {
            host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
            let player_action_sprite_3 =
                r.get(player, field::player::ACTION_SPRITE)?.r();
            let remainder_value_3 =
                arithmetic::remainder_int(r, width, 2)?;
            host::graphics_draw_image(
                r,
                graphics,
                player_action_sprite_3,
                0,
                (remainder_value_3.wrapping_neg())
                    .wrapping_shl((3) as u32),
                0,
            )?;
        } else {
            host::graphics_translate(r, graphics, 0, -4)?;
            host::graphics_set_clip(r, graphics, 0, 0, 8, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 16)?;
            let player_action_sprite_4 =
                r.get(player, field::player::ACTION_SPRITE)?.r();
            host::graphics_draw_image(
                r,
                graphics,
                player_action_sprite_4,
                -8,
                0,
                0,
            )?;
        }
        width = width.wrapping_add(1);
        continue 'draw_loop_75;
    }
    } // graphics_get_translate_x_3
    let graphics_get_translate_x_value_8 =
        host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_8 =
        host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_8.wrapping_neg(),
        graphics_get_translate_y_value_8.wrapping_neg(),
    )?;
    let drawable_x_8 = r.get(player, field::drawable::X)?.i();
    let drawable_y_8 = r.get(player, field::drawable::Y)?.i();
    host::graphics_translate(
        r,
        graphics,
        drawable_x_8.wrapping_sub(x),
        drawable_y_8.wrapping_sub(y),
    )?;
    host::graphics_set_clip(r, graphics, 0, 0, 24, 24)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 24, 24)?;
    let player_sprite_4 = r.get(player, field::player::SPRITE)?.r();
    let player_facing_14 = r.get(player, field::player::FACING)?.i();
    host::graphics_draw_image(
        r,
        graphics,
        player_sprite_4,
        -72,
        (player_facing_14.wrapping_neg()).wrapping_mul(24),
        0,
    )?;
    } // check_player_facing_3
    let player_facing_3 = r.get(player, field::player::FACING)?.i();
    if player_facing_3 != 2 {
        break 'check_player_facing_4;
    }
    } // check_player_on_raft_4
    let player_on_raft_5 = r.get(player, field::player::ON_RAFT)?.i();
    if player_on_raft_5 != 0 {
        let player_state_ab_7 = r.get(player, field::player::STATE_AB)?.r();
        if player_state_ab_7 != 0 {
            host::graphics_translate(r, graphics, 3, -4)?;
            host::graphics_set_clip(r, graphics, 0, 0, 18, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 18, 16)?;
            let player_state_ab_13 = r.get(player, field::player::STATE_AB)?.r();
            host::graphics_draw_image(r, graphics, player_state_ab_13, -32, 0, 0)?;
            host::graphics_translate(r, graphics, 0, 16)?;
            host::graphics_set_clip(r, graphics, 0, 0, 18, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 18, 16)?;
            let player_state_ab_14 = r.get(player, field::player::STATE_AB)?.r();
            host::graphics_draw_image(r, graphics, player_state_ab_14, -50, 0, 0)?;
            let graphics_get_translate_x_value_7 =
                host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_7 =
                host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_7.wrapping_neg(),
                graphics_get_translate_y_value_7.wrapping_neg(),
            )?;
            let drawable_x_7 = r.get(player, field::drawable::X)?.i();
            let drawable_y_7 = r.get(player, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                drawable_x_7.wrapping_sub(x),
                drawable_y_7.wrapping_sub(y),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 24, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 24, 24)?;
        }
    }
    let player_attacking_4 = r.get(player, field::player::ATTACKING)?.i();
    if player_attacking_4 != 0 {
        host::graphics_translate(r, graphics, 12, 20)?;
        host::graphics_set_clip(r, graphics, 0, 0, 9, 18)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 9, 18)?;
        let player_equipment_sprite_3 =
            r.get(player, field::player::EQUIPMENT_SPRITE)?.r();
        host::graphics_draw_image(r, graphics, player_equipment_sprite_3, -9, 0, 0)?;
    }
    let player_hookshot_extending_4 =
        r.get(player, field::player::HOOKSHOT_EXTENDING)?.i();
    if player_hookshot_extending_4 == 0 {
        break 'graphics_get_translate_x_4;
    }
    } // next_step_3
    width = 0;
    } // check_player_hookshot_trail_4
    'draw_loop_81: loop {
        'check_player_hookshot_trail_6: {
        let player_hookshot_trail_5 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_4 = host::vector_size(r, player_hookshot_trail_5)?;
        if width >= vector_size_value_4 {
            break 'graphics_get_translate_x_4;
        }
        } // check_player_hookshot_trail_6
        let player_hookshot_trail_9 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_element_at_value_3 =
            host::vector_element_at(r, player_hookshot_trail_9, width)?;
        host::check_cast(r, vector_element_at_value_3, "g")?;
        object_5 = vector_element_at_value_3;
        let graphics_get_translate_x_value_16 =
            host::graphics_get_translate_x(r, graphics)?;
        let graphics_get_translate_y_value_16 =
            host::graphics_get_translate_y(r, graphics)?;
        host::graphics_translate(
            r,
            graphics,
            graphics_get_translate_x_value_16.wrapping_neg(),
            graphics_get_translate_y_value_16.wrapping_neg(),
        )?;
        let trail_point_x_16 = r.get(object_5, field::trail_point::X)?.i();
        let trail_point_y_16 = r.get(object_5, field::trail_point::Y)?.i();
        host::graphics_translate(
            r,
            graphics,
            trail_point_x_16.wrapping_sub(x),
            trail_point_y_16.wrapping_sub(y),
        )?;
        let player_hookshot_trail_10 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_7 = host::vector_size(r, player_hookshot_trail_10)?;
        if width != vector_size_value_7.wrapping_sub(1) {
            host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
            let player_action_sprite_5 = r.get(player, field::player::ACTION_SPRITE)?.r();
            let remainder_value_4 = arithmetic::remainder_int(r, width, 2)?;
            host::graphics_draw_image(
                r,
                graphics,
                player_action_sprite_5,
                0,
                (remainder_value_4.wrapping_neg()).wrapping_shl((3) as u32),
                0,
            )?;
        } else {
            host::graphics_translate(r, graphics, -2, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 12, 8)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 12, 8)?;
            let player_action_sprite_6 = r.get(player, field::player::ACTION_SPRITE)?.r();
            host::graphics_draw_image(r, graphics, player_action_sprite_6, -12, -16, 0)?;
        }
        width = width.wrapping_add(1);
        continue 'draw_loop_81;
    }
    } // graphics_get_translate_x_4
    let graphics_get_translate_x_value_11 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_11 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_11.wrapping_neg(),
        graphics_get_translate_y_value_11.wrapping_neg(),
    )?;
    let drawable_x_11 = r.get(player, field::drawable::X)?.i();
    let drawable_y_11 = r.get(player, field::drawable::Y)?.i();
    host::graphics_translate(
        r,
        graphics,
        drawable_x_11.wrapping_sub(x),
        drawable_y_11.wrapping_sub(y),
    )?;
    host::graphics_set_clip(r, graphics, 0, 0, 24, 24)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 24, 24)?;
    let player_sprite_5 = r.get(player, field::player::SPRITE)?.r();
    let player_facing_18 = r.get(player, field::player::FACING)?.i();
    host::graphics_draw_image(
        r,
        graphics,
        player_sprite_5,
        -72,
        (player_facing_18.wrapping_neg()).wrapping_mul(24),
        0,
    )?;
    } // check_player_facing_4
    let player_facing_4 = r.get(player, field::player::FACING)?.i();
    if player_facing_4 != 3 {
        return Ok(());
    }
    let player_on_raft_6 = r.get(player, field::player::ON_RAFT)?.i();
    if player_on_raft_6 != 0 {
        let player_state_ab_12 = r.get(player, field::player::STATE_AB)?.r();
        if player_state_ab_12 != 0 {
            host::graphics_translate(r, graphics, -3, 12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let player_state_ab_17 = r.get(player, field::player::STATE_AB)?.r();
            host::graphics_draw_image(r, graphics, player_state_ab_17, 0, 0, 0)?;
            host::graphics_translate(r, graphics, 16, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let player_state_ab_18 = r.get(player, field::player::STATE_AB)?.r();
            host::graphics_draw_image(r, graphics, player_state_ab_18, -16, 0, 0)?;
            let graphics_get_translate_x_value_10 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_10 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_10.wrapping_neg(),
                graphics_get_translate_y_value_10.wrapping_neg(),
            )?;
            let drawable_x_10 = r.get(player, field::drawable::X)?.i();
            let drawable_y_10 = r.get(player, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                drawable_x_10.wrapping_sub(x),
                drawable_y_10.wrapping_sub(y),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 24, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 24, 24)?;
        }
    }
    let player_sprite_3 = r.get(player, field::player::SPRITE)?.r();
    let player_facing_13 = r.get(player, field::player::FACING)?.i();
    host::graphics_draw_image(
        r,
        graphics,
        player_sprite_3,
        -72,
        (player_facing_13.wrapping_neg()).wrapping_mul(24),
        0,
    )?;
    let player_attacking_5 = r.get(player, field::player::ATTACKING)?.i();
    if player_attacking_5 != 0 {
        host::graphics_translate(r, graphics, -12, 12)?;
        host::graphics_set_clip(r, graphics, 0, 0, 18, 9)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 18, 9)?;
        let player_equipment_sprite_4 = r.get(player, field::player::EQUIPMENT_SPRITE)?.r();
        host::graphics_draw_image(r, graphics, player_equipment_sprite_4, -18, -9, 0)?;
    }
    let player_hookshot_extending_5 = r.get(player, field::player::HOOKSHOT_EXTENDING)?.i();
    if player_hookshot_extending_5 == 0 {
        return Ok(());
    }
    width = 0;
    loop {
        let player_hookshot_trail_8 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_6 = host::vector_size(r, player_hookshot_trail_8)?;
        if width >= vector_size_value_6 {
            return Ok(());
        }
        let player_hookshot_trail_11 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_element_at_value_4 = host::vector_element_at(r, player_hookshot_trail_11, width)?;
        host::check_cast(r, vector_element_at_value_4, "g")?;
        object_5 = vector_element_at_value_4;
        let graphics_get_translate_x_value_18 = host::graphics_get_translate_x(r, graphics)?;
        let graphics_get_translate_y_value_18 = host::graphics_get_translate_y(r, graphics)?;
        host::graphics_translate(
            r,
            graphics,
            graphics_get_translate_x_value_18.wrapping_neg(),
            graphics_get_translate_y_value_18.wrapping_neg(),
        )?;
        let trail_point_x_18 = r.get(object_5, field::trail_point::X)?.i();
        let trail_point_y_18 = r.get(object_5, field::trail_point::Y)?.i();
        host::graphics_translate(
            r,
            graphics,
            trail_point_x_18.wrapping_sub(x),
            trail_point_y_18.wrapping_sub(y),
        )?;
        let player_hookshot_trail_12 = r.get(player, field::player::HOOKSHOT_TRAIL)?.r();
        let vector_size_value_8 = host::vector_size(r, player_hookshot_trail_12)?;
        if width != vector_size_value_8.wrapping_sub(1) {
            host::graphics_set_clip(r, graphics, 0, 0, 8, 8)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 8)?;
            let player_action_sprite_7 = r.get(player, field::player::ACTION_SPRITE)?.r();
            let remainder_value_5 = arithmetic::remainder_int(r, width, 2)?;
            host::graphics_draw_image(
                r,
                graphics,
                player_action_sprite_7,
                0,
                (remainder_value_5.wrapping_neg()).wrapping_shl((3) as u32),
                0,
            )?;
        } else {
            host::graphics_translate(r, graphics, 0, -4)?;
            host::graphics_set_clip(r, graphics, 0, 0, 8, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 16)?;
            let player_action_sprite_8 = r.get(player, field::player::ACTION_SPRITE)?.r();
            host::graphics_draw_image(r, graphics, player_action_sprite_8, -16, 0, 0)?;
        }
        width = width.wrapping_add(1);
    }
}
