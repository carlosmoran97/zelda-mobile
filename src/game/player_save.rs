//! The original 172-byte player save format, maintained directly in Rust.
//!
//! Words are big endian, flags are single bytes, and room events are bitsets.
//! Keep this order compatible with existing phone and desktop saves.
use super::{
    fields::{drawable, player as field},
    player::count_triforce_pieces,
};
use crate::runtime::{Runtime, V};

const POSITION: [usize; 2] = [drawable::X, drawable::Y];
const EQUIPMENT: [usize; 6] = [
    field::HAS_SWORD,
    field::HAS_BOOMERANG,
    field::HAS_BOMBS,
    field::HAS_BOW,
    field::HAS_CANDLE,
    field::HAS_POTION,
];
const STATS: [usize; 7] = [
    field::HEALTH,
    field::MAX_HEALTH,
    field::RUPEES,
    field::WALLET_CAPACITY,
    field::KEYS,
    field::ZONE,
    field::MAP,
];
const AMMUNITION_AND_UPGRADES: [usize; 6] = [
    field::BOMBS,
    field::BOMB_CAPACITY,
    field::ARROWS,
    field::ARROW_CAPACITY,
    field::SWORD_LEVEL,
    field::TUNIC_COLOR,
];
const PICKUPS: [usize; 2] = [field::HEART_PICKUPS, field::KEY_PICKUPS];
const ROOM_EVENTS: [(usize, i32); 11] = [
    (field::OVERWORLD_EVENTS, 4),
    (field::CAVE_EVENTS, 4),
    (field::DUNGEON_1_EVENTS, 2),
    (field::DUNGEON_2_EVENTS, 2),
    (field::DUNGEON_3_EVENTS, 2),
    (field::DUNGEON_4_EVENTS, 2),
    (field::DUNGEON_5_EVENTS, 2),
    (field::DUNGEON_6_EVENTS, 2),
    (field::DUNGEON_7_EVENTS, 2),
    (field::DUNGEON_8_EVENTS, 2),
    (field::DUNGEON_9_EVENTS, 2),
];
const TRAILING_FLAGS: [usize; 2] = [field::RUPEE_PICKUPS, field::WORLD_FLAGS];

pub(super) fn encode_save(r: &mut Runtime, player: u32) -> Result<u32, u32> {
    let result = (|| -> Result<Vec<u8>, u32> {
        let mut bytes = Vec::with_capacity(172);
        for key in POSITION {
            write_word(&mut bytes, r.get(player, key)?.i());
        }
        for key in EQUIPMENT {
            bytes.push(u8::from(r.get(player, key)?.i() != 0));
        }
        for key in STATS {
            write_word(&mut bytes, r.get(player, key)?.i());
        }
        for key in AMMUNITION_AND_UPGRADES {
            bytes.push(r.get(player, key)?.i() as u8);
        }
        for key in [field::PROGRESSION_FLAGS].into_iter().chain(PICKUPS) {
            write_word(&mut bytes, r.get(player, key)?.i());
        }
        for (key, count) in ROOM_EVENTS {
            let events = r.get(player, key)?.r();
            for index in 0..count {
                write_word(&mut bytes, r.array_get(events, index)?.i());
            }
        }
        for key in TRAILING_FLAGS {
            write_word(&mut bytes, r.get(player, key)?.i());
        }
        Ok(bytes)
    })();
    match result {
        Ok(bytes) => Ok(r.byte_array(&bytes)),
        // The original encoder returns null if a record cannot be encoded.
        Err(error) if r.is_instance(error, "java/lang/Exception") => Ok(0),
        Err(error) => Err(error),
    }
}

