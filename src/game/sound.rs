//! Music selection, effect priority and volume.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize empty. Original reference: `b.<init>()V`.
pub(super) fn initialize_empty(r: &mut Runtime, sound: u32) -> Result<(), u32> {
    host::object_initialize(r, sound)?;
    r.set(sound, field::sound::ENABLED, V::I(1))?;
    r.set(sound, field::sound::PENDING_EFFECT, V::I(0))?;
    r.set(sound, field::sound::QUEUED_EFFECT, V::I(0))?;
    r.set(sound, field::sound::EFFECT_PRIORITY, V::I(0))?;
    r.set(sound, field::sound::EFFECT_STARTED_AT, V::J(0))?;
    r.set(sound, field::sound::STOP_REQUESTED, V::I(0))?;
    return Ok(());
}

/// Initialize. Original reference: `b.<init>(ZZIIII)V`.
pub(super) fn initialize(
    r: &mut Runtime,
    sound: u32,
    enabled: i32,
    use_mpeg: i32,
    zone: i32,
    map: i32,
    music_volume: i32,
    effect_volume: i32,
) -> Result<(), u32> {
    host::object_initialize(r, sound)?;
    r.set(sound, field::sound::ENABLED, V::I(1))?;
    r.set(sound, field::sound::PENDING_EFFECT, V::I(0))?;
    r.set(sound, field::sound::QUEUED_EFFECT, V::I(0))?;
    r.set(sound, field::sound::EFFECT_PRIORITY, V::I(0))?;
    r.set(sound, field::sound::EFFECT_STARTED_AT, V::J(0))?;
    r.set(sound, field::sound::STOP_REQUESTED, V::I(0))?;
    let sound_enabled = r.get(sound, field::sound::ENABLED)?.i();
    if sound_enabled == 0 {
        r.set(sound, field::sound::MUSIC_VOLUME, V::I(0))?;
        r.set(sound, field::sound::EFFECT_VOLUME, V::I(0))?;
        return Ok(());
    }
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let text_value = r.intern("Media constructor");
    host::print_stream_println(r, state_out_value, text_value)?;
    r.set(sound, field::sound::USE_MPEG, V::I(0))?;
    r.set(sound, field::sound::EFFECTS_READY, V::I(1))?;
    r.set(sound, field::sound::MUSIC_TRACK, V::I(-2))?;
    r.set(sound, field::sound::MUSIC_VOLUME, V::I(music_volume))?;
    r.set(sound, field::sound::EFFECT_VOLUME, V::I(effect_volume))?;
    let previous_sound_value = sound;

    let object_value = r.alloc("java/lang/Thread", Native::Empty);
    host::thread_initialize(r, object_value, sound)?;
    r.set(
        previous_sound_value,
        field::sound::WORKER,
        V::R(object_value),
    )?;
    r.set(sound, field::sound::STOP_REQUESTED, V::I(0))?;
    let sound_worker = r.get(sound, field::sound::WORKER)?.r();
    host::thread_start(r, sound_worker)?;
    let sound_worker_2 = r.get(sound, field::sound::WORKER)?.r();
    host::thread_set_priority(r, sound_worker_2, 1)?;
    return Ok(());
}

/// Prepare the cooperative sound worker once.
pub(crate) fn initialize_worker(r: &mut Runtime, sound: u32) -> Result<(), u32> {
    let ready = load_effects(r, sound)?;
    r.set(sound, field::sound::EFFECTS_READY, V::I(ready))?;
    r.set(sound, field::sound::MUSIC_PLAYER, V::R(0))?;
    r.set(sound, field::sound::MUSIC_TRACK, V::I(-10))?;
    select_map_music(r, sound, -1, 0)
}

