//! World rooms.
use super::*;

/// Initialize. Original reference: `j.<init>(Ljavax/microedition/lcdui/Graphics;Ljavax/microedition/lcdui/Graphics;Lh;Lk;)V`.
pub(in crate::game) fn initialize(
    r: &mut Runtime,
    world: u32,
    graphics: u32,
    overlay_graphics: u32,
    player: u32,
    canvas: u32,
) -> Result<(), u32> {
    host::object_initialize(r, world)?;
    r.set(world, field::world::CANVAS, V::R(canvas))?;
    r.set(world, field::world::TILESET, V::I(0))?;
    r.set(world, field::world::TEXT_SCROLL, V::I(0))?;
    r.set(world, field::world::TEXT_FRAME, V::I(0))?;
    r.set(world, field::world::STATE_M, V::I(0))?;
    let object_value = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value)?;
    r.set(world, field::world::ROOM_HISTORY, V::R(object_value))?;
    let object_value_2 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_2)?;
    let object_value_3 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_3)?;
    r.set(world, field::world::PROJECTILES, V::R(object_value_3))?;
    let object_value_4 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_4)?;
    r.set(world, field::world::PORTALS, V::R(object_value_4))?;
    let object_value_5 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_5)?;
    r.set(world, field::world::ITEMS, V::R(object_value_5))?;
    let object_value_6 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_6)?;
    r.set(world, field::world::ENEMIES, V::R(object_value_6))?;
    let object_value_7 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_7)?;
    r.set(world, field::world::OBJECTS, V::R(object_value_7))?;
    let object_value_8 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_8)?;
    r.set(world, field::world::OVERLAY_OBJECTS, V::R(object_value_8))?;
    let object_value_9 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_9)?;
    r.set(world, field::world::DIALOGUE, V::R(object_value_9))?;
    let object_value_10 = r.alloc("java/util/Random", Native::Empty);
    host::random_initialize(r, object_value_10)?;
    r.set(world, field::world::RANDOM, V::R(object_value_10))?;
    r.set(world, field::world::STATE_T, V::R(0))?;
    r.set(world, field::world::ALTERNATE_MAP, V::I(0))?;
    r.set(world, field::world::ALTERNATE_ROOM, V::I(0))?;
    let resources_value = r.alloc("o", Native::Empty);
    super::resources::initialize(r, resources_value, world)?;
    r.set(world, field::world::RESOURCES, V::R(resources_value))?;
    r.set(world, field::world::SWITCH_COOLDOWN, V::I(0))?;
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let text_value = r.intern("Map Constructor");
    host::print_stream_println(r, state_out_value, text_value)?;
    if player == 0 {
        let text_value_2 = r.intern("Maps/intro_new.xmap.xmob");
        let load_map_value =
            super::world::load_map(r, world, text_value_2, graphics, player, 0, 0, 0, 0, 0)?;
        return Ok(());
    }
    let player_world_flags = r.get(player, field::player::WORLD_FLAGS)?.i();
    let player_zone = r.get(player, field::player::ZONE)?.i();
    if !((player_world_flags & (1_i32).wrapping_shl((player_zone) as u32)) <= 0) {
        r.set(world, field::world::ALTERNATE_MAP, V::I(1))?;
    }
    let player_zone_2 = r.get(player, field::player::ZONE)?.i();
    if player_zone_2 != 9 {
        let object_value_11 = r.alloc("java/lang/StringBuffer", Native::Empty);
        host::string_buffer_initialize(r, object_value_11)?;
        let text_value_3 = r.intern("Maps/Map");
        let string_buffer_append_2_value =
            host::string_buffer_append_2(r, object_value_11, text_value_3)?;
        let player_map = r.get(player, field::player::MAP)?.i();
        let string_buffer_append_value =
            host::string_buffer_append(r, string_buffer_append_2_value, player_map)?;
        let text_value_4 = r.intern("-");
        let string_buffer_append_2_value_2 =
            host::string_buffer_append_2(r, string_buffer_append_value, text_value_4)?;
        let player_zone_3 = r.get(player, field::player::ZONE)?.i();
        let string_buffer_append_value_2 =
            host::string_buffer_append(r, string_buffer_append_2_value_2, player_zone_3)?;
        let text_value_5 = r.intern(".xmap.xmob");
        let string_buffer_append_2_value_3 =
            host::string_buffer_append_2(r, string_buffer_append_value_2, text_value_5)?;
        let string_buffer_to_string_value =
            host::string_buffer_to_string(r, string_buffer_append_2_value_3)?;
        let player_map_2 = r.get(player, field::player::MAP)?.i();
        let player_zone_4 = r.get(player, field::player::ZONE)?.i();
        let load_map_value_2 = super::world::load_map(
            r,
            world,
            string_buffer_to_string_value,
            graphics,
            player,
            player_map_2,
            player_zone_4,
            0,
            0,
            1,
        )?;
    } else {
        let world_alternate_map = r.get(world, field::world::ALTERNATE_MAP)?.i();
        if world_alternate_map == 0 {
            let object_value_12 = r.alloc("java/lang/StringBuffer", Native::Empty);
            host::string_buffer_initialize(r, object_value_12)?;
            let text_value_6 = r.intern("Maps/Map");
            let string_buffer_append_2_value_4 =
                host::string_buffer_append_2(r, object_value_12, text_value_6)?;
            let player_map_3 = r.get(player, field::player::MAP)?.i();
            let string_buffer_append_value_3 =
                host::string_buffer_append(r, string_buffer_append_2_value_4, player_map_3)?;
            let text_value_7 = r.intern("-");
            let string_buffer_append_2_value_5 =
                host::string_buffer_append_2(r, string_buffer_append_value_3, text_value_7)?;
            let player_zone_5 = r.get(player, field::player::ZONE)?.i();
            let string_buffer_append_value_4 =
                host::string_buffer_append(r, string_buffer_append_2_value_5, player_zone_5)?;
            let text_value_8 = r.intern(".xmap.xmob");
            let string_buffer_append_2_value_6 =
                host::string_buffer_append_2(r, string_buffer_append_value_4, text_value_8)?;
            let string_buffer_to_string_value_2 =
                host::string_buffer_to_string(r, string_buffer_append_2_value_6)?;
            let player_map_4 = r.get(player, field::player::MAP)?.i();
            let player_zone_6 = r.get(player, field::player::ZONE)?.i();
            let load_map_value_3 = super::world::load_map(
                r,
                world,
                string_buffer_to_string_value_2,
                graphics,
                player,
                player_map_4,
                player_zone_6,
                0,
                0,
                1,
            )?;
        }
        let world_alternate_map_3 = r.get(world, field::world::ALTERNATE_MAP)?.i();
        if world_alternate_map_3 == 1 {
            let object_value_13 = r.alloc("java/lang/StringBuffer", Native::Empty);
            host::string_buffer_initialize(r, object_value_13)?;
            let text_value_9 = r.intern("Maps/Map");
            let string_buffer_append_2_value_7 =
                host::string_buffer_append_2(r, object_value_13, text_value_9)?;
            let player_map_6 = r.get(player, field::player::MAP)?.i();
            let string_buffer_append_value_5 =
                host::string_buffer_append(r, string_buffer_append_2_value_7, player_map_6)?;
            let text_value_10 = r.intern("-");
            let string_buffer_append_2_value_8 =
                host::string_buffer_append_2(r, string_buffer_append_value_5, text_value_10)?;
            let player_zone_9 = r.get(player, field::player::ZONE)?.i();
            let string_buffer_append_value_6 =
                host::string_buffer_append(r, string_buffer_append_2_value_8, player_zone_9)?;
            let text_value_11 = r.intern("-1.xmap.xmob");
            let string_buffer_append_2_value_9 =
                host::string_buffer_append_2(r, string_buffer_append_value_6, text_value_11)?;
            let string_buffer_to_string_value_3 =
                host::string_buffer_to_string(r, string_buffer_append_2_value_9)?;
            let player_map_7 = r.get(player, field::player::MAP)?.i();
            let player_zone_10 = r.get(player, field::player::ZONE)?.i();
            let load_map_value_4 = super::world::load_map(
                r,
                world,
                string_buffer_to_string_value_3,
                graphics,
                player,
                player_map_7,
                player_zone_10,
                0,
                0,
                1,
            )?;
        }
    }
    let world_alternate_map_2 = r.get(world, field::world::ALTERNATE_MAP)?.i();
    r.set(
        world,
        field::world::ALTERNATE_ROOM,
        V::I(world_alternate_map_2),
    )?;
    let player_map_5 = r.get(player, field::player::MAP)?.i();
    r.set(world, field::world::MAP, V::I(player_map_5))?;
    let player_zone_7 = r.get(player, field::player::ZONE)?.i();
    r.set(world, field::world::ZONE, V::I(player_zone_7))?;
    let player_world_flags_2 = r.get(player, field::player::WORLD_FLAGS)?.i();
    r.set(world, field::world::STATE_G, V::I(player_world_flags_2))?;
    let world_state_g = r.get(world, field::world::STATE_G)?.i();
    let world_zone_8 = r.get(world, field::world::ZONE)?.i();
    if !((world_state_g & (1_i32).wrapping_shl((world_zone_8) as u32)) <= 0) {
        r.set(world, field::world::ALTERNATE_MAP, V::I(1))?;
    }
    let world_room_history = r.get(world, field::world::ROOM_HISTORY)?.r();
    host::vector_set_size(r, world_room_history, 16)?;
    return Ok(());
}

