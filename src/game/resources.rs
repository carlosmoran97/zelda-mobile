//! Sprite catalogs and their room-based cache lifetimes.
//!
//! Related IDs deliberately share a sprite sheet. Aging happens on room changes;
//! each family can keep an image alive for three further visits.
use super::{fields as field, host};
use crate::runtime::{Runtime, V};
use field::resources::*;

#[derive(Clone, Copy)]
struct Catalog {
    images: usize,
    ages: usize,
    used: usize,
    capacity: i32,
}
const ITEMS: Catalog = Catalog {
    images: ITEM_IMAGES,
    ages: ITEM_AGES,
    used: ITEM_USED,
    capacity: 256,
};
const OBJECTS: Catalog = Catalog {
    images: OBJECT_IMAGES,
    ages: OBJECT_AGES,
    used: OBJECT_USED,
    capacity: 256,
};
const ENEMIES: Catalog = Catalog {
    images: ENEMY_IMAGES,
    ages: ENEMY_AGES,
    used: ENEMY_USED,
    capacity: 256,
};
const PROJECTILES: Catalog = Catalog {
    images: PROJECTILE_IMAGES,
    ages: PROJECTILE_AGES,
    used: PROJECTILE_USED,
    capacity: 16,
};

pub(super) fn initialize(r: &mut Runtime, resources: u32, world: u32) -> Result<(), u32> {
    for catalog in [ITEMS, OBJECTS, ENEMIES, PROJECTILES] {
        for (field, array_kind) in [(catalog.images, 0), (catalog.ages, 10), (catalog.used, 4)] {
            let array = r.new_array(array_kind, catalog.capacity)?;
            r.set(resources, field, V::R(array))?;
        }
    }
    r.set(resources, WORLD, V::R(world))?;
    Ok(())
}

fn age(r: &mut Runtime, resources: u32, catalog: Catalog) -> Result<(), u32> {
    let images = r.get(resources, catalog.images)?.r();
    let ages = r.get(resources, catalog.ages)?.r();
    let used = r.get(resources, catalog.used)?.r();
    for index in 0..catalog.capacity {
        if r.array_get(images, index)?.r() != 0 {
            let remaining = r.array_get(ages, index)?.i().wrapping_sub(1);
            r.array_set(ages, index, V::I(remaining.max(0)))?;
            if remaining <= 0 {
                r.array_set(images, index, V::R(0))?;
            }
        }
        r.array_set(used, index, V::I(0))?;
    }
    Ok(())
}

pub(super) fn age_items(r: &mut Runtime, resources: u32) -> Result<(), u32> {
    age(r, resources, ITEMS)
}
pub(super) fn age_objects(r: &mut Runtime, resources: u32) -> Result<(), u32> {
    age(r, resources, OBJECTS)
}
pub(super) fn age_enemies(r: &mut Runtime, resources: u32) -> Result<(), u32> {
    age(r, resources, ENEMIES)
}
pub(super) fn age_projectiles(r: &mut Runtime, resources: u32) -> Result<(), u32> {
    age(r, resources, PROJECTILES)
}

fn image(
    r: &mut Runtime,
    resources: u32,
    catalog: Catalog,
    slot: i32,
    usage_slot: i32,
    path: &str,
) -> Result<u32, u32> {
    let ages = r.get(resources, catalog.ages)?.r();
    let used = r.get(resources, catalog.used)?.r();
    if r.array_get(used, usage_slot)?.i() == 0 {
        let remaining = r.array_get(ages, usage_slot)?.i().wrapping_add(3);
        r.array_set(ages, usage_slot, V::I(remaining))?;
    }
    r.array_set(used, usage_slot, V::I(1))?;
    let images = r.get(resources, catalog.images)?.r();
    let mut sprite = r.array_get(images, slot)?.r();
    if sprite == 0 {
        // Variant choices apply when loading a sheet, as in the original cache.
        let path = match path {
            "@ghost" => {
                if classic_ghosts(r, resources)? {
                    "/Images/Ghost2.png"
                } else {
                    "/Images/Ghost2-2.png"
                }
            }
            "@boss5" => {
                if classic_ghosts(r, resources)? {
                    "/Images/Boss5-2.png"
                } else {
                    "/Images/Boss5-3.png"
                }
            }
            "@tingle" => {
                if red_outfit(r, resources)? {
                    "/Images/GreenTingle.png"
                } else {
                    "/Images/RedTingle.png"
                }
            }
            path => path,
        };
        let resource = r.intern(path);
        sprite = match host::image_create_image(r, resource) {
            Ok(sprite) => sprite,
            Err(error) if r.is_instance(error, "java/io/IOException") => return Ok(0),
            Err(error) => return Err(error),
        };
        r.array_set(images, slot, V::R(sprite))?;
    }
    Ok(sprite)
}