/// Perform one finite step of the original sound worker on the native scheduler.
pub(crate) fn poll(r: &mut Runtime, sound: u32) -> Result<(), u32> {
    use field::sound::*;
    if r.get(sound, STOP_REQUESTED)?.i() != 0 {
        return Ok(());
    }
    let mut active = r.get(sound, EFFECT_PRIORITY)?.i();
    let began = r.get(sound, EFFECT_STARTED_AT)?.j() as u64;
    let effect_player = r.get(sound, EFFECT_PLAYER)?.r();
    let duration_ms = [0, 375, 450, 320, 500, 100, 1300, 2300, 500, 1100];
    if active > 0 && r.now.saturating_sub(began) > duration_ms[active as usize] {
        if effect_player != 0 {
            host::player_stop(r, effect_player)?;
        }
        active = 0;
        r.set(sound, EFFECT_PRIORITY, V::I(0))?;
    }
    let mut request = r.get(sound, PENDING_EFFECT)?.i();
    let queued = r.get(sound, QUEUED_EFFECT)?.i();
    if request == 0 && queued != 0 && active == 0 {
        request = queued;
        r.set(sound, PENDING_EFFECT, V::I(request))?;
        r.set(sound, QUEUED_EFFECT, V::I(0))?;
    }
    if request == 0
        || r.get(sound, ENABLED)?.i() == 0
        || r.get(sound, EFFECTS_READY)?.i() == 0
        || r.get(sound, EFFECT_VOLUME)?.i() == 0
    {
        return Ok(());
    }
    // Long effects have priority; the secret sound may queue behind one.
    if matches!(active, 6 | 7 | 9) {
        if queued == 0 && request == 7 && active != 9 {
            r.set(sound, QUEUED_EFFECT, V::I(request))?;
        }
        r.set(sound, PENDING_EFFECT, V::I(0))?;
        return Ok(());
    }
    if active != 0 && effect_player != 0 {
        host::player_stop(r, effect_player)?;
    }
    let (effect, offset_us) = match request {
        1 => match active {
            1 => (2, 585000),
            2 => (3, 1145000),
            _ => (1, 39000),
        },
        2 => (4, 1511000),
        3 => (5, 2057000),
        4 => (6, 2252000),
        5 => (7, 5000000),
        6 => (8, 7450000),
        7 => (9, 3760000),
        _ => {
            r.set(sound, PENDING_EFFECT, V::I(0))?;
            return Ok(());
        }
    };
    if effect_player != 0 {
        r.set(sound, EFFECT_PRIORITY, V::I(effect))?;
        r.set(sound, EFFECT_STARTED_AT, V::J(r.now as i64))?;
        host::player_set_media_time(r, effect_player, offset_us)?;
        host::player_start(r, effect_player)?;
    }
    r.set(sound, PENDING_EFFECT, V::I(0))?;
    Ok(())
}