/// Load map. Original reference: `j.a(Ljava/lang/String;Ljavax/microedition/lcdui/Graphics;Lh;IIIIZ)Z`.
pub(in crate::game) fn load_map(
    r: &mut Runtime,
    world: u32,
    mut path: u32,
    graphics: u32,
    player: u32,
    map: i32,
    zone: i32,
    mut width: i32,
    mut height: i32,
    numbered: i32,
) -> Result<i32, u32> {
    let mut object_6: u32 = 0;
    let mut object_7: u32 = 0;
    let mut number_9: i32 = 0;
    let mut object_9: u32 = 0;
    let mut number_10: i32 = 0;
    let mut object_10: u32 = 0;
    let mut number_11: i32 = 0;
    let mut number_12: i32 = 0;
    let mut number_13: i32 = 0;
    let mut object_13: u32 = 0;
    let mut number_14: i32 = 0;
    let mut number_15: i32 = 0;
    let mut number_16: i32 = 0;
    let mut object_17: u32 = 0;
    let mut carry_0_i: i32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut carry_1_i: i32 = 0;
    let mut carry_1_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    r.set(world, field::world::STATE_J, V::I(height))?;
    r.set(world, field::world::STATE_I, V::I(width))?;
    let state_out_value = r.static_get(field::system::STATE_OUT).r();
    let object_value = r.alloc("java/lang/StringBuffer", Native::Empty);
    host::string_buffer_initialize(r, object_value)?;
    let text_value = r.intern("Load Map");
    let string_buffer_append_2_value = host::string_buffer_append_2(r, object_value, text_value)?;
    let string_buffer_append_2_value_2 =
        host::string_buffer_append_2(r, string_buffer_append_2_value, path)?;
    let string_buffer_to_string_value =
        host::string_buffer_to_string(r, string_buffer_append_2_value_2)?;
    host::print_stream_println(r, state_out_value, string_buffer_to_string_value)?;
    let world_alternate_map = r.get(world, field::world::ALTERNATE_MAP)?.i();
    r.set(
        world,
        field::world::ALTERNATE_ROOM,
        V::I(world_alternate_map),
    )?;
    let array_value = r.new_array(8, 4)?;
    object_6 = array_value;
    height = 0;
    outcome = (|| -> JResult { Ok(V::R(host::object_get_class(r, world)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
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
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let class_get_resource_as_stream_value = outcome.unwrap().r();
    path = class_get_resource_as_stream_value;
    if class_get_resource_as_stream_value == 0 {
        let state_out_value_4 = r.static_get(field::system::STATE_OUT).r();
        let text_value_4 = r.intern("File Not Found");
        outcome = (|| -> JResult {
            host::print_stream_println(r, state_out_value_4, text_value_4)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let text_value_3 = r.intern("File Not Found");
            host::print_stream_println(r, state_out_value_3, text_value_3)?;
            return Ok(1);
        }
        (carry_0_i,) = (0,);
        return Ok(carry_0_i);
    }
    let object_value_2 = r.alloc("java/io/DataInputStream", Native::Empty);
    outcome = (|| -> JResult {
        host::data_input_stream_initialize(r, object_value_2, path)?;
        Ok(V::I(0))
    })();
    (carry_0_r,) = (object_value_2,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    path = carry_0_r;
    outcome = (|| -> JResult {
        Ok(V::I(host::data_input_stream_read(
            r, carry_0_r, object_6, 0, 4,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_value = outcome.unwrap().i();
    let element_value = r.array_get(object_6, 0)?.i();
    if element_value != 88 {
        (carry_0_i,) = (0,);
        return Ok(carry_0_i);
    }
    let room_history_value = r.alloc("i", Native::Empty);
    outcome = (|| -> JResult {
        super::room_history::initialize(r, room_history_value)?;
        Ok(V::I(0))
    })();
    (carry_0_r,) = (room_history_value,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    object_6 = carry_0_r;
    let world_zone_2 = r.get(world, field::world::ZONE)?.i();
    r.set(carry_0_r, field::room_history::ZONE, V::I(world_zone_2))?;
    let world_map = r.get(world, field::world::MAP)?.i();
    r.set(object_6, field::room_history::MAP, V::I(world_map))?;
    let world_enemies = r.get(world, field::world::ENEMIES)?.r();
    let vector_size_value = host::vector_size(r, world_enemies)?;
    r.set(
        object_6,
        field::room_history::ENEMY_COUNT,
        V::I(vector_size_value),
    )?;
    let world_room_history_2 = r.get(world, field::world::ROOM_HISTORY)?.r();
    host::vector_add_element(r, world_room_history_2, object_6)?;
    let world_room_history_3 = r.get(world, field::world::ROOM_HISTORY)?.r();
    let vector_size_value_2 = host::vector_size(r, world_room_history_3)?;
    if vector_size_value_2 > 16 {
        let world_room_history_4 = r.get(world, field::world::ROOM_HISTORY)?.r();
        host::vector_remove_element_at(r, world_room_history_4, 0)?;
    }
    let world_enemies_2 = r.get(world, field::world::ENEMIES)?.r();
    host::vector_remove_all_elements(r, world_enemies_2)?;
    let world_items = r.get(world, field::world::ITEMS)?.r();
    host::vector_remove_all_elements(r, world_items)?;
    let world_portals = r.get(world, field::world::PORTALS)?.r();
    host::vector_remove_all_elements(r, world_portals)?;
    let world_objects = r.get(world, field::world::OBJECTS)?.r();
    host::vector_remove_all_elements(r, world_objects)?;
    let world_overlay_objects = r.get(world, field::world::OVERLAY_OBJECTS)?.r();
    host::vector_remove_all_elements(r, world_overlay_objects)?;
    let world_dialogue = r.get(world, field::world::DIALOGUE)?.r();
    host::vector_remove_all_elements(r, world_dialogue)?;
    let world_projectiles = r.get(world, field::world::PROJECTILES)?.r();
    host::vector_remove_all_elements(r, world_projectiles)?;
    r.set(world, field::world::MAP, V::I(map))?;
    r.set(world, field::world::ZONE, V::I(zone))?;
    if numbered != 0 {
        r.set(player, field::player::BOOMERANG_ACTIVE, V::I(0))?;
    }
    let world_resources = r.get(world, field::world::RESOURCES)?.r();
    object_6 = world_resources;
    outcome = (|| -> JResult {
        super::resources::age_items(r, world_resources)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    outcome = (|| -> JResult {
        super::resources::age_objects(r, object_6)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    outcome = (|| -> JResult {
        super::resources::age_projectiles(r, object_6)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    outcome = (|| -> JResult {
        super::resources::age_enemies(r, object_6)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    r.set(world, field::world::STATE_X, V::R(0))?;
    r.set(world, field::world::STATE_C, V::R(0))?;
    r.set(world, field::world::STATE_Y, V::R(0))?;
    r.set(world, field::world::STATE_D, V::R(0))?;
    r.set(world, field::world::STATE_Z, V::R(0))?;
    outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
    (carry_0_r,) = (world,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_byte_value_2 = outcome.unwrap().i();
    r.set(
        carry_0_r,
        field::world::WIDTH,
        V::I(data_input_stream_read_byte_value_2),
    )?;
    outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
    (carry_0_r,) = (world,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_byte_value_3 = outcome.unwrap().i();
    r.set(
        carry_0_r,
        field::world::HEIGHT,
        V::I(data_input_stream_read_byte_value_3),
    )?;
    outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_byte_value_4 = outcome.unwrap().i();
    height = data_input_stream_read_byte_value_4;
    let world_tileset = r.get(world, field::world::TILESET)?.i();
    if data_input_stream_read_byte_value_4 != world_tileset {
        r.set(world, field::world::STATE_T, V::R(0))?;
        r.set(world, field::world::TILESET, V::I(height))?;
    }
    r.set(world, field::world::TILES, V::R(0))?;
    host::system_gc(r)?;
    let state_out_value_5 = r.static_get(field::system::STATE_OUT).r();
    let object_value_3 = r.alloc("java/lang/StringBuffer", Native::Empty);
    outcome = (|| -> JResult {
        host::string_buffer_initialize(r, object_value_3)?;
        Ok(V::I(0))
    })();
    (carry_0_r, carry_1_r) = (state_out_value_5, object_value_3);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let text_value_5 = r.intern("Tile set ");
    outcome = (|| -> JResult {
        Ok(V::R(host::string_buffer_append_2(
            r,
            carry_1_r,
            text_value_5,
        )?))
    })();
    (carry_0_r,) = (carry_0_r,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let string_buffer_append_2_value_3 = outcome.unwrap().r();
    let world_tileset_2 = r.get(world, field::world::TILESET)?.i();
    outcome = (|| -> JResult {
        Ok(V::R(host::string_buffer_append(
            r,
            string_buffer_append_2_value_3,
            world_tileset_2,
        )?))
    })();
    (carry_0_r,) = (carry_0_r,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let string_buffer_append_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        Ok(V::R(host::string_buffer_to_string(
            r,
            string_buffer_append_value,
        )?))
    })();
    (carry_0_r,) = (carry_0_r,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let string_buffer_to_string_value_2 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::print_stream_println(r, carry_0_r, string_buffer_to_string_value_2)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let world_width = r.get(world, field::world::WIDTH)?.i();
    let world_height = r.get(world, field::world::HEIGHT)?.i();
    let array_value_2 = r.new_array(8, world_width.wrapping_mul(world_height))?;
    r.set(world, field::world::TILES, V::R(array_value_2))?;
    width = 0;
    'load_map_loop_2: loop {
        let world_width_2 = r.get(world, field::world::WIDTH)?.i();
        let world_height_2 = r.get(world, field::world::HEIGHT)?.i();
        if width >= world_width_2.wrapping_mul(world_height_2) {
            break 'load_map_loop_2;
        }
        let world_tiles = r.get(world, field::world::TILES)?.r();
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
        (carry_0_r, carry_1_i) = (world_tiles, width);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let text_value_3 = r.intern("File Not Found");
            host::print_stream_println(r, state_out_value_3, text_value_3)?;
            return Ok(1);
        }
        let data_input_stream_read_byte_value_9 = outcome.unwrap().i();
        r.array_set(
            carry_0_r,
            carry_1_i,
            V::I(data_input_stream_read_byte_value_9),
        )?;
        width = width.wrapping_add(1);
        continue 'load_map_loop_2;
    }
    outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_byte_value_8 = outcome.unwrap().i();
    width = data_input_stream_read_byte_value_8;
    if data_input_stream_read_byte_value_8 > 0 {
        height = 0;
        'load_map_loop_6: loop {
            if height >= width {
                break 'load_map_loop_6;
            }
            let portal_value = r.alloc("e", Native::Empty);
            outcome = (|| -> JResult {
                super::portal::initialize(r, portal_value)?;
                Ok(V::I(0))
            })();
            (carry_0_r,) = (portal_value,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            object_9 = carry_0_r;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            (carry_0_r,) = (carry_0_r,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_13 = outcome.unwrap().i();
            r.set(
                carry_0_r,
                field::portal::DESTINATION_MAP,
                V::I(data_input_stream_read_unsigned_byte_value_13),
            )?;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            (carry_0_r,) = (object_9,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_14 = outcome.unwrap().i();
            r.set(
                carry_0_r,
                field::portal::DESTINATION_ZONE,
                V::I(data_input_stream_read_unsigned_byte_value_14),
            )?;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            (carry_0_r,) = (object_9,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_15 = outcome.unwrap().i();
            r.set(
                carry_0_r,
                field::portal::DESTINATION_X,
                V::I(data_input_stream_read_unsigned_byte_value_15),
            )?;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            (carry_0_r,) = (object_9,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_16 = outcome.unwrap().i();
            r.set(
                carry_0_r,
                field::portal::DESTINATION_Y,
                V::I(data_input_stream_read_unsigned_byte_value_16),
            )?;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            (carry_0_r,) = (object_9,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_17 = outcome.unwrap().i();
            r.set(
                carry_0_r,
                field::portal::TILE_X,
                V::I(data_input_stream_read_unsigned_byte_value_17),
            )?;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            (carry_0_r,) = (object_9,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_18 = outcome.unwrap().i();
            r.set(
                carry_0_r,
                field::portal::TILE_Y,
                V::I(data_input_stream_read_unsigned_byte_value_18),
            )?;
            let world_portals_2 = r.get(world, field::world::PORTALS)?.r();
            host::vector_add_element(r, world_portals_2, object_9)?;
            height = height.wrapping_add(1);
            continue 'load_map_loop_6;
        }
    }
    let world_state_t_3 = r.get(world, field::world::STATE_T)?.r();
    if world_state_t_3 == 0 {
        let world_tileset_3 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_3 == 0 {
            let text_value_7 = r.intern("/Images/ZeldaTileSet01.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_7)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_2 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_2),
            )?;
        }
        let world_tileset_4 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_4 == 1 {
            let text_value_8 = r.intern("/Images/ZeldaTileSet02.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_8)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_3 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_3),
            )?;
        }
        let world_tileset_5 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_5 == 2 {
            let text_value_9 = r.intern("/Images/Zelda1Minish05.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_9)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_4 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_4),
            )?;
        }
        let world_tileset_6 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_6 == 3 {
            let text_value_10 = r.intern("/Images/Zelda1Minish01.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_10)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_5 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_5),
            )?;
        }
        let world_tileset_7 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_7 == 4 {
            let text_value_11 = r.intern("/Images/Zelda1Minish02.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_11)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_6 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_6),
            )?;
        }
        let world_tileset_8 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_8 == 5 {
            let text_value_13 = r.intern("/Images/Zelda1Minish03.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_13)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_8 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_8),
            )?;
        }
        let world_tileset_9 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_9 == 6 {
            let text_value_14 = r.intern("/Images/Zelda1Minish04.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_14)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_9 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_9),
            )?;
        }
        let world_tileset_10 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_10 == 7 {
            let text_value_16 = r.intern("/Images/Zelda1Minish06.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_16)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_11 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_11),
            )?;
        }
        let world_tileset_11 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_11 == 8 {
            let text_value_17 = r.intern("/Images/Zelda1Minish07.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_17)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_12 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_12),
            )?;
        }
        let world_tileset_12 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_12 == 9 {
            let text_value_18 = r.intern("/Images/Zelda1Minish08.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_18)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_13 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_13),
            )?;
        }
        let world_tileset_13 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_13 == 10 {
            let text_value_19 = r.intern("/Images/Zelda1Minish09.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_19)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_14 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_14),
            )?;
        }
        let world_tileset_14 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_14 == 11 {
            let text_value_20 = r.intern("/Images/Zelda1Minish10.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_20)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_15 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_15),
            )?;
        }
        let world_tileset_15 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_15 == 12 {
            let text_value_21 = r.intern("/Images/Zelda1Minish11.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_21)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_16 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_16),
            )?;
        }
        let world_tileset_16 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_16 == 13 {
            let text_value_22 = r.intern("/Images/Zelda1Minish12.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_22)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_17 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_17),
            )?;
        }
        let world_tileset_17 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_17 == 14 {
            let text_value_23 = r.intern("/Images/Zelda1Minish13.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_23)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_18 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_18),
            )?;
        }
        let world_tileset_18 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_18 == 15 {
            let text_value_24 = r.intern("/Images/Zelda1Minish14.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_24)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_19 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_19),
            )?;
        }
        let world_tileset_19 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_19 == 16 {
            let text_value_25 = r.intern("/Images/Zelda1Minish15.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_25)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_20 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_20),
            )?;
        }
        let world_tileset_20 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_20 == 17 {
            let text_value_26 = r.intern("/Images/Zelda1Minish16.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_26)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_21 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_21),
            )?;
        }
        let world_tileset_21 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_21 == 18 {
            let text_value_27 = r.intern("/Images/Zelda1Minish17.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_27)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_22 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_22),
            )?;
        }
        let world_tileset_22 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_22 == 19 {
            let text_value_28 = r.intern("/Images/Zelda1Minish18.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_28)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_23 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_23),
            )?;
        }
        let world_tileset_23 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_23 == 20 {
            let text_value_29 = r.intern("/Images/Zelda1Minish19.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_29)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_24 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_24),
            )?;
        }
        let world_tileset_24 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_24 == 21 {
            let text_value_32 = r.intern("/Images/Zelda1Minish20.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_32)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_25 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_25),
            )?;
        }
        let world_tileset_25 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_25 == 22 {
            let text_value_33 = r.intern("/Images/Zelda1Minish21.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_33)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_26 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_26),
            )?;
        }
        let world_tileset_26 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_26 == 23 {
            let text_value_34 = r.intern("/Images/Zelda1Minish22.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_34)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_27 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_27),
            )?;
        }
        let world_tileset_27 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_27 == 24 {
            let text_value_35 = r.intern("/Images/Zelda1Minish23.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_35)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_28 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_28),
            )?;
        }
        let world_tileset_28 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_28 == 25 {
            let text_value_36 = r.intern("/Images/Zelda1Minish24.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_36)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_29 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_29),
            )?;
        }
        let world_tileset_29 = r.get(world, field::world::TILESET)?.i();
        if world_tileset_29 == 26 {
            let text_value_37 = r.intern("/Images/Zelda1Minish25.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_37)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if r.is_instance(carry_0_r, "java/io/IOException") {
                    (carry_0_i,) = (0,);
                    return Ok(carry_0_i);
                }
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let image_create_image_value_30 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::world::STATE_T,
                V::R(image_create_image_value_30),
            )?;
        }
    }

    let graphics_get_translate_x_value_2 = host::graphics_get_translate_x(r, graphics)?;
    let graphics_get_translate_y_value_2 = host::graphics_get_translate_y(r, graphics)?;
    host::graphics_translate(
        r,
        graphics,
        graphics_get_translate_x_value_2.wrapping_neg(),
        graphics_get_translate_y_value_2.wrapping_neg(),
    )?;
    host::graphics_set_color(r, graphics, 0)?;
    host::graphics_fill_rect(r, graphics, 0, 0, 255, 176)?;
    height = 0;
    'load_map_loop_9: loop {
        let world_height_3 = r.get(world, field::world::HEIGHT)?.i();
        if height >= world_height_3 {
            break 'load_map_loop_9;
        }
        number_9 = 0;
        'load_map_loop_11: loop {
            let world_width_3 = r.get(world, field::world::WIDTH)?.i();
            if number_9 >= world_width_3 {
                break 'load_map_loop_11;
            }
            let world_tiles_2 = r.get(world, field::world::TILES)?.r();
            let world_width_4 = r.get(world, field::world::WIDTH)?.i();
            let element_value_2 = r
                .array_get(
                    world_tiles_2,
                    (height.wrapping_mul(world_width_4)).wrapping_add(number_9),
                )?
                .i();
            width = (element_value_2 & 63);
            if (element_value_2 & 63) != 0 {
                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                number_10 = (width & 3);
                number_11 = width.wrapping_shr((2) as u32);
                let world_state_t_4 = r.get(world, field::world::STATE_T)?.r();
                host::graphics_draw_image(
                    r,
                    graphics,
                    world_state_t_4,
                    (number_10.wrapping_shl((4) as u32)).wrapping_neg(),
                    (number_11.wrapping_shl((4) as u32)).wrapping_neg(),
                    0,
                )?;
                host::graphics_translate(r, graphics, 16, 0)?;
            } else {
                host::graphics_translate(r, graphics, 16, 0)?;
            }
            number_9 = number_9.wrapping_add(1);
            continue 'load_map_loop_11;
        }
        let graphics_get_translate_x_value_3 = host::graphics_get_translate_x(r, graphics)?;
        host::graphics_translate(
            r,
            graphics,
            graphics_get_translate_x_value_3.wrapping_neg(),
            16,
        )?;
        height = height.wrapping_add(1);
        continue 'load_map_loop_9;
    }
    let world_zone_6 = r.get(world, field::world::ZONE)?.i();
    if world_zone_6 == 20 {
        let world_map_6 = r.get(world, field::world::MAP)?.i();
        if world_map_6 == 2 {
            r.set(world, field::world::STATE_T, V::R(0))?;
            host::system_gc(r)?;
            let text_value_12 = r.intern("/Images/ganonbattleBG.png");
            outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_12)?)) })();
            (carry_0_r,) = (world,);
            if outcome.is_ok() {
                let image_create_image_value_7 = outcome.unwrap().r();
                r.set(
                    carry_0_r,
                    field::world::STATE_T,
                    V::R(image_create_image_value_7),
                )?;
            } else {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                        return Err(outcome.unwrap_err());
                    }
                    let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                    let text_value_3 = r.intern("File Not Found");
                    host::print_stream_println(r, state_out_value_3, text_value_3)?;
                    return Ok(1);
                }
            }
            let world_state_t = r.get(world, field::world::STATE_T)?.r();
            if world_state_t != 0 {
                let graphics_get_translate_x_value = host::graphics_get_translate_x(r, graphics)?;
                let graphics_get_translate_y_value = host::graphics_get_translate_y(r, graphics)?;
                host::graphics_translate(
                    r,
                    graphics,
                    graphics_get_translate_x_value.wrapping_neg(),
                    graphics_get_translate_y_value.wrapping_neg(),
                )?;
                host::graphics_set_clip(r, graphics, 0, 0, 256, 176)?;
                host::graphics_clip_rect(r, graphics, 0, 0, 256, 176)?;
                let world_state_t_2 = r.get(world, field::world::STATE_T)?.r();
                host::graphics_draw_image(r, graphics, world_state_t_2, 0, 0, 0)?;
            }
        }
    }
    let world_zone = r.get(world, field::world::ZONE)?.i();
    if world_zone == 20 {
        let world_map_2 = r.get(world, field::world::MAP)?.i();
        if world_map_2 == 3 {
            let world_canvas = r.get(world, field::world::CANVAS)?.r();
            r.set(world_canvas, field::canvas::SCREEN_STATE, V::I(100))?;
            (carry_0_i,) = (1,);
            return Ok(carry_0_i);
        }
    }
    outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_byte_value = outcome.unwrap().i();
    height = data_input_stream_read_byte_value;
    number_9 = data_input_stream_read_byte_value;
    width = 0;
    'load_map_loop_15: loop {
        let world_room_history_5 = r.get(world, field::world::ROOM_HISTORY)?.r();
        let vector_size_value_3 = host::vector_size(r, world_room_history_5)?;
        if width >= vector_size_value_3 {
            break 'load_map_loop_15;
        }
        let world_room_history_6 = r.get(world, field::world::ROOM_HISTORY)?.r();
        let vector_element_at_value = host::vector_element_at(r, world_room_history_6, width)?;
        host::check_cast(r, vector_element_at_value, "i")?;
        object_10 = vector_element_at_value;
        if vector_element_at_value != 0 {
            let room_history_map_4 = r.get(object_10, field::room_history::MAP)?.i();
            let world_map_5 = r.get(world, field::world::MAP)?.i();
            if room_history_map_4 == world_map_5 {
                let room_history_zone_4 = r.get(object_10, field::room_history::ZONE)?.i();
                let world_zone_5 = r.get(world, field::world::ZONE)?.i();
                if room_history_zone_4 == world_zone_5 {
                    let room_history_enemy_count =
                        r.get(object_10, field::room_history::ENEMY_COUNT)?.i();
                    number_9 = room_history_enemy_count;
                }
            }
        }
        width = width.wrapping_add(1);
        continue 'load_map_loop_15;
    }
    let world_map_3 = r.get(world, field::world::MAP)?.i();
    if world_map_3 == 2 {
        let world_zone_3 = r.get(world, field::world::ZONE)?.i();
        if world_zone_3 == 20 {
            let world_canvas_2 = r.get(world, field::world::CANVAS)?.r();
            let canvas_use_phone_buttons =
                r.get(world_canvas_2, field::canvas::USE_PHONE_BUTTONS)?.i();
            if canvas_use_phone_buttons != 0 {
                let world_canvas_3 = r.get(world, field::world::CANVAS)?.r();
                r.set(world_canvas_3, field::canvas::STATE_G, V::I(1))?;
            }
        }
    }

    if height > 0 {
        width = 0;
        'load_map_loop_19: loop {
            if width >= height {
                break 'load_map_loop_19;
            }
            outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_byte_value_11 = outcome.unwrap().i();
            number_10 = data_input_stream_read_byte_value_11;
            outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_byte_value_12 = outcome.unwrap().i();
            number_11 = data_input_stream_read_byte_value_12;
            number_10 = number_10.wrapping_shl((4) as u32);
            number_11 = number_11.wrapping_shl((4) as u32);
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_4 = outcome.unwrap().i();
            number_12 = data_input_stream_read_unsigned_byte_value_4;
            if data_input_stream_read_unsigned_byte_value_4 != 133 {
                if width < number_9 {
                    let enemy_value = r.alloc("l", Native::Empty);
                    let world_resources_3_read = r.get(world, field::world::RESOURCES)?.r();
                    outcome = (|| -> JResult {
                        Ok(V::R(super::resources::enemy_image(
                            r,
                            world_resources_3_read,
                            number_12,
                        )?))
                    })();
                    (carry_0_r, carry_1_r) = (enemy_value, enemy_value);
                    if outcome.is_err() {
                        carry_0_r = outcome.unwrap_err();
                        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                            return Err(outcome.unwrap_err());
                        }
                        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                        let text_value_3 = r.intern("File Not Found");
                        host::print_stream_println(r, state_out_value_3, text_value_3)?;
                        return Ok(1);
                    }
                    let enemy_image_value = outcome.unwrap().r();
                    outcome = (|| -> JResult {
                        super::enemy::initialize(
                            r,
                            carry_1_r,
                            enemy_image_value,
                            number_10,
                            number_11,
                            1,
                            number_12,
                        )?;
                        Ok(V::I(0))
                    })();
                    (carry_0_r,) = (carry_0_r,);
                    if outcome.is_err() {
                        carry_0_r = outcome.unwrap_err();
                        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                            return Err(outcome.unwrap_err());
                        }
                        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                        let text_value_3 = r.intern("File Not Found");
                        host::print_stream_println(r, state_out_value_3, text_value_3)?;
                        return Ok(1);
                    }
                    object_13 = carry_0_r;
                    let world_enemies_3_read = r.get(world, field::world::ENEMIES)?.r();
                    host::vector_add_element(r, world_enemies_3_read, object_13)?;
                }
            } else {
                let player_progression_flags_2 =
                    r.get(player, field::player::PROGRESSION_FLAGS)?.i();
                if !((player_progression_flags_2 & 16777216) > 0) {
                    if width < number_9 {
                        let enemy_value = r.alloc("l", Native::Empty);
                        let world_resources_3_read = r.get(world, field::world::RESOURCES)?.r();
                        outcome = (|| -> JResult {
                            Ok(V::R(super::resources::enemy_image(
                                r,
                                world_resources_3_read,
                                number_12,
                            )?))
                        })();
                        (carry_0_r, carry_1_r) = (enemy_value, enemy_value);
                        if outcome.is_err() {
                            carry_0_r = outcome.unwrap_err();
                            if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                                return Err(outcome.unwrap_err());
                            }
                            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                            let text_value_3 = r.intern("File Not Found");
                            host::print_stream_println(r, state_out_value_3, text_value_3)?;
                            return Ok(1);
                        }
                        let enemy_image_value = outcome.unwrap().r();
                        outcome = (|| -> JResult {
                            super::enemy::initialize(
                                r,
                                carry_1_r,
                                enemy_image_value,
                                number_10,
                                number_11,
                                1,
                                number_12,
                            )?;
                            Ok(V::I(0))
                        })();
                        (carry_0_r,) = (carry_0_r,);
                        if outcome.is_err() {
                            carry_0_r = outcome.unwrap_err();
                            if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                                return Err(outcome.unwrap_err());
                            }
                            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                            let text_value_3 = r.intern("File Not Found");
                            host::print_stream_println(r, state_out_value_3, text_value_3)?;
                            return Ok(1);
                        }
                        object_13 = carry_0_r;
                        let world_enemies_3_read = r.get(world, field::world::ENEMIES)?.r();
                        host::vector_add_element(r, world_enemies_3_read, object_13)?;
                    }
                }
            }
            width = width.wrapping_add(1);
            continue 'load_map_loop_19;
        }
        let text_value_6 = r.intern("/Images/Death.png");
        outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_6)?)) })();
        (carry_0_r,) = (world,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                return Err(outcome.unwrap_err());
            }
            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
            let text_value_3 = r.intern("File Not Found");
            host::print_stream_println(r, state_out_value_3, text_value_3)?;
            return Ok(1);
        }
        let image_create_image_value = outcome.unwrap().r();
        r.set(
            carry_0_r,
            field::world::DEATH_IMAGE,
            V::R(image_create_image_value),
        )?;
    }
    outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_byte_value_5 = outcome.unwrap().i();
    width = data_input_stream_read_byte_value_5;
    if data_input_stream_read_byte_value_5 > 0 {
        number_10 = 0;
        'load_map_loop_27: loop {
            if number_10 >= width {
                break 'load_map_loop_27;
            }
            outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_byte_value_6 = outcome.unwrap().i();
            number_11 = data_input_stream_read_byte_value_6;
            outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_byte_value_7 = outcome.unwrap().i();
            number_12 = data_input_stream_read_byte_value_7;
            number_11 = number_11.wrapping_shl((4) as u32);
            number_12 = number_12.wrapping_shl((4) as u32);
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_2 = outcome.unwrap().i();
            number_13 = data_input_stream_read_unsigned_byte_value_2;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_3 = outcome.unwrap().i();
            height = data_input_stream_read_unsigned_byte_value_3;
            let item_value = r.alloc("c", Native::Empty);
            let world_resources_2 = r.get(world, field::world::RESOURCES)?.r();
            outcome = (|| -> JResult {
                Ok(V::R(super::resources::item_image(
                    r,
                    world_resources_2,
                    number_13,
                )?))
            })();
            (carry_0_r, carry_1_r) = (item_value, item_value);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let item_image_value = outcome.unwrap().r();
            outcome = (|| -> JResult {
                super::item::initialize(
                    r,
                    carry_1_r,
                    item_image_value,
                    number_13,
                    number_11,
                    number_12,
                    height,
                    player,
                )?;
                Ok(V::I(0))
            })();
            (carry_0_r,) = (carry_0_r,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            object_9 = carry_0_r;
            if numbered != 0 {
                number_14 = 0;
                let player_progression_flags = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
                if !((player_progression_flags & (1_i32).wrapping_shl((height) as u32)) > 0) {
                    number_14 = 1;
                }

                if !((number_13 != 128) && (number_13 != 129)) {
                    let player_heart_pickups = r.get(player, field::player::HEART_PICKUPS)?.i();
                    if (player_heart_pickups & (1_i32).wrapping_shl((height) as u32)) <= 0 {
                        number_14 = 1;
                    } else {
                        number_14 = 0;
                    }
                }

                if number_13 == 131 {
                    let player_key_pickups = r.get(player, field::player::KEY_PICKUPS)?.i();
                    if (player_key_pickups & (1_i32).wrapping_shl((height) as u32)) <= 0 {
                        number_14 = 1;
                    } else {
                        number_14 = 0;
                    }
                }

                if !(((number_13 != 147) && (number_13 != 148)) && (number_13 != 149)) {
                    let player_rupee_pickups = r.get(player, field::player::RUPEE_PICKUPS)?.i();
                    if (player_rupee_pickups & (1_i32).wrapping_shl((height) as u32)) <= 0 {
                        number_14 = 1;
                    } else {
                        number_14 = 0;
                    }
                }

                if number_13 > 128 {
                    let world_state_x_3 = r.get(world, field::world::STATE_X)?.r();
                    if world_state_x_3 == 0 {
                        let text_value_15 = r.intern("/Images/BigChest.png");
                        outcome = (|| -> JResult {
                            Ok(V::R(host::image_create_image(r, text_value_15)?))
                        })();
                        (carry_0_r,) = (world,);
                        if outcome.is_ok() {
                            let image_create_image_value_10 = outcome.unwrap().r();
                            r.set(
                                carry_0_r,
                                field::world::STATE_X,
                                V::R(image_create_image_value_10),
                            )?;
                        } else {
                            carry_0_r = outcome.unwrap_err();
                            if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                                carry_0_r = outcome.unwrap_err();
                                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                                    return Err(outcome.unwrap_err());
                                }
                                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                                let text_value_3 = r.intern("File Not Found");
                                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                                return Ok(1);
                            }
                            let state_out_value_2 = r.static_get(field::system::STATE_OUT).r();
                            let text_value_2 = r.intern("error loading big chest");
                            outcome = (|| -> JResult {
                                host::print_stream_println(r, state_out_value_2, text_value_2)?;
                                Ok(V::I(0))
                            })();
                            if outcome.is_err() {
                                carry_0_r = outcome.unwrap_err();
                                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                                    return Err(outcome.unwrap_err());
                                }
                                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                                let text_value_3 = r.intern("File Not Found");
                                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                                return Ok(1);
                            }
                        }
                    }
                    let world_state_x = r.get(world, field::world::STATE_X)?.r();
                    r.set(object_9, field::item::OVERLAY_IMAGE, V::R(world_state_x))?;
                }
                (carry_0_r,) = (object_9,);
                if number_14 != 0 {
                    (carry_0_r, carry_1_i) = (carry_0_r, 0);
                } else {
                    (carry_0_r, carry_1_i) = (carry_0_r, 1);
                }
                r.set(carry_0_r, field::item::COLLECTED, V::I(carry_1_i))?;
            }
            let world_items_2 = r.get(world, field::world::ITEMS)?.r();
            host::vector_add_element(r, world_items_2, object_9)?;
            number_10 = number_10.wrapping_add(1);
            continue 'load_map_loop_27;
        }
    }
    outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_unsigned_byte_value = outcome.unwrap().i();
    number_10 = data_input_stream_read_unsigned_byte_value;
    if data_input_stream_read_unsigned_byte_value > 0 {
        number_11 = 0;
        'load_map_loop_31: loop {
            if number_11 >= number_10 {
                break 'load_map_loop_31;
            }
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_5 = outcome.unwrap().i();
            number_12 = data_input_stream_read_unsigned_byte_value_5;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_6 = outcome.unwrap().i();
            number_13 = 0;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_7 = outcome.unwrap().i();
            height = data_input_stream_read_unsigned_byte_value_7;
            outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_byte_value_13 = outcome.unwrap().i();
            number_9 = data_input_stream_read_byte_value_13;
            outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_byte_value_14 = outcome.unwrap().i();
            number_14 = data_input_stream_read_byte_value_14;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_8 = outcome.unwrap().i();
            width = data_input_stream_read_unsigned_byte_value_8;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_9 = outcome.unwrap().i();
            number_13 = data_input_stream_read_unsigned_byte_value_9;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_10 = outcome.unwrap().i();
            number_9 = number_9.wrapping_shl((4) as u32);
            number_14 = number_14.wrapping_shl((4) as u32);
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_11 = outcome.unwrap().i();
            number_15 = data_input_stream_read_unsigned_byte_value_11;
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_12 = outcome.unwrap().i();
            number_16 = data_input_stream_read_unsigned_byte_value_12;
            let map_object_value = r.alloc("n", Native::Empty);
            outcome = (|| -> JResult {
                super::map_object::initialize(r, map_object_value, number_12)?;
                Ok(V::I(0))
            })();
            (carry_0_r,) = (map_object_value,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            object_17 = carry_0_r;
            r.set(carry_0_r, field::map_object::SOLID, V::I(1))?;
            r.set(object_17, field::map_object::TRIGGER_VALUE, V::I(number_13))?;
            r.set(object_17, field::drawable::X, V::I(number_9))?;
            r.set(
                object_17,
                field::drawable::Y,
                V::I(number_14.wrapping_add(32)),
            )?;
            r.set(object_17, field::map_object::KIND, V::I(number_12))?;
            r.set(object_17, field::map_object::TEXT_INDEX, V::I(height))?;
            r.set(object_17, field::map_object::PERSISTENCE_ID, V::I(width))?;
            r.set(object_17, field::map_object::STATE_D, V::I((number_15 & 3)))?;
            (carry_0_r,) = (object_17,);
            if (number_15 & 4) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_E, V::I(carry_1_i))?;
            (carry_0_r,) = (object_17,);
            if (number_15 & 8) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_F, V::I(carry_1_i))?;
            (carry_0_r,) = (object_17,);
            if (number_15 & 16) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_G, V::I(carry_1_i))?;
            (carry_0_r,) = (object_17,);
            if (number_15 & 32) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_H, V::I(carry_1_i))?;
            (carry_0_r,) = (object_17,);
            if (number_15 & 64) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_N, V::I(carry_1_i))?;
            (carry_0_r,) = (object_17,);
            if (number_15 & 128) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_O, V::I(carry_1_i))?;
            (carry_0_r,) = (object_17,);
            if (number_16 & 1) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_P, V::I(carry_1_i))?;
            (carry_0_r,) = (object_17,);
            if (number_16 & 2) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_Q, V::I(carry_1_i))?;
            r.set(
                object_17,
                field::map_object::HAS_DIALOGUE,
                V::I((number_16 & 12).wrapping_shr((2) as u32)),
            )?;
            (carry_0_r,) = (object_17,);
            if (number_16 & 16) <= 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_V, V::I(carry_1_i))?;
            r.set(
                object_17,
                field::map_object::TRIGGER_KIND,
                V::I(((number_16 & 192).wrapping_shr((6) as u32)) as i8 as i32),
            )?;
            let map_object_trigger_kind = r.get(object_17, field::map_object::TRIGGER_KIND)?.i();
            if map_object_trigger_kind == 3 {
                let map_object_kind_5 = r.get(object_17, field::map_object::KIND)?.i();
                if map_object_kind_5 == 0 {
                    r.set(object_17, field::map_object::TRIGGER_KIND, V::I(0))?;
                }
            }
            let map_object_state_q = r.get(object_17, field::map_object::STATE_Q)?.i();
            (carry_0_r,) = (object_17,);
            if map_object_state_q != 0 {
                (carry_0_r, carry_1_i) = (carry_0_r, 0);
            } else {
                (carry_0_r, carry_1_i) = (carry_0_r, 1);
            }
            r.set(carry_0_r, field::map_object::STATE_Q, V::I(carry_1_i))?;
            let state_out_value_7 = r.static_get(field::system::STATE_OUT).r();
            let object_value_5 = r.alloc("java/lang/StringBuffer", Native::Empty);
            outcome = (|| -> JResult {
                host::string_buffer_initialize(r, object_value_5)?;
                Ok(V::I(0))
            })();
            (carry_0_r, carry_1_r) = (state_out_value_7, object_value_5);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let text_value_30 = r.intern("type= ");
            outcome = (|| -> JResult {
                Ok(V::R(host::string_buffer_append_2(
                    r,
                    carry_1_r,
                    text_value_30,
                )?))
            })();
            (carry_0_r,) = (carry_0_r,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let string_buffer_append_2_value_4 = outcome.unwrap().r();
            let map_object_kind_6 = r.get(object_17, field::map_object::KIND)?.i();
            outcome = (|| -> JResult {
                Ok(V::R(host::string_buffer_append(
                    r,
                    string_buffer_append_2_value_4,
                    map_object_kind_6,
                )?))
            })();
            (carry_0_r,) = (carry_0_r,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let string_buffer_append_value_2 = outcome.unwrap().r();
            let text_value_31 = r.intern(" ");
            outcome = (|| -> JResult {
                Ok(V::R(host::string_buffer_append_2(
                    r,
                    string_buffer_append_value_2,
                    text_value_31,
                )?))
            })();
            (carry_0_r,) = (carry_0_r,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let string_buffer_append_2_value_5 = outcome.unwrap().r();
            let map_object_state_q_2 = r.get(object_17, field::map_object::STATE_Q)?.i();
            outcome = (|| -> JResult {
                Ok(V::R(host::string_buffer_append_3(
                    r,
                    string_buffer_append_2_value_5,
                    map_object_state_q_2,
                )?))
            })();
            (carry_0_r,) = (carry_0_r,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let string_buffer_append_3_value = outcome.unwrap().r();
            outcome = (|| -> JResult {
                Ok(V::R(host::string_buffer_to_string(
                    r,
                    string_buffer_append_3_value,
                )?))
            })();
            (carry_0_r,) = (carry_0_r,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let string_buffer_to_string_value_3 = outcome.unwrap().r();
            outcome = (|| -> JResult {
                host::print_stream_println(r, carry_0_r, string_buffer_to_string_value_3)?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let map_object_kind_7 = r.get(object_17, field::map_object::KIND)?.i();
            if map_object_kind_7 > 0 {
                let map_object_kind_8 = r.get(object_17, field::map_object::KIND)?.i();
                if map_object_kind_8 == 17 {
                    r.set(object_17, field::map_object::STATE_Q, V::I(0))?;
                } else {
                    let map_object_kind_9 = r.get(object_17, field::map_object::KIND)?.i();
                    if map_object_kind_9 == 18 {
                        r.set(object_17, field::map_object::STATE_Q, V::I(0))?;
                    } else {
                        let world_resources_5 = r.get(world, field::world::RESOURCES)?.r();
                        let map_object_kind_10 = r.get(object_17, field::map_object::KIND)?.i();
                        outcome = (|| -> JResult {
                            Ok(V::R(super::resources::object_image(
                                r,
                                world_resources_5,
                                map_object_kind_10,
                            )?))
                        })();
                        (carry_0_r,) = (object_17,);
                        if outcome.is_err() {
                            carry_0_r = outcome.unwrap_err();
                            if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                                return Err(outcome.unwrap_err());
                            }
                            let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                            let text_value_3 = r.intern("File Not Found");
                            host::print_stream_println(r, state_out_value_3, text_value_3)?;
                            return Ok(1);
                        }
                        let object_image_value = outcome.unwrap().r();
                        r.set(
                            carry_0_r,
                            field::map_object::IMAGE,
                            V::R(object_image_value),
                        )?;
                    }
                }
            }
            height = 1;
            let quotient_value = arithmetic::divide_int(r, width, 32)?;
            number_9 = quotient_value;
            let remainder_value = arithmetic::remainder_int(r, width, 32)?;
            width = remainder_value;
            if numbered != 0 {
                let map_object_persistence_id =
                    r.get(object_17, field::map_object::PERSISTENCE_ID)?.i();
                if map_object_persistence_id != 0 {
                    if zone == 0 {
                        let player_overworld_events =
                            r.get(player, field::player::OVERWORLD_EVENTS)?.r();
                        let element_value_3 = r.array_get(player_overworld_events, number_9)?.i();
                        if !((element_value_3 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 1 {
                        let player_cave_events = r.get(player, field::player::CAVE_EVENTS)?.r();
                        let element_value_4 = r.array_get(player_cave_events, number_9)?.i();
                        if !((element_value_4 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 3 {
                        let player_dungeon_1_events =
                            r.get(player, field::player::DUNGEON_1_EVENTS)?.r();
                        let element_value_5 = r.array_get(player_dungeon_1_events, number_9)?.i();
                        if !((element_value_5 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 4 {
                        let player_dungeon_2_events =
                            r.get(player, field::player::DUNGEON_2_EVENTS)?.r();
                        let element_value_6 = r.array_get(player_dungeon_2_events, number_9)?.i();
                        if !((element_value_6 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 5 {
                        let player_dungeon_3_events =
                            r.get(player, field::player::DUNGEON_3_EVENTS)?.r();
                        let element_value_7 = r.array_get(player_dungeon_3_events, number_9)?.i();
                        if !((element_value_7 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 6 {
                        let player_dungeon_4_events =
                            r.get(player, field::player::DUNGEON_4_EVENTS)?.r();
                        let element_value_8 = r.array_get(player_dungeon_4_events, number_9)?.i();
                        if !((element_value_8 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 7 {
                        let player_dungeon_5_events =
                            r.get(player, field::player::DUNGEON_5_EVENTS)?.r();
                        let element_value_9 = r.array_get(player_dungeon_5_events, number_9)?.i();
                        if !((element_value_9 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 8 {
                        let player_dungeon_6_events =
                            r.get(player, field::player::DUNGEON_6_EVENTS)?.r();
                        let element_value_10 = r.array_get(player_dungeon_6_events, number_9)?.i();
                        if !((element_value_10 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 9 {
                        let player_dungeon_7_events =
                            r.get(player, field::player::DUNGEON_7_EVENTS)?.r();
                        let element_value_11 = r.array_get(player_dungeon_7_events, number_9)?.i();
                        if !((element_value_11 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 10 {
                        let player_dungeon_8_events =
                            r.get(player, field::player::DUNGEON_8_EVENTS)?.r();
                        let element_value_12 = r.array_get(player_dungeon_8_events, number_9)?.i();
                        if !((element_value_12 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }

                    if zone == 11 {
                        let player_dungeon_9_events =
                            r.get(player, field::player::DUNGEON_9_EVENTS)?.r();
                        let element_value_13 = r.array_get(player_dungeon_9_events, number_9)?.i();
                        if !((element_value_13 & (1_i32).wrapping_shl((width) as u32)) <= 0) {
                            height = 0;
                        }
                    }
                }
            }

            if height != 0 {
                let map_object_state_d_2 = r.get(object_17, field::map_object::STATE_D)?.i();
                if map_object_state_d_2 > 1 {
                    let map_object_kind_11 = r.get(object_17, field::map_object::KIND)?.i();
                    if map_object_kind_11 == 18 {
                        let world_objects_17 = r.get(world, field::world::OBJECTS)?.r();
                        host::vector_add_element(r, world_objects_17, object_17)?;
                    }
                    let world_overlay_objects_2 = r.get(world, field::world::OVERLAY_OBJECTS)?.r();
                    host::vector_add_element(r, world_overlay_objects_2, object_17)?;
                } else {
                    let world_objects_16 = r.get(world, field::world::OBJECTS)?.r();
                    host::vector_add_element(r, world_objects_16, object_17)?;
                }
            }
            number_11 = number_11.wrapping_add(1);
            continue 'load_map_loop_31;
        }
    }
    outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_byte(r, path)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
        let text_value_3 = r.intern("File Not Found");
        host::print_stream_println(r, state_out_value_3, text_value_3)?;
        return Ok(1);
    }
    let data_input_stream_read_byte_value_10 = outcome.unwrap().i();
    number_11 = data_input_stream_read_byte_value_10;
    if data_input_stream_read_byte_value_10 > 0 {
        number_12 = 0;
        'load_map_loop_40: loop {
            if number_12 >= number_11 {
                break 'load_map_loop_40;
            }
            outcome =
                (|| -> JResult { Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?)) })(
                );
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let data_input_stream_read_unsigned_byte_value_19 = outcome.unwrap().i();
            number_13 = data_input_stream_read_unsigned_byte_value_19;
            let array_value_3 = r.new_array(
                5,
                data_input_stream_read_unsigned_byte_value_19.wrapping_add(1),
            )?;
            object_7 = array_value_3;
            number_9 = 0;
            'load_map_loop_42: loop {
                if number_9 >= number_13 {
                    break 'load_map_loop_42;
                }
                outcome = (|| -> JResult {
                    Ok(V::I(host::data_input_stream_read_unsigned_byte(r, path)?))
                })();
                (carry_0_r, carry_1_i) = (object_7, number_9);
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                    let text_value_3 = r.intern("File Not Found");
                    host::print_stream_println(r, state_out_value_3, text_value_3)?;
                    return Ok(1);
                }
                let data_input_stream_read_unsigned_byte_value_20 = outcome.unwrap().i();
                r.array_set(
                    carry_0_r,
                    carry_1_i,
                    V::I((data_input_stream_read_unsigned_byte_value_20) as u16 as i32),
                )?;
                number_9 = number_9.wrapping_add(1);
                continue 'load_map_loop_42;
            }
            r.array_set(object_7, number_13, V::I(32))?;
            let object_value_4 = r.alloc("java/lang/String", Native::Empty);
            outcome = (|| -> JResult {
                host::string_initialize(r, object_value_4, object_7)?;
                Ok(V::I(0))
            })();
            (carry_0_r,) = (object_value_4,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            object_9 = carry_0_r;
            let world_dialogue_2 = r.get(world, field::world::DIALOGUE)?.r();
            host::vector_add_element(r, world_dialogue_2, object_9)?;
            let state_out_value_6 = r.static_get(field::system::STATE_OUT).r();
            outcome = (|| -> JResult {
                host::print_stream_println(r, state_out_value_6, object_9)?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            number_12 = number_12.wrapping_add(1);
            continue 'load_map_loop_40;
        }
    }
    number_12 = 0;
    loop {
        let world_objects_2 = r.get(world, field::world::OBJECTS)?.r();
        let vector_size_value_4 = host::vector_size(r, world_objects_2)?;
        if number_12 >= vector_size_value_4 {
            if (zone == 3) && (map == 11) {
                r.set(world, field::world::STATE_M, V::I(1))?;
            }

            if (zone == 4) && (map == 3) {
                r.set(world, field::world::STATE_M, V::I(1))?;
            }

            if (zone == 5) && (map == 15) {
                r.set(world, field::world::STATE_M, V::I(1))?;
            }

            if (zone == 6) && (map == 8) {
                r.set(world, field::world::STATE_M, V::I(1))?;
            }

            if (zone == 7) && (map == 9) {
                r.set(world, field::world::STATE_M, V::I(1))?;
            }

            if (zone == 8) && (map == 11) {
                r.set(world, field::world::STATE_M, V::I(1))?;
            }

            if (zone == 9) && (map == 15) {
                r.set(world, field::world::STATE_M, V::I(1))?;
            }

            if (zone == 10) && (map == 17) {
                r.set(world, field::world::STATE_M, V::I(1))?;
            }

            if zone != 12 {
                return Ok(1);
            }
            number_12 = 0;
            if map == 20 {
                let player_progression_flags_4 =
                    r.get(player, field::player::PROGRESSION_FLAGS)?.i();
                if !((player_progression_flags_4 & 33554432) != 0) {
                    let player_progression_flags_6 =
                        r.get(player, field::player::PROGRESSION_FLAGS)?.i();
                    if (player_progression_flags_6 & 16777216) <= 0 {
                        number_12 = 1;
                    } else {
                        number_12 = 0;
                    }
                    let player_progression_flags_7 =
                        r.get(player, field::player::PROGRESSION_FLAGS)?.i();
                    r.set(
                        player,
                        field::player::PROGRESSION_FLAGS,
                        V::I((player_progression_flags_7 | 16777216)),
                    )?;
                }
            }
            let player_progression_flags_3 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            if (player_progression_flags_3 & 16777216) <= 0 {
                return Ok(1);
            }
            let player_progression_flags_5 = r.get(player, field::player::PROGRESSION_FLAGS)?.i();
            if (player_progression_flags_5 & 33554432) != 0 {
                return Ok(1);
            }

            if number_12 != 0 {
                return Ok(1);
            }
            let enemy_value_2 = r.alloc("l", Native::Empty);
            let world_resources_4 = r.get(world, field::world::RESOURCES)?.r();
            outcome = (|| -> JResult {
                Ok(V::R(super::resources::enemy_image(
                    r,
                    world_resources_4,
                    133,
                )?))
            })();
            (carry_0_r, carry_1_r) = (enemy_value_2, enemy_value_2);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let enemy_image_value_2 = outcome.unwrap().r();
            outcome = (|| -> JResult {
                super::enemy::initialize(r, carry_1_r, enemy_image_value_2, 1000, 1000, 1, 133)?;
                Ok(V::I(0))
            })();
            (carry_0_r,) = (carry_0_r,);
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            object_9 = carry_0_r;
            let world_enemies_4 = r.get(world, field::world::ENEMIES)?.r();
            host::vector_add_element(r, world_enemies_4, object_9)?;
            return Ok(1);
        }
        let world_objects_3 = r.get(world, field::world::OBJECTS)?.r();
        let vector_element_at_value_2 = host::vector_element_at(r, world_objects_3, number_12)?;
        host::check_cast(r, vector_element_at_value_2, "n")?;
        let kind_value = r
            .get(vector_element_at_value_2, field::map_object::KIND)?
            .i();
        if kind_value == 204 {
            let world_objects_5 = r.get(world, field::world::OBJECTS)?.r();
            let vector_element_at_value_4 = host::vector_element_at(r, world_objects_5, number_12)?;
            host::check_cast(r, vector_element_at_value_4, "n")?;
            outcome = (|| -> JResult {
                super::map_object::draw(r, vector_element_at_value_4, graphics, 0, 0, 256, 256)?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let world_objects_6 = r.get(world, field::world::OBJECTS)?.r();
            let vector_element_at_value_5 = host::vector_element_at(r, world_objects_6, number_12)?;
            host::check_cast(r, vector_element_at_value_5, "n")?;
            r.set(
                vector_element_at_value_5,
                field::map_object::STATE_Z,
                V::I(1),
            )?;
        }
        let world_objects_4 = r.get(world, field::world::OBJECTS)?.r();
        let vector_element_at_value_3 = host::vector_element_at(r, world_objects_4, number_12)?;
        host::check_cast(r, vector_element_at_value_3, "n")?;
        let state_d_value = r
            .get(vector_element_at_value_3, field::map_object::STATE_D)?
            .i();
        if state_d_value == 0 {
            let world_objects_7 = r.get(world, field::world::OBJECTS)?.r();
            let vector_element_at_value_6 = host::vector_element_at(r, world_objects_7, number_12)?;
            host::check_cast(r, vector_element_at_value_6, "n")?;
            outcome = (|| -> JResult {
                super::map_object::draw(r, vector_element_at_value_6, graphics, 0, 32, 256, 256)?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                    return Err(outcome.unwrap_err());
                }
                let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                let text_value_3 = r.intern("File Not Found");
                host::print_stream_println(r, state_out_value_3, text_value_3)?;
                return Ok(1);
            }
            let world_objects_8 = r.get(world, field::world::OBJECTS)?.r();
            let vector_element_at_value_7 = host::vector_element_at(r, world_objects_8, number_12)?;
            host::check_cast(r, vector_element_at_value_7, "n")?;
            r.set(
                vector_element_at_value_7,
                field::map_object::STATE_Z,
                V::I(1),
            )?;
        }
        let world_canvas_4 = r.get(world, field::world::CANVAS)?.r();
        let canvas_state_x_2 = r.get(world_canvas_4, field::canvas::STATE_X)?.i();
        if canvas_state_x_2 == 0 {
            let world_objects_9 = r.get(world, field::world::OBJECTS)?.r();
            let vector_element_at_value_8 = host::vector_element_at(r, world_objects_9, number_12)?;
            host::check_cast(r, vector_element_at_value_8, "n")?;
            let kind_value_2 = r
                .get(vector_element_at_value_8, field::map_object::KIND)?
                .i();
            if kind_value_2 >= 192 {
                let world_objects_11 = r.get(world, field::world::OBJECTS)?.r();
                let vector_element_at_value_10 =
                    host::vector_element_at(r, world_objects_11, number_12)?;
                host::check_cast(r, vector_element_at_value_10, "n")?;
                let kind_value_4 = r
                    .get(vector_element_at_value_10, field::map_object::KIND)?
                    .i();
                if kind_value_4 <= 203 {
                    let world_objects_14 = r.get(world, field::world::OBJECTS)?.r();
                    let vector_element_at_value_13 =
                        host::vector_element_at(r, world_objects_14, number_12)?;
                    host::check_cast(r, vector_element_at_value_13, "n")?;
                    outcome = (|| -> JResult {
                        super::map_object::draw(
                            r,
                            vector_element_at_value_13,
                            graphics,
                            0,
                            32,
                            256,
                            256,
                        )?;
                        Ok(V::I(0))
                    })();
                    if outcome.is_err() {
                        carry_0_r = outcome.unwrap_err();
                        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                            return Err(outcome.unwrap_err());
                        }
                        let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                        let text_value_3 = r.intern("File Not Found");
                        host::print_stream_println(r, state_out_value_3, text_value_3)?;
                        return Ok(1);
                    }
                    let world_objects_15 = r.get(world, field::world::OBJECTS)?.r();
                    let vector_element_at_value_14 =
                        host::vector_element_at(r, world_objects_15, number_12)?;
                    host::check_cast(r, vector_element_at_value_14, "n")?;
                    r.set(
                        vector_element_at_value_14,
                        field::map_object::STATE_Z,
                        V::I(1),
                    )?;
                }
            }
            let world_objects_10 = r.get(world, field::world::OBJECTS)?.r();
            let vector_element_at_value_9 =
                host::vector_element_at(r, world_objects_10, number_12)?;
            host::check_cast(r, vector_element_at_value_9, "n")?;
            let kind_value_3 = r
                .get(vector_element_at_value_9, field::map_object::KIND)?
                .i();
            if kind_value_3 == 28 {
                let world_objects_12 = r.get(world, field::world::OBJECTS)?.r();
                let vector_element_at_value_11 =
                    host::vector_element_at(r, world_objects_12, number_12)?;
                host::check_cast(r, vector_element_at_value_11, "n")?;
                outcome = (|| -> JResult {
                    super::map_object::draw(
                        r,
                        vector_element_at_value_11,
                        graphics,
                        0,
                        32,
                        256,
                        256,
                    )?;
                    Ok(V::I(0))
                })();
                if outcome.is_err() {
                    carry_0_r = outcome.unwrap_err();
                    if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                        return Err(outcome.unwrap_err());
                    }
                    let state_out_value_3 = r.static_get(field::system::STATE_OUT).r();
                    let text_value_3 = r.intern("File Not Found");
                    host::print_stream_println(r, state_out_value_3, text_value_3)?;
                    return Ok(1);
                }
                let world_objects_13 = r.get(world, field::world::OBJECTS)?.r();
                let vector_element_at_value_12 =
                    host::vector_element_at(r, world_objects_13, number_12)?;
                host::check_cast(r, vector_element_at_value_12, "n")?;
                r.set(
                    vector_element_at_value_12,
                    field::map_object::STATE_Z,
                    V::I(1),
                )?;
            }
        }
        number_12 = number_12.wrapping_add(1);
    }
}

/// Mark persistent event. Original reference: `j.b(IILh;)V`.
pub(in crate::game) fn mark_persistent_event(
    r: &mut Runtime,
    world: u32,
    event: i32,
    mut room: i32,
    player: u32,
) -> Result<(), u32> {
    let mut number_4: i32 = 0;
    let quotient_value = arithmetic::divide_int(r, room, 32)?;
    room = quotient_value;
    let remainder_value = arithmetic::remainder_int(r, event, 32)?;
    number_4 = remainder_value;
    r.set(world, field::world::TEXT_FRAME, V::I(0))?;
    if event == 0 {
        return Ok(());
    }
    let world_zone = r.get(world, field::world::ZONE)?.i();
    if world_zone == 0 {
        let player_overworld_events = r.get(player, field::player::OVERWORLD_EVENTS)?.r();
        let element_value = r.array_get(player_overworld_events, room)?.i();
        r.array_set(
            player_overworld_events,
            room,
            V::I((element_value | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_2 = r.get(world, field::world::ZONE)?.i();
    if world_zone_2 == 1 {
        let player_cave_events = r.get(player, field::player::CAVE_EVENTS)?.r();
        let element_value_2 = r.array_get(player_cave_events, room)?.i();
        r.array_set(
            player_cave_events,
            room,
            V::I((element_value_2 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_3 = r.get(world, field::world::ZONE)?.i();
    if world_zone_3 == 3 {
        let player_dungeon_1_events = r.get(player, field::player::DUNGEON_1_EVENTS)?.r();
        let element_value_3 = r.array_get(player_dungeon_1_events, room)?.i();
        r.array_set(
            player_dungeon_1_events,
            room,
            V::I((element_value_3 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_4 = r.get(world, field::world::ZONE)?.i();
    if world_zone_4 == 4 {
        let player_dungeon_2_events = r.get(player, field::player::DUNGEON_2_EVENTS)?.r();
        let element_value_4 = r.array_get(player_dungeon_2_events, room)?.i();
        r.array_set(
            player_dungeon_2_events,
            room,
            V::I((element_value_4 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_5 = r.get(world, field::world::ZONE)?.i();
    if world_zone_5 == 5 {
        let player_dungeon_3_events = r.get(player, field::player::DUNGEON_3_EVENTS)?.r();
        let element_value_5 = r.array_get(player_dungeon_3_events, room)?.i();
        r.array_set(
            player_dungeon_3_events,
            room,
            V::I((element_value_5 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_6 = r.get(world, field::world::ZONE)?.i();
    if world_zone_6 == 6 {
        let player_dungeon_4_events = r.get(player, field::player::DUNGEON_4_EVENTS)?.r();
        let element_value_6 = r.array_get(player_dungeon_4_events, room)?.i();
        r.array_set(
            player_dungeon_4_events,
            room,
            V::I((element_value_6 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_7 = r.get(world, field::world::ZONE)?.i();
    if world_zone_7 == 7 {
        let player_dungeon_5_events = r.get(player, field::player::DUNGEON_5_EVENTS)?.r();
        let element_value_7 = r.array_get(player_dungeon_5_events, room)?.i();
        r.array_set(
            player_dungeon_5_events,
            room,
            V::I((element_value_7 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_8 = r.get(world, field::world::ZONE)?.i();
    if world_zone_8 == 8 {
        let player_dungeon_6_events = r.get(player, field::player::DUNGEON_6_EVENTS)?.r();
        let element_value_8 = r.array_get(player_dungeon_6_events, room)?.i();
        r.array_set(
            player_dungeon_6_events,
            room,
            V::I((element_value_8 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_9 = r.get(world, field::world::ZONE)?.i();
    if world_zone_9 == 9 {
        let player_dungeon_7_events = r.get(player, field::player::DUNGEON_7_EVENTS)?.r();
        let element_value_9 = r.array_get(player_dungeon_7_events, room)?.i();
        r.array_set(
            player_dungeon_7_events,
            room,
            V::I((element_value_9 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_10 = r.get(world, field::world::ZONE)?.i();
    if world_zone_10 == 10 {
        let player_dungeon_8_events = r.get(player, field::player::DUNGEON_8_EVENTS)?.r();
        let element_value_10 = r.array_get(player_dungeon_8_events, room)?.i();
        r.array_set(
            player_dungeon_8_events,
            room,
            V::I((element_value_10 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
    }
    let world_zone_11 = r.get(world, field::world::ZONE)?.i();
    if world_zone_11 == 11 {
        let player_dungeon_9_events_read = r.get(player, field::player::DUNGEON_9_EVENTS)?.r();
        let element_value_11 = r.array_get(player_dungeon_9_events_read, room)?.i();
        r.array_set(
            player_dungeon_9_events_read,
            room,
            V::I((element_value_11 | (1_i32).wrapping_shl((number_4) as u32))),
        )?;
        return Ok(());
    }
    let world_zone_12 = r.get(world, field::world::ZONE)?.i();
    if world_zone_12 != 20 {
        return Ok(());
    }
    let player_dungeon_9_events_read = r.get(player, field::player::DUNGEON_9_EVENTS)?.r();
    let element_value_11 = r.array_get(player_dungeon_9_events_read, room)?.i();
    r.array_set(
        player_dungeon_9_events_read,
        room,
        V::I((element_value_11 | (1_i32).wrapping_shl((number_4) as u32))),
    )?;
    return Ok(());
}