pub(super) fn item_image(r: &mut Runtime, resources: u32, kind: i32) -> Result<u32, u32> {
    let (slot, usage_slot, path) = match kind {
        0..=3 => (0, 0, "/Images/item_Rupies.png"),
        4 => (kind, kind, "/Images/Bomb.png"),
        5..=7 => (5, 5, "/Images/Item5-7.png"),
        128..=130 => (128, 128, "/Images/Item128-131.png"),
        131..=133 => (131, 131, "/Images/Item131-133.png"),
        134..=136 => (134, 134, "/Images/Item134-136.png"),
        137..=139 => (137, 137, "/Images/Item137-139.png"),
        140..=142 => (140, 140, "/Images/Item140-142.png"),
        143 | 153 => (143, 143, "/Images/Item153-155.png"),
        144 => (144, 144, "/Images/Triforce2.png"),
        145 => (145, 145, "/Images/Mastersword.png"),
        146 => (146, kind, "/Images/Item131-133.png"),
        147..=149 => (147, 147, "/Images/Rupies2.png"),
        150..=152 => (150, 150, "/Images/Item150-152.png"),
        154 => (154, kind, "/Images/BigKey.png"),
        155 => (155, kind, "/Images/Outfit.png"),
        _ => return Ok(0),
    };
    image(r, resources, ITEMS, slot, usage_slot, path)
}

pub(super) fn projectile_image(r: &mut Runtime, resources: u32, kind: i32) -> Result<u32, u32> {
    let (slot, usage_slot, path) = match kind {
        0 => (kind, kind, "/Images/ProjStone.png"),
        1 => (kind, kind, "/Images/ProjBall.png"),
        2 => (kind, kind, "/Images/Bomerang1.png"),
        3 => (kind, kind, "/Images/Arrows.png"),
        4 => (kind, kind, "/Images/HookShot.png"),
        5 => (kind, kind, "/Images/WandShot01.png"),
        6 => (kind, kind, "/Images/Smoke01.png"),
        7 => (kind, kind, "/Images/Shot7.png"),
        8 => (kind, kind, "/Images/Boomerang01.png"),
        9 => (kind, kind, "/Images/ProjFire.png"),
        10 => (kind, kind, "/Images/ProjIce.png"),
        11 => (kind, kind, "/Images/LightArrow.png"),
        _ => return Ok(0),
    };
    image(r, resources, PROJECTILES, slot, usage_slot, path)
}

