//! Scenery, triggers and scripted interactions.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `n.<init>(I)V`.
pub(super) fn initialize(r: &mut Runtime, object: u32, kind: i32) -> Result<(), u32> {
    let mut object_1: u32 = 0;
    super::drawable::initialize(r, object)?;
    r.set(object, field::map_object::STATE_Z, V::I(0))?;
    r.set(object, field::map_object::STATE_C, V::I(0))?;
    r.set(object, field::map_object::KIND, V::I(kind))?;
    r.set(object, field::map_object::STATE_L, V::I(1))?;
    r.set(object, field::map_object::STATE_B, V::I(0))?;
    r.set(object, field::map_object::STATE_D_UPPER, V::I(0))?;
    r.set(object, field::map_object::STATE_E_UPPER, V::I(0))?;
    r.set(object, field::map_object::WIDTH, V::I(0))?;
    r.set(object, field::map_object::HEIGHT, V::I(0))?;
    r.set(object, field::map_object::WIDTH, V::I(16))?;
    r.set(object, field::map_object::HEIGHT, V::I(16))?;
    r.set(object, field::map_object::STATE_D_UPPER, V::I(0))?;
    r.set(object, field::map_object::STATE_E_UPPER, V::I(0))?;
    r.set(object, field::map_object::STATE_I, V::I(1))?;
    r.set(object, field::map_object::STATE_Y, V::I(1))?;
    r.set(object, field::map_object::STATE_Z, V::I(0))?;
    let map_object_kind = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind == 12 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-7))?;
        r.set(object, field::map_object::STATE_D_UPPER, V::I(0))?;
        r.set(object, field::map_object::WIDTH, V::I(32))?;
        r.set(object, field::map_object::HEIGHT, V::I(32))?;
    }
    let map_object_kind_2 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_2 == 62 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-7))?;
        r.set(object, field::map_object::STATE_D_UPPER, V::I(0))?;
        r.set(object, field::map_object::WIDTH, V::I(32))?;
        r.set(object, field::map_object::HEIGHT, V::I(32))?;
    }
    let map_object_kind_3 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_3 == 92 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-100))?;
        r.set(object, field::map_object::STATE_D_UPPER, V::I(0))?;
        r.set(object, field::map_object::WIDTH, V::I(64))?;
        r.set(object, field::map_object::HEIGHT, V::I(80))?;
    }
    let map_object_kind_4 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_4 == 29 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(0))?;
        r.set(object, field::map_object::STATE_D_UPPER, V::I(0))?;
        r.set(object, field::map_object::WIDTH, V::I(32))?;
        r.set(object, field::map_object::HEIGHT, V::I(32))?;
    }
    let map_object_kind_5 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_5 == 30 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(0))?;
        r.set(object, field::map_object::STATE_D_UPPER, V::I(0))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_6 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_6 == 64 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_7 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_7 == 65 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_8 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_8 == 66 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_9 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_9 == 67 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_10 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_10 == 68 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_11 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_11 == 69 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_12 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_12 == 70 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_13 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_13 == 71 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_14 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_14 == 77 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(44))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_15 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_15 == 78 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_16 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_16 == 96 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_17 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_17 == 97 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_18 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_18 == 98 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_19 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_19 == 99 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_20 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_20 == 100 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_21 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_21 == 101 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_22 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_22 == 102 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_23 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_23 == 103 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_24 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_24 == 112 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_25 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_25 == 113 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_26 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_26 == 114 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_27 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_27 == 115 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_28 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_28 == 116 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_29 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_29 == 117 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_30 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_30 == 118 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_31 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_31 == 119 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_32 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_32 == 128 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_33 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_33 == 129 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_34 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_34 == 130 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_35 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_35 == 131 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_36 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_36 == 132 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_37 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_37 == 133 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_38 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_38 == 134 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_39 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_39 == 135 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_40 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_40 == 144 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_41 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_41 == 145 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_42 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_42 == 146 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_43 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_43 == 147 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_44 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_44 == 148 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_45 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_45 == 149 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_46 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_46 == 150 {
        r.set(object, field::map_object::STATE_D_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(24))?;
        r.set(object, field::map_object::HEIGHT, V::I(16))?;
    }
    let map_object_kind_47 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_47 == 151 {
        r.set(object, field::map_object::STATE_E_UPPER, V::I(-4))?;
        r.set(object, field::map_object::WIDTH, V::I(16))?;
        r.set(object, field::map_object::HEIGHT, V::I(24))?;
    }
    let map_object_kind_48 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_48 < 192 {
        return Ok(());
    }
    let map_object_kind_49 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_49 > 205 {
        return Ok(());
    }
    let object_value = r.alloc("java/util/Random", Native::Empty);
    host::random_initialize(r, object_value)?;
    object_1 = object_value;
    let random_next_int_value = host::random_next_int(r, object_1)?;
    let remainder_value = arithmetic::remainder_int(r, random_next_int_value, 8)?;
    let math_abs_value = host::math_abs(r, remainder_value)?;
    r.set(
        object,
        field::map_object::STATE_B,
        V::I((math_abs_value) as i8 as i32),
    )?;
    return Ok(());
}