pub(super) fn load_save(
    r: &mut Runtime,
    player: u32,
    bytes: u32,
    new_game: i32,
    canvas: u32,
) -> Result<(), u32> {
    r.set(player, field::CANVAS, V::R(canvas))?;
    if new_game != 0 {
        return Ok(());
    }
    let bytes = r.bytes(bytes);
    let mut input = SaveReader {
        bytes: &bytes,
        position: 0,
    };
    let result = (|| -> Result<(), u32> {
        for key in POSITION {
            let value = input.word(r)?;
            r.set(player, key, V::I(value))?;
        }
        for key in EQUIPMENT {
            let value = input.boolean(r)?;
            r.set(player, key, V::I(value))?;
        }
        for key in STATS {
            let value = input.word(r)?;
            r.set(player, key, V::I(value))?;
        }
        for key in AMMUNITION_AND_UPGRADES {
            r.set(player, key, V::I(input.signed_byte()))?;
        }
        let flags = input.word(r)?;
        r.set(player, field::PROGRESSION_FLAGS, V::I(flags))?;
        restore_upgrades(r, player, flags)?;
        for key in PICKUPS {
            let value = input.word(r)?;
            r.set(player, key, V::I(value))?;
        }
        for (key, count) in ROOM_EVENTS {
            let events = r.get(player, key)?.r();
            for index in 0..count {
                let value = input.word(r)?;
                r.array_set(events, index, V::I(value))?;
            }
        }
        let pieces = count_triforce_pieces(r, player)?;
        r.set(player, field::TRIFORCE_COUNT, V::I(pieces))?;
        for key in TRAILING_FLAGS {
            let value = input.word(r)?;
            r.set(player, key, V::I(value))?;
        }
        // Refresh the appearance after sword and tunic upgrades are restored.
        r.set(player, field::SPRITE, V::R(0))?;
        r.set(player, field::EQUIPMENT_SPRITE, V::R(0))?;
        Ok(())
    })();
    match result {
        Ok(()) => Ok(()),
        // Preserve the beta's partial-load behavior for damaged records.
        Err(error) if r.is_instance(error, "java/lang/Exception") => Ok(()),
        Err(error) => Err(error),
    }
}

fn restore_upgrades(r: &mut Runtime, player: u32, flags: i32) -> Result<(), u32> {
    r.set(player, field::SWORD_LEVEL, V::I(0))?;
    r.set(player, field::TUNIC_COLOR, V::I(0))?;
    for (mask, key) in [
        (0x100, field::HAS_RAFT),
        (0x1000, field::HAS_HOOKSHOT),
        (0x20000, field::HAS_MIRROR_SHIELD),
        (0x80000, field::HAS_SHOVEL),
        (0x10000000, field::HAS_FISH),
    ] {
        if flags & mask > 0 {
            r.set(player, key, V::I(1))?;
        }
    }
    for (mask, level, damage) in [(0x2000, 1, 2), (0x10000, 2, 4)] {
        if flags & mask > 0 {
            r.set(player, field::SWORD_LEVEL, V::I(level))?;
            r.set(player, field::SWORD_DAMAGE, V::I(damage))?;
        }
    }
    for (mask, color) in [(0x40, 1), (0x8000000, 2)] {
        if flags & mask > 0 {
            r.set(player, field::TUNIC_COLOR, V::I(color))?;
        }
    }
    Ok(())
}

