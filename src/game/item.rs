//! Collectibles and equipment behavior.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `c.<init>(Ljavax/microedition/lcdui/Image;IIIILh;)V`.
pub(super) fn initialize(
    r: &mut Runtime,
    item: u32,
    image: u32,
    kind: i32,
    x: i32,
    y: i32,
    persistence_id: i32,
    player: u32,
) -> Result<(), u32> {
    super::drawable::initialize(r, item)?;
    r.set(item, field::item::ACTIVE, V::I(1))?;
    r.set(item, field::item::COLLECTED, V::I(0))?;
    r.set(item, field::item::IMAGE, V::R(image))?;
    r.set(item, field::item::PLAYER, V::R(player))?;
    r.set(item, field::drawable::X, V::I(x))?;
    r.set(item, field::drawable::Y, V::I(y.wrapping_add(32)))?;
    r.set(item, field::item::PERSISTENCE_ID, V::I(persistence_id))?;
    r.set(item, field::item::KIND, V::I(kind))?;
    r.set(item, field::drawable::HALF_WIDTH, V::I(8))?;
    r.set(item, field::drawable::HALF_HEIGHT, V::I(8))?;
    r.set(item, field::drawable::DRAW_ORDER, V::I(0))?;
    r.set(item, field::item::ARMED, V::I(0))?;
    let object_value = r.alloc("java/util/Random", Native::Empty);
    host::random_initialize(r, object_value)?;
    r.set(item, field::item::RANDOM, V::R(object_value))?;
    let system_current_time_millis_value = host::system_current_time_millis(r)?;
    let remainder_value = arithmetic::remainder_long(r, system_current_time_millis_value, 31)?;
    r.set(
        item,
        field::item::ANIMATION,
        V::I(((remainder_value) as i32) as i8 as i32),
    )?;
    let item_kind = r.get(item, field::item::KIND)?.i();
    if (item_kind == 4) && (persistence_id == 1) {
        r.set(item, field::item::ARMED, V::I(1))?;
        r.set(item, field::item::ANIMATION, V::I(0))?;
        r.set(item, field::item::LIFETIME, V::I(50))?;
        r.set(item, field::drawable::HALF_WIDTH, V::I(8))?;
        r.set(item, field::drawable::HALF_HEIGHT, V::I(8))?;
    }
    let item_kind_2 = r.get(item, field::item::KIND)?.i();
    if item_kind_2 == 5 {
        r.set(item, field::item::ARMED, V::I(0))?;
        r.set(item, field::item::ANIMATION, V::I(0))?;
        r.set(item, field::drawable::HALF_WIDTH, V::I(8))?;
        r.set(item, field::drawable::HALF_HEIGHT, V::I(8))?;
    }
    let item_kind_3 = r.get(item, field::item::KIND)?.i();
    if item_kind_3 <= 3 {
        r.set(item, field::drawable::HALF_WIDTH, V::I(4))?;
        r.set(item, field::drawable::HALF_HEIGHT, V::I(6))?;
        r.set(item, field::item::LIFETIME, V::I(100))?;
    }
    let item_kind_4 = r.get(item, field::item::KIND)?.i();
    if item_kind_4 >= 128 {
        r.set(item, field::drawable::HALF_WIDTH, V::I(8))?;
        r.set(item, field::drawable::HALF_HEIGHT, V::I(8))?;
    }
    let item_kind_5 = r.get(item, field::item::KIND)?.i();
    if item_kind_5 != 145 {
        return Ok(());
    }
    let drawable_x = r.get(item, field::drawable::X)?.i();
    r.set(item, field::drawable::X, V::I(drawable_x.wrapping_add(9)))?;
    let drawable_y = r.get(item, field::drawable::Y)?.i();
    r.set(item, field::drawable::Y, V::I(drawable_y.wrapping_sub(24)))?;
    return Ok(());
}

