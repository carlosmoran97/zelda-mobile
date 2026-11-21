//! Canvas saves.
use super::*;

/// Save settings. Original reference: `k.a()V`.
pub(in crate::game) fn save_settings(r: &mut Runtime, canvas: u32) -> Result<(), u32> {
    let mut object_1: u32 = 0;
    let mut object_2: u32 = 0;
    let mut object_3: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut carry_1_i: i32 = 0;
    let mut carry_1_r: u32 = 0;
    let mut carry_2_i: i32 = 0;
    let mut carry_2_r: u32 = 0;
    let mut carry_3_i: i32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let text_value = r.intern("ZeldaSettings");
    outcome = (|| -> JResult {
        Ok(V::R(host::record_store_open_record_store(
            r, text_value, 1,
        )?))
    })();
    (carry_0_r,) = (canvas,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let record_store_open_record_store_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        r.set(
            carry_0_r,
            field::canvas::RECORD_STORE,
            V::R(record_store_open_record_store_value),
        )?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(canvas, field::canvas::RECORD_STORE)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let record_store_value = outcome.unwrap().r();
    if record_store_value == 0 {
        outcome = (|| -> JResult { Ok(V::R(r.get(canvas, field::canvas::RECORD_STORE)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            return Ok(());
        }
        let record_store_value_2 = outcome.unwrap().r();
        outcome = (|| -> JResult {
            host::record_store_close_record_store(r, record_store_value_2)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            return Ok(());
        }

        return Ok(());
    }
    let object_value = r.alloc("java/io/ByteArrayOutputStream", Native::Empty);
    outcome = (|| -> JResult {
        host::byte_array_output_stream_initialize(r, object_value)?;
        Ok(V::I(0))
    })();
    (carry_0_r,) = (object_value,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if r.is_instance(carry_0_r, "java/lang/Exception") {
            return Ok(());
        }
        return Err(outcome.unwrap_err());
    }
    object_2 = carry_0_r;
    let object_value_2 = r.alloc("java/io/DataOutputStream", Native::Empty);
    outcome = (|| -> JResult {
        host::data_output_stream_initialize(r, object_value_2, object_2)?;
        Ok(V::I(0))
    })();
    (carry_0_r,) = (object_value_2,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    object_3 = carry_0_r;
    outcome = (|| -> JResult {
        host::byte_array_output_stream_reset(r, object_2)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if r.is_instance(carry_0_r, "java/lang/Exception") {
            return Ok(());
        }
        return Err(outcome.unwrap_err());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::MUSIC_VOLUME)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let music_volume_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, music_volume_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::EFFECT_VOLUME)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let effect_volume_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, effect_volume_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::SECONDARY_KEY)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let secondary_key_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, secondary_key_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::UP_KEY)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let up_key_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, up_key_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::DOWN_KEY)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let down_key_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, down_key_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::LEFT_KEY)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let left_key_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, left_key_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::RIGHT_KEY)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let right_key_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, right_key_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::PRIMARY_KEY)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let primary_key_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, primary_key_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::MENU_KEY)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let menu_key_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, menu_key_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::STATE_AP_UPPER)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let state_ap_upper_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_boolean(r, carry_0_r, state_ap_upper_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::USE_PHONE_BUTTONS)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let use_phone_buttons_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_boolean(r, carry_0_r, use_phone_buttons_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::STATE_AK)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let state_ak_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_boolean(r, carry_0_r, state_ak_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::STATE_AO_UPPER)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let state_ao_upper_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_boolean(r, carry_0_r, state_ao_upper_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let running_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_boolean(r, carry_0_r, running_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::STATE_W)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let state_w_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_boolean(r, carry_0_r, state_w_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::STATE_AX)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let state_ax_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_int(r, carry_0_r, state_ax_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(canvas, field::canvas::STATE_X)?.i())) })();
    (carry_0_r,) = (object_3,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let state_x_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        host::data_output_stream_write_boolean(r, carry_0_r, state_x_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult {
        host::data_output_stream_flush(r, object_3)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    outcome = (|| -> JResult {
        Ok(V::R(host::byte_array_output_stream_to_byte_array(
            r, object_2,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if r.is_instance(carry_0_r, "java/lang/Exception") {
            return Ok(());
        }
        return Err(outcome.unwrap_err());
    }
    let byte_array_output_stream_to_byte_array_value = outcome.unwrap().r();
    object_1 = byte_array_output_stream_to_byte_array_value;
    outcome = (|| -> JResult { Ok(V::R(r.get(canvas, field::canvas::RECORD_STORE)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let record_store_value_3 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        Ok(V::I(host::record_store_get_num_records(
            r,
            record_store_value_3,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let record_store_get_num_records_value = outcome.unwrap().i();
    if record_store_get_num_records_value != 0 {
        outcome = (|| -> JResult { Ok(V::R(r.get(canvas, field::canvas::RECORD_STORE)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            return Ok(());
        }
        let record_store_value_4 = outcome.unwrap().r();
        outcome = (|| -> JResult { Ok(V::I(r.array_len(object_1)?)) })();
        (carry_0_r, carry_1_i, carry_2_r, carry_3_i) = (record_store_value_4, 1, object_1, 0);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            return Ok(());
        }
        let length_value = outcome.unwrap().i();
        outcome = (|| -> JResult {
            host::record_store_set_record(
                r,
                carry_0_r,
                carry_1_i,
                carry_2_r,
                carry_3_i,
                length_value,
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            return Ok(());
        }
    } else {
        outcome = (|| -> JResult { Ok(V::R(r.get(canvas, field::canvas::RECORD_STORE)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            return Ok(());
        }
        let record_store_value_5 = outcome.unwrap().r();
        outcome = (|| -> JResult { Ok(V::I(r.array_len(object_1)?)) })();
        (carry_0_r, carry_1_r, carry_2_i) = (record_store_value_5, object_1, 0);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            return Ok(());
        }
        let length_value_2 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            Ok(V::I(host::record_store_add_record(
                r,
                carry_0_r,
                carry_1_r,
                carry_2_i,
                length_value_2,
            )?))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            return Ok(());
        }
        let record_store_add_record_value = outcome.unwrap().i();
    }
    outcome = (|| -> JResult {
        host::byte_array_output_stream_reset(r, object_2)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if r.is_instance(carry_0_r, "java/lang/Exception") {
            return Ok(());
        }
        return Err(outcome.unwrap_err());
    }
    outcome = (|| -> JResult {
        host::byte_array_output_stream_close(r, object_2)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if r.is_instance(carry_0_r, "java/lang/Exception") {
            return Ok(());
        }
        return Err(outcome.unwrap_err());
    }
    outcome = (|| -> JResult {
        host::data_output_stream_close(r, object_3)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }

    outcome = (|| -> JResult { Ok(V::R(r.get(canvas, field::canvas::RECORD_STORE)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }
    let record_store_value_2 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::record_store_close_record_store(r, record_store_value_2)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        return Ok(());
    }

    return Ok(());
}

/// Start game. Original reference: `k.b(Z)V`.
pub(in crate::game) fn start_game(r: &mut Runtime, canvas: u32, new_game: i32) -> Result<(), u32> {
    use field::{canvas as screen, drawable as body, player as hero};
    r.set(canvas, screen::STATE_V_UPPER, V::R(0))?;
    r.set(canvas, screen::WORLD, V::R(0))?;
    let player = r.alloc("h", Native::Empty);
    super::player::initialize(r, player)?;
    r.set(canvas, screen::PLAYER, V::R(player))?;
    for key in [
        screen::STATE_AD_UPPER,
        screen::STATE_AE,
        screen::STATE_P,
        screen::STATE_Q,
        screen::SCREEN_STATE,
        screen::KEY_CONFIGURATION_STEP,
        screen::CONFIGURE_PHONE_KEYS,
    ] {
        r.set(canvas, key, V::I(0))?;
    }
    r.set(canvas, screen::REDRAW, V::I(1))?;
    r.set(canvas, screen::REDRAW_WORLD, V::I(1))?;
    let sound = r.get(canvas, screen::SOUND)?.r();
    let slot = r.get(canvas, screen::SAVE_SLOT)?.i();
    let load = (|| -> Result<(), u32> {
        let name = ["ZeldaSave", "ZeldaSave2", "ZeldaSave3"]
            .get(slot as usize)
            .copied();
        if new_game != 0 {
            if let Some(name) = name {
                let name = r.intern(name);
                host::record_store_delete_record_store(r, name)?;
            }
        }
        let empty = host::record_store_list_record_stores(r)? == 0;
        if empty {
            r.set(canvas, screen::CONFIGURE_PHONE_KEYS, V::I(1))?;
        }
        let mut bytes = r.new_array(8, 200)?;
        if let Some(name) = name {
            let name = r.intern(name);
            let store = host::record_store_open_record_store(r, name, 0)?;
            r.set(canvas, screen::RECORD_STORE, V::R(store))?;
        }
        let store = r.get(canvas, screen::RECORD_STORE)?.r();
        if r.get(canvas, screen::CONFIGURE_PHONE_KEYS)?.i() == 0 {
            bytes = host::record_store_get_record(r, store, 1)?;
        }
        if bytes != 0 {
            let fresh = r.get(canvas, screen::CONFIGURE_PHONE_KEYS)?.i();
            super::player::load_save(r, player, bytes, fresh, canvas)?;
            r.set(canvas, screen::ZONE, V::I(-1))?;
            let zone = r.get(player, hero::ZONE)?.i();
            super::sound::select_map_music(r, sound, zone, 0)?;
            r.set(canvas, screen::ZONE, V::I(zone))?;
            let map = r.get(player, hero::MAP)?.i();
            r.set(canvas, screen::MAP, V::I(map))?;
        }
        host::record_store_close_record_store(r, store)
    })();
    if let Err(error) = load {
        if !r.is_instance(error, "javax/microedition/rms/RecordStoreException") {
            return Err(error);
        }
    }
    r.set(canvas, screen::SCREEN_STATE, V::I(1))?;
    r.set(canvas, screen::REFRESH_COMMANDS, V::I(1))?;
    let zone = r.get(player, hero::ZONE)?.i();
    super::sound::select_map_music(r, sound, zone, 0)?;
    let shield = if r.get(canvas, screen::STATE_AX)?.i() == 6 {
        r.get(player, hero::HAS_MIRROR_SHIELD)?.i()
    } else {
        0
    };
    r.set(player, hero::SHIELD_RAISED, V::I(shield))?;
    let x = r.get(player, body::X)?.i();
    let y = r.get(player, body::Y)?.i();
    super::canvas::center_camera(r, canvas, x, y)?;
    let mut world = create_world(r, canvas, player)?;
    let scene = (|| -> Result<(), u32> {
        let life = r.load_image("/Images/Life2.png")?;
        r.set(canvas, screen::STATE_T, V::R(life))?;
        let drawables = r.alloc("java/util/Vector", Native::Vector(Vec::new()));
        r.set(canvas, screen::STATE_AW, V::R(drawables))?;
        if !can_step_any_direction(r, world, player)? {
            r.set(player, hero::ON_RAFT, V::I(1))?;
            if !can_step_any_direction(r, world, player)? {
                // Preserve the beta's recovery position when a saved room is blocked.
                for (key, value) in [
                    (hero::ZONE, 0),
                    (hero::ON_RAFT, 0),
                    (hero::MAP, 120),
                    (body::X, 75),
                    (body::Y, 85),
                ] {
                    r.set(player, key, V::I(value))?;
                }
                world = create_world(r, canvas, player)?;
            }
        }
        Ok(())
    })();
    if let Err(error) = scene {
        if !r.is_instance(error, "java/io/IOException") {
            return Err(error);
        }
    }
    let appearance = (|| -> Result<(), u32> {
        let tunic = r.get(player, hero::TUNIC_COLOR)?.i();
        if let Some(path) = [
            "/Images/LinkWalk.png",
            "/Images/LinkWalk_Blue.png",
            "/Images/LinkWalk_Red.png",
        ]
        .get(tunic as usize)
        {
            let image = r.load_image(path)?;
            r.set(player, hero::SPRITE, V::R(image))?;
            r.set(player, hero::DEFENSE, V::I(1 << tunic))?;
        }
        let sword = r.get(player, hero::SWORD_LEVEL)?.i();
        if let Some(path) = [
            "/Images/Sword01.png",
            "/Images/Sword02.png",
            "/Images/Sword03.png",
        ]
        .get(sword as usize)
        {
            let image = r.load_image(path)?;
            r.set(player, hero::EQUIPMENT_SPRITE, V::R(image))?;
            r.set(player, hero::SWORD_DAMAGE, V::I(1 << sword))?;
        }
        Ok(())
    })();
    if let Err(error) = appearance {
        if !r.is_instance(error, "java/io/IOException") {
            return Err(error);
        }
    }
    Ok(())
}

fn create_world(r: &mut Runtime, canvas: u32, player: u32) -> Result<u32, u32> {
    let image = r.get(canvas, field::canvas::SCENE_IMAGE)?.r();
    let graphics = host::image_get_graphics(r, image)?;
    let overlay = r.get(canvas, field::canvas::GRAPHICS)?.r();
    let world = r.alloc("j", Native::Empty);
    super::world::initialize(r, world, graphics, overlay, player, canvas)?;
    r.set(canvas, field::canvas::WORLD, V::R(world))?;
    Ok(world)
}

fn can_step_any_direction(r: &mut Runtime, world: u32, player: u32) -> Result<bool, u32> {
    for direction in 0..4 {
        let x = r.get(player, field::drawable::X)?.i();
        let y = r.get(player, field::drawable::Y)?.i();
        if super::world::can_player_move(r, world, x, y, direction, player, 0)? != 0 {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Load save slots. Original reference: `k.c()V`.
#[rustfmt::skip]
pub(in crate::game) fn load_save_slots(r: &mut Runtime, canvas: u32) -> Result<(), u32> {
    let mut object_1: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    'next_step: {
    'handle_result_3: {
    'next_step_4: {
    'check_canvas_record_store_2: {
    'check_canvas_save_slots_3: {
    'check_canvas_save_slots_6: {
    'next_step_5: {
    'check_canvas_save_slots_7: {
    'handle_result_9: {
    'handle_result_8: {
    'handle_result_5: {
    'handle_result_4: {
    'handle_result: {
    'next_step_2: {
    'check_canvas_record_store: {
    'check_canvas_save_slots_2: {
    'check_canvas_save_slots_4: {
    'next_step_3: {
    'check_canvas_save_slots_5: {
    'handle_result_7: {
    'handle_result_6: {
    'handle_result_2: {
    let canvas_save_slots = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let player_value = r.alloc("h", Native::Empty);
    super::player::initialize(r, player_value)?;
    r.array_set(canvas_save_slots, 0, V::R(player_value))?;
    let canvas_save_slots_2 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let player_value_2 = r.alloc("h", Native::Empty);
    super::player::initialize(r, player_value_2)?;
    r.array_set(canvas_save_slots_2, 1, V::R(player_value_2))?;
    let canvas_save_slots_3 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let player_value_3 = r.alloc("h", Native::Empty);
    super::player::initialize(r, player_value_3)?;
    r.array_set(canvas_save_slots_3, 2, V::R(player_value_3))?;
    host::system_gc(r)?;
    let text_value_4 = r.intern("ZeldaSave");
    outcome = (|| -> JResult { Ok(V::R(host::record_store_open_record_store(r, text_value_4, 0)?)) })();
    (carry_0_r,) = (canvas,);
    if outcome.is_err() { break 'handle_result; }
    } // handle_result_2
    let record_store_open_record_store_value = outcome.unwrap().r();
    r.set(carry_0_r, field::canvas::RECORD_STORE, V::R(record_store_open_record_store_value))?;
    let canvas_record_store = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    if canvas_record_store == 0 { break 'check_canvas_save_slots_2; }
    } // handle_result_6
    object_1 = 0;
    let canvas_record_store_4 = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    outcome = (|| -> JResult { Ok(V::R(host::record_store_get_record(r, canvas_record_store_4, 1)?)) })();
    if outcome
        .is_err()
    {
        break 'handle_result;
    }
    } // handle_result_7
    let record_store_get_record_value =
        outcome
            .unwrap()
            .r();
    object_1 = record_store_get_record_value;
    if record_store_get_record_value == 0 { break 'check_canvas_save_slots_4; }
    } // check_canvas_save_slots_5
    let canvas_save_slots_11 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let element_value = r.array_get(canvas_save_slots_11, 0)?.r();
    outcome =
        (|| -> JResult {
            super::player::load_save(r, element_value, object_1, 0, canvas)?;
            Ok(V::I(0))
        })(
        );
    if outcome.is_err() {
        break 'handle_result;
    }
    } // next_step_3

    break 'check_canvas_record_store;
    } // check_canvas_save_slots_4
    let canvas_save_slots_10 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    r.array_set(
        canvas_save_slots_10,
        0,
        V::R(0),
    )?;
    break 'check_canvas_record_store;
    } // check_canvas_save_slots_2
    let save_slots_value_7 = r
        .get(
            canvas,
            field::canvas::SAVE_SLOTS,
        )?
        .r();
    r.array_set(
        save_slots_value_7,
        0,
        V::R(0),
    )?;
    } // check_canvas_record_store
    let record_store_value_7 = r
        .get(
            canvas,
            field::canvas::RECORD_STORE,
        )?
        .r();
    outcome = (|| -> JResult {
        host::record_store_close_record_store(r, record_store_value_7)?;
        Ok(V::I(0))
    })(
    );
    if outcome.is_err() {
        break 'handle_result;
    }
    } // next_step_2

    break 'handle_result_4;
    } // handle_result
    carry_0_r = outcome.unwrap_err();
    if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
return Err(outcome.unwrap_err());
}
    let state_out_value =
        r.static_get(field::system::STATE_OUT).r();
    let text_value =
        r.intern("error loading char 0");
    host::print_stream_println(
        r,
        state_out_value,
        text_value,
    )?;
    let save_slots_value_4 = r
        .get(canvas, field::canvas::SAVE_SLOTS)?
        .r();
    r.array_set(save_slots_value_4, 0, V::R(0))?;
    } // handle_result_4
    let text_value_5 = r.intern("ZeldaSave2");
    outcome = (|| -> JResult {
        Ok(V::R(host::record_store_open_record_store(
            r,
            text_value_5,
            0,
        )?))
    })(
    );
    (carry_0_r,) = (canvas,);
    if outcome.is_err() {
        break 'handle_result_3;
    }
    } // handle_result_5
    let record_store_open_record_store_value_2 =
        outcome.unwrap().r();
    r.set(
        carry_0_r,
        field::canvas::RECORD_STORE,
        V::R(record_store_open_record_store_value_2),
    )?;
    let canvas_record_store_2 =
        r.get(canvas, field::canvas::RECORD_STORE)?.r();
    if canvas_record_store_2 == 0 {
        break 'check_canvas_save_slots_3;
    }
    } // handle_result_8
    object_1 = 0;
    let canvas_record_store_5 =
        r.get(canvas, field::canvas::RECORD_STORE)?.r();
    outcome = (|| -> JResult {
        Ok(V::R(host::record_store_get_record(
            r,
            canvas_record_store_5,
            1,
        )?))
    })();
    if outcome.is_err() {
        break 'handle_result_3;
    }
    } // handle_result_9
    let record_store_get_record_value_2 = outcome.unwrap().r();
    object_1 = record_store_get_record_value_2;
    if record_store_get_record_value_2 == 0 {
        break 'check_canvas_save_slots_6;
    }
    } // check_canvas_save_slots_7
    let canvas_save_slots_13 =
        r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let element_value_2 = r.array_get(canvas_save_slots_13, 1)?.r();
    outcome = (|| -> JResult {
        super::player::load_save(
            r,
            element_value_2,
            object_1,
            0,
            canvas,
        )?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        break 'handle_result_3;
    }
    } // next_step_5

    break 'check_canvas_record_store_2;
    } // check_canvas_save_slots_6
    let canvas_save_slots_12 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    r.array_set(canvas_save_slots_12, 1, V::R(0))?;
    break 'check_canvas_record_store_2;
    } // check_canvas_save_slots_3
    let canvas_save_slots_8 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    r.array_set(canvas_save_slots_8, 1, V::R(0))?;
    } // check_canvas_record_store_2
    let canvas_record_store_8 = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    outcome = (|| -> JResult {
        host::record_store_close_record_store(r, canvas_record_store_8)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        break 'handle_result_3;
    }
    } // next_step_4

    break 'next_step;
    } // handle_result_3
    carry_0_r = outcome.unwrap_err();
    if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
        return Err(outcome.unwrap_err());
    }
    let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
    let text_value_2 = r.intern("error loading char 1");
    host::print_stream_println(r, state_out_value_2, text_value_2)?;
    let canvas_save_slots_5 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    r.array_set(canvas_save_slots_5, 1, V::R(0))?;
    } // next_step
    let text_value_6 = r.intern("ZeldaSave3");
    outcome = (|| -> JResult {
        Ok(V::R(host::record_store_open_record_store(
            r,
            text_value_6,
            0,
        )?))
    })();
    (carry_0_r,) = (canvas,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("error loading char 2");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        let canvas_save_slots_6_read = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
        r.array_set(canvas_save_slots_6_read, 2, V::R(0))?;
        let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
        let text_value_7 = r.intern("exit load all players");
        host::print_stream_println(r, state_out_value_4, text_value_7)?;
        return Ok(());
    }
    let record_store_open_record_store_value_3 = outcome.unwrap().r();
    r.set(
        carry_0_r,
        field::canvas::RECORD_STORE,
        V::R(record_store_open_record_store_value_3),
    )?;
    let canvas_record_store_3 = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    if canvas_record_store_3 == 0 {
        let canvas_save_slots_9 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
        r.array_set(canvas_save_slots_9, 2, V::R(0))?;
        let canvas_record_store_9_read_read = r.get(canvas, field::canvas::RECORD_STORE)?.r();
        outcome = (|| -> JResult {
            host::record_store_close_record_store(r, canvas_record_store_9_read_read)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let text_value_3 = r.intern("error loading char 2");
            host::print_stream_println(r, state_out_value_3, text_value_3)?;
            let canvas_save_slots_6_read = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            r.array_set(canvas_save_slots_6_read, 2, V::R(0))?;
            let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
            let text_value_7 = r.intern("exit load all players");
            host::print_stream_println(r, state_out_value_4, text_value_7)?;
            return Ok(());
        }

        let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
        let text_value_7 = r.intern("exit load all players");
        host::print_stream_println(r, state_out_value_4, text_value_7)?;
        return Ok(());
    }
    object_1 = 0;
    let canvas_record_store_6 = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    outcome = (|| -> JResult {
        Ok(V::R(host::record_store_get_record(
            r,
            canvas_record_store_6,
            1,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("error loading char 2");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        let canvas_save_slots_6_read = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
        r.array_set(canvas_save_slots_6_read, 2, V::R(0))?;
        let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
        let text_value_7 = r.intern("exit load all players");
        host::print_stream_println(r, state_out_value_4, text_value_7)?;
        return Ok(());
    }
    let record_store_get_record_value_3 = outcome.unwrap().r();
    object_1 = record_store_get_record_value_3;
    if record_store_get_record_value_3 == 0 {
        let canvas_save_slots_14 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
        r.array_set(canvas_save_slots_14, 2, V::R(0))?;
        let canvas_record_store_9_read_read = r.get(canvas, field::canvas::RECORD_STORE)?.r();
        outcome = (|| -> JResult {
            host::record_store_close_record_store(r, canvas_record_store_9_read_read)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let text_value_3 = r.intern("error loading char 2");
            host::print_stream_println(r, state_out_value_3, text_value_3)?;
            let canvas_save_slots_6_read = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
            r.array_set(canvas_save_slots_6_read, 2, V::R(0))?;
            let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
            let text_value_7 = r.intern("exit load all players");
            host::print_stream_println(r, state_out_value_4, text_value_7)?;
            return Ok(());
        }

        let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
        let text_value_7 = r.intern("exit load all players");
        host::print_stream_println(r, state_out_value_4, text_value_7)?;
        return Ok(());
    }
    let canvas_save_slots_15 = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
    let element_value_3 = r.array_get(canvas_save_slots_15, 2)?.r();
    outcome = (|| -> JResult {
        super::player::load_save(r, element_value_3, object_1, 0, canvas)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("error loading char 2");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        let canvas_save_slots_6_read = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
        r.array_set(canvas_save_slots_6_read, 2, V::R(0))?;
        let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
        let text_value_7 = r.intern("exit load all players");
        host::print_stream_println(r, state_out_value_4, text_value_7)?;
        return Ok(());
    }

    let canvas_record_store_9_read_read = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    outcome = (|| -> JResult {
        host::record_store_close_record_store(r, canvas_record_store_9_read_read)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("error loading char 2");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        let canvas_save_slots_6_read = r.get(canvas, field::canvas::SAVE_SLOTS)?.r();
        r.array_set(canvas_save_slots_6_read, 2, V::R(0))?;
        let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
        let text_value_7 = r.intern("exit load all players");
        host::print_stream_println(r, state_out_value_4, text_value_7)?;
        return Ok(());
    }

    let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
    let text_value_7 = r.intern("exit load all players");
    host::print_stream_println(r, state_out_value_4, text_value_7)?;
    return Ok(());
}

/// Delete save slot. Original reference: `k.b(I)V`.
pub(in crate::game) fn delete_save_slot(
    r: &mut Runtime,
    mut canvas: u32,
    argument_1: i32,
) -> Result<(), u32> {
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));

    if argument_1 == 0 {
        let text_value_2 = r.intern("ZeldaSave");
        outcome = (|| -> JResult {
            host::record_store_delete_record_store(r, text_value_2)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            canvas = carry_0_r;
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
            host::string_buffer_initialize(r, object_value)?;
            let text_value = r.intern("error deleting char ");
            let string_buffer_append_2_value =
                host::string_buffer_append_2(r, object_value, text_value)?;
            let record_store_exception_to_string_value =
                host::record_store_exception_to_string(r, canvas)?;
            let string_buffer_append_2_value_2 = host::string_buffer_append_2(
                r,
                string_buffer_append_2_value,
                record_store_exception_to_string_value,
            )?;
            let string_buffer_to_string_value =
                host::string_buffer_to_string(r, string_buffer_append_2_value_2)?;
            host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
            return Ok(());
        }
    }

    if argument_1 == 1 {
        let text_value_3 = r.intern("ZeldaSave2");
        outcome = (|| -> JResult {
            host::record_store_delete_record_store(r, text_value_3)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            canvas = carry_0_r;
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
            host::string_buffer_initialize(r, object_value)?;
            let text_value = r.intern("error deleting char ");
            let string_buffer_append_2_value =
                host::string_buffer_append_2(r, object_value, text_value)?;
            let record_store_exception_to_string_value =
                host::record_store_exception_to_string(r, canvas)?;
            let string_buffer_append_2_value_2 = host::string_buffer_append_2(
                r,
                string_buffer_append_2_value,
                record_store_exception_to_string_value,
            )?;
            let string_buffer_to_string_value =
                host::string_buffer_to_string(r, string_buffer_append_2_value_2)?;
            host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
            return Ok(());
        }
    }

    if argument_1 != 2 {
        outcome = (|| -> JResult {
            super::canvas::load_save_slots(r, canvas)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            canvas = carry_0_r;
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
            host::string_buffer_initialize(r, object_value)?;
            let text_value = r.intern("error deleting char ");
            let string_buffer_append_2_value =
                host::string_buffer_append_2(r, object_value, text_value)?;
            let record_store_exception_to_string_value =
                host::record_store_exception_to_string(r, canvas)?;
            let string_buffer_append_2_value_2 = host::string_buffer_append_2(
                r,
                string_buffer_append_2_value,
                record_store_exception_to_string_value,
            )?;
            let string_buffer_to_string_value =
                host::string_buffer_to_string(r, string_buffer_append_2_value_2)?;
            host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
            return Ok(());
        }

        return Ok(());
    }
    let text_value_4 = r.intern("ZeldaSave3");
    outcome = (|| -> JResult {
        host::record_store_delete_record_store(r, text_value_4)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        canvas = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
        host::string_buffer_initialize(r, object_value)?;
        let text_value = r.intern("error deleting char ");
        let string_buffer_append_2_value =
            host::string_buffer_append_2(r, object_value, text_value)?;
        let record_store_exception_to_string_value =
            host::record_store_exception_to_string(r, canvas)?;
        let string_buffer_append_2_value_2 = host::string_buffer_append_2(
            r,
            string_buffer_append_2_value,
            record_store_exception_to_string_value,
        )?;
        let string_buffer_to_string_value =
            host::string_buffer_to_string(r, string_buffer_append_2_value_2)?;
        host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
        return Ok(());
    }

    outcome = (|| -> JResult {
        super::canvas::load_save_slots(r, canvas)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        canvas = carry_0_r;
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
        host::string_buffer_initialize(r, object_value)?;
        let text_value = r.intern("error deleting char ");
        let string_buffer_append_2_value =
            host::string_buffer_append_2(r, object_value, text_value)?;
        let record_store_exception_to_string_value =
            host::record_store_exception_to_string(r, canvas)?;
        let string_buffer_append_2_value_2 = host::string_buffer_append_2(
            r,
            string_buffer_append_2_value,
            record_store_exception_to_string_value,
        )?;
        let string_buffer_to_string_value =
            host::string_buffer_to_string(r, string_buffer_append_2_value_2)?;
        host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
        return Ok(());
    }

    return Ok(());
}

/// Save game. Original reference: `k.d()V`.
pub(in crate::game) fn save_game(r: &mut Runtime, canvas: u32) -> Result<(), u32> {
    let mut object_1: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let canvas_player = r.get(canvas, field::canvas::PLAYER)?.r();
    if canvas_player == 0 {
        return Ok(());
    }
    let canvas_save_slot = r.get(canvas, field::canvas::SAVE_SLOT)?.i();
    if canvas_save_slot == 0 {
        let text_value_2 = r.intern("ZeldaSave");
        outcome = (|| -> JResult {
            Ok(V::R(host::record_store_open_record_store(
                r,
                text_value_2,
                1,
            )?))
        })();
        (carry_0_r,) = (canvas,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let text_value = r.intern("error saving char");
            host::print_stream_println(r, state_out_value, text_value)?;
            return Ok(());
        }
        let record_store_open_record_store_value = outcome.unwrap().r();
        r.set(
            carry_0_r,
            field::canvas::RECORD_STORE,
            V::R(record_store_open_record_store_value),
        )?;
    }
    let canvas_save_slot_2 = r.get(canvas, field::canvas::SAVE_SLOT)?.i();
    if canvas_save_slot_2 == 1 {
        let text_value_3 = r.intern("ZeldaSave2");
        outcome = (|| -> JResult {
            Ok(V::R(host::record_store_open_record_store(
                r,
                text_value_3,
                1,
            )?))
        })();
        (carry_0_r,) = (canvas,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let text_value = r.intern("error saving char");
            host::print_stream_println(r, state_out_value, text_value)?;
            return Ok(());
        }
        let record_store_open_record_store_value_2 = outcome.unwrap().r();
        r.set(
            carry_0_r,
            field::canvas::RECORD_STORE,
            V::R(record_store_open_record_store_value_2),
        )?;
    }
    let canvas_save_slot_3 = r.get(canvas, field::canvas::SAVE_SLOT)?.i();
    if canvas_save_slot_3 == 2 {
        let text_value_4 = r.intern("ZeldaSave3");
        outcome = (|| -> JResult {
            Ok(V::R(host::record_store_open_record_store(
                r,
                text_value_4,
                1,
            )?))
        })();
        (carry_0_r,) = (canvas,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let text_value = r.intern("error saving char");
            host::print_stream_println(r, state_out_value, text_value)?;
            return Ok(());
        }
        let record_store_open_record_store_value_3 = outcome.unwrap().r();
        r.set(
            carry_0_r,
            field::canvas::RECORD_STORE,
            V::R(record_store_open_record_store_value_3),
        )?;
    }
    let canvas_record_store = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    if canvas_record_store == 0 {
        let canvas_record_store_2_read = r.get(canvas, field::canvas::RECORD_STORE)?.r();
        outcome = (|| -> JResult {
            host::record_store_close_record_store(r, canvas_record_store_2_read)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let text_value = r.intern("error saving char");
            host::print_stream_println(r, state_out_value, text_value)?;
            return Ok(());
        }

        return Ok(());
    }
    let canvas_player_2 = r.get(canvas, field::canvas::PLAYER)?.r();
    let canvas_zone = r.get(canvas, field::canvas::ZONE)?.i();
    r.set(canvas_player_2, field::player::ZONE, V::I(canvas_zone))?;
    let canvas_player_3 = r.get(canvas, field::canvas::PLAYER)?.r();
    let canvas_map = r.get(canvas, field::canvas::MAP)?.i();
    r.set(canvas_player_3, field::player::MAP, V::I(canvas_map))?;
    let canvas_player_4 = r.get(canvas, field::canvas::PLAYER)?.r();
    outcome = (|| -> JResult { Ok(V::R(super::player::encode_save(r, canvas_player_4)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let text_value = r.intern("error saving char");
        host::print_stream_println(r, state_out_value, text_value)?;
        return Ok(());
    }
    let encode_save_value = outcome.unwrap().r();
    object_1 = encode_save_value;
    if encode_save_value == 0 {
        let canvas_record_store_2_read = r.get(canvas, field::canvas::RECORD_STORE)?.r();
        outcome = (|| -> JResult {
            host::record_store_close_record_store(r, canvas_record_store_2_read)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let text_value = r.intern("error saving char");
            host::print_stream_println(r, state_out_value, text_value)?;
            return Ok(());
        }

        return Ok(());
    }
    let canvas_record_store_3 = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    outcome = (|| -> JResult {
        Ok(V::I(host::record_store_get_num_records(
            r,
            canvas_record_store_3,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let text_value = r.intern("error saving char");
        host::print_stream_println(r, state_out_value, text_value)?;
        return Ok(());
    }
    let record_store_get_num_records_value = outcome.unwrap().i();
    if record_store_get_num_records_value != 0 {
        let canvas_record_store_4 = r.get(canvas, field::canvas::RECORD_STORE)?.r();
        let length_value = r.array_len(object_1)?;
        outcome = (|| -> JResult {
            host::record_store_set_record(r, canvas_record_store_4, 1, object_1, 0, length_value)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let text_value = r.intern("error saving char");
            host::print_stream_println(r, state_out_value, text_value)?;
            return Ok(());
        }

        let canvas_record_store_2_read = r.get(canvas, field::canvas::RECORD_STORE)?.r();
        outcome = (|| -> JResult {
            host::record_store_close_record_store(r, canvas_record_store_2_read)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value = r.static_get(field::system::STATE_OUT).r();
            let text_value = r.intern("error saving char");
            host::print_stream_println(r, state_out_value, text_value)?;
            return Ok(());
        }

        return Ok(());
    }
    let canvas_record_store_5 = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    let length_value_2 = r.array_len(object_1)?;
    outcome = (|| -> JResult {
        Ok(V::I(host::record_store_add_record(
            r,
            canvas_record_store_5,
            object_1,
            0,
            length_value_2,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let text_value = r.intern("error saving char");
        host::print_stream_println(r, state_out_value, text_value)?;
        return Ok(());
    }
    let record_store_add_record_value = outcome.unwrap().i();
    let canvas_record_store_2_read = r.get(canvas, field::canvas::RECORD_STORE)?.r();
    outcome = (|| -> JResult {
        host::record_store_close_record_store(r, canvas_record_store_2_read)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "javax/microedition/rms/RecordStoreException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value = r.static_get(field::system::STATE_OUT).r();
        let text_value = r.intern("error saving char");
        host::print_stream_println(r, state_out_value, text_value)?;
        return Ok(());
    }

    return Ok(());
}