pub(super) fn object_image(r: &mut Runtime, resources: u32, kind: i32) -> Result<u32, u32> {
    let (slot, usage_slot, path) = match kind {
        3 => (kind, kind, "/Images/Bush.png"),
        4 => (kind, kind, "/Images/TealBush.png"),
        5 => (kind, kind, "/Images/const1.png"),
        6 => (kind, kind, "/Images/GreenFlowers.png"),
        7 => (kind, kind, "/Images/TealFlowers.png"),
        8 => (kind, kind, "/Images/Candle1.png"),
        9 => (kind, kind, "/Images/Waterfall02.png"),
        10 => (kind, kind, "/Images/Waterfall01.png"),
        11 => (kind, kind, "/Images/Candle2.png"),
        12 => (kind, kind, "/Images/Gravestone.png"),
        13..=14 => (13, 13, "/Images/Fence.png"),
        15 => (kind, kind, "/Images/BrownBush.png"),
        16 => (kind, kind, "/Images/Sign.png"),
        19 => (kind, kind, "/Images/HappyFlowers.png"),
        20 => (kind, kind, "/Images/HookTarget.png"),
        21..=24 => (21, 21, "/Images/Brygga.png"),
        25 => (kind, kind, "/Images/GreenWaterTest01.png"),
        26..=27 => (26, kind, "/Images/Bridge.png"),
        28 => (kind, kind, "/Images/WaterTest01.png"),
        29 => (kind, kind, "/Images/Pedestal.png"),
        30 => (kind, kind, "/Images/Globes.png"),
        32 => (kind, kind, "/Images/Stone.png"),
        34..=42 => (34, 34, "/Images/Net.png"),
        43..=45 => (43, 43, "/Images/Stairs.png"),
        46 => (kind, kind, "/Images/Stone.png"),
        47 => (kind, kind, "/Images/Warp.png"),
        48 => (kind, kind, "/Images/Trigger01.png"),
        52..=59 => (52, 52, "/Images/Claws.png"),
        60 => (kind, kind, "/Images/WallEye.png"),
        61 => (kind, kind, "/Images/Monster10.png"),
        62 => (kind, kind, "/Images/Gravestone2.png"),
        63 => (kind, kind, "/Images/Ball.png"),
        64..=67 => (64, 64, "/Images/Dungeon Doors.png"),
        68..=71 => (68, 68, "/Images/Dungeon Doors 2.png"),
        72..=75 => (72, 72, "/Images/Wall.png"),
        104..=107 => (104, 104, "/Images/Wall2.png"),
        108..=111 => (108, 108, "/Images/BehindTree02.png"),
        120..=123 => (120, 120, "/Images/Wall3.png"),
        76 => (kind, kind, "/Images/BombWall.png"),
        77 => (kind, kind, "/Images/BossDoor01.png"),
        78 => (kind, kind, "/Images/BossDoor02.png"),
        79 => (kind, kind, "/Images/Lightning05.png"),
        96..=99 => (96, 96, "/Images/Dungeon Doors 3.png"),
        100..=103 => (100, 100, "/Images/Dungeon Doors 4.png"),
        112..=115 => (112, 112, "/Images/Dungeon Doors 5.png"),
        116..=119 => (116, 116, "/Images/Dungeon Doors 6.png"),
        124..=127 => (124, 124, "/Images/FairyStatue.png"),
        128..=131 => (128, 128, "/Images/Dungeon Doors 7.png"),
        132..=135 => (132, 132, "/Images/Dungeon Doors 8.png"),
        136..=139 => (136, 136, "/Images/Wall4.png"),
        140..=143 => (140, 140, "/Images/Statue02.png"),
        144..=147 => (144, 144, "/Images/Dungeon Doors 10.png"),
        148..=151 => (148, 148, "/Images/Dungeon Doors 9.png"),
        152 => (152, 152, "/Images/Lightning01.png"),
        153 => (153, 153, "/Images/Lightning02.png"),
        154 => (154, 154, "/Images/Lightning03.png"),
        155 => (155, 155, "/Images/Lightning04.png"),
        80..=83 => (80, 80, "/Images/GreenBombTree.png"),
        84..=87 => (84, 84, "/Images/TealBombTree.png"),
        88..=91 => (88, 88, "/Images/BehindTree01.png"),
        92 => (92, 92, "/Images/PalmTree.png"),
        160..=168 => (160, 160, "/Images/Carpet2.png"),
        169..=170 => (169, 169, "/Images/Pillar.png"),
        171..=174 => (171, 171, "/Images/Statue.png"),
        176..=184 => (176, 176, "/Images/Carpet1.png"),
        185 => (185, 185, "/Images/Spikes.png"),
        186..=187 => (186, 186, "/Images/FloorThingy.png"),
        188..=191 => (188, 188, "/Images/WallLamps.png"),
        192..=204 => (192, 192, "/Images/AnimatedWater01.png"),
        205 | 206 | 207 | 221 => (205, 205, "/Images/Stairs01.png"),
        208..=211 => (208, 208, "/Images/Waterfall02.png"),
        212..=215 => (212, 212, "/Images/Waterfall01.png"),
        216..=220 => (216, 216, "/Images/FloorObjects.png"),
        222 | 223 | 237 | 238 => (222, 222, "/Images/BigDoor01.png"),
        224..=228 => (224, 224, "/Images/HouseTop.png"),
        229..=232 => (229, 229, "/Images/BehindTree03.png"),
        233..=236 => (233, 233, "/Images/AboveDoor01.png"),
        240..=255 => (240, 240, "/Images/Entrance.png"),
        _ => return Ok(0),
    };
    image(r, resources, OBJECTS, slot, usage_slot, path)
}