/// Collect. Original reference: `c.a(Lh;Lb;Lk;Z)I`.
pub(super) fn collect(
    r: &mut Runtime,
    mut item: u32,
    player: u32,
    sound: u32,
    canvas: u32,
    mut interacting: i32,
) -> Result<i32, u32> {
    let mut number_2: i32 = 0;
    let mut carry_0_i: i32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    r.set(item, field::item::PLAYER, V::R(player))?;
    let item_active = r.get(item, field::item::ACTIVE)?.i();
    if item_active == 0 {
        return Ok(0);
    }
    let item_collected = r.get(item, field::item::COLLECTED)?.i();
    if item_collected != 0 {
        return Ok(0);
    }
    let item_kind = r.get(item, field::item::KIND)?.i();
    match item_kind {
        0 => {
            let player_rupees = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees == player_wallet_capacity {
                return Ok(1);
            }
            let player_rupees_5 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_5.wrapping_add(1)),
            )?;
            number_2 = 2;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            let player_rupees_6 = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity_4 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees_6 <= player_wallet_capacity_4 {
                return Ok(1);
            }
            let player_wallet_capacity_7 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_wallet_capacity_7),
            )?;
            return Ok(1);
        }
        1 => {
            let player_rupees_2 = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity_2 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees_2 == player_wallet_capacity_2 {
                return Ok(1);
            }
            let player_rupees_7 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_7.wrapping_add(5)),
            )?;
            number_2 = 2;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            let player_rupees_8 = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity_5 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees_8 <= player_wallet_capacity_5 {
                return Ok(1);
            }
            let player_wallet_capacity_8 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_wallet_capacity_8),
            )?;
            return Ok(1);
        }
        2 => {
            let player_rupees_3 = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity_3 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees_3 == player_wallet_capacity_3 {
                return Ok(1);
            }
            let player_rupees_9 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_9.wrapping_add(10)),
            )?;
            number_2 = 2;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            let player_rupees_10 = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity_6 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees_10 <= player_wallet_capacity_6 {
                return Ok(1);
            }
            let player_wallet_capacity_9 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_wallet_capacity_9),
            )?;
            return Ok(1);
        }
        3 => {
            let player_health = r.get(player, field::player::HEALTH)?.i();
            let player_max_health = r.get(player, field::player::MAX_HEALTH)?.i();
            let quotient_value = arithmetic::divide_int(r, player_max_health, 4)?;
            if !(player_health != quotient_value.wrapping_shl((2) as u32)) {
                return Ok(1);
            }
            let player_health_3 = r.get(player, field::player::HEALTH)?.i();
            r.set(
                player,
                field::player::HEALTH,
                V::I(player_health_3.wrapping_add(4)),
            )?;
            number_2 = 3;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            let player_health_4 = r.get(player, field::player::HEALTH)?.i();
            let player_max_health_5 = r.get(player, field::player::MAX_HEALTH)?.i();
            let quotient_value_3 = arithmetic::divide_int(r, player_max_health_5, 4)?;
            if player_health_4 <= quotient_value_3.wrapping_shl((2) as u32) {
                return Ok(1);
            }
            let player_max_health_6 = r.get(player, field::player::MAX_HEALTH)?.i();
            let quotient_value_4 = arithmetic::divide_int(r, player_max_health_6, 4)?;
            r.set(
                player,
                field::player::HEALTH,
                V::I(quotient_value_4.wrapping_shl((2) as u32)),
            )?;
            return Ok(1);
        }
        4 => {
            let item_armed = r.get(item, field::item::ARMED)?.i();
            if item_armed != 0 {
                return Ok(0);
            }
            let player_bomb_capacity = r.get(player, field::player::BOMB_CAPACITY)?.i();
            let player_bombs = r.get(player, field::player::BOMBS)?.i();
            if player_bomb_capacity == player_bombs {
                return Ok(1);
            }
            let player_bombs_3 = r.get(player, field::player::BOMBS)?.i();
            r.set(
                player,
                field::player::BOMBS,
                V::I((player_bombs_3.wrapping_add(1)) as i8 as i32),
            )?;
            let player_bombs_4 = r.get(player, field::player::BOMBS)?.i();
            let player_bomb_capacity_3 = r.get(player, field::player::BOMB_CAPACITY)?.i();
            if player_bombs_4 <= player_bomb_capacity_3 {
                return Ok(1);
            }
            let player_bomb_capacity_4 = r.get(player, field::player::BOMB_CAPACITY)?.i();
            r.set(player, field::player::BOMBS, V::I(player_bomb_capacity_4))?;
            return Ok(1);
        }
        5 => {
            let item_armed_2 = r.get(item, field::item::ARMED)?.i();
            if item_armed_2 != 0 {
                return Ok(0);
            }
            let player_arrow_capacity = r.get(player, field::player::ARROW_CAPACITY)?.i();
            let player_arrows = r.get(player, field::player::ARROWS)?.i();
            if player_arrow_capacity == player_arrows {
                return Ok(1);
            }
            let player_arrows_3 = r.get(player, field::player::ARROWS)?.i();
            r.set(
                player,
                field::player::ARROWS,
                V::I((player_arrows_3.wrapping_add(5)) as i8 as i32),
            )?;
            let player_arrows_4 = r.get(player, field::player::ARROWS)?.i();
            let player_arrow_capacity_3 = r.get(player, field::player::ARROW_CAPACITY)?.i();
            if player_arrows_4 <= player_arrow_capacity_3 {
                return Ok(1);
            }
            let player_arrow_capacity_4 = r.get(player, field::player::ARROW_CAPACITY)?.i();
            r.set(player, field::player::ARROWS, V::I(player_arrow_capacity_4))?;
            return Ok(1);
        }
        128 => {
            let player_heart_pickups = r.get(player, field::player::HEART_PICKUPS)?.i();
            let item_persistence_id = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_heart_pickups & (1_i32).wrapping_shl((item_persistence_id) as u32)) > 0 {
                return Ok(0);
            }
            let item_persistence_id_22 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if item_persistence_id_22 > 0 {
                let player_heart_pickups_3 = r.get(player, field::player::HEART_PICKUPS)?.i();
                let item_persistence_id_42 = r.get(item, field::item::PERSISTENCE_ID)?.i();
                r.set(
                    player,
                    field::player::HEART_PICKUPS,
                    V::I(
                        (player_heart_pickups_3
                            | (1_i32).wrapping_shl((item_persistence_id_42) as u32)),
                    ),
                )?;
            }
            let player_max_health_7 = r.get(player, field::player::MAX_HEALTH)?.i();
            r.set(
                player,
                field::player::MAX_HEALTH,
                V::I(player_max_health_7.wrapping_add(1)),
            )?;
            let player_max_health_8 = r.get(player, field::player::MAX_HEALTH)?.i();
            if player_max_health_8 > 80 {
                r.set(player, field::player::MAX_HEALTH, V::I(80))?;
            }
            let player_max_health_12 = r.get(player, field::player::MAX_HEALTH)?.i();
            let quotient_value_6 = arithmetic::divide_int(r, player_max_health_12, 4)?;
            r.set(
                player,
                field::player::HEALTH,
                V::I(quotient_value_6.wrapping_shl((2) as u32)),
            )?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            let player_max_health_13 = r.get(player, field::player::MAX_HEALTH)?.i();
            let remainder_value = arithmetic::remainder_int(r, player_max_health_13, 4)?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(remainder_value))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        129 => {
            let player_heart_pickups_2 = r.get(player, field::player::HEART_PICKUPS)?.i();
            let item_persistence_id_2 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_heart_pickups_2 & (1_i32).wrapping_shl((item_persistence_id_2) as u32)) > 0 {
                return Ok(0);
            }
            let item_persistence_id_23 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if item_persistence_id_23 > 0 {
                let player_heart_pickups_4 = r.get(player, field::player::HEART_PICKUPS)?.i();
                let item_persistence_id_43 = r.get(item, field::item::PERSISTENCE_ID)?.i();
                r.set(
                    player,
                    field::player::HEART_PICKUPS,
                    V::I(
                        (player_heart_pickups_4
                            | (1_i32).wrapping_shl((item_persistence_id_43) as u32)),
                    ),
                )?;
            }
            let player_max_health_9 = r.get(player, field::player::MAX_HEALTH)?.i();
            r.set(
                player,
                field::player::MAX_HEALTH,
                V::I(player_max_health_9.wrapping_add(4)),
            )?;
            let player_max_health_10 = r.get(player, field::player::MAX_HEALTH)?.i();
            if player_max_health_10 > 80 {
                r.set(player, field::player::MAX_HEALTH, V::I(80))?;
            }
            let player_max_health_14 = r.get(player, field::player::MAX_HEALTH)?.i();
            let quotient_value_7 = arithmetic::divide_int(r, player_max_health_14, 4)?;
            r.set(
                player,
                field::player::HEALTH,
                V::I(quotient_value_7.wrapping_shl((2) as u32)),
            )?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(24))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        130 => {
            let player_progression_flags = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_3 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags & (1_i32).wrapping_shl((item_persistence_id_3) as u32)) > 0
            {
                return Ok(0);
            }
            let player_progression_flags_16 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_24 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_16
                        | (1_i32).wrapping_shl((item_persistence_id_24) as u32)),
                ),
            )?;
            r.set(player, field::player::HAS_SWORD, V::I(1))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(4))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        131 => {
            let player_key_pickups = r.get(player, field::player::KEY_PICKUPS)?.i();
            let item_persistence_id_4 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_key_pickups & (1_i32).wrapping_shl((item_persistence_id_4) as u32)) > 0 {
                return Ok(0);
            }
            let player_key_pickups_2 = r.get(player, field::player::KEY_PICKUPS)?.i();
            let item_persistence_id_25 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::KEY_PICKUPS,
                V::I(
                    (player_key_pickups_2 | (1_i32).wrapping_shl((item_persistence_id_25) as u32)),
                ),
            )?;
            let player_keys = r.get(player, field::player::KEYS)?.i();
            r.set(
                player,
                field::player::KEYS,
                V::I(player_keys.wrapping_add(1)),
            )?;
            let player_keys_2 = r.get(player, field::player::KEYS)?.i();
            if player_keys_2 <= 99 {
                return Ok(1);
            }
            r.set(player, field::player::KEYS, V::I(99))?;
            return Ok(1);
        }
        132 => {
            let player_progression_flags_2 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_5 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_2 & (1_i32).wrapping_shl((item_persistence_id_5) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_17 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_26 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_17
                        | (1_i32).wrapping_shl((item_persistence_id_26) as u32)),
                ),
            )?;
            r.set(player, field::player::HAS_BOOMERANG, V::I(1))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(7))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        133 => {
            let player_progression_flags_3 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_6 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_3 & (1_i32).wrapping_shl((item_persistence_id_6) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_18 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_27 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_18
                        | (1_i32).wrapping_shl((item_persistence_id_27) as u32)),
                ),
            )?;
            r.set(player, field::player::HAS_BOW, V::I(1))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(5))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            r.set(player, field::player::ARROW_CAPACITY, V::I(30))?;
            r.set(player, field::player::ARROWS, V::I(30))?;
            return Ok(2);
        }
        134 => {
            let player_progression_flags_4 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_7 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_4 & (1_i32).wrapping_shl((item_persistence_id_7) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_19 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_28 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_19
                        | (1_i32).wrapping_shl((item_persistence_id_28) as u32)),
                ),
            )?;
            r.set(player, field::player::HAS_CANDLE, V::I(1))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(14))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        135 => {
            let player_progression_flags_5 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_8 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_5 & (1_i32).wrapping_shl((item_persistence_id_8) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_rupees_11 = r.get(player, field::player::RUPEES)?.i();
            if player_rupees_11 < 255 {
                return Ok(0);
            }
            let player_rupees_18 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_18.wrapping_sub(255)),
            )?;
            let player_progression_flags_30 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_44 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_30
                        | (1_i32).wrapping_shl((item_persistence_id_44) as u32)),
                ),
            )?;
            let player_tunic_color = r.get(player, field::player::TUNIC_COLOR)?.i();
            if player_tunic_color > 1 {
                return Ok(1);
            }
            r.set(player, field::player::TUNIC_COLOR, V::I(1))?;
            r.set(player, field::player::DEFENSE, V::I(2))?;
            r.set(player, field::player::SPRITE, V::R(0))?;
            r.set(player, field::player::SPRITE, V::R(0))?;
            let text_value_3 = r.intern("/Images/LinkWalk_Blue.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_3)?)) })();
            (carry_0_r,) = (player,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(0);
                }
                return Err(outcome.unwrap_err());
            }
        }
        136 => {
            let player_progression_flags_6 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_9 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_6 & (1_i32).wrapping_shl((item_persistence_id_9) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_20 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_29 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_20
                        | (1_i32).wrapping_shl((item_persistence_id_29) as u32)),
                ),
            )?;
            r.set(player, field::player::HAS_BOMBS, V::I(1))?;
            r.set(player, field::player::BOMB_CAPACITY, V::I(10))?;
            r.set(player, field::player::BOMBS, V::I(10))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(11))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        137 => {
            let player_progression_flags_7 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_10 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_7 & (1_i32).wrapping_shl((item_persistence_id_10) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_21 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_30 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_21
                        | (1_i32).wrapping_shl((item_persistence_id_30) as u32)),
                ),
            )?;
            r.set(player, field::player::WALLET_CAPACITY, V::I(500))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(6))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        138 => {
            let player_has_potion = r.get(player, field::player::HAS_POTION)?.i();
            if player_has_potion != 0 {
                return Ok(0);
            }
            let player_rupees_12 = r.get(player, field::player::RUPEES)?.i();
            if player_rupees_12 < 50 {
                return Ok(0);
            }
            let player_rupees_19 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_19.wrapping_sub(50)),
            )?;
            r.set(player, field::player::HAS_POTION, V::I(1))?;
            return Ok(1);
        }
        139 => {
            let player_progression_flags_8 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_11 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_8 & (1_i32).wrapping_shl((item_persistence_id_11) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_22 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_31 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_22
                        | (1_i32).wrapping_shl((item_persistence_id_31) as u32)),
                ),
            )?;
            r.set(player, field::player::HAS_RAFT, V::I(1))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(13))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        140 => {
            let player_max_health_2 = r.get(player, field::player::MAX_HEALTH)?.i();
            if player_max_health_2 < 20 {
                return Ok(0);
            }
            let player_progression_flags_23 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_32 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_23
                        | (1_i32).wrapping_shl((item_persistence_id_32) as u32)),
                ),
            )?;
            let player_sword_level = r.get(player, field::player::SWORD_LEVEL)?.i();
            if player_sword_level <= 1 {
                r.set(player, field::player::EQUIPMENT_SPRITE, V::R(0))?;
                r.set(player, field::player::SWORD_LEVEL, V::I(1))?;
                host::system_gc(r)?;
                let text_value_2 = r.intern("/Images/Sword02.png");
                outcome =
                    (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_2)?)) })();
                (carry_0_r,) = (player,);
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if r.is_instance(carry_0_r, "java/io/IOException") {
                        return Ok(1);
                    }
                    return Err(outcome.unwrap_err());
                }
                let image_create_image_value_2 = outcome.unwrap().r();
                r.set(
                    carry_0_r,
                    field::player::EQUIPMENT_SPRITE,
                    V::R(image_create_image_value_2),
                )?;
                r.set(player, field::player::SWORD_DAMAGE, V::I(2))?;
                (carry_0_i,) = (1,);
                return Ok(carry_0_i);
            }
            (carry_0_i,) = (1,);
            return Ok(carry_0_i);
        }
        141 => {
            let player_progression_flags_9 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_12 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_9
                        | (1_i32).wrapping_shl((item_persistence_id_12) as u32)),
                ),
            )?;
            return Ok(1);
        }
        142 => {
            let player_progression_flags_10 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_13 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_10 & (1_i32).wrapping_shl((item_persistence_id_13) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_24 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_33 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_24
                        | (1_i32).wrapping_shl((item_persistence_id_33) as u32)),
                ),
            )?;
            r.set(player, field::player::HAS_HOOKSHOT, V::I(1))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(12))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        143 => {
            let player_progression_flags_11 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_14 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_11 & (1_i32).wrapping_shl((item_persistence_id_14) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_25 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_34 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_25
                        | (1_i32).wrapping_shl((item_persistence_id_34) as u32)),
                ),
            )?;
            r.set(player, field::player::HAS_MIRROR_SHIELD, V::I(1))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(15))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        144 => {
            let player_progression_flags_12 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_15 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_12 & (1_i32).wrapping_shl((item_persistence_id_15) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_26 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_35 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_26
                        | (1_i32).wrapping_shl((item_persistence_id_35) as u32)),
                ),
            )?;
            let count_triforce_pieces_value = super::player::count_triforce_pieces(r, player)?;
            interacting = count_triforce_pieces_value;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(
                canvas,
                field::canvas::ITEM_MESSAGE,
                V::I(interacting.wrapping_add(15)),
            )?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        145 => {
            let player_max_health_3 = r.get(player, field::player::MAX_HEALTH)?.i();
            if player_max_health_3 < 40 {
                return Ok(0);
            }
            let player_progression_flags_27 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_36 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_27
                        | (1_i32).wrapping_shl((item_persistence_id_36) as u32)),
                ),
            )?;
            r.set(player, field::player::EQUIPMENT_SPRITE, V::R(0))?;
            r.set(player, field::player::SWORD_LEVEL, V::I(2))?;
            host::system_gc(r)?;
            let text_value = r.intern("/Images/Sword03.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value)?)) })();
            (carry_0_r,) = (player,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    return Ok(1);
                }
                return Err(outcome.unwrap_err());
            }
            let image_create_image_value = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::player::EQUIPMENT_SPRITE,
                V::R(image_create_image_value),
            )?;
            r.set(player, field::player::SWORD_DAMAGE, V::I(4))?;
            (carry_0_i,) = (1,);
            return Ok(carry_0_i);
        }
        146 => {
            let player_rupees_4 = r.get(player, field::player::RUPEES)?.i();
            if player_rupees_4 < 99 {
                return Ok(0);
            }
            let player_rupees_13 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_13.wrapping_sub(99)),
            )?;
            let player_keys_3 = r.get(player, field::player::KEYS)?.i();
            r.set(
                player,
                field::player::KEYS,
                V::I(player_keys_3.wrapping_add(1)),
            )?;
            let player_keys_4 = r.get(player, field::player::KEYS)?.i();
            if player_keys_4 <= 99 {
                return Ok(1);
            }
            r.set(player, field::player::KEYS, V::I(99))?;
            return Ok(1);
        }
        147 => {
            let player_rupee_pickups = r.get(player, field::player::RUPEE_PICKUPS)?.i();
            let item_persistence_id_16 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_rupee_pickups & (1_i32).wrapping_shl((item_persistence_id_16) as u32)) > 0 {
                return Ok(0);
            }
            let player_rupees_14 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_14.wrapping_add(50)),
            )?;
            let item_persistence_id_37 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if item_persistence_id_37 > 0 {
                let player_rupee_pickups_4 = r.get(player, field::player::RUPEE_PICKUPS)?.i();
                let item_persistence_id_45 = r.get(item, field::item::PERSISTENCE_ID)?.i();
                r.set(
                    player,
                    field::player::RUPEE_PICKUPS,
                    V::I(
                        (player_rupee_pickups_4
                            | (1_i32).wrapping_shl((item_persistence_id_45) as u32)),
                    ),
                )?;
            }
            number_2 = 2;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            let player_rupees_20 = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity_10 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees_20 <= player_wallet_capacity_10 {
                return Ok(1);
            }
            let player_wallet_capacity_13 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_wallet_capacity_13),
            )?;
            return Ok(1);
        }
        148 => {
            let player_rupee_pickups_2 = r.get(player, field::player::RUPEE_PICKUPS)?.i();
            let item_persistence_id_17 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_rupee_pickups_2 & (1_i32).wrapping_shl((item_persistence_id_17) as u32)) > 0
            {
                return Ok(0);
            }
            let player_rupees_15 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_15.wrapping_add(100)),
            )?;
            let item_persistence_id_38 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if item_persistence_id_38 > 0 {
                let player_rupee_pickups_5 = r.get(player, field::player::RUPEE_PICKUPS)?.i();
                let item_persistence_id_46 = r.get(item, field::item::PERSISTENCE_ID)?.i();
                r.set(
                    player,
                    field::player::RUPEE_PICKUPS,
                    V::I(
                        (player_rupee_pickups_5
                            | (1_i32).wrapping_shl((item_persistence_id_46) as u32)),
                    ),
                )?;
            }
            number_2 = 2;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            let player_rupees_21 = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity_11 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees_21 <= player_wallet_capacity_11 {
                return Ok(1);
            }
            let player_wallet_capacity_14 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_wallet_capacity_14),
            )?;
            return Ok(1);
        }
        149 => {
            let player_rupee_pickups_3 = r.get(player, field::player::RUPEE_PICKUPS)?.i();
            let item_persistence_id_18 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_rupee_pickups_3 & (1_i32).wrapping_shl((item_persistence_id_18) as u32)) > 0
            {
                return Ok(0);
            }
            let player_rupees_16 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_16.wrapping_add(200)),
            )?;
            let item_persistence_id_39 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if item_persistence_id_39 > 0 {
                let player_rupee_pickups_6 = r.get(player, field::player::RUPEE_PICKUPS)?.i();
                let item_persistence_id_47 = r.get(item, field::item::PERSISTENCE_ID)?.i();
                r.set(
                    player,
                    field::player::RUPEE_PICKUPS,
                    V::I(
                        (player_rupee_pickups_6
                            | (1_i32).wrapping_shl((item_persistence_id_47) as u32)),
                    ),
                )?;
            }
            number_2 = 2;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            let player_rupees_22 = r.get(player, field::player::RUPEES)?.i();
            let player_wallet_capacity_12 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            if player_rupees_22 <= player_wallet_capacity_12 {
                return Ok(1);
            }
            let player_wallet_capacity_15 = r.get(player, field::player::WALLET_CAPACITY)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_wallet_capacity_15),
            )?;
            return Ok(1);
        }
        150 => {
            let player_health_2 = r.get(player, field::player::HEALTH)?.i();
            let player_max_health_4 = r.get(player, field::player::MAX_HEALTH)?.i();
            let quotient_value_2 = arithmetic::divide_int(r, player_max_health_4, 4)?;
            if !(player_health_2 != quotient_value_2.wrapping_shl((2) as u32)) {
                return Ok(0);
            }
            let player_rupees_17 = r.get(player, field::player::RUPEES)?.i();
            if player_rupees_17 < 10 {
                return Ok(0);
            }
            let player_rupees_23 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_23.wrapping_sub(10)),
            )?;
            let player_health_5 = r.get(player, field::player::HEALTH)?.i();
            r.set(
                player,
                field::player::HEALTH,
                V::I(player_health_5.wrapping_add(12)),
            )?;
            number_2 = 3;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            let player_health_6 = r.get(player, field::player::HEALTH)?.i();
            let player_max_health_11 = r.get(player, field::player::MAX_HEALTH)?.i();
            let quotient_value_5 = arithmetic::divide_int(r, player_max_health_11, 4)?;
            if player_health_6 <= quotient_value_5.wrapping_shl((2) as u32) {
                return Ok(1);
            }
            let player_max_health_15 = r.get(player, field::player::MAX_HEALTH)?.i();
            let quotient_value_8 = arithmetic::divide_int(r, player_max_health_15, 4)?;
            r.set(
                player,
                field::player::HEALTH,
                V::I(quotient_value_8.wrapping_shl((2) as u32)),
            )?;
            return Ok(1);
        }
        151 => {
            let player_has_bombs = r.get(player, field::player::HAS_BOMBS)?.i();
            if player_has_bombs == 0 {
                return Ok(0);
            }
            let player_bomb_capacity_2 = r.get(player, field::player::BOMB_CAPACITY)?.i();
            let player_bombs_2 = r.get(player, field::player::BOMBS)?.i();
            if player_bomb_capacity_2 == player_bombs_2 {
                return Ok(0);
            }
            let player_rupees_24 = r.get(player, field::player::RUPEES)?.i();
            if player_rupees_24 < 25 {
                return Ok(0);
            }
            let player_rupees_26 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_26.wrapping_sub(25)),
            )?;
            let player_bombs_5 = r.get(player, field::player::BOMBS)?.i();
            r.set(
                player,
                field::player::BOMBS,
                V::I((player_bombs_5.wrapping_add(10)) as i8 as i32),
            )?;
            let player_bombs_6 = r.get(player, field::player::BOMBS)?.i();
            let player_bomb_capacity_5 = r.get(player, field::player::BOMB_CAPACITY)?.i();
            if player_bombs_6 <= player_bomb_capacity_5 {
                return Ok(1);
            }
            let player_bomb_capacity_6 = r.get(player, field::player::BOMB_CAPACITY)?.i();
            r.set(player, field::player::BOMBS, V::I(player_bomb_capacity_6))?;
            return Ok(1);
        }
        152 => {
            let player_has_bow = r.get(player, field::player::HAS_BOW)?.i();
            if player_has_bow == 0 {
                return Ok(0);
            }
            let player_arrow_capacity_2 = r.get(player, field::player::ARROW_CAPACITY)?.i();
            let player_arrows_2 = r.get(player, field::player::ARROWS)?.i();
            if player_arrow_capacity_2 == player_arrows_2 {
                return Ok(0);
            }
            let player_rupees_25 = r.get(player, field::player::RUPEES)?.i();
            if player_rupees_25 < 15 {
                return Ok(0);
            }
            let player_arrows_5 = r.get(player, field::player::ARROWS)?.i();
            r.set(
                player,
                field::player::ARROWS,
                V::I((player_arrows_5.wrapping_add(10)) as i8 as i32),
            )?;
            let player_rupees_27 = r.get(player, field::player::RUPEES)?.i();
            r.set(
                player,
                field::player::RUPEES,
                V::I(player_rupees_27.wrapping_sub(15)),
            )?;
            let player_arrows_6 = r.get(player, field::player::ARROWS)?.i();
            let player_arrow_capacity_5 = r.get(player, field::player::ARROW_CAPACITY)?.i();
            if player_arrows_6 <= player_arrow_capacity_5 {
                return Ok(1);
            }
            let player_arrow_capacity_6 = r.get(player, field::player::ARROW_CAPACITY)?.i();
            r.set(player, field::player::ARROWS, V::I(player_arrow_capacity_6))?;
            return Ok(1);
        }
        153 => {
            let player_progression_flags_13 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_19 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_13 & (1_i32).wrapping_shl((item_persistence_id_19) as u32))
                > 0
            {
                return Ok(0);
            }

            if interacting == 0 {
                return Ok(0);
            }
            let player_progression_flags_31 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_48 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_31
                        | (1_i32).wrapping_shl((item_persistence_id_48) as u32)),
                ),
            )?;
            r.set(player, field::player::ATTACKING, V::I(0))?;
            r.set(player, field::player::HAS_SHOVEL, V::I(1))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(8))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        154 => {
            let player_progression_flags_14 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_20 = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if !((player_progression_flags_14
                & (1_i32).wrapping_shl((item_persistence_id_20) as u32))
                > 0)
            {
                let player_progression_flags_28 =
                    r.get(player, field::player::PROGRESSION_FLAGS)?.i();
                let item_persistence_id_40 = r.get(item, field::item::PERSISTENCE_ID)?.i();
                r.set(
                    player,
                    field::player::PROGRESSION_FLAGS,
                    V::I(
                        (player_progression_flags_28
                            | (1_i32).wrapping_shl((item_persistence_id_40) as u32)),
                    ),
                )?;
                r.set(player, field::player::ATTACKING, V::I(0))?;
                r.set(item, field::item::COLLECTED, V::I(1))?;
                r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
                r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
                r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
                r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(10))?;
                number_2 = 5;
                item = sound;
                r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
                return Ok(2);
            }
            let player_progression_flags_15_read =
                r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_21_read = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_15_read
                & (1_i32).wrapping_shl((item_persistence_id_21_read) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_29_read =
                r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_41_read = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_29_read
                        | (1_i32).wrapping_shl((item_persistence_id_41_read) as u32)),
                ),
            )?;
            r.set(player, field::player::ATTACKING, V::I(0))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(26))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        155 => {
            let player_progression_flags_15_read =
                r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_21_read = r.get(item, field::item::PERSISTENCE_ID)?.i();
            if (player_progression_flags_15_read
                & (1_i32).wrapping_shl((item_persistence_id_21_read) as u32))
                > 0
            {
                return Ok(0);
            }
            let player_progression_flags_29_read =
                r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            let item_persistence_id_41_read = r.get(item, field::item::PERSISTENCE_ID)?.i();
            r.set(
                player,
                field::player::PROGRESSION_FLAGS,
                V::I(
                    (player_progression_flags_29_read
                        | (1_i32).wrapping_shl((item_persistence_id_41_read) as u32)),
                ),
            )?;
            r.set(player, field::player::ATTACKING, V::I(0))?;
            r.set(item, field::item::COLLECTED, V::I(1))?;
            r.set(player, field::player::ITEM_POSE_TIMER, V::I(25))?;
            r.set(player, field::player::HOLDING_ITEM, V::I(1))?;
            r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(1))?;
            r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(26))?;
            number_2 = 5;
            item = sound;
            r.set(sound, field::sound::PENDING_EFFECT, V::I(number_2))?;
            return Ok(2);
        }
        _ => {
            return Ok(0);
        }
    }
    let image_create_image_value_3 = outcome.unwrap().r();
    r.set(
        carry_0_r,
        field::player::SPRITE,
        V::R(image_create_image_value_3),
    )?;
    r.set(player, field::player::DEFENSE, V::I(2))?;
    host::system_gc(r)?;
    (carry_0_i,) = (1,);
    return Ok(carry_0_i);
}