fn write_word(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}
struct SaveReader<'a> {
    bytes: &'a [u8],
    position: usize,
}
impl SaveReader<'_> {
    fn word(&mut self, r: &mut Runtime) -> Result<i32, u32> {
        let word = self
            .bytes
            .get(self.position..self.position + 4)
            .ok_or_else(|| r.exception("java/io/EOFException", "truncated player save"))?;
        self.position += 4;
        Ok(i32::from_be_bytes(word.try_into().unwrap()))
    }
    fn boolean(&mut self, r: &mut Runtime) -> Result<i32, u32> {
        let value = *self
            .bytes
            .get(self.position)
            .ok_or_else(|| r.exception("java/io/EOFException", "truncated player save"))?;
        self.position += 1;
        Ok(i32::from(value != 0))
    }
    fn signed_byte(&mut self) -> i32 {
        match self.bytes.get(self.position) {
            Some(byte) => {
                self.position += 1;
                *byte as i8 as i32
            }
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Native;
    use std::path::PathBuf;

    fn player() -> (Runtime, u32) {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut r =
            Runtime::new(root.join("assets"), root.join("target/player-save-tests")).unwrap();
        let player = r.alloc("h", Native::Empty);
        super::super::player::initialize(&mut r, player).unwrap();
        (r, player)
    }

    #[test]
    fn default_player_matches_original_wire_layout() {
        let (mut r, player) = player();
        let encoded = encode_save(&mut r, player).unwrap();
        let mut expected = [0_u8; 172];
        expected[0..4].copy_from_slice(&75_i32.to_be_bytes());
        expected[4..8].copy_from_slice(&85_i32.to_be_bytes());
        expected[14..18].copy_from_slice(&12_i32.to_be_bytes());
        expected[18..22].copy_from_slice(&12_i32.to_be_bytes());
        expected[26..30].copy_from_slice(&99_i32.to_be_bytes());
        expected[38..42].copy_from_slice(&120_i32.to_be_bytes());
        assert_eq!(r.bytes(encoded), expected);
    }

    #[test]
    fn upgrades_and_all_triforce_pieces_survive_loading() {
        let (mut r, player) = player();
        let flags = 0x100
            | 0x1000
            | 0x20000
            | 0x80000
            | 0x10000000
            | 0x2000
            | 0x10000
            | 0x40
            | 0x8000000
            | 0x200
            | 0x400
            | 0x800
            | 0x4000
            | 0x8000
            | 0x40000
            | 0x400000
            | 0x800000;
        r.set(player, field::PROGRESSION_FLAGS, V::I(flags))
            .unwrap();
        r.set(player, field::SWORD_LEVEL, V::I(2)).unwrap();
        r.set(player, field::TUNIC_COLOR, V::I(2)).unwrap();
        r.set(player, drawable::X, V::I(-12345)).unwrap();
        let events = r.get(player, field::OVERWORLD_EVENTS).unwrap().r();
        r.array_set(events, 0, V::I(0x40)).unwrap();
        r.array_set(events, 1, V::I(i32::MIN)).unwrap();
        let encoded = encode_save(&mut r, player).unwrap();
        let restored = r.alloc("h", Native::Empty);
        super::super::player::initialize(&mut r, restored).unwrap();
        load_save(&mut r, restored, encoded, 0, 0).unwrap();
        assert_eq!(r.get(restored, field::SWORD_DAMAGE).unwrap().i(), 4);
        assert_eq!(r.get(restored, field::TRIFORCE_COUNT).unwrap().i(), 8);
        for key in [
            field::HAS_RAFT,
            field::HAS_HOOKSHOT,
            field::HAS_MIRROR_SHIELD,
            field::HAS_SHOVEL,
            field::HAS_FISH,
        ] {
            assert_eq!(r.get(restored, key).unwrap().i(), 1);
        }
        let reencoded = encode_save(&mut r, restored).unwrap();
        assert_eq!(r.bytes(encoded), r.bytes(reencoded));
    }

    #[test]
    fn truncated_save_keeps_the_original_partial_load_behavior() {
        let (mut r, player) = player();
        let mut bytes = Vec::from((-77_i32).to_be_bytes());
        bytes.extend_from_slice(&123_i32.to_be_bytes());
        let encoded = r.byte_array(&bytes);
        load_save(&mut r, player, encoded, 0, 0).unwrap();
        assert_eq!(r.get(player, drawable::X).unwrap().i(), -77);
        assert_eq!(r.get(player, drawable::Y).unwrap().i(), 123);
        assert_eq!(r.get(player, field::HEALTH).unwrap().i(), 12);
    }
}