/// Select map music. Original reference: `b.a(II)V`.
pub(super) fn select_map_music(
    r: &mut Runtime,
    sound: u32,
    zone: i32,
    map: i32,
) -> Result<(), u32> {
    let mut number_3: i32 = 0;
    let mut carry_0_i: i32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value)?;
    let text_value = r.intern("MapChange word=");
    let string_buffer_append_2_value = host::string_buffer_append_2(r, object_value, text_value)?;
    let string_buffer_append_value =
        host::string_buffer_append(r, string_buffer_append_2_value, zone)?;
    let text_value_2 = r.intern("Map =");
    let string_buffer_append_2_value_2 =
        host::string_buffer_append_2(r, string_buffer_append_value, text_value_2)?;
    let string_buffer_append_value_2 =
        host::string_buffer_append(r, string_buffer_append_2_value_2, map)?;
    let string_buffer_to_string_value =
        host::string_buffer_to_string(r, string_buffer_append_value_2)?;
    host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
    let sound_enabled = r.get(sound, field::sound::ENABLED)?.i();
    if sound_enabled == 0 {
        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    number_3 = 0;
    if (zone == 0) && (!((map != 58) && (map != 68))) {
        number_3 = 1;
    }

    if (zone == 9) && (map == 15) {
        number_3 = 1;
    }

    if (zone == 10) && (map == 17) {
        number_3 = 1;
    }

    if (zone == 11) && (map == 28) {
        number_3 = 1;
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::ZONE)?.i())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    let zone_value = outcome.unwrap().i();
    if zone_value == 10 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MAP)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let map_value = outcome.unwrap().i();
        if map_value == 17 {
            number_3 = 1;
        }
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::ZONE)?.i())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    let zone_value_2 = outcome.unwrap().i();
    if zone_value_2 == 11 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MAP)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let map_value_2 = outcome.unwrap().i();
        if map_value_2 == 28 {
            number_3 = 1;
        }
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::ZONE)?.i())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    let zone_value_3 = outcome.unwrap().i();
    if zone_value_3 == 9 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MAP)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let map_value_3 = outcome.unwrap().i();
        if map_value_3 == 15 {
            number_3 = 1;
        }
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::ZONE)?.i())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    let zone_value_4 = outcome.unwrap().i();
    if zone_value_4 == 0 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MAP)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let map_value_4 = outcome.unwrap().i();
        if map_value_4 == 58 {
            number_3 = 1;
        } else {
            outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MAP)?.i())) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let map_value_5 = outcome.unwrap().i();
            if map_value_5 == 68 {
                number_3 = 1;
            }
        }
    }

    if zone == 20 {
        number_3 = 1;
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MUSIC_TRACK)?.i())) })();
    (carry_0_i,) = (zone,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    let state_l_value = outcome.unwrap().i();
    if (carry_0_i == state_l_value) && (number_3 == 0) {
        outcome = (|| -> JResult {
            r.set(sound, field::sound::ZONE, V::I(zone))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(sound, field::sound::MAP, V::I(map))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }

        return Ok(());
    }
    outcome = (|| -> JResult {
        r.set(sound, field::sound::MUSIC_TRACK, V::I(zone))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }

    if zone == -1 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value = outcome.unwrap().i();
        if use_mpeg_value == 0 {
            let text_value_3 = r.intern("/Sound/midi/intro.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_3)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value = outcome.unwrap().i();
        } else {
            let text_value_4 = r.intern("/Sound/Mp3/xelda22m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_4)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_2 = outcome.unwrap().i();
        }
    }

    if zone == 0 {
        if (map != 58) && (map != 68) {
            outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let use_mpeg_value_4 = outcome.unwrap().i();
            if use_mpeg_value_4 == 0 {
                let text_value_10 = r.intern("/Sound/midi/xelda1.mid");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_10)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_8 = outcome.unwrap().i();
            } else {
                let text_value_11 = r.intern("/Sound/Mp3/xelda12m.mp3");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_11)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_9 = outcome.unwrap().i();
            }
        } else {
            let text_value_5 = r.intern("/Sound/midi/Z3 Fairy.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_5)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_3 = outcome.unwrap().i();
        }
    }

    if zone == 1 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_2 = outcome.unwrap().i();
        if use_mpeg_value_2 == 0 {
            let text_value_6 = r.intern("/Sound/midi/z3cave.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_6)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_4 = outcome.unwrap().i();
        } else {
            let text_value_7 = r.intern("/Sound/Mp3/xelda12m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_7)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_5 = outcome.unwrap().i();
        }
    }

    if zone == 3 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_3 = outcome.unwrap().i();
        if use_mpeg_value_3 == 0 {
            let text_value_8 = r.intern("/Sound/midi/Xelda2.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_8)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_6 = outcome.unwrap().i();
        } else {
            let text_value_9 = r.intern("/Sound/Mp3/xelda62m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_9)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_7 = outcome.unwrap().i();
        }
    }

    if zone == 4 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_5 = outcome.unwrap().i();
        if use_mpeg_value_5 == 0 {
            let text_value_12 = r.intern("/Sound/midi/Xelda2.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_12)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_10 = outcome.unwrap().i();
        } else {
            let text_value_13 = r.intern("/Sound/Mp3/xelda62m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_13)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_11 = outcome.unwrap().i();
        }
    }

    if zone == 5 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_6 = outcome.unwrap().i();
        if use_mpeg_value_6 == 0 {
            let text_value_14 = r.intern("/Sound/midi/Xelda2.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_14)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_12 = outcome.unwrap().i();
        } else {
            let text_value_15 = r.intern("/Sound/Mp3/xelda62m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_15)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_13 = outcome.unwrap().i();
        }
    }

    if zone == 6 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_7 = outcome.unwrap().i();
        if use_mpeg_value_7 == 0 {
            let text_value_16 = r.intern("/Sound/midi/Xelda2.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_16)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_14 = outcome.unwrap().i();
        } else {
            let text_value_17 = r.intern("/Sound/Mp3/xelda62m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_17)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_15 = outcome.unwrap().i();
        }
    }

    if zone == 7 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_8 = outcome.unwrap().i();
        if use_mpeg_value_8 == 0 {
            let text_value_18 = r.intern("/Sound/midi/Xelda2.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_18)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_16 = outcome.unwrap().i();
        } else {
            let text_value_19 = r.intern("/Sound/Mp3/xelda62m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_19)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_17 = outcome.unwrap().i();
        }
    }

    if zone == 8 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_9 = outcome.unwrap().i();
        if use_mpeg_value_9 == 0 {
            let text_value_20 = r.intern("/Sound/midi/Xelda2.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_20)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_18 = outcome.unwrap().i();
        } else {
            let text_value_21 = r.intern("/Sound/Mp3/xelda62m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_21)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_19 = outcome.unwrap().i();
        }
    }

    if zone == 9 {
        if map == 15 {
            let text_value_22 = r.intern("/Sound/midi/Z3-Boss.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_22)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_20 = outcome.unwrap().i();
        } else {
            outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let use_mpeg_value_10 = outcome.unwrap().i();
            if use_mpeg_value_10 == 0 {
                let text_value_24 = r.intern("/Sound/midi/Xelda2.mid");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_24)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_22 = outcome.unwrap().i();
            } else {
                let text_value_25 = r.intern("/Sound/Mp3/xelda62m.mp3");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_25)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_23 = outcome.unwrap().i();
            }
        }
    }

    if zone == 10 {
        if map == 17 {
            let text_value_23 = r.intern("/Sound/midi/Z3-Boss.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_23)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_21 = outcome.unwrap().i();
        } else {
            outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let use_mpeg_value_11 = outcome.unwrap().i();
            if use_mpeg_value_11 == 0 {
                let text_value_28 = r.intern("/Sound/midi/Xelda2.mid");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_28)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_26 = outcome.unwrap().i();
            } else {
                let text_value_29 = r.intern("/Sound/Mp3/xelda62m.mp3");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_29)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_27 = outcome.unwrap().i();
            }
        }
    }

    if zone == 11 {
        if map == 28 {
            let text_value_27 = r.intern("/Sound/midi/Z4 Mini Boss.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_27)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_25 = outcome.unwrap().i();
        } else {
            outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let use_mpeg_value_12 = outcome.unwrap().i();
            if use_mpeg_value_12 == 0 {
                let text_value_30 = r.intern("/Sound/midi/Z4 Level 3.mid");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_30)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_28 = outcome.unwrap().i();
            } else {
                let text_value_31 = r.intern("/Sound/Mp3/xelda62m.mp3");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_31)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_29 = outcome.unwrap().i();
            }
        }
    }

    if zone == 12 {
        let text_value_26 = r.intern("/Sound/midi/Z4 Mini Boss.mid");
        outcome =
            (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_26)?)) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let play_music_value_24 = outcome.unwrap().i();
    }

    if zone == 14 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_13 = outcome.unwrap().i();
        if use_mpeg_value_13 == 0 {
            let text_value_32 = r.intern("/Sound/midi/z3cave.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_32)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_30 = outcome.unwrap().i();
        } else {
            let text_value_33 = r.intern("/Sound/Mp3/xelda12m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_33)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_31 = outcome.unwrap().i();
        }
    }

    if zone == 15 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_14 = outcome.unwrap().i();
        if use_mpeg_value_14 == 0 {
            let text_value_34 = r.intern("/Sound/midi/z3cave.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_34)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_32 = outcome.unwrap().i();
        } else {
            let text_value_35 = r.intern("/Sound/Mp3/xelda12m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_35)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_33 = outcome.unwrap().i();
        }
    }

    if zone == 16 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_15 = outcome.unwrap().i();
        if use_mpeg_value_15 == 0 {
            let text_value_36 = r.intern("/Sound/midi/z3cave.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_36)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_34 = outcome.unwrap().i();
        } else {
            let text_value_37 = r.intern("/Sound/Mp3/xelda12m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_37)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_35 = outcome.unwrap().i();
        }
    }

    if zone == 17 {
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let use_mpeg_value_16 = outcome.unwrap().i();
        if use_mpeg_value_16 == 0 {
            let text_value_38 = r.intern("/Sound/midi/z3cave.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_38)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_36 = outcome.unwrap().i();
        } else {
            let text_value_39 = r.intern("/Sound/Mp3/xelda12m.mp3");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_39)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_37 = outcome.unwrap().i();
        }
    }

    if zone == 20 {
        if map != 1 {
            if map != 3 {
                let text_value_43 = r.intern("/Sound/midi/Z3 Final Battle.mid");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_43)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_41 = outcome.unwrap().i();
            } else {
                let text_value_44 = r.intern("/Sound/midi/Z4-Credits.mid");
                outcome = (|| -> JResult {
                    Ok(V::I(super::sound::play_music(r, sound, text_value_44)?))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                        return Err(outcome.unwrap_err());
                    }

                    r.set(sound, field::sound::ZONE, V::I(zone))?;
                    r.set(sound, field::sound::MAP, V::I(map))?;
                    return Ok(());
                }
                let play_music_value_42 = outcome.unwrap().i();
            }
        } else {
            let text_value_40 = r.intern("/Sound/midi/z3cave.mid");
            outcome =
                (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_40)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }

                r.set(sound, field::sound::ZONE, V::I(zone))?;
                r.set(sound, field::sound::MAP, V::I(map))?;
                return Ok(());
            }
            let play_music_value_38 = outcome.unwrap().i();
        }
    }

    if zone != 100 {
        outcome = (|| -> JResult {
            r.set(sound, field::sound::ZONE, V::I(zone))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(sound, field::sound::MAP, V::I(map))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }

        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    let use_mpeg_value_17 = outcome.unwrap().i();
    if use_mpeg_value_17 == 0 {
        let text_value_41 = r.intern("/Sound/midi/Z3 Lost Forest.mid");
        outcome =
            (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_41)?)) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        let play_music_value_39 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(sound, field::sound::ZONE, V::I(zone))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(sound, field::sound::MAP, V::I(map))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();

            r.set(sound, field::sound::ZONE, V::I(zone))?;
            r.set(sound, field::sound::MAP, V::I(map))?;
            return Ok(());
        }

        return Ok(());
    }
    let text_value_42 = r.intern("/Sound/Mp3/xelda12m.mp3");
    outcome = (|| -> JResult { Ok(V::I(super::sound::play_music(r, sound, text_value_42)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    let play_music_value_40 = outcome.unwrap().i();
    outcome = (|| -> JResult {
        r.set(sound, field::sound::ZONE, V::I(zone))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }
    outcome = (|| -> JResult {
        r.set(sound, field::sound::MAP, V::I(map))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();

        r.set(sound, field::sound::ZONE, V::I(zone))?;
        r.set(sound, field::sound::MAP, V::I(map))?;
        return Ok(());
    }

    return Ok(());
}

/// Set effect volume. Original reference: `b.a(I)Z`.
pub(super) fn set_effect_volume(r: &mut Runtime, mut sound: u32, volume: i32) -> Result<i32, u32> {
    let mut carry_0_r: u32 = 0;
    let mut carry_1_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value)?;
    let text_value = r.intern("SetFxVolume: ");
    let string_buffer_append_2_value = host::string_buffer_append_2(r, object_value, text_value)?;
    let string_buffer_append_value =
        host::string_buffer_append(r, string_buffer_append_2_value, volume)?;
    let string_buffer_to_string_value =
        host::string_buffer_to_string(r, string_buffer_append_value)?;
    host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
    let sound_enabled = r.get(sound, field::sound::ENABLED)?.i();
    if sound_enabled == 0 {
        return Ok(0);
    }
    outcome = (|| -> JResult {
        r.set(sound, field::sound::EFFECT_VOLUME, V::I(volume))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_PLAYER)?.r())) })();
    (carry_0_r,) = (sound,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    let effect_player_value = outcome.unwrap().r();
    let text_value_2 = r.intern("VolumeControl");
    outcome = (|| -> JResult {
        Ok(V::R(host::player_get_control(
            r,
            effect_player_value,
            text_value_2,
        )?))
    })();
    (carry_0_r,) = (carry_0_r,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    let player_get_control_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::check_cast(
            r,
            player_get_control_value,
            "javax/microedition/media/control/VolumeControl",
        )?;
        Ok(V::I(0))
    })();
    (carry_0_r, carry_1_r) = (carry_0_r, player_get_control_value);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    outcome = (|| -> JResult {
        r.set(carry_0_r, field::sound::EFFECT_CONTROL, V::R(carry_1_r))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_CONTROL)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    let effect_control_value = outcome.unwrap().r();
    if effect_control_value == 0 {
        return Ok(1);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_CONTROL)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    let effect_control_value_2 = outcome.unwrap().r();
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::EFFECT_VOLUME)?.i())) })();
    (carry_0_r,) = (effect_control_value_2,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    let effect_volume_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        Ok(V::I(host::volume_control_set_level(
            r,
            carry_0_r,
            effect_volume_value,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(0);
    }
    let volume_control_set_level_value = outcome.unwrap().i();

    return Ok(1);
}

/// Set music volume. Original reference: `b.b(I)V`.
pub(super) fn set_music_volume(r: &mut Runtime, mut sound: u32, volume: i32) -> Result<(), u32> {
    let mut carry_0_r: u32 = 0;
    let mut carry_1_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value)?;
    let text_value = r.intern("SetMusicVolume: ");
    let string_buffer_append_2_value = host::string_buffer_append_2(r, object_value, text_value)?;
    let string_buffer_append_value =
        host::string_buffer_append(r, string_buffer_append_2_value, volume)?;
    let string_buffer_to_string_value =
        host::string_buffer_to_string(r, string_buffer_append_value)?;
    host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
    let sound_enabled = r.get(sound, field::sound::ENABLED)?.i();
    if sound_enabled == 0 {
        return Ok(());
    }
    outcome = (|| -> JResult {
        r.set(sound, field::sound::MUSIC_CONTROL, V::R(0))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    (carry_0_r,) = (sound,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    let music_player_value = outcome.unwrap().r();
    let text_value_2 = r.intern("VolumeControl");
    outcome = (|| -> JResult {
        Ok(V::R(host::player_get_control(
            r,
            music_player_value,
            text_value_2,
        )?))
    })();
    (carry_0_r,) = (carry_0_r,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    let player_get_control_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::check_cast(
            r,
            player_get_control_value,
            "javax/microedition/media/control/VolumeControl",
        )?;
        Ok(V::I(0))
    })();
    (carry_0_r, carry_1_r) = (carry_0_r, player_get_control_value);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    outcome = (|| -> JResult {
        r.set(carry_0_r, field::sound::MUSIC_CONTROL, V::R(carry_1_r))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_CONTROL)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    let music_control_value = outcome.unwrap().r();
    if music_control_value != 0 {
        outcome = (|| -> JResult {
            r.set(sound, field::sound::MUSIC_VOLUME, V::I(volume))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_CONTROL)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let music_control_value_2 = outcome.unwrap().r();
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MUSIC_VOLUME)?.i())) })();
        (carry_0_r,) = (music_control_value_2,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let music_volume_value = outcome.unwrap().i();
        outcome = (|| -> JResult {
            Ok(V::I(host::volume_control_set_level(
                r,
                carry_0_r,
                music_volume_value,
            )?))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let volume_control_set_level_value = outcome.unwrap().i();
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MUSIC_VOLUME)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let music_volume_value_2 = outcome.unwrap().i();
        if music_volume_value_2 != 0 {
            outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                sound = carry_0_r;
                let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
                let exception_to_string_value = host::exception_to_string(r, sound)?;
                host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
                return Ok(());
            }
            let music_player_value_3 = outcome.unwrap().r();
            outcome = (|| -> JResult {
                host::player_start(r, music_player_value_3)?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }
                sound = carry_0_r;
                let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
                let exception_to_string_value = host::exception_to_string(r, sound)?;
                host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
                return Ok(());
            }

            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let music_player_value_4 = outcome.unwrap().r();
        outcome = (|| -> JResult { Ok(V::I(host::player_get_state(r, music_player_value_4)?)) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let player_get_state_value = outcome.unwrap().i();
        if player_get_state_value != 400 {
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let music_player_value_7 = outcome.unwrap().r();
        outcome = (|| -> JResult {
            host::player_stop(r, music_player_value_7)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }

        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    let music_player_value_2 = outcome.unwrap().r();
    if music_player_value_2 == 0 {
        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MUSIC_VOLUME)?.i())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    let music_volume_value_3 = outcome.unwrap().i();
    if music_volume_value_3 <= 0 {
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let music_player_value_5 = outcome.unwrap().r();
        outcome = (|| -> JResult { Ok(V::I(host::player_get_state(r, music_player_value_5)?)) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let player_get_state_value_2 = outcome.unwrap().i();
        if player_get_state_value_2 != 400 {
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }
        let music_player_value_8 = outcome.unwrap().r();
        outcome = (|| -> JResult {
            host::player_stop(r, music_player_value_8)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            sound = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, sound)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            return Ok(());
        }

        return Ok(());
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }
    let music_player_value_6 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_start(r, music_player_value_6)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        sound = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, sound)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        return Ok(());
    }

    return Ok(());
}

/// Play music. Original reference: `b.a(Ljava/lang/String;)Z`.
pub(super) fn play_music(r: &mut Runtime, sound: u32, mut path: u32) -> Result<i32, u32> {
    let mut carry_0_r: u32 = 0;
    let mut carry_1_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let text_value = r.intern("playFile");
    host::print_stream_println(r, state_out_value, text_value)?;
    let sound_enabled = r.get(sound, field::sound::ENABLED)?.i();
    if sound_enabled == 0 {
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_player_value = outcome.unwrap().r();
    if music_player_value != 0 {
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let music_player_value_2 = outcome.unwrap().r();
        outcome = (|| -> JResult { Ok(V::I(host::player_get_state(r, music_player_value_2)?)) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let player_get_state_value = outcome.unwrap().i();
        if player_get_state_value == 400 {
            outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                path = carry_0_r;
                let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
                let exception_to_string_value = host::exception_to_string(r, path)?;
                host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
                r.set(sound, field::sound::ENABLED, V::I(0))?;
                return Ok(0);
            }
            let music_player_value_5 = outcome.unwrap().r();
            outcome = (|| -> JResult {
                host::player_stop(r, music_player_value_5)?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                    return Err(outcome.unwrap_err());
                }
                path = carry_0_r;
                let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
                let exception_to_string_value = host::exception_to_string(r, path)?;
                host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
                r.set(sound, field::sound::ENABLED, V::I(0))?;
                return Ok(0);
            }
        }
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let music_player_value_3 = outcome.unwrap().r();
        outcome = (|| -> JResult {
            host::player_remove_player_listener(r, music_player_value_3, sound)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let music_player_value_4 = outcome.unwrap().r();
        outcome = (|| -> JResult {
            host::player_close(r, music_player_value_4)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        outcome = (|| -> JResult {
            r.set(sound, field::sound::MUSIC_PLAYER, V::R(0))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        host::system_gc(r)?;
    }
    outcome = (|| -> JResult { Ok(V::R(host::object_get_class(r, sound)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let object_get_class_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        Ok(V::R(host::class_get_resource_as_stream(
            r,
            object_get_class_value,
            path,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let class_get_resource_as_stream_value = outcome.unwrap().r();
    path = class_get_resource_as_stream_value;
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::USE_MPEG)?.i())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let use_mpeg_value = outcome.unwrap().i();
    if use_mpeg_value == 0 {
        let text_value_2 = r.intern("audio/midi");
        outcome =
            (|| -> JResult { Ok(V::R(host::manager_create_player(r, path, text_value_2)?)) })();
        (carry_0_r,) = (sound,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let manager_create_player_value = outcome.unwrap().r();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::sound::MUSIC_PLAYER,
                V::R(manager_create_player_value),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
    } else {
        let text_value_3 = r.intern("audio/mpeg");
        outcome =
            (|| -> JResult { Ok(V::R(host::manager_create_player(r, path, text_value_3)?)) })();
        (carry_0_r,) = (sound,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let manager_create_player_value_2 = outcome.unwrap().r();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::sound::MUSIC_PLAYER,
                V::R(manager_create_player_value_2),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_player_value_6 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_realize(r, music_player_value_6)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_player_value_7 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_add_player_listener(r, music_player_value_7, sound)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult {
        r.set(sound, field::sound::MUSIC_CONTROL, V::R(0))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    (carry_0_r,) = (sound,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_player_value_8 = outcome.unwrap().r();
    let text_value_4 = r.intern("VolumeControl");
    outcome = (|| -> JResult {
        Ok(V::R(host::player_get_control(
            r,
            music_player_value_8,
            text_value_4,
        )?))
    })();
    (carry_0_r,) = (carry_0_r,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let player_get_control_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::check_cast(
            r,
            player_get_control_value,
            "javax/microedition/media/control/VolumeControl",
        )?;
        Ok(V::I(0))
    })();
    (carry_0_r, carry_1_r) = (carry_0_r, player_get_control_value);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult {
        r.set(carry_0_r, field::sound::MUSIC_CONTROL, V::R(carry_1_r))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_CONTROL)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_control_value = outcome.unwrap().r();
    if music_control_value != 0 {
        outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_CONTROL)?.r())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let music_control_value_2 = outcome.unwrap().r();
        outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MUSIC_VOLUME)?.i())) })();
        (carry_0_r,) = (music_control_value_2,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let music_volume_value_2 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            Ok(V::I(host::volume_control_set_level(
                r,
                carry_0_r,
                music_volume_value_2,
            )?))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            path = carry_0_r;
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value = host::exception_to_string(r, path)?;
            host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let volume_control_set_level_value = outcome.unwrap().i();
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_player_value_9 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_prefetch(r, music_player_value_9)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_player_value_10 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_set_loop_count(r, music_player_value_10, -1)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::MUSIC_VOLUME)?.i())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_volume_value = outcome.unwrap().i();
    if music_volume_value <= 0 {
        return Ok(1);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::MUSIC_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let music_player_value_11 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_start(r, music_player_value_11)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        path = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value = host::exception_to_string(r, path)?;
        host::print_stream_println(r, state_out_value_2, exception_to_string_value)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }

    return Ok(1);
}

/// Player event. Original reference: `b.playerUpdate(Ljavax/microedition/media/Player;Ljava/lang/String;Ljava/lang/Object;)V`.
pub(super) fn player_event(
    r: &mut Runtime,
    sound: u32,
    player: u32,
    event: u32,
    data: u32,
) -> Result<(), u32> {
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let text_value = r.intern("playerUpdate");
    host::print_stream_println(r, state_out_value, text_value)?;
    let sound_enabled = r.get(sound, field::sound::ENABLED)?.i();
    if sound_enabled == 0 {
        return Ok(());
    }
    let text_value_2 = r.intern("started");
    if event != text_value_2 {
        let text_value_3 = r.intern("stopped");
        if event == text_value_3 {
            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
            let text_value_5 = r.intern("Player STOPPED");
            host::print_stream_println(r, state_out_value_2, text_value_5)?;
        }
        let text_value_4 = r.intern("volumeChanged");
        if event != text_value_4 {
            return Ok(());
        }
        let state_out_value_5 = r.static_get(field::system::STATE_OUT).r();
        let text_value_8 = r.intern("Player VOLUME_CHANGED");
        host::print_stream_println(r, state_out_value_5, text_value_8)?;
        return Ok(());
    }
    let sound_music_player = r.get(sound, field::sound::MUSIC_PLAYER)?.r();
    if player != sound_music_player {
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_6 = r.intern("sfx STARTED");
        host::print_stream_println(r, state_out_value_3, text_value_6)?;
    } else {
        let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
        let text_value_7 = r.intern("Player STARTED");
        host::print_stream_println(r, state_out_value_4, text_value_7)?;
    }
    let text_value_3 = r.intern("stopped");
    if event == text_value_3 {
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        let text_value_5 = r.intern("Player STOPPED");
        host::print_stream_println(r, state_out_value_2, text_value_5)?;
    }
    let text_value_4 = r.intern("volumeChanged");
    if event != text_value_4 {
        return Ok(());
    }
    let state_out_value_5 = r.static_get(field::system::STATE_OUT).r();
    let text_value_8 = r.intern("Player VOLUME_CHANGED");
    host::print_stream_println(r, state_out_value_5, text_value_8)?;
    return Ok(());
}

/// Load effects. Original reference: `b.a()Z`.
pub(super) fn load_effects(r: &mut Runtime, sound: u32) -> Result<i32, u32> {
    let mut object_1: u32 = 0;
    let mut object_2: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut carry_1_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    'check_sound_effect_player: {
        'next_step: {
            'next_step_2: {
                'handle_result_2: {
                    'handle_result_3: {
                        'handle_result: {
                            let state_out_value = r.static_get(field::system::STATE_OUT).r();
                            let text_value = r.intern("LoadSfxPlayers");
                            host::print_stream_println(r, state_out_value, text_value)?;
                            let sound_enabled = r.get(sound, field::sound::ENABLED)?.i();
                            if sound_enabled == 0 {
                                return Ok(0);
                            }
                            outcome =
                                (|| -> JResult { Ok(V::R(host::object_get_class(r, sound)?)) })();
                            if outcome.is_err() {
                                carry_0_r = outcome.unwrap_err();
                                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                                    return Err(outcome.unwrap_err());
                                }
                                object_1 = carry_0_r;
                                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                                let exception_to_string_value_2 =
                                    host::exception_to_string(r, object_1)?;
                                host::print_stream_println(
                                    r,
                                    state_out_value_3,
                                    exception_to_string_value_2,
                                )?;
                                r.set(sound, field::sound::ENABLED, V::I(0))?;
                                return Ok(0);
                            }
                            let object_get_class_value = outcome.unwrap().r();
                            let text_value_3 = r.intern("/Sound/Effects.wav");
                            outcome = (|| -> JResult {
                                Ok(V::R(host::class_get_resource_as_stream(
                                    r,
                                    object_get_class_value,
                                    text_value_3,
                                )?))
                            })();
                            if outcome.is_err() {
                                carry_0_r = outcome.unwrap_err();
                                if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                                    return Err(outcome.unwrap_err());
                                }
                                object_1 = carry_0_r;
                                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                                let exception_to_string_value_2 =
                                    host::exception_to_string(r, object_1)?;
                                host::print_stream_println(
                                    r,
                                    state_out_value_3,
                                    exception_to_string_value_2,
                                )?;
                                r.set(sound, field::sound::ENABLED, V::I(0))?;
                                return Ok(0);
                            }
                            let class_get_resource_as_stream_value = outcome.unwrap().r();
                            object_1 = class_get_resource_as_stream_value;
                            let text_value_5 = r.intern("audio/X-wav");
                            outcome = (|| -> JResult {
                                Ok(V::R(host::manager_create_player(
                                    r,
                                    object_1,
                                    text_value_5,
                                )?))
                            })();
                            (carry_0_r,) = (sound,);
                            if outcome.is_ok() {
                                break 'handle_result_3;
                            }
                        }
                        carry_0_r = outcome.unwrap_err();
                        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                            carry_0_r = outcome.unwrap_err();
                            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                                return Err(outcome.unwrap_err());
                            }
                            object_1 = carry_0_r;
                            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                            let exception_to_string_value_2 =
                                host::exception_to_string(r, object_1)?;
                            host::print_stream_println(
                                r,
                                state_out_value_3,
                                exception_to_string_value_2,
                            )?;
                            r.set(sound, field::sound::ENABLED, V::I(0))?;
                            return Ok(0);
                        }
                        break 'next_step;
                    }
                    let manager_create_player_value_2 = outcome.unwrap().r();
                    outcome = (|| -> JResult {
                        r.set(
                            carry_0_r,
                            field::sound::EFFECT_PLAYER,
                            V::R(manager_create_player_value_2),
                        )?;
                        Ok(V::I(0))
                    })();
                    if outcome.is_ok() {
                        break 'next_step_2;
                    }
                }
                carry_0_r = outcome.unwrap_err();
                break 'next_step;
            }

            break 'check_sound_effect_player;
        }
        object_2 = carry_0_r;
        let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
        outcome = (|| -> JResult { Ok(V::R(host::exception_to_string(r, object_2)?)) })();
        (carry_0_r,) = (state_out_value_2,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            object_1 = carry_0_r;
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
            host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let exception_to_string_value = outcome.unwrap().r();
        outcome = (|| -> JResult {
            host::print_stream_println(r, carry_0_r, exception_to_string_value)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            object_1 = carry_0_r;
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
            host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let text_value_2 = r.intern("audio/wav");
        outcome = (|| -> JResult {
            Ok(V::R(host::manager_create_player(
                r,
                object_1,
                text_value_2,
            )?))
        })();
        (carry_0_r,) = (sound,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            object_1 = carry_0_r;
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
            host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
        let manager_create_player_value = outcome.unwrap().r();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::sound::EFFECT_PLAYER,
                V::R(manager_create_player_value),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            object_1 = carry_0_r;
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
            host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
            r.set(sound, field::sound::ENABLED, V::I(0))?;
            return Ok(0);
        }
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let effect_player_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_realize(r, effect_player_value)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let effect_player_value_2 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_add_player_listener(r, effect_player_value_2, sound)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let effect_player_value_3 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_set_loop_count(r, effect_player_value_3, 1)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_PLAYER)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let effect_player_value_4 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::player_prefetch(r, effect_player_value_4)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_PLAYER)?.r())) })();
    (carry_0_r,) = (sound,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let effect_player_value_5 = outcome.unwrap().r();
    let text_value_4 = r.intern("VolumeControl");
    outcome = (|| -> JResult {
        Ok(V::R(host::player_get_control(
            r,
            effect_player_value_5,
            text_value_4,
        )?))
    })();
    (carry_0_r,) = (carry_0_r,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let player_get_control_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::check_cast(
            r,
            player_get_control_value,
            "javax/microedition/media/control/VolumeControl",
        )?;
        Ok(V::I(0))
    })();
    (carry_0_r, carry_1_r) = (carry_0_r, player_get_control_value);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult {
        r.set(carry_0_r, field::sound::EFFECT_CONTROL, V::R(carry_1_r))?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_CONTROL)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let effect_control_value = outcome.unwrap().r();
    if effect_control_value == 0 {
        return Ok(1);
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(sound, field::sound::EFFECT_CONTROL)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let effect_control_value_2 = outcome.unwrap().r();
    outcome = (|| -> JResult { Ok(V::I(r.get(sound, field::sound::EFFECT_VOLUME)?.i())) })();
    (carry_0_r,) = (effect_control_value_2,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let effect_volume_value = outcome.unwrap().i();
    outcome = (|| -> JResult {
        Ok(V::I(host::volume_control_set_level(
            r,
            carry_0_r,
            effect_volume_value,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
            return Err(outcome.unwrap_err());
        }
        object_1 = carry_0_r;
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let exception_to_string_value_2 = host::exception_to_string(r, object_1)?;
        host::print_stream_println(r, state_out_value_3, exception_to_string_value_2)?;
        r.set(sound, field::sound::ENABLED, V::I(0))?;
        return Ok(0);
    }
    let volume_control_set_level_value = outcome.unwrap().i();

    return Ok(1);
}