pub(super) fn enemy_image(r: &mut Runtime, resources: u32, kind: i32) -> Result<u32, u32> {
    let (slot, usage_slot, path) = match kind {
        0 => (kind, kind, "/Images/Octo2.png"),
        1 => (kind, kind, "/Images/Fire.png"),
        2 => (kind, kind, "/Images/Monster3.png"),
        3 => (kind, kind, "/Images/Monster4.png"),
        4 => (kind, kind, "/Images/Monster5.png"),
        5 => (kind, kind, "/Images/Monster6.png"),
        6 => (kind, kind, "/Images/Monster7.png"),
        7 => (kind, kind, "/Images/Monster8.png"),
        8 => (kind, kind, "/Images/Moldorm.png"),
        9 => (kind, kind, "/Images/Monster10.png"),
        10 => (kind, kind, "/Images/Monster11.png"),
        11 => (kind, kind, "/Images/Monster12.png"),
        12 => (kind, kind, "/Images/Monster13.png"),
        13 => (kind, kind, "/Images/Monster16.png"),
        14 => (kind, kind, "@ghost"),
        15 => (kind, kind, "/Images/Monster19.png"),
        16 => (kind, kind, "/Images/Boss1.png"),
        17 => (kind, kind, "/Images/Boss2-2.png"),
        18 => (kind, kind, "/Images/Boss3.png"),
        19 => (kind, kind, "/Images/Monster18.png"),
        20 => (kind, kind, "/Images/Boss4-2.png"),
        21 => (kind, kind, "@boss5"),
        22 => (kind, kind, "/Images/Boss6.png"),
        23 => (kind, kind, "/Images/Ganondorf.png"),
        24 => (kind, kind, "/Images/Ganon.png"),
        25 => (kind, kind, "/Images/BigOcto.png"),
        26 => (kind, kind, "/Images/waterfly.png"),
        27 => (kind, kind, "/Images/Knight2.png"),
        28 => (kind, kind, "/Images/wormbabies.png"),
        29 => (kind, kind, "/Images/WormBoss.png"),
        30 => (kind, kind, "/Images/Komune.png"),
        31 => (kind, kind, "/Images/Kotakune.png"),
        32 => (kind, kind, "/Images/Monster32.png"),
        33 => (kind, kind, "/Images/Monster33.png"),
        34 => (kind, kind, "/Images/Fairy.png"),
        35 => (kind, kind, "/Images/Moblin01.png"),
        36 => (kind, kind, "/Images/GreenFlames.png"),
        37 => (kind, kind, "/Images/Digdogger.png"),
        38 => (kind, kind, "/Images/Gooma.png"),
        128..=130 => (128, 128, "/Images/Monster128-130.png"),
        131 => (kind, kind, "/Images/Monster13.png"),
        132 => (kind, kind, "/Images/Fisherman01.png"),
        133 => (kind, kind, "/Images/Zelda01.png"),
        134 => (kind, kind, "/Images/ZeldaBow.png"),
        135 => (kind, kind, "@tingle"),
        _ => return Ok(0),
    };
    image(r, resources, ENEMIES, slot, usage_slot, path)
}

fn classic_ghosts(r: &mut Runtime, resources: u32) -> Result<bool, u32> {
    let world = r.get(resources, WORLD)?.r();
    let canvas = r.get(world, field::world::CANVAS)?.r();
    Ok(r.get(canvas, field::canvas::VISUAL_EFFECTS)?.i() != 0)
}
fn red_outfit(r: &mut Runtime, resources: u32) -> Result<bool, u32> {
    let world = r.get(resources, WORLD)?.r();
    let canvas = r.get(world, field::world::CANVAS)?.r();
    let player = r.get(canvas, field::canvas::PLAYER)?.r();
    Ok(r.get(player, field::player::TUNIC_COLOR)?.i() == 2)
}