/// Draw. Original reference: `c.a(Ljavax/microedition/lcdui/Graphics;IIII)V`.
pub(super) fn draw(
    r: &mut Runtime,
    item: u32,
    graphics: u32,
    x: i32,
    y: i32,
    mut width: i32,
    height: i32,
) -> Result<(), u32> {
    let drawable_x = r.get(item, field::drawable::X)?.i();
    if !(width.wrapping_add(x) >= drawable_x) {
        return Ok(());
    }
    let drawable_x_2 = r.get(item, field::drawable::X)?.i();
    let drawable_half_width = r.get(item, field::drawable::HALF_WIDTH)?.i();
    let drawable_half_width_2 = r.get(item, field::drawable::HALF_WIDTH)?.i();
    if !((drawable_x_2.wrapping_add(drawable_half_width)).wrapping_add(drawable_half_width_2) >= x)
    {
        return Ok(());
    }
    let drawable_y = r.get(item, field::drawable::Y)?.i();
    if !(height.wrapping_add(y) >= drawable_y) {
        return Ok(());
    }
    let drawable_y_2 = r.get(item, field::drawable::Y)?.i();
    let drawable_half_height = r.get(item, field::drawable::HALF_HEIGHT)?.i();
    let drawable_half_height_2 = r.get(item, field::drawable::HALF_HEIGHT)?.i();
    if !(((drawable_y_2.wrapping_add(drawable_half_height)).wrapping_add(drawable_half_height_2))
        .wrapping_sub(32)
        >= y)
    {
        return Ok(());
    }
    let item_active = r.get(item, field::item::ACTIVE)?.i();
    if item_active == 0 {
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
    let drawable_x_3 = r.get(item, field::drawable::X)?.i();
    let drawable_y_3 = r.get(item, field::drawable::Y)?.i();
    host::graphics_translate(
        r,
        graphics,
        drawable_x_3.wrapping_sub(x),
        drawable_y_3.wrapping_sub(y),
    )?;
    let item_kind = r.get(item, field::item::KIND)?.i();
    if item_kind < 3 {
        host::graphics_set_clip(r, graphics, 0, 0, 8, 15)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 8, 15)?;
        width = 0;
        let item_animation = r.get(item, field::item::ANIMATION)?.i();
        if item_animation > 24 {
            width = width.wrapping_add(1);
        }
        let item_animation_2 = r.get(item, field::item::ANIMATION)?.i();
        if item_animation_2 > 28 {
            width = width.wrapping_add(1);
        }
        let item_image_2 = r.get(item, field::item::IMAGE)?.r();
        let item_kind_5 = r.get(item, field::item::KIND)?.i();
        host::graphics_draw_image(
            r,
            graphics,
            item_image_2,
            width.wrapping_mul(-8),
            (-15_i32).wrapping_mul(item_kind_5),
            0,
        )?;
    }
    let item_kind_2 = r.get(item, field::item::KIND)?.i();
    if item_kind_2 == 3 {
        host::graphics_set_clip(r, graphics, 0, 0, 8, 15)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 8, 15)?;
        let item_image = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image, 0, -45, 0)?;
    }
    let item_kind_3 = r.get(item, field::item::KIND)?.i();
    if item_kind_3 == 4 {
        let item_armed = r.get(item, field::item::ARMED)?.i();
        if item_armed == 0 {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_4 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_4, 0, 0, 0)?;
            return Ok(());
        }
        let item_explosion_timer = r.get(item, field::item::EXPLOSION_TIMER)?.i();
        if item_explosion_timer != 0 {
            let item_random = r.get(item, field::item::RANDOM)?.r();
            let random_next_int_value = host::random_next_int(r, item_random)?;
            let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 5)?;
            width = remainder_value;
            host::graphics_translate(r, graphics, 0, width.wrapping_add(-12))?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_5 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_5, -32, 0, 0)?;
            host::graphics_translate(
                r,
                graphics,
                width.wrapping_add(-12),
                (12_i32).wrapping_sub(width),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_6 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_6, -32, 0, 0)?;
            host::graphics_translate(
                r,
                graphics,
                (12_i32).wrapping_sub(width),
                width.wrapping_add(12),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_7 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_7, -32, 0, 0)?;
            host::graphics_translate(
                r,
                graphics,
                width.wrapping_add(12),
                (-12_i32).wrapping_sub(width),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_8 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_8, -32, 0, 0)?;
            let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_2 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_2.wrapping_neg(),
                graphics_get_translate_y_value_2.wrapping_neg(),
            )?;
            let drawable_x_4 = r.get(item, field::drawable::X)?.i();
            let drawable_y_4 = r.get(item, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                drawable_x_4.wrapping_sub(x),
                drawable_y_4.wrapping_sub(y),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_9 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_9, -32, 0, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_10 = r.get(item, field::item::IMAGE)?.r();
            let item_animation_3 = r.get(item, field::item::ANIMATION)?.i();
            let remainder_value_2 = arithmetic::remainder_int(r, item_animation_3, 2)?;
            host::graphics_draw_image(
                r,
                graphics,
                item_image_10,
                (-16_i32).wrapping_mul(remainder_value_2),
                0,
                0,
            )?;
        }
    }
    let item_kind_4 = r.get(item, field::item::KIND)?.i();
    if item_kind_4 == 5 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_3 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_3, 0, 0, 0)?;
    }
    let item_kind_6 = r.get(item, field::item::KIND)?.i();
    if item_kind_6 == 128 {
        let item_player = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item = r.get(item_player, field::player::HOLDING_ITEM)?.i();
        if player_holding_item == 0 {
            let item_collected = r.get(item, field::item::COLLECTED)?.i();
            if item_collected == 0 {
                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                let item_image_13 = r.get(item, field::item::IMAGE)?.r();
                host::graphics_draw_image(r, graphics, item_image_13, 0, 0, 0)?;
            }
        } else {
            let graphics_get_translate_x_value_3 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_3 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_3.wrapping_neg(),
                graphics_get_translate_y_value_3.wrapping_neg(),
            )?;
            let item_player_3 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_5 = r.get(item_player_3, field::drawable::X)?.i();
            let item_player_4 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_5 = r.get(item_player_4, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_5.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_5.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_11 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_11, 0, 0, 0)?;
        }
    }
    let item_kind_7 = r.get(item, field::item::KIND)?.i();
    if item_kind_7 == 129 {
        let item_player_2 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_2 = r.get(item_player_2, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_2 == 0 {
            let item_collected_3 = r.get(item, field::item::COLLECTED)?.i();
            if item_collected_3 == 0 {
                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                let item_image_14 = r.get(item, field::item::IMAGE)?.r();
                host::graphics_draw_image(r, graphics, item_image_14, -16, 0, 0)?;
            }
        } else {
            let graphics_get_translate_x_value_4 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_4 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_4.wrapping_neg(),
                graphics_get_translate_y_value_4.wrapping_neg(),
            )?;
            let item_player_5 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_6 = r.get(item_player_5, field::drawable::X)?.i();
            let item_player_6 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_6 = r.get(item_player_6, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_6.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_6.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_12 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_12, -16, 0, 0)?;
        }
    }
    let item_kind_8 = r.get(item, field::item::KIND)?.i();
    if item_kind_8 == 130 {
        let item_collected_2 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_2 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_2 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_2, 0, 0, 0)?;
        }
        let item_player_7 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_3 = r.get(item_player_7, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_3 != 0 {
            let graphics_get_translate_x_value_5 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_5 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_5.wrapping_neg(),
                graphics_get_translate_y_value_5.wrapping_neg(),
            )?;
            let item_player_8 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_7 = r.get(item_player_8, field::drawable::X)?.i();
            let item_player_9 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_7 = r.get(item_player_9, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_7.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_7.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_16 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_16, -32, 0, 0)?;
        }
    }
    let item_kind_9 = r.get(item, field::item::KIND)?.i();
    if item_kind_9 == 131 {
        let item_collected_4 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_4 == 0 {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_15 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_15, 0, 0, 0)?;
        }
    }
    let item_kind_10 = r.get(item, field::item::KIND)?.i();
    if item_kind_10 == 132 {
        let item_collected_5 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_5 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image_3 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_3, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_4 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_4, 0, 0, 0)?;
        }
        let item_player_10 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_4 = r.get(item_player_10, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_4 != 0 {
            let graphics_get_translate_x_value_6 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_6 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_6.wrapping_neg(),
                graphics_get_translate_y_value_6.wrapping_neg(),
            )?;
            let item_player_12 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_8 = r.get(item_player_12, field::drawable::X)?.i();
            let item_player_13 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_8 = r.get(item_player_13, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_8.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_8.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_17 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_17, -16, 0, 0)?;
        }
    }
    let item_kind_11 = r.get(item, field::item::KIND)?.i();
    if item_kind_11 == 133 {
        let item_collected_6 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_6 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image_5 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_5, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_6 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_6, 0, 0, 0)?;
        }
        let item_player_11 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_5 = r.get(item_player_11, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_5 != 0 {
            let graphics_get_translate_x_value_7 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_7 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_7.wrapping_neg(),
                graphics_get_translate_y_value_7.wrapping_neg(),
            )?;
            let item_player_15 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_9 = r.get(item_player_15, field::drawable::X)?.i();
            let item_player_16 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_9 = r.get(item_player_16, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_9.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_9.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_19 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_19, -32, 0, 0)?;
        }
    }
    let item_kind_12 = r.get(item, field::item::KIND)?.i();
    if item_kind_12 == 134 {
        let item_collected_7 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_7 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image_7 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_7, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_8 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_8, 0, 0, 0)?;
        }
        let item_player_14 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_6 = r.get(item_player_14, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_6 != 0 {
            let graphics_get_translate_x_value_8 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_8 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_8.wrapping_neg(),
                graphics_get_translate_y_value_8.wrapping_neg(),
            )?;
            let item_player_18 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_10 = r.get(item_player_18, field::drawable::X)?.i();
            let item_player_19 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_10 = r.get(item_player_19, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_10.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_10.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_20 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_20, 0, 0, 0)?;
        }
    }
    let item_kind_13 = r.get(item, field::item::KIND)?.i();
    if item_kind_13 == 135 {
        let item_collected_8 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_8 != 0 {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_18 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_18, -16, 0, 0)?;
        host::graphics_translate(r, graphics, 0, 16)?;
        host::graphics_set_color(r, graphics, 16777215)?;
        host::graphics_set_clip(r, graphics, 0, 0, 50, 50)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 50, 50)?;
        let text_value = r.intern("255:-");
        host::graphics_draw_string(r, graphics, text_value, 0, 0, 20)?;
    }
    let item_kind_14 = r.get(item, field::item::KIND)?.i();
    if item_kind_14 == 136 {
        let item_collected_9 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_9 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image_9 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_9, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_10 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_10, 0, 0, 0)?;
        }
        let item_player_22 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_8 = r.get(item_player_22, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_8 != 0 {
            let graphics_get_translate_x_value_10 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_10 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_10.wrapping_neg(),
                graphics_get_translate_y_value_10.wrapping_neg(),
            )?;
            let item_player_23 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_12 = r.get(item_player_23, field::drawable::X)?.i();
            let item_player_24 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_12 = r.get(item_player_24, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_12.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_12.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_24 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_24, -32, 0, 0)?;
        }
    }
    let item_kind_15 = r.get(item, field::item::KIND)?.i();
    if item_kind_15 == 137 {
        let item_player_17 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_7 = r.get(item_player_17, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_7 == 0 {
            let item_collected_10 = r.get(item, field::item::COLLECTED)?.i();
            if item_collected_10 == 0 {
                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                let item_image_23 = r.get(item, field::item::IMAGE)?.r();
                host::graphics_draw_image(r, graphics, item_image_23, 0, 0, 0)?;
            }
        } else {
            let graphics_get_translate_x_value_9 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_9 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_9.wrapping_neg(),
                graphics_get_translate_y_value_9.wrapping_neg(),
            )?;
            let item_player_20 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_11 = r.get(item_player_20, field::drawable::X)?.i();
            let item_player_21 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_11 = r.get(item_player_21, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_11.wrapping_sub(x)).wrapping_add(3),
                (drawable_y_11.wrapping_sub(y)).wrapping_sub(18),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_22 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_22, 0, 0, 0)?;
        }
    }
    let item_kind_16 = r.get(item, field::item::KIND)?.i();
    if item_kind_16 == 138 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_21 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_21, -16, 0, 0)?;
        host::graphics_translate(r, graphics, 0, 16)?;
        host::graphics_set_color(r, graphics, 16777215)?;
        host::graphics_set_clip(r, graphics, 0, 0, 50, 50)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 50, 50)?;
        let text_value_2 = r.intern("50:-");
        host::graphics_draw_string(r, graphics, text_value_2, 0, 0, 20)?;
    }
    let item_kind_17 = r.get(item, field::item::KIND)?.i();
    if item_kind_17 == 139 {
        let item_collected_11 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_11 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image_11 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_11, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_12 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_12, 0, 0, 0)?;
        }
        let item_player_25 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_9 = r.get(item_player_25, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_9 != 0 {
            let graphics_get_translate_x_value_11 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_11 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_11.wrapping_neg(),
                graphics_get_translate_y_value_11.wrapping_neg(),
            )?;
            let item_player_26 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_13 = r.get(item_player_26, field::drawable::X)?.i();
            let item_player_27 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_13 = r.get(item_player_27, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_13.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_13.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_27 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_27, -32, 0, 0)?;
        }
    }
    let item_kind_18 = r.get(item, field::item::KIND)?.i();
    if item_kind_18 == 140 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_25 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_25, 0, 0, 0)?;
    }
    let item_kind_19 = r.get(item, field::item::KIND)?.i();
    if item_kind_19 == 141 {
        let item_collected_12 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_12 != 0 {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_26 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_26, -16, 0, 0)?;
    }
    let item_kind_20 = r.get(item, field::item::KIND)?.i();
    if item_kind_20 == 142 {
        let item_collected_13 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_13 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image_13 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_13, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_14 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_14, 0, 0, 0)?;
        }
        let item_player_29 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_11 = r.get(item_player_29, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_11 != 0 {
            let graphics_get_translate_x_value_13 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_13 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_13.wrapping_neg(),
                graphics_get_translate_y_value_13.wrapping_neg(),
            )?;
            let item_player_33 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_15 = r.get(item_player_33, field::drawable::X)?.i();
            let item_player_34 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_15 = r.get(item_player_34, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_15.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_15.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_30 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_30, -32, 0, 0)?;
        }
    }
    let item_kind_21 = r.get(item, field::item::KIND)?.i();
    if item_kind_21 == 143 {
        let item_collected_14 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_14 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image_15 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_15, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_16 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_16, 0, 0, 0)?;
        }
        let item_player_32 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_12 = r.get(item_player_32, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_12 != 0 {
            let graphics_get_translate_x_value_14 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_14 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_14.wrapping_neg(),
                graphics_get_translate_y_value_14.wrapping_neg(),
            )?;
            let item_player_35 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_16 = r.get(item_player_35, field::drawable::X)?.i();
            let item_player_36 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_16 = r.get(item_player_36, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_16.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_16.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_33 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_33, 0, 0, 0)?;
        }
    }
    let item_kind_22 = r.get(item, field::item::KIND)?.i();
    if item_kind_22 == 144 {
        let item_player_28 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_10 = r.get(item_player_28, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_10 != 0 {
            let graphics_get_translate_x_value_12 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_12 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_12.wrapping_neg(),
                graphics_get_translate_y_value_12.wrapping_neg(),
            )?;
            let item_player_30 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_14 = r.get(item_player_30, field::drawable::X)?.i();
            let item_player_31 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_14 = r.get(item_player_31, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_14.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_14.wrapping_sub(y)).wrapping_sub(21),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_29 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_29, 0, 0, 0)?;
        }
        let item_collected_15 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_15 == 0 {
            host::graphics_translate(r, graphics, 0, -16)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_32 = r.get(item, field::item::IMAGE)?.r();
            let item_animation_5 = r.get(item, field::item::ANIMATION)?.i();
            let quotient_value = arithmetic::divide_int(r, item_animation_5, 3)?;
            host::graphics_draw_image(
                r,
                graphics,
                item_image_32,
                (-16_i32).wrapping_mul(quotient_value),
                0,
                0,
            )?;
            let item_animation_6 = r.get(item, field::item::ANIMATION)?.i();
            r.set(
                item,
                field::item::ANIMATION,
                V::I((item_animation_6.wrapping_add(1)) as i8 as i32),
            )?;
            let item_animation_7 = r.get(item, field::item::ANIMATION)?.i();
            if item_animation_7 > 25 {
                r.set(item, field::item::ANIMATION, V::I(0))?;
            }
            host::graphics_translate(r, graphics, 0, 16)?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_34 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_34, -144, 0, 0)?;
        }
    }
    let item_kind_23 = r.get(item, field::item::KIND)?.i();
    if item_kind_23 == 145 {
        host::graphics_set_clip(r, graphics, 0, 0, 12, 22)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 12, 22)?;
        let item_image_28 = r.get(item, field::item::IMAGE)?.r();
        let item_animation_4 = r.get(item, field::item::ANIMATION)?.i();
        let remainder_value_3 = arithmetic::remainder_int(r, item_animation_4, 3)?;
        host::graphics_draw_image(
            r,
            graphics,
            item_image_28,
            (-12_i32).wrapping_mul(remainder_value_3),
            0,
            0,
        )?;
    }
    let item_kind_24 = r.get(item, field::item::KIND)?.i();
    if item_kind_24 == 146 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_31 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_31, 0, 0, 0)?;
        host::graphics_translate(r, graphics, 0, 16)?;
        host::graphics_set_color(r, graphics, 16777215)?;
        host::graphics_set_clip(r, graphics, 0, 0, 50, 50)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 50, 50)?;
        let text_value_3 = r.intern("99:-");
        host::graphics_draw_string(r, graphics, text_value_3, 0, 0, 20)?;
    }
    let item_kind_25 = r.get(item, field::item::KIND)?.i();
    if item_kind_25 >= 147 {
        let item_kind_27 = r.get(item, field::item::KIND)?.i();
        if item_kind_27 <= 149 {
            let item_collected_16 = r.get(item, field::item::COLLECTED)?.i();
            if item_collected_16 != 0 {
                return Ok(());
            }
            host::graphics_set_clip(r, graphics, 0, 0, 8, 15)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 8, 15)?;
            width = 0;
            let item_animation_8 = r.get(item, field::item::ANIMATION)?.i();
            if item_animation_8 > 24 {
                width = width.wrapping_add(1);
            }
            let item_animation_9 = r.get(item, field::item::ANIMATION)?.i();
            if item_animation_9 > 28 {
                width = width.wrapping_add(1);
            }
            let item_image_38 = r.get(item, field::item::IMAGE)?.r();
            let item_kind_32 = r.get(item, field::item::KIND)?.i();
            host::graphics_draw_image(
                r,
                graphics,
                item_image_38,
                width.wrapping_mul(-8),
                (-15_i32).wrapping_mul(item_kind_32.wrapping_sub(147)),
                0,
            )?;
        }
    }
    let item_kind_26 = r.get(item, field::item::KIND)?.i();
    if item_kind_26 == 150 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_35 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_35, 0, 0, 0)?;
        host::graphics_translate(r, graphics, 0, 16)?;
        host::graphics_set_color(r, graphics, 16777215)?;
        host::graphics_set_clip(r, graphics, 0, 0, 50, 50)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 50, 50)?;
        let text_value_4 = r.intern("10:-");
        host::graphics_draw_string(r, graphics, text_value_4, 0, 0, 20)?;
    }
    let item_kind_28 = r.get(item, field::item::KIND)?.i();
    if item_kind_28 == 151 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_36 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_36, -16, 0, 0)?;
        host::graphics_translate(r, graphics, 0, 16)?;
        host::graphics_set_color(r, graphics, 16777215)?;
        host::graphics_set_clip(r, graphics, 0, 0, 50, 50)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 50, 50)?;
        let text_value_5 = r.intern("25:-");
        host::graphics_draw_string(r, graphics, text_value_5, 0, 0, 20)?;
    }
    let item_kind_29 = r.get(item, field::item::KIND)?.i();
    if item_kind_29 == 152 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let item_image_37 = r.get(item, field::item::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_image_37, -32, 0, 0)?;
        host::graphics_translate(r, graphics, 0, 16)?;
        host::graphics_set_color(r, graphics, 16777215)?;
        host::graphics_set_clip(r, graphics, 0, 0, 50, 50)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 50, 50)?;
        let text_value_6 = r.intern("15:-");
        host::graphics_draw_string(r, graphics, text_value_6, 0, 0, 20)?;
    }
    let item_kind_30 = r.get(item, field::item::KIND)?.i();
    if item_kind_30 == 153 {
        let item_collected_17 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_17 != 0 {
            host::graphics_translate(r, graphics, 0, -12)?;
            host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
            let item_overlay_image_17 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_17, 0, -24, 0)?;
        } else {
            host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
            let item_overlay_image_18 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_overlay_image_18, 0, 0, 0)?;
        }
        let item_player_40 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_14 = r.get(item_player_40, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_14 != 0 {
            let graphics_get_translate_x_value_16 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_16 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_16.wrapping_neg(),
                graphics_get_translate_y_value_16.wrapping_neg(),
            )?;
            let item_player_41 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_18 = r.get(item_player_41, field::drawable::X)?.i();
            let item_player_42 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_18 = r.get(item_player_42, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_18.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_18.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_41 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_41, -16, 0, 0)?;
        }
    }
    let item_kind_31 = r.get(item, field::item::KIND)?.i();
    if item_kind_31 == 154 {
        let item_player_37 = r.get(item, field::item::PLAYER)?.r();
        let player_holding_item_13 = r.get(item_player_37, field::player::HOLDING_ITEM)?.i();
        if player_holding_item_13 != 0 {
            let graphics_get_translate_x_value_15 = host::graphics_get_translate_x(r, graphics)?;
            let graphics_get_translate_y_value_15 = host::graphics_get_translate_y(r, graphics)?;
            host::graphics_translate(
                r,
                graphics,
                graphics_get_translate_x_value_15.wrapping_neg(),
                graphics_get_translate_y_value_15.wrapping_neg(),
            )?;
            let item_player_38 = r.get(item, field::item::PLAYER)?.r();
            let drawable_x_17 = r.get(item_player_38, field::drawable::X)?.i();
            let item_player_39 = r.get(item, field::item::PLAYER)?.r();
            let drawable_y_17 = r.get(item_player_39, field::drawable::Y)?.i();
            host::graphics_translate(
                r,
                graphics,
                (drawable_x_17.wrapping_sub(x)).wrapping_add(4),
                (drawable_y_17.wrapping_sub(y)).wrapping_sub(16),
            )?;
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_39 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_39, 0, 0, 0)?;
        }
        let item_collected_19 = r.get(item, field::item::COLLECTED)?.i();
        if item_collected_19 == 0 {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let item_image_40 = r.get(item, field::item::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, item_image_40, 0, 0, 0)?;
        }
    }
    let item_kind_33 = r.get(item, field::item::KIND)?.i();
    if item_kind_33 != 155 {
        let previous_item_value = item;

        let item_animation_10_read_read = r.get(previous_item_value, field::item::ANIMATION)?.i();
        r.set(
            previous_item_value,
            field::item::ANIMATION,
            V::I((item_animation_10_read_read.wrapping_add(1)) as i8 as i32),
        )?;
        let item_animation_11_read_read = r.get(item, field::item::ANIMATION)?.i();
        if item_animation_11_read_read <= 31 {
            return Ok(());
        }
        r.set(item, field::item::ANIMATION, V::I(0))?;
        return Ok(());
    }
    let item_collected_18 = r.get(item, field::item::COLLECTED)?.i();
    if item_collected_18 != 0 {
        host::graphics_translate(r, graphics, 0, -12)?;
        host::graphics_set_clip(r, graphics, 0, 0, 32, 38)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 38)?;
        let item_overlay_image_19 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_overlay_image_19, 0, -24, 0)?;
    } else {
        host::graphics_set_clip(r, graphics, 0, 0, 32, 24)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 32, 24)?;
        let item_overlay_image_20 = r.get(item, field::item::OVERLAY_IMAGE)?.r();
        host::graphics_draw_image(r, graphics, item_overlay_image_20, 0, 0, 0)?;
    }
    let item_player_43 = r.get(item, field::item::PLAYER)?.r();
    let player_holding_item_15 = r.get(item_player_43, field::player::HOLDING_ITEM)?.i();
    if player_holding_item_15 == 0 {
        let previous_item_value = item;

        let item_animation_10_read_read = r.get(previous_item_value, field::item::ANIMATION)?.i();
        r.set(
            previous_item_value,
            field::item::ANIMATION,
            V::I((item_animation_10_read_read.wrapping_add(1)) as i8 as i32),
        )?;
        let item_animation_11_read_read = r.get(item, field::item::ANIMATION)?.i();
        if item_animation_11_read_read <= 31 {
            return Ok(());
        }
        r.set(item, field::item::ANIMATION, V::I(0))?;
        return Ok(());
    }
    let graphics_get_translate_x_value_17 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_17 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_17.wrapping_neg(),
        graphics_get_translate_y_value_17.wrapping_neg(),
    )?;
    let item_player_44 = r.get(item, field::item::PLAYER)?.r();
    let drawable_x_19 = r.get(item_player_44, field::drawable::X)?.i();
    let item_player_45 = r.get(item, field::item::PLAYER)?.r();
    let drawable_y_19 = r.get(item_player_45, field::drawable::Y)?.i();
    host::graphics_translate(
        r,
        graphics,
        (drawable_x_19.wrapping_sub(x)).wrapping_add(4),
        (drawable_y_19.wrapping_sub(y)).wrapping_sub(16),
    )?;
    host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
    let item_image_42 = r.get(item, field::item::IMAGE)?.r();
    host::graphics_draw_image(r, graphics, item_image_42, 0, 0, 0)?;
    let previous_item_value = item;

    let item_animation_10_read_read = r.get(previous_item_value, field::item::ANIMATION)?.i();
    r.set(
        previous_item_value,
        field::item::ANIMATION,
        V::I((item_animation_10_read_read.wrapping_add(1)) as i8 as i32),
    )?;
    let item_animation_11_read_read = r.get(item, field::item::ANIMATION)?.i();
    if item_animation_11_read_read <= 31 {
        return Ok(());
    }
    r.set(item, field::item::ANIMATION, V::I(0))?;
    return Ok(());
}