/// Draw. Original reference: `n.a(Ljavax/microedition/lcdui/Graphics;IIII)V`.
pub(super) fn draw(
    r: &mut Runtime,
    object: u32,
    graphics: u32,
    mut x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<(), u32> {
    'check_map_object_kind_7: {
        'check_map_object_kind_9: {
            'check_map_object_kind_8: {
                'check_map_object_kind_5: {
                    'check_map_object_kind_6: {
                        'check_map_object_kind_3: {
                            'check_map_object_kind_4: {
                                'check_map_object_kind: {
                                    'check_map_object_kind_2: {
                                        let map_object_state_q =
                                            r.get(object, field::map_object::STATE_Q)?.i();
                                        if map_object_state_q == 0 {
                                            return Ok(());
                                        }
                                        let map_object_kind =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind == 0 {
                                            return Ok(());
                                        }
                                        let map_object_image =
                                            r.get(object, field::map_object::IMAGE)?.r();
                                        if map_object_image == 0 {
                                            return Ok(());
                                        }
                                        let drawable_x = r.get(object, field::drawable::X)?.i();
                                        if !(width.wrapping_add(x) >= drawable_x) {
                                            return Ok(());
                                        }
                                        let drawable_x_2 = r.get(object, field::drawable::X)?.i();
                                        if !(drawable_x_2.wrapping_add(32) >= x) {
                                            let map_object_kind_2 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_2 != 92 {
                                                return Ok(());
                                            }
                                        }
                                        let drawable_y = r.get(object, field::drawable::Y)?.i();
                                        if !(height.wrapping_add(y) >= drawable_y) {
                                            let map_object_kind_3 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_3 != 222 {
                                                return Ok(());
                                            }
                                        }
                                        let drawable_y_2 = r.get(object, field::drawable::Y)?.i();
                                        if !((drawable_y_2.wrapping_add(32)).wrapping_sub(32) >= y)
                                        {
                                            return Ok(());
                                        }
                                        let map_object_solid =
                                            r.get(object, field::map_object::SOLID)?.i();
                                        if map_object_solid == 0 {
                                            return Ok(());
                                        }
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
                                        let drawable_x_3 = r.get(object, field::drawable::X)?.i();
                                        let drawable_y_3 = r.get(object, field::drawable::Y)?.i();
                                        host::graphics_translate(
                                            r,
                                            graphics,
                                            drawable_x_3.wrapping_sub(x),
                                            drawable_y_3.wrapping_sub(y),
                                        )?;
                                        let map_object_kind_4 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_4 == 3 {
                                            x = 0;
                                            let map_object_state_x =
                                                r.get(object, field::map_object::STATE_X)?.i();
                                            if map_object_state_x != 0 {
                                                let map_object_state_a =
                                                    r.get(object, field::map_object::STATE_A)?.i();
                                                if map_object_state_a >= 2 {
                                                    x = 1;
                                                }
                                                let map_object_state_a_3 =
                                                    r.get(object, field::map_object::STATE_A)?.i();
                                                if map_object_state_a_3 < 2 {
                                                    x = 2;
                                                }
                                            }
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_2 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_2,
                                                x.wrapping_mul(-16),
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_5 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_5 == 4 {
                                            x = 0;
                                            let map_object_state_x_2 =
                                                r.get(object, field::map_object::STATE_X)?.i();
                                            if map_object_state_x_2 != 0 {
                                                let map_object_state_a_2 =
                                                    r.get(object, field::map_object::STATE_A)?.i();
                                                if map_object_state_a_2 >= 2 {
                                                    x = 1;
                                                }
                                                let map_object_state_a_4 =
                                                    r.get(object, field::map_object::STATE_A)?.i();
                                                if map_object_state_a_4 < 2 {
                                                    x = 2;
                                                }
                                            }
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_4 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_4,
                                                x.wrapping_mul(-16),
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_6 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_6 == 5 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_3 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_3,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_7 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_7 == 6 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_5 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_5,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b.wrapping_shr((2) as u32),
                                                ),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_b_2 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_2.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_3 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_3 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_8 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_8 == 7 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_6 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_4 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_6,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b_4.wrapping_shr((2) as u32),
                                                ),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_b_5 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_5.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_6 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_6 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_9 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_9 == 8 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_7 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_7 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_7,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b_7.wrapping_shr((2) as u32),
                                                ),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_b_8 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_8.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_9 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_9 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_10 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_10 == 9 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_8 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_10 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_8,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b_10.wrapping_shr((2) as u32),
                                                ),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_b_11 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_11.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_12 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_12 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_11 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_11 == 10 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_9 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_13 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_9,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b_13.wrapping_shr((2) as u32),
                                                ),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_b_14 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_14.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_15 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_15 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_12 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_12 == 11 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_10 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_10,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_13 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_13 == 12 {
                                            let state_d_upper_value = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value,
                                                state_e_upper_value,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 32, 32)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 32, 32)?;
                                            let map_object_image_11 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_11,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_14 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_14 == 13 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_12 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_12,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_15 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_15 == 14 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_13 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_13,
                                                0,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_16 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_16 == 15 {
                                            x = 0;
                                            let map_object_state_x_3 =
                                                r.get(object, field::map_object::STATE_X)?.i();
                                            if map_object_state_x_3 != 0 {
                                                let map_object_state_a_5 =
                                                    r.get(object, field::map_object::STATE_A)?.i();
                                                if map_object_state_a_5 >= 2 {
                                                    x = 1;
                                                }
                                                let map_object_state_a_6 =
                                                    r.get(object, field::map_object::STATE_A)?.i();
                                                if map_object_state_a_6 < 2 {
                                                    x = 2;
                                                }
                                            }
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_15 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_15,
                                                x.wrapping_mul(-16),
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_17 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_17 == 16 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_14 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_14,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_18 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_18 == 19 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_16 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_16 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_16,
                                                0,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b_16.wrapping_shr((2) as u32),
                                                ),
                                                0,
                                            )?;
                                            let map_object_state_b_17 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_17.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_18 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_18 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_19 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_19 == 20 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_17 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_17,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_20 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_20 >= 21 {
                                            let map_object_kind_22 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_22 <= 24 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_19 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_24 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_19,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_24.wrapping_sub(21),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_21 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_21 == 25 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_18 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_19 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_18,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b_19.wrapping_shr((3) as u32),
                                                ),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_b_20 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_20.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_21 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_21 > 30 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_23 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_23 >= 26 {
                                            let map_object_kind_26 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_26 <= 27 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_21 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_28 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_21,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_28.wrapping_sub(26),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_25 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_25 == 28 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_20 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_22 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_20,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b_22.wrapping_shr((2) as u32),
                                                ),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_b_23 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_23.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_24 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_24 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_27 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_27 == 29 {
                                            let state_d_upper_value_2 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_2 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_2,
                                                state_e_upper_value_2,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 32, 32)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 32, 32)?;
                                            let map_object_image_22 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_22,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_29 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_29 == 30 {
                                            let state_d_upper_value_3 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_3 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_3,
                                                state_e_upper_value_3,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_23 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_c =
                                                r.get(object, field::map_object::STATE_C)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_23,
                                                (-16_i32).wrapping_mul(map_object_state_c),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_m =
                                                r.get(object, field::map_object::STATE_M)?.i();
                                            if map_object_state_m > 0 {
                                                let map_object_state_m_2 =
                                                    r.get(object, field::map_object::STATE_M)?.i();
                                                r.set(
                                                    object,
                                                    field::map_object::STATE_M,
                                                    V::I(map_object_state_m_2.wrapping_sub(1)),
                                                )?;
                                            }
                                        }
                                        let map_object_kind_30 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_30 == 32 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_24 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_24,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_31 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_31 >= 34 {
                                            let map_object_kind_33 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_33 <= 42 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_25 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_36 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_36.wrapping_sub(34),
                                                    3,
                                                )?;
                                                let map_object_kind_37 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let quotient_value = arithmetic::divide_int(
                                                    r,
                                                    map_object_kind_37.wrapping_sub(34),
                                                    3,
                                                )?;
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_25,
                                                    (-16_i32).wrapping_mul(remainder_value),
                                                    (-16_i32).wrapping_mul(quotient_value),
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_32 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_32 >= 43 {
                                            let map_object_kind_35 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_35 <= 45 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_26 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_39 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_2 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_39.wrapping_sub(43),
                                                    3,
                                                )?;
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_26,
                                                    (-16_i32).wrapping_mul(remainder_value_2),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_34 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_34 == 48 {
                                            x = 0;
                                            let trigger_active_value = r
                                                .get(object, field::map_object::TRIGGER_ACTIVE)?
                                                .i();
                                            if trigger_active_value != 0 {
                                                x = 16;
                                            }
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_28 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_28,
                                                x.wrapping_neg(),
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_38 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_38 == 46 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_27 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_27,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_40 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_40 == 47 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_29 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_25 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            let remainder_value_3 = arithmetic::remainder_int(
                                                r,
                                                map_object_state_b_25.wrapping_shr((1) as u32),
                                                2,
                                            )?;
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_29,
                                                (-16_i32).wrapping_mul(remainder_value_3),
                                                0,
                                                0,
                                            )?;
                                            let map_object_state_b_26 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_26.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_27 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_27 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_41 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_41 >= 52 {
                                            let map_object_kind_43 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_43 <= 59 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_30 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_45 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_30,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_45.wrapping_sub(52),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_42 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_42 == 60 {
                                            let state_d_upper_value_4 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_4 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_4,
                                                state_e_upper_value_4,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let trigger_active_value_2 = r
                                                .get(object, field::map_object::TRIGGER_ACTIVE)?
                                                .i();
                                            if trigger_active_value_2 != 0 {
                                                let map_object_image_32 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_32,
                                                    -48,
                                                    0,
                                                    0,
                                                )?;
                                            } else {
                                                let state_h_upper_value = r
                                                    .get(object, field::map_object::STATE_H_UPPER)?
                                                    .i();
                                                let drawable_x_4 =
                                                    r.get(object, field::drawable::X)?.i();
                                                if state_h_upper_value
                                                    >= drawable_x_4.wrapping_sub(30)
                                                {
                                                    let state_h_upper_value_2 = r
                                                        .get(
                                                            object,
                                                            field::map_object::STATE_H_UPPER,
                                                        )?
                                                        .i();
                                                    let drawable_x_5 =
                                                        r.get(object, field::drawable::X)?.i();
                                                    if state_h_upper_value_2
                                                        <= drawable_x_5.wrapping_add(30)
                                                    {
                                                        let image_value_36 = r
                                                            .get(object, field::map_object::IMAGE)?
                                                            .r();
                                                        host::graphics_draw_image(
                                                            r,
                                                            graphics,
                                                            image_value_36,
                                                            0,
                                                            0,
                                                            0,
                                                        )?;
                                                    } else {
                                                        let image_value_37 = r
                                                            .get(object, field::map_object::IMAGE)?
                                                            .r();
                                                        host::graphics_draw_image(
                                                            r,
                                                            graphics,
                                                            image_value_37,
                                                            -32,
                                                            0,
                                                            0,
                                                        )?;
                                                    }
                                                } else {
                                                    let image_value_34 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    host::graphics_draw_image(
                                                        r,
                                                        graphics,
                                                        image_value_34,
                                                        -16,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                            }
                                        }
                                        let map_object_kind_44 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_44 == 61 {
                                            let state_d_upper_value_5 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_5 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_5,
                                                state_e_upper_value_5,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_31 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_31,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_46 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_46 == 62 {
                                            let state_d_upper_value_6 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_6 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_6,
                                                state_e_upper_value_6,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 32, 32)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 32, 32)?;
                                            let map_object_image_33 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_33,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_47 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_47 == 63 {
                                            host::graphics_translate(r, graphics, -8, 0)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 32, 32)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 32, 32)?;
                                            let map_object_image_35 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_35,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_48 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_48 == 64 {
                                            let state_d_upper_value_7 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_7 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_7,
                                                state_e_upper_value_7,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_38 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_38,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_49 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_49 == 65 {
                                            let state_d_upper_value_8 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_8 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_8,
                                                state_e_upper_value_8,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_39 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_39,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_50 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_50 == 66 {
                                            let state_d_upper_value_9 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_9 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_9,
                                                state_e_upper_value_9,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_40 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_40,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_51 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_51 == 67 {
                                            let state_d_upper_value_10 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_10 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_10,
                                                state_e_upper_value_10,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_41 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_41,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_52 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_52 == 68 {
                                            let state_d_upper_value_11 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_11 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_11,
                                                state_e_upper_value_11,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_42 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_42,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_53 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_53 == 69 {
                                            let state_d_upper_value_12 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_12 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_12,
                                                state_e_upper_value_12,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_43 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_43,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_54 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_54 == 70 {
                                            let state_d_upper_value_13 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_13 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_13,
                                                state_e_upper_value_13,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_44 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_44,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_55 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_55 == 71 {
                                            let state_d_upper_value_14 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_14 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_14,
                                                state_e_upper_value_14,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_45 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_45,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_56 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_56 == 72 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_46 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_46,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_57 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_57 == 73 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_47 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_47,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_58 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_58 == 74 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_48 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_48,
                                                -32,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_59 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_59 == 75 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_49 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_49,
                                                -48,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_60 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_60 == 76 {
                                            host::graphics_translate(r, graphics, -2, -1)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 20, 17)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 20, 17)?;
                                            let map_object_image_50 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_50,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_61 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_61 == 77 {
                                            host::graphics_translate(r, graphics, -2, -1)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 44, 15)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 44, 15)?;
                                            let map_object_image_51 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_51,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_62 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_62 == 78 {
                                            host::graphics_translate(r, graphics, 0, -4)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_52 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_52,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_63 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_63 == 79 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 64, 32)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 64, 32)?;
                                            let map_object_state_b_28 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            let quotient_value_2 = arithmetic::divide_int(
                                                r,
                                                map_object_state_b_28,
                                                2,
                                            )?;
                                            let remainder_value_4 =
                                                arithmetic::remainder_int(r, quotient_value_2, 2)?;
                                            if remainder_value_4 == 0 {
                                                let map_object_image_53 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                host::graphics_draw_region(
                                                    r,
                                                    graphics,
                                                    map_object_image_53,
                                                    0,
                                                    0,
                                                    64,
                                                    32,
                                                    0,
                                                    0,
                                                    0,
                                                    0,
                                                )?;
                                            }
                                            let map_object_state_b_29 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            let quotient_value_3 = arithmetic::divide_int(
                                                r,
                                                map_object_state_b_29,
                                                2,
                                            )?;
                                            let remainder_value_5 =
                                                arithmetic::remainder_int(r, quotient_value_3, 2)?;
                                            if remainder_value_5 == 1 {
                                                let map_object_image_55 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                host::graphics_draw_region(
                                                    r,
                                                    graphics,
                                                    map_object_image_55,
                                                    0,
                                                    0,
                                                    64,
                                                    32,
                                                    2,
                                                    0,
                                                    0,
                                                    0,
                                                )?;
                                            }
                                            let map_object_state_b_30 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_30.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_31 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_31 > 15 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_64 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_64 >= 80 {
                                            let map_object_kind_66 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_66 <= 83 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_54 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_69 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_54,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_69.wrapping_sub(80),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_65 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_65 >= 84 {
                                            let map_object_kind_68 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_68 <= 87 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_56 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_72 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_56,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_72.wrapping_sub(84),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_67 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_67 >= 88 {
                                            let map_object_kind_71 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_71 <= 91 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_58 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_74 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_58,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_74.wrapping_sub(88),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_70 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_70 == 92 {
                                            host::graphics_translate(r, graphics, 0, -60)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 64, 74)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 64, 74)?;
                                            let map_object_image_57 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_57,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_73 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_73 == 96 {
                                            host::graphics_translate(r, graphics, 0, -4)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_59 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_59,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_75 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_75 == 97 {
                                            host::graphics_translate(r, graphics, -4, 0)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_60 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_60,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_76 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_76 == 98 {
                                            host::graphics_translate(r, graphics, -4, 0)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_61 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_61,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_77 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_77 == 99 {
                                            host::graphics_translate(r, graphics, 0, -4)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_62 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_62,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_78 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_78 == 100 {
                                            host::graphics_translate(r, graphics, 0, -4)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_63 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_63,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_79 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_79 == 101 {
                                            host::graphics_translate(r, graphics, -4, 0)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_64 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_64,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_80 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_80 == 102 {
                                            host::graphics_translate(r, graphics, -4, 0)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_65 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_65,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_81 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_81 == 103 {
                                            host::graphics_translate(r, graphics, 0, -4)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_66 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_66,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_82 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_82 == 104 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_67 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_67,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_83 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_83 == 105 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_68 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_68,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_84 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_84 == 106 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_69 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_69,
                                                -32,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_85 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_85 == 107 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_70 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_70,
                                                -48,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_86 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_86 >= 108 {
                                            let map_object_kind_88 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_88 <= 111 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_72 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_90 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_72,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_90.wrapping_sub(108),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_87 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_87 == 112 {
                                            let state_d_upper_value_15 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_15 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_15,
                                                state_e_upper_value_15,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_71 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_71,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_89 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_89 == 113 {
                                            let state_d_upper_value_16 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_16 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_16,
                                                state_e_upper_value_16,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_73 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_73,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_91 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_91 == 114 {
                                            let state_d_upper_value_17 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_17 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_17,
                                                state_e_upper_value_17,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_74 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_74,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_92 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_92 == 115 {
                                            let state_d_upper_value_18 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_18 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_18,
                                                state_e_upper_value_18,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_75 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_75,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_93 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_93 == 116 {
                                            let state_d_upper_value_19 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_19 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_19,
                                                state_e_upper_value_19,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_76 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_76,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_94 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_94 == 117 {
                                            let state_d_upper_value_20 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_20 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_20,
                                                state_e_upper_value_20,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_77 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_77,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_95 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_95 == 118 {
                                            let state_d_upper_value_21 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_21 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_21,
                                                state_e_upper_value_21,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_78 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_78,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_96 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_96 == 119 {
                                            let state_d_upper_value_22 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_22 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_22,
                                                state_e_upper_value_22,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_79 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_79,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_97 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_97 == 120 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_80 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_80,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_98 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_98 == 121 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_81 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_81,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_99 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_99 == 122 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_82 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_82,
                                                -32,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_100 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_100 == 123 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_83 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_83,
                                                -48,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_101 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_101 >= 124 {
                                            let map_object_kind_103 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_103 <= 127 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_85 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_105 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_6 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_105.wrapping_sub(124),
                                                    2,
                                                )?;
                                                let map_object_kind_106 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let quotient_value_4 = arithmetic::divide_int(
                                                    r,
                                                    map_object_kind_106.wrapping_sub(124),
                                                    2,
                                                )?;
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_85,
                                                    (-16_i32).wrapping_mul(remainder_value_6),
                                                    (-16_i32).wrapping_mul(quotient_value_4),
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_102 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_102 == 128 {
                                            let state_d_upper_value_23 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_23 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_23,
                                                state_e_upper_value_23,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_84 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_84,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_104 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_104 == 129 {
                                            let state_d_upper_value_24 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_24 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_24,
                                                state_e_upper_value_24,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_86 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_86,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_107 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_107 == 130 {
                                            let state_d_upper_value_25 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_25 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_25,
                                                state_e_upper_value_25,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_87 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_87,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_108 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_108 == 131 {
                                            let state_d_upper_value_26 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_26 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_26,
                                                state_e_upper_value_26,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_88 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_88,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_109 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_109 == 132 {
                                            let state_d_upper_value_27 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_27 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_27,
                                                state_e_upper_value_27,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_89 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_89,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_110 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_110 == 133 {
                                            let state_d_upper_value_28 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_28 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_28,
                                                state_e_upper_value_28,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_90 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_90,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_111 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_111 == 134 {
                                            let state_d_upper_value_29 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_29 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_29,
                                                state_e_upper_value_29,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_91 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_91,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_112 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_112 == 135 {
                                            let state_d_upper_value_30 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_30 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_30,
                                                state_e_upper_value_30,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_92 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_92,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_113 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_113 >= 136 {
                                            let map_object_kind_115 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_115 <= 139 {
                                                let state_d_upper_value_31 = r
                                                    .get(object, field::map_object::STATE_D_UPPER)?
                                                    .i();
                                                let state_e_upper_value_31 = r
                                                    .get(object, field::map_object::STATE_E_UPPER)?
                                                    .i();
                                                host::graphics_translate(
                                                    r,
                                                    graphics,
                                                    state_d_upper_value_31,
                                                    state_e_upper_value_31,
                                                )?;
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_93 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_118 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_93,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_118.wrapping_sub(136),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_114 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_114 >= 140 {
                                            let map_object_kind_117 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_117 <= 143 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_95 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_120 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_7 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_120.wrapping_sub(140),
                                                    2,
                                                )?;
                                                let map_object_kind_121 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let quotient_value_5 = arithmetic::divide_int(
                                                    r,
                                                    map_object_kind_121.wrapping_sub(140),
                                                    2,
                                                )?;
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_95,
                                                    (-16_i32).wrapping_mul(remainder_value_7),
                                                    (-16_i32).wrapping_mul(quotient_value_5),
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_116 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_116 == 144 {
                                            let state_d_upper_value_32 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_32 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_32,
                                                state_e_upper_value_32,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_94 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_94,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_119 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_119 == 145 {
                                            let state_d_upper_value_33 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_33 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_33,
                                                state_e_upper_value_33,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_96 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_96,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_122 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_122 == 146 {
                                            let state_d_upper_value_34 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_34 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_34,
                                                state_e_upper_value_34,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_97 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_97,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_123 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_123 == 147 {
                                            let state_d_upper_value_35 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_35 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_35,
                                                state_e_upper_value_35,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_98 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_98,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_124 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_124 == 148 {
                                            let state_d_upper_value_36 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_36 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_36,
                                                state_e_upper_value_36,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_99 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_99,
                                                0,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_125 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_125 == 149 {
                                            let state_d_upper_value_37 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_37 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_37,
                                                state_e_upper_value_37,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_100 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_100,
                                                0,
                                                -24,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_126 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_126 == 150 {
                                            let state_d_upper_value_38 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_38 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_38,
                                                state_e_upper_value_38,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 24, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 24, 16)?;
                                            let map_object_image_101 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_101,
                                                -16,
                                                0,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_127 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_127 == 151 {
                                            let state_d_upper_value_39 = r
                                                .get(object, field::map_object::STATE_D_UPPER)?
                                                .i();
                                            let state_e_upper_value_39 = r
                                                .get(object, field::map_object::STATE_E_UPPER)?
                                                .i();
                                            host::graphics_translate(
                                                r,
                                                graphics,
                                                state_d_upper_value_39,
                                                state_e_upper_value_39,
                                            )?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 24)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 24)?;
                                            let map_object_image_102 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_102,
                                                -24,
                                                -16,
                                                0,
                                            )?;
                                        }
                                        let map_object_kind_128 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_128 >= 152 {
                                            let map_object_kind_130 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_130 <= 156 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 64, 32)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 64, 32,
                                                )?;
                                                let map_object_state_b_32 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                let quotient_value_6 = arithmetic::divide_int(
                                                    r,
                                                    map_object_state_b_32,
                                                    2,
                                                )?;
                                                let remainder_value_8 = arithmetic::remainder_int(
                                                    r,
                                                    quotient_value_6,
                                                    2,
                                                )?;
                                                if remainder_value_8 == 0 {
                                                    let image_value_104 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_104,
                                                        0,
                                                        0,
                                                        64,
                                                        32,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_state_b_33 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                let quotient_value_8 = arithmetic::divide_int(
                                                    r,
                                                    map_object_state_b_33,
                                                    2,
                                                )?;
                                                let remainder_value_10 = arithmetic::remainder_int(
                                                    r,
                                                    quotient_value_8,
                                                    2,
                                                )?;
                                                if remainder_value_10 == 1 {
                                                    let image_value_106 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_106,
                                                        0,
                                                        0,
                                                        64,
                                                        32,
                                                        2,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_state_b_34 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                r.set(
                                                    object,
                                                    field::map_object::STATE_B,
                                                    V::I(
                                                        (map_object_state_b_34.wrapping_add(1))
                                                            as i8
                                                            as i32,
                                                    ),
                                                )?;
                                                let map_object_state_b_35 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                if map_object_state_b_35 > 15 {
                                                    r.set(
                                                        object,
                                                        field::map_object::STATE_B,
                                                        V::I(0),
                                                    )?;
                                                }
                                            }
                                        }
                                        let map_object_kind_129 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_129 >= 160 {
                                            let map_object_kind_132 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_132 <= 168 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_103 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_135 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_9 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_135.wrapping_sub(160),
                                                    3,
                                                )?;
                                                let map_object_kind_136 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let quotient_value_7 = arithmetic::divide_int(
                                                    r,
                                                    map_object_kind_136.wrapping_sub(160),
                                                    3,
                                                )?;
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_103,
                                                    (-16_i32).wrapping_mul(remainder_value_9),
                                                    (-16_i32).wrapping_mul(quotient_value_7),
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_131 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_131 >= 169 {
                                            let map_object_kind_134 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_134 <= 170 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_105 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_139 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_105,
                                                    0,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_139.wrapping_sub(169),
                                                    ),
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_133 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_133 >= 171 {
                                            let map_object_kind_138 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_138 <= 174 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_107 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_142 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_11 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_142.wrapping_sub(171),
                                                    2,
                                                )?;
                                                let map_object_kind_143 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let quotient_value_9 = arithmetic::divide_int(
                                                    r,
                                                    map_object_kind_143.wrapping_sub(171),
                                                    2,
                                                )?;
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_107,
                                                    (-16_i32).wrapping_mul(remainder_value_11),
                                                    (-16_i32).wrapping_mul(quotient_value_9),
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_137 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_137 >= 176 {
                                            let map_object_kind_141 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_141 <= 184 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_108 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_146 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_12 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_146.wrapping_sub(176),
                                                    3,
                                                )?;
                                                let map_object_kind_147 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let quotient_value_10 = arithmetic::divide_int(
                                                    r,
                                                    map_object_kind_147.wrapping_sub(176),
                                                    3,
                                                )?;
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_108,
                                                    (-16_i32).wrapping_mul(remainder_value_12),
                                                    (-16_i32).wrapping_mul(quotient_value_10),
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_140 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_140 >= 192 {
                                            let map_object_kind_145 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_145 <= 203 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_kind_149 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_13 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_149.wrapping_sub(192),
                                                    4,
                                                )?;
                                                if remainder_value_13 == 0 {
                                                    let image_value_110 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let map_object_kind_154 =
                                                        r.get(object, field::map_object::KIND)?.i();
                                                    let quotient_value_12 = arithmetic::divide_int(
                                                        r,
                                                        map_object_kind_154.wrapping_sub(192),
                                                        4,
                                                    )?;
                                                    let state_b_value_36 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    let quotient_value_13 = arithmetic::divide_int(
                                                        r,
                                                        state_b_value_36,
                                                        4,
                                                    )?;
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_110,
                                                        (16_i32).wrapping_mul(quotient_value_12),
                                                        (16_i32).wrapping_mul(quotient_value_13),
                                                        16,
                                                        16,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_153 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_14 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_153.wrapping_sub(192),
                                                    4,
                                                )?;
                                                if remainder_value_14 == 1 {
                                                    let image_value_111 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let map_object_kind_159 =
                                                        r.get(object, field::map_object::KIND)?.i();
                                                    let quotient_value_14 = arithmetic::divide_int(
                                                        r,
                                                        map_object_kind_159.wrapping_sub(192),
                                                        4,
                                                    )?;
                                                    let state_b_value_37 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    let quotient_value_15 = arithmetic::divide_int(
                                                        r,
                                                        state_b_value_37,
                                                        4,
                                                    )?;
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_111,
                                                        (16_i32).wrapping_mul(quotient_value_14),
                                                        (16_i32).wrapping_mul(quotient_value_15),
                                                        16,
                                                        16,
                                                        5,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_158 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_16 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_158.wrapping_sub(192),
                                                    4,
                                                )?;
                                                if remainder_value_16 == 2 {
                                                    let image_value_113 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let map_object_kind_165 =
                                                        r.get(object, field::map_object::KIND)?.i();
                                                    let quotient_value_16 = arithmetic::divide_int(
                                                        r,
                                                        map_object_kind_165.wrapping_sub(192),
                                                        4,
                                                    )?;
                                                    let state_b_value_39 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    let quotient_value_17 = arithmetic::divide_int(
                                                        r,
                                                        state_b_value_39,
                                                        4,
                                                    )?;
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_113,
                                                        (16_i32).wrapping_mul(quotient_value_16),
                                                        (16_i32).wrapping_mul(quotient_value_17),
                                                        16,
                                                        16,
                                                        3,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_164 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_19 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_164.wrapping_sub(192),
                                                    4,
                                                )?;
                                                if remainder_value_19 == 3 {
                                                    let image_value_118 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let map_object_kind_170 =
                                                        r.get(object, field::map_object::KIND)?.i();
                                                    let quotient_value_18 = arithmetic::divide_int(
                                                        r,
                                                        map_object_kind_170.wrapping_sub(192),
                                                        4,
                                                    )?;
                                                    let state_b_value_46 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    let quotient_value_19 = arithmetic::divide_int(
                                                        r,
                                                        state_b_value_46,
                                                        4,
                                                    )?;
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_118,
                                                        (16_i32).wrapping_mul(quotient_value_18),
                                                        (16_i32).wrapping_mul(quotient_value_19),
                                                        16,
                                                        16,
                                                        6,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_state_b_45 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                if map_object_state_b_45 >= 11 {
                                                    r.set(
                                                        object,
                                                        field::map_object::STATE_L,
                                                        V::I(-1),
                                                    )?;
                                                }
                                                let map_object_state_b_49 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                if map_object_state_b_49 <= 0 {
                                                    r.set(
                                                        object,
                                                        field::map_object::STATE_L,
                                                        V::I(1),
                                                    )?;
                                                }
                                                let map_object_state_b_54 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                let map_object_state_l =
                                                    r.get(object, field::map_object::STATE_L)?.i();
                                                r.set(
                                                    object,
                                                    field::map_object::STATE_B,
                                                    V::I(
                                                        ((map_object_state_b_54)
                                                            .wrapping_add(map_object_state_l))
                                                            as i8
                                                            as i32,
                                                    ),
                                                )?;
                                            }
                                        }
                                        let map_object_kind_144 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_144 == 204 {
                                            let map_object_state_z =
                                                r.get(object, field::map_object::STATE_Z)?.i();
                                            if map_object_state_z != 0 {
                                                return Ok(());
                                            }
                                            host::graphics_translate(r, graphics, 0, -32)?;
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_109 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_kind_152 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            let quotient_value_11 = arithmetic::divide_int(
                                                r,
                                                map_object_kind_152.wrapping_sub(192),
                                                4,
                                            )?;
                                            host::graphics_draw_region(
                                                r,
                                                graphics,
                                                map_object_image_109,
                                                (16_i32).wrapping_mul(quotient_value_11),
                                                0,
                                                16,
                                                16,
                                                6,
                                                0,
                                                0,
                                                0,
                                            )?;
                                            r.set(object, field::map_object::STATE_Z, V::I(1))?;
                                        }
                                        let map_object_kind_148 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_148 >= 208 {
                                            let map_object_kind_151 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_151 <= 211 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_kind_157 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_15 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_157.wrapping_sub(208),
                                                    4,
                                                )?;
                                                if remainder_value_15 == 0 {
                                                    let image_value_112 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_38 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_112,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_38)
                                                                .wrapping_shr((2) as u32),
                                                        ),
                                                        0,
                                                        16,
                                                        16,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_163 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_18 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_163.wrapping_sub(208),
                                                    4,
                                                )?;
                                                if remainder_value_18 == 1 {
                                                    let image_value_117 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_44 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_117,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_44)
                                                                .wrapping_shr((2) as u32),
                                                        ),
                                                        0,
                                                        16,
                                                        16,
                                                        5,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_169 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_21 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_169.wrapping_sub(208),
                                                    4,
                                                )?;
                                                if remainder_value_21 == 2 {
                                                    let image_value_121 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_48 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_121,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_48)
                                                                .wrapping_shr((2) as u32),
                                                        ),
                                                        0,
                                                        16,
                                                        16,
                                                        3,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_173 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_23 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_173.wrapping_sub(208),
                                                    4,
                                                )?;
                                                if remainder_value_23 == 3 {
                                                    let image_value_124 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_53 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_124,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_53)
                                                                .wrapping_shr((2) as u32),
                                                        ),
                                                        0,
                                                        16,
                                                        16,
                                                        6,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_state_b_51 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                r.set(
                                                    object,
                                                    field::map_object::STATE_B,
                                                    V::I(
                                                        (map_object_state_b_51.wrapping_add(1))
                                                            as i8
                                                            as i32,
                                                    ),
                                                )?;
                                                let map_object_state_b_52 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                if map_object_state_b_52 > 15 {
                                                    r.set(
                                                        object,
                                                        field::map_object::STATE_B,
                                                        V::I(0),
                                                    )?;
                                                }
                                            }
                                        }
                                        let map_object_kind_150 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_150 >= 212 {
                                            let map_object_kind_156 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_156 <= 215 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_kind_162 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_17 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_162.wrapping_sub(212),
                                                    4,
                                                )?;
                                                if remainder_value_17 == 0 {
                                                    let image_value_116 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_43 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_116,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_43)
                                                                .wrapping_shr((2) as u32),
                                                        ),
                                                        0,
                                                        16,
                                                        16,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_168 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_20 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_168.wrapping_sub(212),
                                                    4,
                                                )?;
                                                if remainder_value_20 == 1 {
                                                    let image_value_120 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_47 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_120,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_47)
                                                                .wrapping_shr((2) as u32),
                                                        ),
                                                        0,
                                                        16,
                                                        16,
                                                        5,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_172 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_22 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_172.wrapping_sub(212),
                                                    4,
                                                )?;
                                                if remainder_value_22 == 2 {
                                                    let image_value_123 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_50 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_123,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_50)
                                                                .wrapping_shr((2) as u32),
                                                        ),
                                                        0,
                                                        16,
                                                        16,
                                                        3,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_175 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_24 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_175.wrapping_sub(212),
                                                    4,
                                                )?;
                                                if remainder_value_24 == 3 {
                                                    let image_value_125 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_57 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_125,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_57)
                                                                .wrapping_shr((2) as u32),
                                                        ),
                                                        0,
                                                        16,
                                                        16,
                                                        6,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_state_b_55 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                r.set(
                                                    object,
                                                    field::map_object::STATE_B,
                                                    V::I(
                                                        (map_object_state_b_55.wrapping_add(1))
                                                            as i8
                                                            as i32,
                                                    ),
                                                )?;
                                                let map_object_state_b_56 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                if map_object_state_b_56 > 15 {
                                                    r.set(
                                                        object,
                                                        field::map_object::STATE_B,
                                                        V::I(0),
                                                    )?;
                                                }
                                            }
                                        }
                                        let map_object_kind_155 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_155 >= 216 {
                                            let map_object_kind_161 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_161 <= 220 {
                                                host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                                host::graphics_clip_rect(
                                                    r, graphics, 0, 0, 16, 16,
                                                )?;
                                                let map_object_image_115 =
                                                    r.get(object, field::map_object::IMAGE)?.r();
                                                let map_object_kind_167 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                host::graphics_draw_image(
                                                    r,
                                                    graphics,
                                                    map_object_image_115,
                                                    (-16_i32).wrapping_mul(
                                                        map_object_kind_167.wrapping_sub(216),
                                                    ),
                                                    0,
                                                    0,
                                                )?;
                                            }
                                        }
                                        let map_object_kind_160 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_160 == 185 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_114 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_b_40 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_114,
                                                0,
                                                (-16_i32).wrapping_mul(
                                                    map_object_state_b_40.wrapping_shr((2) as u32),
                                                ),
                                                0,
                                            )?;
                                            let map_object_state_b_41 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            r.set(
                                                object,
                                                field::map_object::STATE_B,
                                                V::I(
                                                    (map_object_state_b_41.wrapping_add(1)) as i8
                                                        as i32,
                                                ),
                                            )?;
                                            let map_object_state_b_42 =
                                                r.get(object, field::map_object::STATE_B)?.i();
                                            if map_object_state_b_42 > 10 {
                                                r.set(object, field::map_object::STATE_B, V::I(0))?;
                                            }
                                        }
                                        let map_object_kind_166 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_166 == 186 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_119 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_c_2 =
                                                r.get(object, field::map_object::STATE_C)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_119,
                                                0,
                                                (-16_i32).wrapping_mul(map_object_state_c_2),
                                                0,
                                            )?;
                                            let map_object_state_c_3 =
                                                r.get(object, field::map_object::STATE_C)?.i();
                                            if map_object_state_c_3 != 1 {
                                                r.set(object, field::map_object::STATE_E, V::I(0))?;
                                            } else {
                                                r.set(object, field::map_object::STATE_E, V::I(1))?;
                                            }
                                        }
                                        let map_object_kind_171 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_171 == 187 {
                                            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
                                            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
                                            let map_object_image_122 =
                                                r.get(object, field::map_object::IMAGE)?.r();
                                            let map_object_state_c_4 =
                                                r.get(object, field::map_object::STATE_C)?.i();
                                            host::graphics_draw_image(
                                                r,
                                                graphics,
                                                map_object_image_122,
                                                -16,
                                                (-16_i32).wrapping_mul(
                                                    (1_i32).wrapping_sub(map_object_state_c_4),
                                                ),
                                                0,
                                            )?;
                                            let map_object_state_c_5 =
                                                r.get(object, field::map_object::STATE_C)?.i();
                                            if map_object_state_c_5 != 0 {
                                                r.set(object, field::map_object::STATE_E, V::I(0))?;
                                            } else {
                                                r.set(object, field::map_object::STATE_E, V::I(1))?;
                                            }
                                        }
                                        let map_object_kind_174 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_174 >= 188 {
                                            let map_object_kind_177 =
                                                r.get(object, field::map_object::KIND)?.i();
                                            if map_object_kind_177 <= 191 {
                                                let map_object_kind_180 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_25 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_180.wrapping_sub(188),
                                                    4,
                                                )?;
                                                if remainder_value_25 == 0 {
                                                    host::graphics_translate(r, graphics, 0, 4)?;
                                                    host::graphics_set_clip(
                                                        r, graphics, 0, 0, 32, 16,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, graphics, 0, 0, 32, 16,
                                                    )?;
                                                    let image_value_126 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_58 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_126,
                                                        0,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_58)
                                                                .wrapping_shr((1) as u32),
                                                        ),
                                                        32,
                                                        16,
                                                        0,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_183 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_26 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_183.wrapping_sub(188),
                                                    4,
                                                )?;
                                                if remainder_value_26 == 1 {
                                                    host::graphics_translate(r, graphics, -4, -8)?;
                                                    host::graphics_set_clip(
                                                        r, graphics, 0, 0, 16, 32,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, graphics, 0, 0, 16, 32,
                                                    )?;
                                                    let image_value_128 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_59 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_128,
                                                        0,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_59)
                                                                .wrapping_shr((1) as u32),
                                                        ),
                                                        32,
                                                        16,
                                                        5,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_187 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_27 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_187.wrapping_sub(188),
                                                    4,
                                                )?;
                                                if remainder_value_27 == 2 {
                                                    host::graphics_translate(r, graphics, 0, -4)?;
                                                    host::graphics_set_clip(
                                                        r, graphics, 0, 0, 32, 16,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, graphics, 0, 0, 32, 16,
                                                    )?;
                                                    let image_value_130 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_60 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_130,
                                                        0,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_60)
                                                                .wrapping_shr((1) as u32),
                                                        ),
                                                        32,
                                                        16,
                                                        3,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_kind_192 =
                                                    r.get(object, field::map_object::KIND)?.i();
                                                let remainder_value_28 = arithmetic::remainder_int(
                                                    r,
                                                    map_object_kind_192.wrapping_sub(188),
                                                    4,
                                                )?;
                                                if remainder_value_28 == 3 {
                                                    host::graphics_translate(r, graphics, 0, -8)?;
                                                    host::graphics_set_clip(
                                                        r, graphics, 0, 0, 16, 32,
                                                    )?;
                                                    host::graphics_clip_rect(
                                                        r, graphics, 0, 0, 16, 32,
                                                    )?;
                                                    let image_value_133 = r
                                                        .get(object, field::map_object::IMAGE)?
                                                        .r();
                                                    let state_b_value_63 = r
                                                        .get(object, field::map_object::STATE_B)?
                                                        .i();
                                                    host::graphics_draw_region(
                                                        r,
                                                        graphics,
                                                        image_value_133,
                                                        0,
                                                        (16_i32).wrapping_mul(
                                                            (state_b_value_63)
                                                                .wrapping_shr((1) as u32),
                                                        ),
                                                        32,
                                                        16,
                                                        6,
                                                        0,
                                                        0,
                                                        0,
                                                    )?;
                                                }
                                                let map_object_state_b_61 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                let map_object_state_l_2 =
                                                    r.get(object, field::map_object::STATE_L)?.i();
                                                r.set(
                                                    object,
                                                    field::map_object::STATE_B,
                                                    V::I(
                                                        ((map_object_state_b_61)
                                                            .wrapping_add(map_object_state_l_2))
                                                            as i8
                                                            as i32,
                                                    ),
                                                )?;
                                                let map_object_state_b_62 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                if map_object_state_b_62 >= 5 {
                                                    r.set(
                                                        object,
                                                        field::map_object::STATE_L,
                                                        V::I(-1),
                                                    )?;
                                                }
                                                let map_object_state_b_64 =
                                                    r.get(object, field::map_object::STATE_B)?.i();
                                                if map_object_state_b_64 <= 0 {
                                                    r.set(
                                                        object,
                                                        field::map_object::STATE_L,
                                                        V::I(1),
                                                    )?;
                                                }
                                            }
                                        }
                                        let map_object_kind_176 =
                                            r.get(object, field::map_object::KIND)?.i();
                                        if map_object_kind_176 < 205 {
                                            break 'check_map_object_kind;
                                        }
                                    }
                                    let map_object_kind_179 =
                                        r.get(object, field::map_object::KIND)?.i();
                                    if map_object_kind_179 <= 207 {
                                        break 'check_map_object_kind_4;
                                    }
                                }
                                let map_object_kind_178 =
                                    r.get(object, field::map_object::KIND)?.i();
                                if map_object_kind_178 != 221 {
                                    break 'check_map_object_kind_3;
                                }
                            }
                            let map_object_kind_182 = r.get(object, field::map_object::KIND)?.i();
                            if map_object_kind_182 == 205 {
                                host::graphics_translate(r, graphics, -16, 0)?;
                                host::graphics_set_clip(r, graphics, 0, 0, 48, 32)?;
                                host::graphics_clip_rect(r, graphics, 0, 0, 48, 32)?;
                                let map_object_image_127 =
                                    r.get(object, field::map_object::IMAGE)?.r();
                                host::graphics_draw_region(
                                    r,
                                    graphics,
                                    map_object_image_127,
                                    0,
                                    0,
                                    48,
                                    32,
                                    0,
                                    0,
                                    0,
                                    0,
                                )?;
                            }
                            let map_object_kind_186 = r.get(object, field::map_object::KIND)?.i();
                            if map_object_kind_186 == 206 {
                                host::graphics_translate(r, graphics, -16, 0)?;
                                host::graphics_set_clip(r, graphics, 0, 0, 48, 32)?;
                                host::graphics_clip_rect(r, graphics, 0, 0, 48, 32)?;
                                let map_object_image_129 =
                                    r.get(object, field::map_object::IMAGE)?.r();
                                host::graphics_draw_region(
                                    r,
                                    graphics,
                                    map_object_image_129,
                                    0,
                                    0,
                                    48,
                                    32,
                                    3,
                                    0,
                                    0,
                                    0,
                                )?;
                            }
                            let map_object_kind_191 = r.get(object, field::map_object::KIND)?.i();
                            if map_object_kind_191 == 207 {
                                host::graphics_translate(r, graphics, -16, -16)?;
                                host::graphics_set_clip(r, graphics, 0, 0, 32, 48)?;
                                host::graphics_clip_rect(r, graphics, 0, 0, 32, 48)?;
                                let map_object_image_132 =
                                    r.get(object, field::map_object::IMAGE)?.r();
                                host::graphics_draw_region(
                                    r,
                                    graphics,
                                    map_object_image_132,
                                    0,
                                    0,
                                    48,
                                    32,
                                    5,
                                    0,
                                    0,
                                    0,
                                )?;
                            }
                            let map_object_kind_196 = r.get(object, field::map_object::KIND)?.i();
                            if map_object_kind_196 == 221 {
                                host::graphics_translate(r, graphics, 0, -16)?;
                                host::graphics_set_clip(r, graphics, 0, 0, 32, 48)?;
                                host::graphics_clip_rect(r, graphics, 0, 0, 32, 48)?;
                                let map_object_image_136 =
                                    r.get(object, field::map_object::IMAGE)?.r();
                                host::graphics_draw_region(
                                    r,
                                    graphics,
                                    map_object_image_136,
                                    0,
                                    0,
                                    48,
                                    32,
                                    6,
                                    0,
                                    0,
                                    0,
                                )?;
                            }
                        }
                        let map_object_kind_181 = r.get(object, field::map_object::KIND)?.i();
                        if map_object_kind_181 < 222 {
                            break 'check_map_object_kind_5;
                        }
                    }
                    let map_object_kind_185 = r.get(object, field::map_object::KIND)?.i();
                    if map_object_kind_185 <= 223 {
                        break 'check_map_object_kind_9;
                    }
                }
                let map_object_kind_184 = r.get(object, field::map_object::KIND)?.i();
                if map_object_kind_184 < 237 {
                    break 'check_map_object_kind_7;
                }
            }
            let map_object_kind_189 = r.get(object, field::map_object::KIND)?.i();
            if map_object_kind_189 > 238 {
                break 'check_map_object_kind_7;
            }
        }
        let map_object_kind_190 = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_190 == 222 {
            host::graphics_translate(r, graphics, -16, -16)?;
            host::graphics_set_clip(r, graphics, 0, 0, 96, 22)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 96, 22)?;
            let map_object_image_131 = r.get(object, field::map_object::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, map_object_image_131, 0, -26, 0)?;
        }
        let map_object_kind_195 = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_195 == 223 {
            host::graphics_translate(r, graphics, -16, 0)?;
            host::graphics_set_clip(r, graphics, 0, 0, 96, 22)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 96, 22)?;
            let map_object_image_135 = r.get(object, field::map_object::IMAGE)?.r();
            host::graphics_draw_region(
                r,
                graphics,
                map_object_image_135,
                0,
                26,
                96,
                22,
                3,
                0,
                0,
                0,
            )?;
        }
        let map_object_kind_200 = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_200 == 237 {
            host::graphics_translate(r, graphics, -16, -10)?;
            host::graphics_set_clip(r, graphics, 0, 0, 96, 26)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 96, 26)?;
            let map_object_image_138 = r.get(object, field::map_object::IMAGE)?.r();
            host::graphics_draw_image(r, graphics, map_object_image_138, 0, 0, 0)?;
        }
        let map_object_kind_204 = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_204 == 238 {
            host::graphics_translate(r, graphics, -16, -10)?;
            host::graphics_set_clip(r, graphics, 0, 0, 96, 26)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 96, 26)?;
            let map_object_image_139 = r.get(object, field::map_object::IMAGE)?.r();
            host::graphics_draw_region(
                r,
                graphics,
                map_object_image_139,
                0,
                0,
                96,
                26,
                3,
                0,
                0,
                0,
            )?;
        }
    }
    let map_object_kind_188 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_188 >= 224 {
        let map_object_kind_194 = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_194 <= 228 {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let map_object_image_134 = r.get(object, field::map_object::IMAGE)?.r();
            let map_object_kind_199 = r.get(object, field::map_object::KIND)?.i();
            host::graphics_draw_image(
                r,
                graphics,
                map_object_image_134,
                (-16_i32).wrapping_mul(map_object_kind_199.wrapping_sub(224)),
                0,
                0,
            )?;
        }
    }
    let map_object_kind_193 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_193 >= 229 {
        let map_object_kind_198 = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_198 <= 232 {
            host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
            host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
            let map_object_image_137 = r.get(object, field::map_object::IMAGE)?.r();
            let map_object_kind_203 = r.get(object, field::map_object::KIND)?.i();
            host::graphics_draw_image(
                r,
                graphics,
                map_object_image_137,
                (-16_i32).wrapping_mul(map_object_kind_203.wrapping_sub(229)),
                0,
                0,
            )?;
        }
    }
    let map_object_kind_197 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_197 < 233 {
        let map_object_kind_201_read_read = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_201_read_read < 240 {
            return Ok(());
        }
        let map_object_kind_205_read_read = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_205_read_read > 255 {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let map_object_image_140_read_read = r.get(object, field::map_object::IMAGE)?.r();
        let map_object_kind_207_read_read = r.get(object, field::map_object::KIND)?.i();
        let remainder_value_29 =
            arithmetic::remainder_int(r, map_object_kind_207_read_read.wrapping_sub(240), 4)?;
        let map_object_kind_208_read_read = r.get(object, field::map_object::KIND)?.i();
        let quotient_value_20 =
            arithmetic::divide_int(r, map_object_kind_208_read_read.wrapping_sub(240), 4)?;
        host::graphics_draw_image(
            r,
            graphics,
            map_object_image_140_read_read,
            (-16_i32).wrapping_mul(remainder_value_29),
            (-16_i32).wrapping_mul(quotient_value_20),
            0,
        )?;
        return Ok(());
    }
    let map_object_kind_202 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_202 > 236 {
        let map_object_kind_201_read_read = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_201_read_read < 240 {
            return Ok(());
        }
        let map_object_kind_205_read_read = r.get(object, field::map_object::KIND)?.i();
        if map_object_kind_205_read_read > 255 {
            return Ok(());
        }
        host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
        let map_object_image_140_read_read = r.get(object, field::map_object::IMAGE)?.r();
        let map_object_kind_207_read_read = r.get(object, field::map_object::KIND)?.i();
        let remainder_value_29 =
            arithmetic::remainder_int(r, map_object_kind_207_read_read.wrapping_sub(240), 4)?;
        let map_object_kind_208_read_read = r.get(object, field::map_object::KIND)?.i();
        let quotient_value_20 =
            arithmetic::divide_int(r, map_object_kind_208_read_read.wrapping_sub(240), 4)?;
        host::graphics_draw_image(
            r,
            graphics,
            map_object_image_140_read_read,
            (-16_i32).wrapping_mul(remainder_value_29),
            (-16_i32).wrapping_mul(quotient_value_20),
            0,
        )?;
        return Ok(());
    }
    let map_object_kind_206 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_206 == 233 {
        host::graphics_set_clip(r, graphics, 0, 0, 48, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 48, 16)?;
        let map_object_image_141 = r.get(object, field::map_object::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, map_object_image_141, 0, 0, 0)?;
    }
    let map_object_kind_209 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_209 == 234 {
        host::graphics_set_clip(r, graphics, 0, 0, 48, 16)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 48, 16)?;
        let map_object_image_142 = r.get(object, field::map_object::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, map_object_image_142, 0, -16, 0)?;
    }
    let map_object_kind_210 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_210 == 235 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 48)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 48)?;
        let map_object_image_143 = r.get(object, field::map_object::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, map_object_image_143, -64, 0, 0)?;
    }
    let map_object_kind_211 = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_211 == 236 {
        host::graphics_set_clip(r, graphics, 0, 0, 16, 48)?;
        host::graphics_clip_rect(r, graphics, 0, 0, 16, 48)?;
        let map_object_image_144 = r.get(object, field::map_object::IMAGE)?.r();
        host::graphics_draw_image(r, graphics, map_object_image_144, -48, 0, 0)?;
    }
    let map_object_kind_201_read_read = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_201_read_read < 240 {
        return Ok(());
    }
    let map_object_kind_205_read_read = r.get(object, field::map_object::KIND)?.i();
    if map_object_kind_205_read_read > 255 {
        return Ok(());
    }
    host::graphics_set_clip(r, graphics, 0, 0, 16, 16)?;
    host::graphics_clip_rect(r, graphics, 0, 0, 16, 16)?;
    let map_object_image_140_read_read = r.get(object, field::map_object::IMAGE)?.r();
    let map_object_kind_207_read_read = r.get(object, field::map_object::KIND)?.i();
    let remainder_value_29 =
        arithmetic::remainder_int(r, map_object_kind_207_read_read.wrapping_sub(240), 4)?;
    let map_object_kind_208_read_read = r.get(object, field::map_object::KIND)?.i();
    let quotient_value_20 =
        arithmetic::divide_int(r, map_object_kind_208_read_read.wrapping_sub(240), 4)?;
    host::graphics_draw_image(
        r,
        graphics,
        map_object_image_140_read_read,
        (-16_i32).wrapping_mul(remainder_value_29),
        (-16_i32).wrapping_mul(quotient_value_20),
        0,
    )?;
    return Ok(());
}
