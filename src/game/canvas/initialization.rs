//! Canvas initialization.
use super::*;

/// Initialize empty. Original reference: `k.<init>()V`.
pub(in crate::game) fn initialize_empty(r: &mut Runtime, canvas: u32) -> Result<(), u32> {
    host::canvas_initialize(r, canvas)?;
    r.set(canvas, field::canvas::VISUAL_EFFECTS, V::I(1))?;
    r.set(canvas, field::canvas::STATE_AX, V::I(0))?;
    r.set(canvas, field::canvas::PREVIOUS_SCREEN, V::I(0))?;
    r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1))?;
    r.set(canvas, field::canvas::EXIT_REQUESTED, V::I(0))?;
    r.set(canvas, field::canvas::MUSIC_VOLUME, V::I(50))?;
    r.set(canvas, field::canvas::EFFECT_VOLUME, V::I(50))?;
    r.set(canvas, field::canvas::STATE_AK_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AL_UPPER, V::I(176))?;
    r.set(canvas, field::canvas::STATE_AM_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AN_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
    r.set(canvas, field::canvas::STATE_AO_UPPER, V::I(1))?;
    r.set(canvas, field::canvas::STATE_U, V::I(0))?;
    r.set(canvas, field::canvas::STATE_V, V::I(1))?;
    r.set(canvas, field::canvas::STATE_AP_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_W, V::I(1))?;
    r.set(canvas, field::canvas::STATE_X, V::I(1))?;
    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AS_UPPER, V::I(1))?;
    r.set(canvas, field::canvas::STATE_G, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AY, V::I(0))?;
    let array_value = r.new_array(0, 6)?;
    let array_value_2 = r.new_array(10, 6)?;
    r.array_set(array_value_2, 0, V::I(0))?;
    r.array_set(array_value_2, 1, V::I(1))?;
    r.array_set(array_value_2, 2, V::I(1))?;
    r.array_set(array_value_2, 3, V::I(0))?;
    r.array_set(array_value_2, 4, V::I(0))?;
    r.array_set(array_value_2, 5, V::I(0))?;
    r.array_set(array_value, 0, V::R(array_value_2))?;
    let array_value_3 = r.new_array(10, 6)?;
    r.array_set(array_value_3, 0, V::I(0))?;
    r.array_set(array_value_3, 1, V::I(0))?;
    r.array_set(array_value_3, 2, V::I(1))?;
    r.array_set(array_value_3, 3, V::I(0))?;
    r.array_set(array_value_3, 4, V::I(1))?;
    r.array_set(array_value_3, 5, V::I(1))?;
    r.array_set(array_value, 1, V::R(array_value_3))?;
    let array_value_4 = r.new_array(10, 6)?;
    r.array_set(array_value_4, 0, V::I(1))?;
    r.array_set(array_value_4, 1, V::I(1))?;
    r.array_set(array_value_4, 2, V::I(1))?;
    r.array_set(array_value_4, 3, V::I(1))?;
    r.array_set(array_value_4, 4, V::I(1))?;
    r.array_set(array_value_4, 5, V::I(0))?;
    r.array_set(array_value, 2, V::R(array_value_4))?;
    let array_value_5 = r.new_array(10, 6)?;
    r.array_set(array_value_5, 0, V::I(0))?;
    r.array_set(array_value_5, 1, V::I(1))?;
    r.array_set(array_value_5, 2, V::I(1))?;
    r.array_set(array_value_5, 3, V::I(1))?;
    r.array_set(array_value_5, 4, V::I(0))?;
    r.array_set(array_value_5, 5, V::I(0))?;
    r.array_set(array_value, 3, V::R(array_value_5))?;
    let array_value_6 = r.new_array(10, 6)?;
    r.array_set(array_value_6, 0, V::I(0))?;
    r.array_set(array_value_6, 1, V::I(0))?;
    r.array_set(array_value_6, 2, V::I(1))?;
    r.array_set(array_value_6, 3, V::I(0))?;
    r.array_set(array_value_6, 4, V::I(0))?;
    r.array_set(array_value_6, 5, V::I(0))?;
    r.array_set(array_value, 4, V::R(array_value_6))?;
    let array_value_7 = r.new_array(10, 6)?;
    r.array_set(array_value_7, 0, V::I(0))?;
    r.array_set(array_value_7, 1, V::I(1))?;
    r.array_set(array_value_7, 2, V::I(1))?;
    r.array_set(array_value_7, 3, V::I(1))?;
    r.array_set(array_value_7, 4, V::I(0))?;
    r.array_set(array_value_7, 5, V::I(0))?;
    r.array_set(array_value, 5, V::R(array_value_7))?;
    r.set(canvas, field::canvas::STATE_AZ, V::R(array_value))?;
    let array_value_8 = r.new_array(0, 8)?;
    let array_value_9 = r.new_array(10, 4)?;
    r.array_set(array_value_9, 0, V::I(0))?;
    r.array_set(array_value_9, 1, V::I(1))?;
    r.array_set(array_value_9, 2, V::I(1))?;
    r.array_set(array_value_9, 3, V::I(0))?;
    r.array_set(array_value_8, 0, V::R(array_value_9))?;
    let array_value_10 = r.new_array(10, 4)?;
    r.array_set(array_value_10, 0, V::I(0))?;
    r.array_set(array_value_10, 1, V::I(0))?;
    r.array_set(array_value_10, 2, V::I(1))?;
    r.array_set(array_value_10, 3, V::I(1))?;
    r.array_set(array_value_8, 1, V::R(array_value_10))?;
    let array_value_11 = r.new_array(10, 4)?;
    r.array_set(array_value_11, 0, V::I(0))?;
    r.array_set(array_value_11, 1, V::I(0))?;
    r.array_set(array_value_11, 2, V::I(1))?;
    r.array_set(array_value_11, 3, V::I(1))?;
    r.array_set(array_value_8, 2, V::R(array_value_11))?;
    let array_value_12 = r.new_array(10, 4)?;
    r.array_set(array_value_12, 0, V::I(0))?;
    r.array_set(array_value_12, 1, V::I(0))?;
    r.array_set(array_value_12, 2, V::I(1))?;
    r.array_set(array_value_12, 3, V::I(1))?;
    r.array_set(array_value_8, 3, V::R(array_value_12))?;
    let array_value_13 = r.new_array(10, 4)?;
    r.array_set(array_value_13, 0, V::I(0))?;
    r.array_set(array_value_13, 1, V::I(0))?;
    r.array_set(array_value_13, 2, V::I(1))?;
    r.array_set(array_value_13, 3, V::I(1))?;
    r.array_set(array_value_8, 4, V::R(array_value_13))?;
    let array_value_14 = r.new_array(10, 4)?;
    r.array_set(array_value_14, 0, V::I(0))?;
    r.array_set(array_value_14, 1, V::I(0))?;
    r.array_set(array_value_14, 2, V::I(1))?;
    r.array_set(array_value_14, 3, V::I(1))?;
    r.array_set(array_value_8, 5, V::R(array_value_14))?;
    let array_value_15 = r.new_array(10, 4)?;
    r.array_set(array_value_15, 0, V::I(1))?;
    r.array_set(array_value_15, 1, V::I(1))?;
    r.array_set(array_value_15, 2, V::I(1))?;
    r.array_set(array_value_15, 3, V::I(1))?;
    r.array_set(array_value_8, 6, V::R(array_value_15))?;
    let array_value_16 = r.new_array(10, 4)?;
    r.array_set(array_value_16, 0, V::I(0))?;
    r.array_set(array_value_16, 1, V::I(1))?;
    r.array_set(array_value_16, 2, V::I(1))?;
    r.array_set(array_value_16, 3, V::I(0))?;
    r.array_set(array_value_8, 7, V::R(array_value_16))?;
    r.set(canvas, field::canvas::STATE_BA, V::R(array_value_8))?;
    let array_value_17 = r.new_array(0, 6)?;
    let array_value_18 = r.new_array(10, 5)?;
    r.array_set(array_value_18, 0, V::I(0))?;
    r.array_set(array_value_18, 1, V::I(1))?;
    r.array_set(array_value_18, 2, V::I(1))?;
    r.array_set(array_value_18, 3, V::I(0))?;
    r.array_set(array_value_18, 4, V::I(0))?;
    r.array_set(array_value_17, 0, V::R(array_value_18))?;
    let array_value_19 = r.new_array(10, 5)?;
    r.array_set(array_value_19, 0, V::I(0))?;
    r.array_set(array_value_19, 1, V::I(0))?;
    r.array_set(array_value_19, 2, V::I(1))?;
    r.array_set(array_value_19, 3, V::I(0))?;
    r.array_set(array_value_19, 4, V::I(1))?;
    r.array_set(array_value_17, 1, V::R(array_value_19))?;
    let array_value_20 = r.new_array(10, 5)?;
    r.array_set(array_value_20, 0, V::I(1))?;
    r.array_set(array_value_20, 1, V::I(1))?;
    r.array_set(array_value_20, 2, V::I(1))?;
    r.array_set(array_value_20, 3, V::I(1))?;
    r.array_set(array_value_20, 4, V::I(1))?;
    r.array_set(array_value_17, 2, V::R(array_value_20))?;
    let array_value_21 = r.new_array(10, 5)?;
    r.array_set(array_value_21, 0, V::I(1))?;
    r.array_set(array_value_21, 1, V::I(1))?;
    r.array_set(array_value_21, 2, V::I(1))?;
    r.array_set(array_value_21, 3, V::I(1))?;
    r.array_set(array_value_21, 4, V::I(1))?;
    r.array_set(array_value_17, 3, V::R(array_value_21))?;
    let array_value_22 = r.new_array(10, 5)?;
    r.array_set(array_value_22, 0, V::I(1))?;
    r.array_set(array_value_22, 1, V::I(0))?;
    r.array_set(array_value_22, 2, V::I(1))?;
    r.array_set(array_value_22, 3, V::I(0))?;
    r.array_set(array_value_22, 4, V::I(0))?;
    r.array_set(array_value_17, 4, V::R(array_value_22))?;
    let array_value_23 = r.new_array(10, 5)?;
    r.array_set(array_value_23, 0, V::I(0))?;
    r.array_set(array_value_23, 1, V::I(0))?;
    r.array_set(array_value_23, 2, V::I(1))?;
    r.array_set(array_value_23, 3, V::I(1))?;
    r.array_set(array_value_23, 4, V::I(0))?;
    r.array_set(array_value_17, 5, V::R(array_value_23))?;
    r.set(canvas, field::canvas::STATE_BB, V::R(array_value_17))?;
    let array_value_24 = r.new_array(0, 8)?;
    let array_value_25 = r.new_array(10, 4)?;
    r.array_set(array_value_25, 0, V::I(1))?;
    r.array_set(array_value_25, 1, V::I(1))?;
    r.array_set(array_value_25, 2, V::I(1))?;
    r.array_set(array_value_25, 3, V::I(1))?;
    r.array_set(array_value_24, 0, V::R(array_value_25))?;
    let array_value_26 = r.new_array(10, 4)?;
    r.array_set(array_value_26, 0, V::I(1))?;
    r.array_set(array_value_26, 1, V::I(0))?;
    r.array_set(array_value_26, 2, V::I(1))?;
    r.array_set(array_value_26, 3, V::I(1))?;
    r.array_set(array_value_24, 1, V::R(array_value_26))?;
    let array_value_27 = r.new_array(10, 4)?;
    r.array_set(array_value_27, 0, V::I(1))?;
    r.array_set(array_value_27, 1, V::I(1))?;
    r.array_set(array_value_27, 2, V::I(0))?;
    r.array_set(array_value_27, 3, V::I(0))?;
    r.array_set(array_value_24, 2, V::R(array_value_27))?;
    let array_value_28 = r.new_array(10, 4)?;
    r.array_set(array_value_28, 0, V::I(1))?;
    r.array_set(array_value_28, 1, V::I(1))?;
    r.array_set(array_value_28, 2, V::I(1))?;
    r.array_set(array_value_28, 3, V::I(0))?;
    r.array_set(array_value_24, 3, V::R(array_value_28))?;
    let array_value_29 = r.new_array(10, 4)?;
    r.array_set(array_value_29, 0, V::I(1))?;
    r.array_set(array_value_29, 1, V::I(0))?;
    r.array_set(array_value_29, 2, V::I(0))?;
    r.array_set(array_value_29, 3, V::I(0))?;
    r.array_set(array_value_24, 4, V::R(array_value_29))?;
    let array_value_30 = r.new_array(10, 4)?;
    r.array_set(array_value_30, 0, V::I(1))?;
    r.array_set(array_value_30, 1, V::I(1))?;
    r.array_set(array_value_30, 2, V::I(0))?;
    r.array_set(array_value_30, 3, V::I(0))?;
    r.array_set(array_value_24, 5, V::R(array_value_30))?;
    let array_value_31 = r.new_array(10, 4)?;
    r.array_set(array_value_31, 0, V::I(0))?;
    r.array_set(array_value_31, 1, V::I(1))?;
    r.array_set(array_value_31, 2, V::I(1))?;
    r.array_set(array_value_31, 3, V::I(0))?;
    r.array_set(array_value_24, 6, V::R(array_value_31))?;
    let array_value_32 = r.new_array(10, 4)?;
    r.array_set(array_value_32, 0, V::I(1))?;
    r.array_set(array_value_32, 1, V::I(1))?;
    r.array_set(array_value_32, 2, V::I(0))?;
    r.array_set(array_value_32, 3, V::I(0))?;
    r.array_set(array_value_24, 7, V::R(array_value_32))?;
    r.set(canvas, field::canvas::STATE_BC, V::R(array_value_24))?;
    let array_value_33 = r.new_array(0, 8)?;
    let array_value_34 = r.new_array(10, 4)?;
    r.array_set(array_value_34, 0, V::I(0))?;
    r.array_set(array_value_34, 1, V::I(1))?;
    r.array_set(array_value_34, 2, V::I(1))?;
    r.array_set(array_value_34, 3, V::I(0))?;
    r.array_set(array_value_33, 0, V::R(array_value_34))?;
    let array_value_35 = r.new_array(10, 4)?;
    r.array_set(array_value_35, 0, V::I(1))?;
    r.array_set(array_value_35, 1, V::I(1))?;
    r.array_set(array_value_35, 2, V::I(1))?;
    r.array_set(array_value_35, 3, V::I(1))?;
    r.array_set(array_value_33, 1, V::R(array_value_35))?;
    let array_value_36 = r.new_array(10, 4)?;
    r.array_set(array_value_36, 0, V::I(1))?;
    r.array_set(array_value_36, 1, V::I(1))?;
    r.array_set(array_value_36, 2, V::I(1))?;
    r.array_set(array_value_36, 3, V::I(1))?;
    r.array_set(array_value_33, 2, V::R(array_value_36))?;
    let array_value_37 = r.new_array(10, 4)?;
    r.array_set(array_value_37, 0, V::I(1))?;
    r.array_set(array_value_37, 1, V::I(0))?;
    r.array_set(array_value_37, 2, V::I(0))?;
    r.array_set(array_value_37, 3, V::I(1))?;
    r.array_set(array_value_33, 3, V::R(array_value_37))?;
    let array_value_38 = r.new_array(10, 4)?;
    r.array_set(array_value_38, 0, V::I(0))?;
    r.array_set(array_value_38, 1, V::I(0))?;
    r.array_set(array_value_38, 2, V::I(1))?;
    r.array_set(array_value_38, 3, V::I(1))?;
    r.array_set(array_value_33, 4, V::R(array_value_38))?;
    let array_value_39 = r.new_array(10, 4)?;
    r.array_set(array_value_39, 0, V::I(0))?;
    r.array_set(array_value_39, 1, V::I(1))?;
    r.array_set(array_value_39, 2, V::I(1))?;
    r.array_set(array_value_39, 3, V::I(1))?;
    r.array_set(array_value_33, 5, V::R(array_value_39))?;
    let array_value_40 = r.new_array(10, 4)?;
    r.array_set(array_value_40, 0, V::I(1))?;
    r.array_set(array_value_40, 1, V::I(1))?;
    r.array_set(array_value_40, 2, V::I(1))?;
    r.array_set(array_value_40, 3, V::I(1))?;
    r.array_set(array_value_33, 6, V::R(array_value_40))?;
    let array_value_41 = r.new_array(10, 4)?;
    r.array_set(array_value_41, 0, V::I(0))?;
    r.array_set(array_value_41, 1, V::I(0))?;
    r.array_set(array_value_41, 2, V::I(1))?;
    r.array_set(array_value_41, 3, V::I(1))?;
    r.array_set(array_value_33, 7, V::R(array_value_41))?;
    r.set(canvas, field::canvas::STATE_BD, V::R(array_value_33))?;
    let array_value_42 = r.new_array(0, 8)?;
    let array_value_43 = r.new_array(10, 6)?;
    r.array_set(array_value_43, 0, V::I(0))?;
    r.array_set(array_value_43, 1, V::I(1))?;
    r.array_set(array_value_43, 2, V::I(1))?;
    r.array_set(array_value_43, 3, V::I(1))?;
    r.array_set(array_value_43, 4, V::I(1))?;
    r.array_set(array_value_43, 5, V::I(0))?;
    r.array_set(array_value_42, 0, V::R(array_value_43))?;
    let array_value_44 = r.new_array(10, 6)?;
    r.array_set(array_value_44, 0, V::I(1))?;
    r.array_set(array_value_44, 1, V::I(1))?;
    r.array_set(array_value_44, 2, V::I(1))?;
    r.array_set(array_value_44, 3, V::I(1))?;
    r.array_set(array_value_44, 4, V::I(1))?;
    r.array_set(array_value_44, 5, V::I(1))?;
    r.array_set(array_value_42, 1, V::R(array_value_44))?;
    let array_value_45 = r.new_array(10, 6)?;
    r.array_set(array_value_45, 0, V::I(1))?;
    r.array_set(array_value_45, 1, V::I(1))?;
    r.array_set(array_value_45, 2, V::I(0))?;
    r.array_set(array_value_45, 3, V::I(0))?;
    r.array_set(array_value_45, 4, V::I(1))?;
    r.array_set(array_value_45, 5, V::I(1))?;
    r.array_set(array_value_42, 2, V::R(array_value_45))?;
    let array_value_46 = r.new_array(10, 6)?;
    r.array_set(array_value_46, 0, V::I(1))?;
    r.array_set(array_value_46, 1, V::I(1))?;
    r.array_set(array_value_46, 2, V::I(1))?;
    r.array_set(array_value_46, 3, V::I(0))?;
    r.array_set(array_value_46, 4, V::I(1))?;
    r.array_set(array_value_46, 5, V::I(0))?;
    r.array_set(array_value_42, 3, V::R(array_value_46))?;
    let array_value_47 = r.new_array(10, 6)?;
    r.array_set(array_value_47, 0, V::I(1))?;
    r.array_set(array_value_47, 1, V::I(0))?;
    r.array_set(array_value_47, 2, V::I(0))?;
    r.array_set(array_value_47, 3, V::I(0))?;
    r.array_set(array_value_47, 4, V::I(0))?;
    r.array_set(array_value_47, 5, V::I(0))?;
    r.array_set(array_value_42, 4, V::R(array_value_47))?;
    let array_value_48 = r.new_array(10, 6)?;
    r.array_set(array_value_48, 0, V::I(1))?;
    r.array_set(array_value_48, 1, V::I(0))?;
    r.array_set(array_value_48, 2, V::I(0))?;
    r.array_set(array_value_48, 3, V::I(0))?;
    r.array_set(array_value_48, 4, V::I(0))?;
    r.array_set(array_value_48, 5, V::I(0))?;
    r.array_set(array_value_42, 5, V::R(array_value_48))?;
    let array_value_49 = r.new_array(10, 6)?;
    r.array_set(array_value_49, 0, V::I(1))?;
    r.array_set(array_value_49, 1, V::I(0))?;
    r.array_set(array_value_49, 2, V::I(1))?;
    r.array_set(array_value_49, 3, V::I(0))?;
    r.array_set(array_value_49, 4, V::I(0))?;
    r.array_set(array_value_49, 5, V::I(0))?;
    r.array_set(array_value_42, 6, V::R(array_value_49))?;
    let array_value_50 = r.new_array(10, 6)?;
    r.array_set(array_value_50, 0, V::I(1))?;
    r.array_set(array_value_50, 1, V::I(1))?;
    r.array_set(array_value_50, 2, V::I(1))?;
    r.array_set(array_value_50, 3, V::I(0))?;
    r.array_set(array_value_50, 4, V::I(0))?;
    r.array_set(array_value_50, 5, V::I(0))?;
    r.array_set(array_value_42, 7, V::R(array_value_50))?;
    r.set(canvas, field::canvas::STATE_BE, V::R(array_value_42))?;
    let array_value_51 = r.new_array(0, 8)?;
    let array_value_52 = r.new_array(10, 6)?;
    r.array_set(array_value_52, 0, V::I(1))?;
    r.array_set(array_value_52, 1, V::I(1))?;
    r.array_set(array_value_52, 2, V::I(1))?;
    r.array_set(array_value_52, 3, V::I(1))?;
    r.array_set(array_value_52, 4, V::I(1))?;
    r.array_set(array_value_52, 5, V::I(1))?;
    r.array_set(array_value_51, 0, V::R(array_value_52))?;
    let array_value_53 = r.new_array(10, 6)?;
    r.array_set(array_value_53, 0, V::I(1))?;
    r.array_set(array_value_53, 1, V::I(1))?;
    r.array_set(array_value_53, 2, V::I(1))?;
    r.array_set(array_value_53, 3, V::I(1))?;
    r.array_set(array_value_53, 4, V::I(1))?;
    r.array_set(array_value_53, 5, V::I(0))?;
    r.array_set(array_value_51, 1, V::R(array_value_53))?;
    let array_value_54 = r.new_array(10, 6)?;
    r.array_set(array_value_54, 0, V::I(1))?;
    r.array_set(array_value_54, 1, V::I(1))?;
    r.array_set(array_value_54, 2, V::I(1))?;
    r.array_set(array_value_54, 3, V::I(1))?;
    r.array_set(array_value_54, 4, V::I(0))?;
    r.array_set(array_value_54, 5, V::I(0))?;
    r.array_set(array_value_51, 2, V::R(array_value_54))?;
    let array_value_55 = r.new_array(10, 6)?;
    r.array_set(array_value_55, 0, V::I(1))?;
    r.array_set(array_value_55, 1, V::I(1))?;
    r.array_set(array_value_55, 2, V::I(1))?;
    r.array_set(array_value_55, 3, V::I(0))?;
    r.array_set(array_value_55, 4, V::I(0))?;
    r.array_set(array_value_55, 5, V::I(0))?;
    r.array_set(array_value_51, 3, V::R(array_value_55))?;
    let array_value_56 = r.new_array(10, 6)?;
    r.array_set(array_value_56, 0, V::I(1))?;
    r.array_set(array_value_56, 1, V::I(1))?;
    r.array_set(array_value_56, 2, V::I(0))?;
    r.array_set(array_value_56, 3, V::I(0))?;
    r.array_set(array_value_56, 4, V::I(0))?;
    r.array_set(array_value_56, 5, V::I(0))?;
    r.array_set(array_value_51, 4, V::R(array_value_56))?;
    let array_value_57 = r.new_array(10, 6)?;
    r.array_set(array_value_57, 0, V::I(1))?;
    r.array_set(array_value_57, 1, V::I(1))?;
    r.array_set(array_value_57, 2, V::I(1))?;
    r.array_set(array_value_57, 3, V::I(1))?;
    r.array_set(array_value_57, 4, V::I(0))?;
    r.array_set(array_value_57, 5, V::I(0))?;
    r.array_set(array_value_51, 5, V::R(array_value_57))?;
    let array_value_58 = r.new_array(10, 6)?;
    r.array_set(array_value_58, 0, V::I(1))?;
    r.array_set(array_value_58, 1, V::I(1))?;
    r.array_set(array_value_58, 2, V::I(1))?;
    r.array_set(array_value_58, 3, V::I(1))?;
    r.array_set(array_value_58, 4, V::I(1))?;
    r.array_set(array_value_58, 5, V::I(1))?;
    r.array_set(array_value_51, 6, V::R(array_value_58))?;
    let array_value_59 = r.new_array(10, 6)?;
    r.array_set(array_value_59, 0, V::I(1))?;
    r.array_set(array_value_59, 1, V::I(1))?;
    r.array_set(array_value_59, 2, V::I(1))?;
    r.array_set(array_value_59, 3, V::I(0))?;
    r.array_set(array_value_59, 4, V::I(0))?;
    r.array_set(array_value_59, 5, V::I(0))?;
    r.array_set(array_value_51, 7, V::R(array_value_59))?;
    r.set(canvas, field::canvas::STATE_BF, V::R(array_value_51))?;
    let array_value_60 = r.new_array(0, 8)?;
    let array_value_61 = r.new_array(10, 5)?;
    r.array_set(array_value_61, 0, V::I(0))?;
    r.array_set(array_value_61, 1, V::I(0))?;
    r.array_set(array_value_61, 2, V::I(0))?;
    r.array_set(array_value_61, 3, V::I(1))?;
    r.array_set(array_value_61, 4, V::I(0))?;
    r.array_set(array_value_60, 0, V::R(array_value_61))?;
    let array_value_62 = r.new_array(10, 5)?;
    r.array_set(array_value_62, 0, V::I(0))?;
    r.array_set(array_value_62, 1, V::I(0))?;
    r.array_set(array_value_62, 2, V::I(1))?;
    r.array_set(array_value_62, 3, V::I(1))?;
    r.array_set(array_value_62, 4, V::I(1))?;
    r.array_set(array_value_60, 1, V::R(array_value_62))?;
    let array_value_63 = r.new_array(10, 5)?;
    r.array_set(array_value_63, 0, V::I(0))?;
    r.array_set(array_value_63, 1, V::I(1))?;
    r.array_set(array_value_63, 2, V::I(1))?;
    r.array_set(array_value_63, 3, V::I(1))?;
    r.array_set(array_value_63, 4, V::I(0))?;
    r.array_set(array_value_60, 2, V::R(array_value_63))?;
    let array_value_64 = r.new_array(10, 5)?;
    r.array_set(array_value_64, 0, V::I(1))?;
    r.array_set(array_value_64, 1, V::I(1))?;
    r.array_set(array_value_64, 2, V::I(1))?;
    r.array_set(array_value_64, 3, V::I(1))?;
    r.array_set(array_value_64, 4, V::I(1))?;
    r.array_set(array_value_60, 3, V::R(array_value_64))?;
    let array_value_65 = r.new_array(10, 5)?;
    r.array_set(array_value_65, 0, V::I(1))?;
    r.array_set(array_value_65, 1, V::I(1))?;
    r.array_set(array_value_65, 2, V::I(1))?;
    r.array_set(array_value_65, 3, V::I(1))?;
    r.array_set(array_value_65, 4, V::I(0))?;
    r.array_set(array_value_60, 4, V::R(array_value_65))?;
    let array_value_66 = r.new_array(10, 5)?;
    r.array_set(array_value_66, 0, V::I(0))?;
    r.array_set(array_value_66, 1, V::I(1))?;
    r.array_set(array_value_66, 2, V::I(1))?;
    r.array_set(array_value_66, 3, V::I(1))?;
    r.array_set(array_value_66, 4, V::I(1))?;
    r.array_set(array_value_60, 5, V::R(array_value_66))?;
    let array_value_67 = r.new_array(10, 6)?;
    r.array_set(array_value_67, 0, V::I(0))?;
    r.array_set(array_value_67, 1, V::I(0))?;
    r.array_set(array_value_67, 2, V::I(0))?;
    r.array_set(array_value_67, 3, V::I(0))?;
    r.array_set(array_value_67, 4, V::I(1))?;
    r.array_set(array_value_67, 5, V::I(0))?;
    r.array_set(array_value_60, 6, V::R(array_value_67))?;
    let array_value_68 = r.new_array(10, 5)?;
    r.array_set(array_value_68, 0, V::I(0))?;
    r.array_set(array_value_68, 1, V::I(1))?;
    r.array_set(array_value_68, 2, V::I(1))?;
    r.array_set(array_value_68, 3, V::I(1))?;
    r.array_set(array_value_68, 4, V::I(1))?;
    r.array_set(array_value_60, 7, V::R(array_value_68))?;
    r.set(canvas, field::canvas::STATE_BG, V::R(array_value_60))?;
    let array_value_69 = r.new_array(0, 33)?;
    let text_value =
        r.intern("You found a piece of heart.. your maximum life increases by 1 heart..");
    r.array_set(array_value_69, 0, V::R(text_value))?;
    let text_value_2 = r.intern("You found a piece of heart.. you need 3 more for a full heart..");
    r.array_set(array_value_69, 1, V::R(text_value_2))?;
    let text_value_3 = r.intern("You got a piece of heart.. you need 2 more for a full heart..");
    r.array_set(array_value_69, 2, V::R(text_value_3))?;
    let text_value_4 = r.intern("You got a piece of heart.. you need 1 more for a full heart..");
    r.array_set(array_value_69, 3, V::R(text_value_4))?;
    let text_value_5 = r.intern("You got the wooden sword..");
    r.array_set(array_value_69, 4, V::R(text_value_5))?;
    let text_value_6 = r.intern("You got the bow and arrows..");
    r.array_set(array_value_69, 5, V::R(text_value_6))?;
    let text_value_7 = r.intern("You can now carry 500 rupies..");
    r.array_set(array_value_69, 6, V::R(text_value_7))?;
    let text_value_8 = r.intern("You got the boomerang..");
    r.array_set(array_value_69, 7, V::R(text_value_8))?;
    let text_value_9 = r.intern("You got the showel, lets start digging for treasure..");
    r.array_set(array_value_69, 8, V::R(text_value_9))?;
    let text_value_10 = r.intern("You need the boss key to enter..");
    r.array_set(array_value_69, 9, V::R(text_value_10))?;
    let text_value_11 = r.intern("You found the boss key!");
    r.array_set(array_value_69, 10, V::R(text_value_11))?;
    let text_value_12 = r.intern("You found the bomb bag, you can now carry 10 bombs..");
    r.array_set(array_value_69, 11, V::R(text_value_12))?;
    let text_value_13 = r.intern("You found the raft, you can use it to travel on water..");
    r.array_set(array_value_69, 12, V::R(text_value_13))?;
    let text_value_14 = r.intern("You found the hookshot..");
    r.array_set(array_value_69, 13, V::R(text_value_14))?;
    let text_value_15 = r.intern("You found the candle, use it to light up dark places..");
    r.array_set(array_value_69, 14, V::R(text_value_15))?;
    let text_value_16 =
        r.intern("You found the mirror shield, use it to push objects and reflect magic..");
    r.array_set(array_value_69, 15, V::R(text_value_16))?;
    let text_value_17 = r.intern("You found a piece of the triforce.. only 7 more to collect..");
    r.array_set(array_value_69, 16, V::R(text_value_17))?;
    let text_value_18 = r.intern("You found a piece of the triforce.. only 6 more to collect..");
    r.array_set(array_value_69, 17, V::R(text_value_18))?;
    let text_value_19 = r.intern("You found a piece of the triforce.. only 5 more to collect..");
    r.array_set(array_value_69, 18, V::R(text_value_19))?;
    let text_value_20 = r.intern("You found a piece of the triforce.. only 4 more to collect..");
    r.array_set(array_value_69, 19, V::R(text_value_20))?;
    let text_value_21 = r.intern("You found a piece of the triforce.. only 3 more to collect..");
    r.array_set(array_value_69, 20, V::R(text_value_21))?;
    let text_value_22 = r.intern("You found a piece of the triforce.. only 2 more to collect..");
    r.array_set(array_value_69, 21, V::R(text_value_22))?;
    let text_value_23 = r.intern("You found a piece of the triforce.. only 1 more to collect..");
    r.array_set(array_value_69, 22, V::R(text_value_23))?;
    let text_value_24 = r.intern("The triforce is complete, you can now fight ganon!");
    r.array_set(array_value_69, 23, V::R(text_value_24))?;
    let text_value_25 =
        r.intern("You found a heart container, maximum life increases by 1 heart..");
    r.array_set(array_value_69, 24, V::R(text_value_25))?;
    let text_value_26 = r.intern("I will heal your wounds..");
    r.array_set(array_value_69, 25, V::R(text_value_26))?;
    let text_value_27 = r.intern("You found a strange green suit...?!?");
    r.array_set(array_value_69, 26, V::R(text_value_27))?;
    let text_value_28 = r.intern("Traded the green suit for red armor");
    r.array_set(array_value_69, 27, V::R(text_value_28))?;
    let text_value_29 = r.intern("Have you seen my green suit??");
    r.array_set(array_value_69, 28, V::R(text_value_29))?;
    let text_value_30 = r.intern("Thanks for finding it, good luck with the quest...");
    r.array_set(array_value_69, 29, V::R(text_value_30))?;
    let text_value_31 = r.intern("Come back when you can travel on water....");
    r.array_set(array_value_69, 30, V::R(text_value_31))?;
    let text_value_32 = r.intern("Here, have a fishing rod, now go get some fish!");
    r.array_set(array_value_69, 31, V::R(text_value_32))?;
    let text_value_33 = r.intern("What are you waiting for go get some fish !!");
    r.array_set(array_value_69, 32, V::R(text_value_33))?;
    r.set(canvas, field::canvas::ITEM_MESSAGES, V::R(array_value_69))?;
    return Ok(());
}

/// Initialize. Original reference: `k.<init>(Ljavax/microedition/lcdui/Display;LZelda;)V`.
pub(in crate::game) fn initialize(
    r: &mut Runtime,
    canvas: u32,
    mut display: u32,
    mut application: u32,
) -> Result<(), u32> {
    let mut object_3: u32 = 0;
    let mut carry_0_r: u32 = 0;
    let mut outcome: JResult = Ok(V::I(0));
    host::canvas_initialize(r, canvas)?;
    r.set(canvas, field::canvas::VISUAL_EFFECTS, V::I(1))?;
    r.set(canvas, field::canvas::STATE_AX, V::I(0))?;
    r.set(canvas, field::canvas::PREVIOUS_SCREEN, V::I(0))?;
    r.set(canvas, field::canvas::REFRESH_COMMANDS, V::I(1))?;
    r.set(canvas, field::canvas::EXIT_REQUESTED, V::I(0))?;
    r.set(canvas, field::canvas::MUSIC_VOLUME, V::I(50))?;
    r.set(canvas, field::canvas::EFFECT_VOLUME, V::I(50))?;
    r.set(canvas, field::canvas::STATE_AK_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AL_UPPER, V::I(176))?;
    r.set(canvas, field::canvas::STATE_AM_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AN_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
    r.set(canvas, field::canvas::STATE_AO_UPPER, V::I(1))?;
    r.set(canvas, field::canvas::STATE_U, V::I(0))?;
    r.set(canvas, field::canvas::STATE_V, V::I(1))?;
    r.set(canvas, field::canvas::STATE_AP_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_W, V::I(1))?;
    r.set(canvas, field::canvas::STATE_X, V::I(1))?;
    r.set(canvas, field::canvas::STATE_AQ_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AS_UPPER, V::I(1))?;
    r.set(canvas, field::canvas::STATE_G, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AY, V::I(0))?;
    let array_value = r.new_array(0, 6)?;
    let array_value_2 = r.new_array(10, 6)?;
    r.array_set(array_value_2, 0, V::I(0))?;
    r.array_set(array_value_2, 1, V::I(1))?;
    r.array_set(array_value_2, 2, V::I(1))?;
    r.array_set(array_value_2, 3, V::I(0))?;
    r.array_set(array_value_2, 4, V::I(0))?;
    r.array_set(array_value_2, 5, V::I(0))?;
    r.array_set(array_value, 0, V::R(array_value_2))?;
    let array_value_3 = r.new_array(10, 6)?;
    r.array_set(array_value_3, 0, V::I(0))?;
    r.array_set(array_value_3, 1, V::I(0))?;
    r.array_set(array_value_3, 2, V::I(1))?;
    r.array_set(array_value_3, 3, V::I(0))?;
    r.array_set(array_value_3, 4, V::I(1))?;
    r.array_set(array_value_3, 5, V::I(1))?;
    r.array_set(array_value, 1, V::R(array_value_3))?;
    let array_value_4 = r.new_array(10, 6)?;
    r.array_set(array_value_4, 0, V::I(1))?;
    r.array_set(array_value_4, 1, V::I(1))?;
    r.array_set(array_value_4, 2, V::I(1))?;
    r.array_set(array_value_4, 3, V::I(1))?;
    r.array_set(array_value_4, 4, V::I(1))?;
    r.array_set(array_value_4, 5, V::I(0))?;
    r.array_set(array_value, 2, V::R(array_value_4))?;
    let array_value_5 = r.new_array(10, 6)?;
    r.array_set(array_value_5, 0, V::I(0))?;
    r.array_set(array_value_5, 1, V::I(1))?;
    r.array_set(array_value_5, 2, V::I(1))?;
    r.array_set(array_value_5, 3, V::I(1))?;
    r.array_set(array_value_5, 4, V::I(0))?;
    r.array_set(array_value_5, 5, V::I(0))?;
    r.array_set(array_value, 3, V::R(array_value_5))?;
    let array_value_6 = r.new_array(10, 6)?;
    r.array_set(array_value_6, 0, V::I(0))?;
    r.array_set(array_value_6, 1, V::I(0))?;
    r.array_set(array_value_6, 2, V::I(1))?;
    r.array_set(array_value_6, 3, V::I(0))?;
    r.array_set(array_value_6, 4, V::I(0))?;
    r.array_set(array_value_6, 5, V::I(0))?;
    r.array_set(array_value, 4, V::R(array_value_6))?;
    let array_value_7 = r.new_array(10, 6)?;
    r.array_set(array_value_7, 0, V::I(0))?;
    r.array_set(array_value_7, 1, V::I(1))?;
    r.array_set(array_value_7, 2, V::I(1))?;
    r.array_set(array_value_7, 3, V::I(1))?;
    r.array_set(array_value_7, 4, V::I(0))?;
    r.array_set(array_value_7, 5, V::I(0))?;
    r.array_set(array_value, 5, V::R(array_value_7))?;
    r.set(canvas, field::canvas::STATE_AZ, V::R(array_value))?;
    let array_value_8 = r.new_array(0, 8)?;
    let array_value_9 = r.new_array(10, 4)?;
    r.array_set(array_value_9, 0, V::I(0))?;
    r.array_set(array_value_9, 1, V::I(1))?;
    r.array_set(array_value_9, 2, V::I(1))?;
    r.array_set(array_value_9, 3, V::I(0))?;
    r.array_set(array_value_8, 0, V::R(array_value_9))?;
    let array_value_10 = r.new_array(10, 4)?;
    r.array_set(array_value_10, 0, V::I(0))?;
    r.array_set(array_value_10, 1, V::I(0))?;
    r.array_set(array_value_10, 2, V::I(1))?;
    r.array_set(array_value_10, 3, V::I(1))?;
    r.array_set(array_value_8, 1, V::R(array_value_10))?;
    let array_value_11 = r.new_array(10, 4)?;
    r.array_set(array_value_11, 0, V::I(0))?;
    r.array_set(array_value_11, 1, V::I(0))?;
    r.array_set(array_value_11, 2, V::I(1))?;
    r.array_set(array_value_11, 3, V::I(1))?;
    r.array_set(array_value_8, 2, V::R(array_value_11))?;
    let array_value_12 = r.new_array(10, 4)?;
    r.array_set(array_value_12, 0, V::I(0))?;
    r.array_set(array_value_12, 1, V::I(0))?;
    r.array_set(array_value_12, 2, V::I(1))?;
    r.array_set(array_value_12, 3, V::I(1))?;
    r.array_set(array_value_8, 3, V::R(array_value_12))?;
    let array_value_13 = r.new_array(10, 4)?;
    r.array_set(array_value_13, 0, V::I(0))?;
    r.array_set(array_value_13, 1, V::I(0))?;
    r.array_set(array_value_13, 2, V::I(1))?;
    r.array_set(array_value_13, 3, V::I(1))?;
    r.array_set(array_value_8, 4, V::R(array_value_13))?;
    let array_value_14 = r.new_array(10, 4)?;
    r.array_set(array_value_14, 0, V::I(0))?;
    r.array_set(array_value_14, 1, V::I(0))?;
    r.array_set(array_value_14, 2, V::I(1))?;
    r.array_set(array_value_14, 3, V::I(1))?;
    r.array_set(array_value_8, 5, V::R(array_value_14))?;
    let array_value_15 = r.new_array(10, 4)?;
    r.array_set(array_value_15, 0, V::I(1))?;
    r.array_set(array_value_15, 1, V::I(1))?;
    r.array_set(array_value_15, 2, V::I(1))?;
    r.array_set(array_value_15, 3, V::I(1))?;
    r.array_set(array_value_8, 6, V::R(array_value_15))?;
    let array_value_16 = r.new_array(10, 4)?;
    r.array_set(array_value_16, 0, V::I(0))?;
    r.array_set(array_value_16, 1, V::I(1))?;
    r.array_set(array_value_16, 2, V::I(1))?;
    r.array_set(array_value_16, 3, V::I(0))?;
    r.array_set(array_value_8, 7, V::R(array_value_16))?;
    r.set(canvas, field::canvas::STATE_BA, V::R(array_value_8))?;
    let array_value_17 = r.new_array(0, 6)?;
    let array_value_18 = r.new_array(10, 5)?;
    r.array_set(array_value_18, 0, V::I(0))?;
    r.array_set(array_value_18, 1, V::I(1))?;
    r.array_set(array_value_18, 2, V::I(1))?;
    r.array_set(array_value_18, 3, V::I(0))?;
    r.array_set(array_value_18, 4, V::I(0))?;
    r.array_set(array_value_17, 0, V::R(array_value_18))?;
    let array_value_19 = r.new_array(10, 5)?;
    r.array_set(array_value_19, 0, V::I(0))?;
    r.array_set(array_value_19, 1, V::I(0))?;
    r.array_set(array_value_19, 2, V::I(1))?;
    r.array_set(array_value_19, 3, V::I(0))?;
    r.array_set(array_value_19, 4, V::I(1))?;
    r.array_set(array_value_17, 1, V::R(array_value_19))?;
    let array_value_20 = r.new_array(10, 5)?;
    r.array_set(array_value_20, 0, V::I(1))?;
    r.array_set(array_value_20, 1, V::I(1))?;
    r.array_set(array_value_20, 2, V::I(1))?;
    r.array_set(array_value_20, 3, V::I(1))?;
    r.array_set(array_value_20, 4, V::I(1))?;
    r.array_set(array_value_17, 2, V::R(array_value_20))?;
    let array_value_21 = r.new_array(10, 5)?;
    r.array_set(array_value_21, 0, V::I(1))?;
    r.array_set(array_value_21, 1, V::I(1))?;
    r.array_set(array_value_21, 2, V::I(1))?;
    r.array_set(array_value_21, 3, V::I(1))?;
    r.array_set(array_value_21, 4, V::I(1))?;
    r.array_set(array_value_17, 3, V::R(array_value_21))?;
    let array_value_22 = r.new_array(10, 5)?;
    r.array_set(array_value_22, 0, V::I(1))?;
    r.array_set(array_value_22, 1, V::I(0))?;
    r.array_set(array_value_22, 2, V::I(1))?;
    r.array_set(array_value_22, 3, V::I(0))?;
    r.array_set(array_value_22, 4, V::I(0))?;
    r.array_set(array_value_17, 4, V::R(array_value_22))?;
    let array_value_23 = r.new_array(10, 5)?;
    r.array_set(array_value_23, 0, V::I(0))?;
    r.array_set(array_value_23, 1, V::I(0))?;
    r.array_set(array_value_23, 2, V::I(1))?;
    r.array_set(array_value_23, 3, V::I(1))?;
    r.array_set(array_value_23, 4, V::I(0))?;
    r.array_set(array_value_17, 5, V::R(array_value_23))?;
    r.set(canvas, field::canvas::STATE_BB, V::R(array_value_17))?;
    let array_value_24 = r.new_array(0, 8)?;
    let array_value_25 = r.new_array(10, 4)?;
    r.array_set(array_value_25, 0, V::I(1))?;
    r.array_set(array_value_25, 1, V::I(1))?;
    r.array_set(array_value_25, 2, V::I(1))?;
    r.array_set(array_value_25, 3, V::I(1))?;
    r.array_set(array_value_24, 0, V::R(array_value_25))?;
    let array_value_26 = r.new_array(10, 4)?;
    r.array_set(array_value_26, 0, V::I(1))?;
    r.array_set(array_value_26, 1, V::I(0))?;
    r.array_set(array_value_26, 2, V::I(1))?;
    r.array_set(array_value_26, 3, V::I(1))?;
    r.array_set(array_value_24, 1, V::R(array_value_26))?;
    let array_value_27 = r.new_array(10, 4)?;
    r.array_set(array_value_27, 0, V::I(1))?;
    r.array_set(array_value_27, 1, V::I(1))?;
    r.array_set(array_value_27, 2, V::I(0))?;
    r.array_set(array_value_27, 3, V::I(0))?;
    r.array_set(array_value_24, 2, V::R(array_value_27))?;
    let array_value_28 = r.new_array(10, 4)?;
    r.array_set(array_value_28, 0, V::I(1))?;
    r.array_set(array_value_28, 1, V::I(1))?;
    r.array_set(array_value_28, 2, V::I(1))?;
    r.array_set(array_value_28, 3, V::I(0))?;
    r.array_set(array_value_24, 3, V::R(array_value_28))?;
    let array_value_29 = r.new_array(10, 4)?;
    r.array_set(array_value_29, 0, V::I(1))?;
    r.array_set(array_value_29, 1, V::I(0))?;
    r.array_set(array_value_29, 2, V::I(0))?;
    r.array_set(array_value_29, 3, V::I(0))?;
    r.array_set(array_value_24, 4, V::R(array_value_29))?;
    let array_value_30 = r.new_array(10, 4)?;
    r.array_set(array_value_30, 0, V::I(1))?;
    r.array_set(array_value_30, 1, V::I(1))?;
    r.array_set(array_value_30, 2, V::I(0))?;
    r.array_set(array_value_30, 3, V::I(0))?;
    r.array_set(array_value_24, 5, V::R(array_value_30))?;
    let array_value_31 = r.new_array(10, 4)?;
    r.array_set(array_value_31, 0, V::I(0))?;
    r.array_set(array_value_31, 1, V::I(1))?;
    r.array_set(array_value_31, 2, V::I(1))?;
    r.array_set(array_value_31, 3, V::I(0))?;
    r.array_set(array_value_24, 6, V::R(array_value_31))?;
    let array_value_32 = r.new_array(10, 4)?;
    r.array_set(array_value_32, 0, V::I(1))?;
    r.array_set(array_value_32, 1, V::I(1))?;
    r.array_set(array_value_32, 2, V::I(0))?;
    r.array_set(array_value_32, 3, V::I(0))?;
    r.array_set(array_value_24, 7, V::R(array_value_32))?;
    r.set(canvas, field::canvas::STATE_BC, V::R(array_value_24))?;
    let array_value_33 = r.new_array(0, 8)?;
    let array_value_34 = r.new_array(10, 4)?;
    r.array_set(array_value_34, 0, V::I(0))?;
    r.array_set(array_value_34, 1, V::I(1))?;
    r.array_set(array_value_34, 2, V::I(1))?;
    r.array_set(array_value_34, 3, V::I(0))?;
    r.array_set(array_value_33, 0, V::R(array_value_34))?;
    let array_value_35 = r.new_array(10, 4)?;
    r.array_set(array_value_35, 0, V::I(1))?;
    r.array_set(array_value_35, 1, V::I(1))?;
    r.array_set(array_value_35, 2, V::I(1))?;
    r.array_set(array_value_35, 3, V::I(1))?;
    r.array_set(array_value_33, 1, V::R(array_value_35))?;
    let array_value_36 = r.new_array(10, 4)?;
    r.array_set(array_value_36, 0, V::I(1))?;
    r.array_set(array_value_36, 1, V::I(1))?;
    r.array_set(array_value_36, 2, V::I(1))?;
    r.array_set(array_value_36, 3, V::I(1))?;
    r.array_set(array_value_33, 2, V::R(array_value_36))?;
    let array_value_37 = r.new_array(10, 4)?;
    r.array_set(array_value_37, 0, V::I(1))?;
    r.array_set(array_value_37, 1, V::I(0))?;
    r.array_set(array_value_37, 2, V::I(0))?;
    r.array_set(array_value_37, 3, V::I(1))?;
    r.array_set(array_value_33, 3, V::R(array_value_37))?;
    let array_value_38 = r.new_array(10, 4)?;
    r.array_set(array_value_38, 0, V::I(0))?;
    r.array_set(array_value_38, 1, V::I(0))?;
    r.array_set(array_value_38, 2, V::I(1))?;
    r.array_set(array_value_38, 3, V::I(1))?;
    r.array_set(array_value_33, 4, V::R(array_value_38))?;
    let array_value_39 = r.new_array(10, 4)?;
    r.array_set(array_value_39, 0, V::I(0))?;
    r.array_set(array_value_39, 1, V::I(1))?;
    r.array_set(array_value_39, 2, V::I(1))?;
    r.array_set(array_value_39, 3, V::I(1))?;
    r.array_set(array_value_33, 5, V::R(array_value_39))?;
    let array_value_40 = r.new_array(10, 4)?;
    r.array_set(array_value_40, 0, V::I(1))?;
    r.array_set(array_value_40, 1, V::I(1))?;
    r.array_set(array_value_40, 2, V::I(1))?;
    r.array_set(array_value_40, 3, V::I(1))?;
    r.array_set(array_value_33, 6, V::R(array_value_40))?;
    let array_value_41 = r.new_array(10, 4)?;
    r.array_set(array_value_41, 0, V::I(0))?;
    r.array_set(array_value_41, 1, V::I(0))?;
    r.array_set(array_value_41, 2, V::I(1))?;
    r.array_set(array_value_41, 3, V::I(1))?;
    r.array_set(array_value_33, 7, V::R(array_value_41))?;
    r.set(canvas, field::canvas::STATE_BD, V::R(array_value_33))?;
    let array_value_42 = r.new_array(0, 8)?;
    let array_value_43 = r.new_array(10, 6)?;
    r.array_set(array_value_43, 0, V::I(0))?;
    r.array_set(array_value_43, 1, V::I(1))?;
    r.array_set(array_value_43, 2, V::I(1))?;
    r.array_set(array_value_43, 3, V::I(1))?;
    r.array_set(array_value_43, 4, V::I(1))?;
    r.array_set(array_value_43, 5, V::I(0))?;
    r.array_set(array_value_42, 0, V::R(array_value_43))?;
    let array_value_44 = r.new_array(10, 6)?;
    r.array_set(array_value_44, 0, V::I(1))?;
    r.array_set(array_value_44, 1, V::I(1))?;
    r.array_set(array_value_44, 2, V::I(1))?;
    r.array_set(array_value_44, 3, V::I(1))?;
    r.array_set(array_value_44, 4, V::I(1))?;
    r.array_set(array_value_44, 5, V::I(1))?;
    r.array_set(array_value_42, 1, V::R(array_value_44))?;
    let array_value_45 = r.new_array(10, 6)?;
    r.array_set(array_value_45, 0, V::I(1))?;
    r.array_set(array_value_45, 1, V::I(1))?;
    r.array_set(array_value_45, 2, V::I(0))?;
    r.array_set(array_value_45, 3, V::I(0))?;
    r.array_set(array_value_45, 4, V::I(1))?;
    r.array_set(array_value_45, 5, V::I(1))?;
    r.array_set(array_value_42, 2, V::R(array_value_45))?;
    let array_value_46 = r.new_array(10, 6)?;
    r.array_set(array_value_46, 0, V::I(1))?;
    r.array_set(array_value_46, 1, V::I(1))?;
    r.array_set(array_value_46, 2, V::I(1))?;
    r.array_set(array_value_46, 3, V::I(0))?;
    r.array_set(array_value_46, 4, V::I(1))?;
    r.array_set(array_value_46, 5, V::I(0))?;
    r.array_set(array_value_42, 3, V::R(array_value_46))?;
    let array_value_47 = r.new_array(10, 6)?;
    r.array_set(array_value_47, 0, V::I(1))?;
    r.array_set(array_value_47, 1, V::I(0))?;
    r.array_set(array_value_47, 2, V::I(0))?;
    r.array_set(array_value_47, 3, V::I(0))?;
    r.array_set(array_value_47, 4, V::I(0))?;
    r.array_set(array_value_47, 5, V::I(0))?;
    r.array_set(array_value_42, 4, V::R(array_value_47))?;
    let array_value_48 = r.new_array(10, 6)?;
    r.array_set(array_value_48, 0, V::I(1))?;
    r.array_set(array_value_48, 1, V::I(0))?;
    r.array_set(array_value_48, 2, V::I(0))?;
    r.array_set(array_value_48, 3, V::I(0))?;
    r.array_set(array_value_48, 4, V::I(0))?;
    r.array_set(array_value_48, 5, V::I(0))?;
    r.array_set(array_value_42, 5, V::R(array_value_48))?;
    let array_value_49 = r.new_array(10, 6)?;
    r.array_set(array_value_49, 0, V::I(1))?;
    r.array_set(array_value_49, 1, V::I(0))?;
    r.array_set(array_value_49, 2, V::I(1))?;
    r.array_set(array_value_49, 3, V::I(0))?;
    r.array_set(array_value_49, 4, V::I(0))?;
    r.array_set(array_value_49, 5, V::I(0))?;
    r.array_set(array_value_42, 6, V::R(array_value_49))?;
    let array_value_50 = r.new_array(10, 6)?;
    r.array_set(array_value_50, 0, V::I(1))?;
    r.array_set(array_value_50, 1, V::I(1))?;
    r.array_set(array_value_50, 2, V::I(1))?;
    r.array_set(array_value_50, 3, V::I(0))?;
    r.array_set(array_value_50, 4, V::I(0))?;
    r.array_set(array_value_50, 5, V::I(0))?;
    r.array_set(array_value_42, 7, V::R(array_value_50))?;
    r.set(canvas, field::canvas::STATE_BE, V::R(array_value_42))?;
    let array_value_51 = r.new_array(0, 8)?;
    let array_value_52 = r.new_array(10, 6)?;
    r.array_set(array_value_52, 0, V::I(1))?;
    r.array_set(array_value_52, 1, V::I(1))?;
    r.array_set(array_value_52, 2, V::I(1))?;
    r.array_set(array_value_52, 3, V::I(1))?;
    r.array_set(array_value_52, 4, V::I(1))?;
    r.array_set(array_value_52, 5, V::I(1))?;
    r.array_set(array_value_51, 0, V::R(array_value_52))?;
    let array_value_53 = r.new_array(10, 6)?;
    r.array_set(array_value_53, 0, V::I(1))?;
    r.array_set(array_value_53, 1, V::I(1))?;
    r.array_set(array_value_53, 2, V::I(1))?;
    r.array_set(array_value_53, 3, V::I(1))?;
    r.array_set(array_value_53, 4, V::I(1))?;
    r.array_set(array_value_53, 5, V::I(0))?;
    r.array_set(array_value_51, 1, V::R(array_value_53))?;
    let array_value_54 = r.new_array(10, 6)?;
    r.array_set(array_value_54, 0, V::I(1))?;
    r.array_set(array_value_54, 1, V::I(1))?;
    r.array_set(array_value_54, 2, V::I(1))?;
    r.array_set(array_value_54, 3, V::I(1))?;
    r.array_set(array_value_54, 4, V::I(0))?;
    r.array_set(array_value_54, 5, V::I(0))?;
    r.array_set(array_value_51, 2, V::R(array_value_54))?;
    let array_value_55 = r.new_array(10, 6)?;
    r.array_set(array_value_55, 0, V::I(1))?;
    r.array_set(array_value_55, 1, V::I(1))?;
    r.array_set(array_value_55, 2, V::I(1))?;
    r.array_set(array_value_55, 3, V::I(0))?;
    r.array_set(array_value_55, 4, V::I(0))?;
    r.array_set(array_value_55, 5, V::I(0))?;
    r.array_set(array_value_51, 3, V::R(array_value_55))?;
    let array_value_56 = r.new_array(10, 6)?;
    r.array_set(array_value_56, 0, V::I(1))?;
    r.array_set(array_value_56, 1, V::I(1))?;
    r.array_set(array_value_56, 2, V::I(0))?;
    r.array_set(array_value_56, 3, V::I(0))?;
    r.array_set(array_value_56, 4, V::I(0))?;
    r.array_set(array_value_56, 5, V::I(0))?;
    r.array_set(array_value_51, 4, V::R(array_value_56))?;
    let array_value_57 = r.new_array(10, 6)?;
    r.array_set(array_value_57, 0, V::I(1))?;
    r.array_set(array_value_57, 1, V::I(1))?;
    r.array_set(array_value_57, 2, V::I(1))?;
    r.array_set(array_value_57, 3, V::I(1))?;
    r.array_set(array_value_57, 4, V::I(0))?;
    r.array_set(array_value_57, 5, V::I(0))?;
    r.array_set(array_value_51, 5, V::R(array_value_57))?;
    let array_value_58 = r.new_array(10, 6)?;
    r.array_set(array_value_58, 0, V::I(1))?;
    r.array_set(array_value_58, 1, V::I(1))?;
    r.array_set(array_value_58, 2, V::I(1))?;
    r.array_set(array_value_58, 3, V::I(1))?;
    r.array_set(array_value_58, 4, V::I(1))?;
    r.array_set(array_value_58, 5, V::I(1))?;
    r.array_set(array_value_51, 6, V::R(array_value_58))?;
    let array_value_59 = r.new_array(10, 6)?;
    r.array_set(array_value_59, 0, V::I(1))?;
    r.array_set(array_value_59, 1, V::I(1))?;
    r.array_set(array_value_59, 2, V::I(1))?;
    r.array_set(array_value_59, 3, V::I(0))?;
    r.array_set(array_value_59, 4, V::I(0))?;
    r.array_set(array_value_59, 5, V::I(0))?;
    r.array_set(array_value_51, 7, V::R(array_value_59))?;
    r.set(canvas, field::canvas::STATE_BF, V::R(array_value_51))?;
    let array_value_60 = r.new_array(0, 8)?;
    let array_value_61 = r.new_array(10, 5)?;
    r.array_set(array_value_61, 0, V::I(0))?;
    r.array_set(array_value_61, 1, V::I(0))?;
    r.array_set(array_value_61, 2, V::I(0))?;
    r.array_set(array_value_61, 3, V::I(1))?;
    r.array_set(array_value_61, 4, V::I(0))?;
    r.array_set(array_value_60, 0, V::R(array_value_61))?;
    let array_value_62 = r.new_array(10, 5)?;
    r.array_set(array_value_62, 0, V::I(0))?;
    r.array_set(array_value_62, 1, V::I(0))?;
    r.array_set(array_value_62, 2, V::I(1))?;
    r.array_set(array_value_62, 3, V::I(1))?;
    r.array_set(array_value_62, 4, V::I(1))?;
    r.array_set(array_value_60, 1, V::R(array_value_62))?;
    let array_value_63 = r.new_array(10, 5)?;
    r.array_set(array_value_63, 0, V::I(0))?;
    r.array_set(array_value_63, 1, V::I(1))?;
    r.array_set(array_value_63, 2, V::I(1))?;
    r.array_set(array_value_63, 3, V::I(1))?;
    r.array_set(array_value_63, 4, V::I(0))?;
    r.array_set(array_value_60, 2, V::R(array_value_63))?;
    let array_value_64 = r.new_array(10, 5)?;
    r.array_set(array_value_64, 0, V::I(1))?;
    r.array_set(array_value_64, 1, V::I(1))?;
    r.array_set(array_value_64, 2, V::I(1))?;
    r.array_set(array_value_64, 3, V::I(1))?;
    r.array_set(array_value_64, 4, V::I(1))?;
    r.array_set(array_value_60, 3, V::R(array_value_64))?;
    let array_value_65 = r.new_array(10, 5)?;
    r.array_set(array_value_65, 0, V::I(1))?;
    r.array_set(array_value_65, 1, V::I(1))?;
    r.array_set(array_value_65, 2, V::I(1))?;
    r.array_set(array_value_65, 3, V::I(1))?;
    r.array_set(array_value_65, 4, V::I(0))?;
    r.array_set(array_value_60, 4, V::R(array_value_65))?;
    let array_value_66 = r.new_array(10, 5)?;
    r.array_set(array_value_66, 0, V::I(0))?;
    r.array_set(array_value_66, 1, V::I(1))?;
    r.array_set(array_value_66, 2, V::I(1))?;
    r.array_set(array_value_66, 3, V::I(1))?;
    r.array_set(array_value_66, 4, V::I(1))?;
    r.array_set(array_value_60, 5, V::R(array_value_66))?;
    let array_value_67 = r.new_array(10, 6)?;
    r.array_set(array_value_67, 0, V::I(0))?;
    r.array_set(array_value_67, 1, V::I(0))?;
    r.array_set(array_value_67, 2, V::I(0))?;
    r.array_set(array_value_67, 3, V::I(0))?;
    r.array_set(array_value_67, 4, V::I(1))?;
    r.array_set(array_value_67, 5, V::I(0))?;
    r.array_set(array_value_60, 6, V::R(array_value_67))?;
    let array_value_68 = r.new_array(10, 5)?;
    r.array_set(array_value_68, 0, V::I(0))?;
    r.array_set(array_value_68, 1, V::I(1))?;
    r.array_set(array_value_68, 2, V::I(1))?;
    r.array_set(array_value_68, 3, V::I(1))?;
    r.array_set(array_value_68, 4, V::I(1))?;
    r.array_set(array_value_60, 7, V::R(array_value_68))?;
    r.set(canvas, field::canvas::STATE_BG, V::R(array_value_60))?;
    let array_value_69 = r.new_array(0, 33)?;
    let text_value =
        r.intern("You found a piece of heart.. your maximum life increases by 1 heart..");
    r.array_set(array_value_69, 0, V::R(text_value))?;
    let text_value_2 = r.intern("You found a piece of heart.. you need 3 more for a full heart..");
    r.array_set(array_value_69, 1, V::R(text_value_2))?;
    let text_value_3 = r.intern("You got a piece of heart.. you need 2 more for a full heart..");
    r.array_set(array_value_69, 2, V::R(text_value_3))?;
    let text_value_4 = r.intern("You got a piece of heart.. you need 1 more for a full heart..");
    r.array_set(array_value_69, 3, V::R(text_value_4))?;
    let text_value_5 = r.intern("You got the wooden sword..");
    r.array_set(array_value_69, 4, V::R(text_value_5))?;
    let text_value_6 = r.intern("You got the bow and arrows..");
    r.array_set(array_value_69, 5, V::R(text_value_6))?;
    let text_value_7 = r.intern("You can now carry 500 rupies..");
    r.array_set(array_value_69, 6, V::R(text_value_7))?;
    let text_value_8 = r.intern("You got the boomerang..");
    r.array_set(array_value_69, 7, V::R(text_value_8))?;
    let text_value_9 = r.intern("You got the showel, lets start digging for treasure..");
    r.array_set(array_value_69, 8, V::R(text_value_9))?;
    let text_value_10 = r.intern("You need the boss key to enter..");
    r.array_set(array_value_69, 9, V::R(text_value_10))?;
    let text_value_11 = r.intern("You found the boss key!");
    r.array_set(array_value_69, 10, V::R(text_value_11))?;
    let text_value_12 = r.intern("You found the bomb bag, you can now carry 10 bombs..");
    r.array_set(array_value_69, 11, V::R(text_value_12))?;
    let text_value_13 = r.intern("You found the raft, you can use it to travel on water..");
    r.array_set(array_value_69, 12, V::R(text_value_13))?;
    let text_value_14 = r.intern("You found the hookshot..");
    r.array_set(array_value_69, 13, V::R(text_value_14))?;
    let text_value_15 = r.intern("You found the candle, use it to light up dark places..");
    r.array_set(array_value_69, 14, V::R(text_value_15))?;
    let text_value_16 =
        r.intern("You found the mirror shield, use it to push objects and reflect magic..");
    r.array_set(array_value_69, 15, V::R(text_value_16))?;
    let text_value_17 = r.intern("You found a piece of the triforce.. only 7 more to collect..");
    r.array_set(array_value_69, 16, V::R(text_value_17))?;
    let text_value_18 = r.intern("You found a piece of the triforce.. only 6 more to collect..");
    r.array_set(array_value_69, 17, V::R(text_value_18))?;
    let text_value_19 = r.intern("You found a piece of the triforce.. only 5 more to collect..");
    r.array_set(array_value_69, 18, V::R(text_value_19))?;
    let text_value_20 = r.intern("You found a piece of the triforce.. only 4 more to collect..");
    r.array_set(array_value_69, 19, V::R(text_value_20))?;
    let text_value_21 = r.intern("You found a piece of the triforce.. only 3 more to collect..");
    r.array_set(array_value_69, 20, V::R(text_value_21))?;
    let text_value_22 = r.intern("You found a piece of the triforce.. only 2 more to collect..");
    r.array_set(array_value_69, 21, V::R(text_value_22))?;
    let text_value_23 = r.intern("You found a piece of the triforce.. only 1 more to collect..");
    r.array_set(array_value_69, 22, V::R(text_value_23))?;
    let text_value_24 = r.intern("The triforce is complete, you can now fight ganon!");
    r.array_set(array_value_69, 23, V::R(text_value_24))?;
    let text_value_25 =
        r.intern("You found a heart container, maximum life increases by 1 heart..");
    r.array_set(array_value_69, 24, V::R(text_value_25))?;
    let text_value_26 = r.intern("I will heal your wounds..");
    r.array_set(array_value_69, 25, V::R(text_value_26))?;
    let text_value_27 = r.intern("You found a strange green suit...?!?");
    r.array_set(array_value_69, 26, V::R(text_value_27))?;
    let text_value_28 = r.intern("Traded the green suit for red armor");
    r.array_set(array_value_69, 27, V::R(text_value_28))?;
    let text_value_29 = r.intern("Have you seen my green suit??");
    r.array_set(array_value_69, 28, V::R(text_value_29))?;
    let text_value_30 = r.intern("Thanks for finding it, good luck with the quest...");
    r.array_set(array_value_69, 29, V::R(text_value_30))?;
    let text_value_31 = r.intern("Come back when you can travel on water....");
    r.array_set(array_value_69, 30, V::R(text_value_31))?;
    let text_value_32 = r.intern("Here, have a fishing rod, now go get some fish!");
    r.array_set(array_value_69, 31, V::R(text_value_32))?;
    let text_value_33 = r.intern("What are you waiting for go get some fish !!");
    r.array_set(array_value_69, 32, V::R(text_value_33))?;
    r.set(canvas, field::canvas::ITEM_MESSAGES, V::R(array_value_69))?;
    r.set(canvas, field::canvas::APPLICATION, V::R(application))?;
    let array_value_70 = r.new_array(0, 3)?;
    r.set(canvas, field::canvas::SAVE_SLOTS, V::R(array_value_70))?;
    r.set(canvas, field::canvas::STATE_AR, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AQ, V::I(0))?;
    r.set(canvas, field::canvas::MENU_SELECTION, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AB_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AC_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::INVENTORY_OPEN, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AK, V::I(1))?;
    r.set(canvas, field::canvas::INVENTORY_PAGE, V::I(0))?;
    r.set(canvas, field::canvas::ZONE, V::I(0))?;
    r.set(canvas, field::canvas::MAP, V::I(120))?;
    let canvas_get_height_value = host::canvas_get_height(r, canvas)?;
    r.set(
        canvas,
        field::canvas::STATE_AL_UPPER,
        V::I(canvas_get_height_value),
    )?;
    r.set(canvas, field::canvas::STATE_AV, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AO, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AP, V::I(0))?;
    r.set(canvas, field::canvas::STATE_I, V::I(0))?;
    r.set(canvas, field::canvas::SHOW_ITEM_MESSAGE, V::I(0))?;
    r.set(canvas, field::canvas::ITEM_MESSAGE, V::I(0))?;
    r.set(canvas, field::canvas::REDRAW, V::I(1))?;
    r.set(canvas, field::canvas::STATE_L, V::I(0))?;
    r.set(canvas, field::canvas::DISPLAY, V::R(display))?;
    let canvas_get_width_value = host::canvas_get_width(r, canvas)?;
    r.set(
        canvas,
        field::canvas::STATE_L_UPPER,
        V::I(canvas_get_width_value),
    )?;
    let canvas_get_height_value_2 = host::canvas_get_height(r, canvas)?;
    r.set(
        canvas,
        field::canvas::STATE_M,
        V::I(canvas_get_height_value_2),
    )?;
    r.set(canvas, field::canvas::STATE_AT_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AU_UPPER, V::I(0))?;
    r.set(canvas, field::canvas::STATE_F, V::I(0))?;
    r.set(canvas, field::canvas::STATE_AV_UPPER, V::I(100))?;
    let canvas_state_l_upper = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    r.set(canvas, field::canvas::STATE_N, V::I(canvas_state_l_upper))?;
    let canvas_state_m = r.get(canvas, field::canvas::STATE_M)?.i();
    r.set(canvas, field::canvas::STATE_O, V::I(canvas_state_m))?;
    r.set(canvas, field::canvas::STATE_AN, V::I(32))?;
    let canvas_state_l_upper_2 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    r.set(
        canvas,
        field::canvas::STATE_AL,
        V::I((256_i32).wrapping_sub(canvas_state_l_upper_2)),
    )?;
    let canvas_state_m_2 = r.get(canvas, field::canvas::STATE_M)?.i();
    let canvas_state_an = r.get(canvas, field::canvas::STATE_AN)?.i();
    r.set(
        canvas,
        field::canvas::STATE_AM,
        V::I((176_i32).wrapping_sub(canvas_state_m_2.wrapping_sub(canvas_state_an))),
    )?;
    r.set(canvas, field::canvas::SCREEN_STATE, V::I(0))?;
    r.set(canvas, field::canvas::UP_KEY, V::I(50))?;
    r.set(canvas, field::canvas::DOWN_KEY, V::I(56))?;
    r.set(canvas, field::canvas::LEFT_KEY, V::I(52))?;
    r.set(canvas, field::canvas::RIGHT_KEY, V::I(54))?;
    r.set(canvas, field::canvas::PRIMARY_KEY, V::I(53))?;
    r.set(canvas, field::canvas::SECONDARY_KEY, V::I(55))?;
    r.set(canvas, field::canvas::MENU_KEY, V::I(57))?;
    let object_value = r.alloc("java/util/Random", Native::Empty);
    host::random_initialize(r, object_value)?;
    r.set(canvas, field::canvas::RANDOM, V::R(object_value))?;
    let object_value_2 = r.alloc("java/util/Vector", Native::Empty);
    host::vector_initialize(r, object_value_2)?;
    r.set(canvas, field::canvas::STATE_AW_UPPER, V::R(object_value_2))?;
    r.set(canvas, field::canvas::STATE_C, V::I(1))?;
    let image_create_image_2_value = host::image_create_image_2(r, 256, 176)?;
    r.set(
        canvas,
        field::canvas::SCENE_IMAGE,
        V::R(image_create_image_2_value),
    )?;
    let canvas_state_l_upper_3 = r.get(canvas, field::canvas::STATE_L_UPPER)?.i();
    let image_create_image_2_value_2 = host::image_create_image_2(r, canvas_state_l_upper_3, 32)?;
    r.set(
        canvas,
        field::canvas::STATE_S,
        V::R(image_create_image_2_value_2),
    )?;
    let text_value_34 = r.intern("/Images/Zelda_mobile_logo.png");
    outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_34)?)) })();
    (carry_0_r,) = (canvas,);
    if outcome.is_ok() {
        let image_create_image_value = outcome.unwrap().r();
        r.set(
            carry_0_r,
            field::canvas::STATE_V_UPPER,
            V::R(image_create_image_value),
        )?;
        let text_value_35 = r.intern("/Images/Numbers2.png");
        outcome = (|| -> JResult { Ok(V::R(host::image_create_image(r, text_value_35)?)) })();
        (carry_0_r,) = (canvas,);
        if outcome.is_ok() {
            let image_create_image_value_2 = outcome.unwrap().r();
            r.set(
                carry_0_r,
                field::canvas::STATE_X_UPPER,
                V::R(image_create_image_value_2),
            )?;
        } else {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/io/IOException")) {
                return Err(outcome.unwrap_err());
            }
        }
    } else {
        carry_0_r = outcome.unwrap_err();
        if !(r.is_instance(carry_0_r, "java/io/IOException")) {
            return Err(outcome.unwrap_err());
        }
    }
    r.set(canvas, field::canvas::PLAYER, V::R(0))?;

    display = canvas;
    outcome = (|| -> JResult { Ok(V::R(host::record_store_list_record_stores(r)?)) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        r.set(display, field::canvas::UP_KEY, V::I(50))?;
        r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
        r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
        r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
        r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
        r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
        r.set(display, field::canvas::MENU_KEY, V::I(57))?;
        r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
        r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
        r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
        r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
        r.set(display, field::canvas::STATE_AK, V::I(1))?;
        r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
        r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
        let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
        if canvas_state_ap_upper_read != 0 {
            host::canvas_set_full_screen_mode(r, canvas, 1)?;
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
        return Ok(());
    }
    let record_store_list_record_stores_value = outcome.unwrap().r();
    if record_store_list_record_stores_value == 0 {
        outcome = (|| -> JResult {
            r.set(display, field::canvas::CONFIGURE_PHONE_KEYS, V::I(1))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::STATE_X, V::I(1))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::STATE_W, V::I(1))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::STATE_AX, V::I(0))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }

        let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
        if canvas_state_ap_upper_read != 0 {
            host::canvas_set_full_screen_mode(r, canvas, 1)?;
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
        return Ok(());
    }
    let text_value_36 = r.intern("ZeldaSettings");
    outcome = (|| -> JResult {
        Ok(V::R(host::record_store_open_record_store(
            r,
            text_value_36,
            0,
        )?))
    })();
    (carry_0_r,) = (display,);
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        r.set(display, field::canvas::UP_KEY, V::I(50))?;
        r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
        r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
        r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
        r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
        r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
        r.set(display, field::canvas::MENU_KEY, V::I(57))?;
        r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
        r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
        r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
        r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
        r.set(display, field::canvas::STATE_AK, V::I(1))?;
        r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
        r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
        let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
        if canvas_state_ap_upper_read != 0 {
            host::canvas_set_full_screen_mode(r, canvas, 1)?;
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
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
        r.set(display, field::canvas::UP_KEY, V::I(50))?;
        r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
        r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
        r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
        r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
        r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
        r.set(display, field::canvas::MENU_KEY, V::I(57))?;
        r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
        r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
        r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
        r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
        r.set(display, field::canvas::STATE_AK, V::I(1))?;
        r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
        r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
        let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
        if canvas_state_ap_upper_read != 0 {
            host::canvas_set_full_screen_mode(r, canvas, 1)?;
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
        return Ok(());
    }
    application = 0;
    outcome = (|| -> JResult { Ok(V::R(r.get(display, field::canvas::RECORD_STORE)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        r.set(display, field::canvas::UP_KEY, V::I(50))?;
        r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
        r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
        r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
        r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
        r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
        r.set(display, field::canvas::MENU_KEY, V::I(57))?;
        r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
        r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
        r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
        r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
        r.set(display, field::canvas::STATE_AK, V::I(1))?;
        r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
        r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
        let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
        if canvas_state_ap_upper_read != 0 {
            host::canvas_set_full_screen_mode(r, canvas, 1)?;
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
        return Ok(());
    }
    let record_store_value = outcome.unwrap().r();
    outcome = (|| -> JResult {
        Ok(V::R(host::record_store_get_record(
            r,
            record_store_value,
            1,
        )?))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        r.set(display, field::canvas::UP_KEY, V::I(50))?;
        r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
        r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
        r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
        r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
        r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
        r.set(display, field::canvas::MENU_KEY, V::I(57))?;
        r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
        r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
        r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
        r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
        r.set(display, field::canvas::STATE_AK, V::I(1))?;
        r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
        r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
        let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
        if canvas_state_ap_upper_read != 0 {
            host::canvas_set_full_screen_mode(r, canvas, 1)?;
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
        return Ok(());
    }
    let record_store_get_record_value = outcome.unwrap().r();
    application = record_store_get_record_value;
    if record_store_get_record_value == 0 {
        outcome = (|| -> JResult {
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::STATE_W, V::I(1))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult {
            r.set(display, field::canvas::STATE_AX, V::I(0))?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
    } else {
        let object_value_3 = r.alloc("java/io/ByteArrayInputStream", Native::Empty);
        outcome = (|| -> JResult {
            host::byte_array_input_stream_initialize(r, object_value_3, application)?;
            Ok(V::I(0))
        })();
        (carry_0_r,) = (object_value_3,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        application = carry_0_r;
        let object_value_4 = r.alloc("java/io/DataInputStream", Native::Empty);
        outcome = (|| -> JResult {
            host::data_input_stream_initialize(r, object_value_4, application)?;
            Ok(V::I(0))
        })();
        (carry_0_r,) = (object_value_4,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        object_3 = carry_0_r;
        outcome = (|| -> JResult {
            host::byte_array_input_stream_reset(r, application)?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            if !(r.is_instance(carry_0_r, "java/lang/Exception")) {
                return Err(outcome.unwrap_err());
            }
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::MUSIC_VOLUME,
                V::I(data_input_stream_read_int_value),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_2 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::EFFECT_VOLUME,
                V::I(data_input_stream_read_int_value_2),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_3 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::SECONDARY_KEY,
                V::I(data_input_stream_read_int_value_3),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_4 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::UP_KEY,
                V::I(data_input_stream_read_int_value_4),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_5 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::DOWN_KEY,
                V::I(data_input_stream_read_int_value_5),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_6 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::LEFT_KEY,
                V::I(data_input_stream_read_int_value_6),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_7 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::RIGHT_KEY,
                V::I(data_input_stream_read_int_value_7),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_8 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::PRIMARY_KEY,
                V::I(data_input_stream_read_int_value_8),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_9 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::MENU_KEY,
                V::I(data_input_stream_read_int_value_9),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome =
            (|| -> JResult { Ok(V::I(host::data_input_stream_read_boolean(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_boolean_value = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::STATE_AP_UPPER,
                V::I(data_input_stream_read_boolean_value),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome =
            (|| -> JResult { Ok(V::I(host::data_input_stream_read_boolean(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_boolean_value_2 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::USE_PHONE_BUTTONS,
                V::I(data_input_stream_read_boolean_value_2),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome =
            (|| -> JResult { Ok(V::I(host::data_input_stream_read_boolean(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_boolean_value_3 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::STATE_AK,
                V::I(data_input_stream_read_boolean_value_3),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome =
            (|| -> JResult { Ok(V::I(host::data_input_stream_read_boolean(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_boolean_value_4 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::STATE_AO_UPPER,
                V::I(data_input_stream_read_boolean_value_4),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome =
            (|| -> JResult { Ok(V::I(host::data_input_stream_read_boolean(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_boolean_value_5 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::VISUAL_EFFECTS,
                V::I(data_input_stream_read_boolean_value_5),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome =
            (|| -> JResult { Ok(V::I(host::data_input_stream_read_boolean(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_boolean_value_6 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::STATE_W,
                V::I(data_input_stream_read_boolean_value_6),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(host::data_input_stream_read_int(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_int_value_10 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::STATE_AX,
                V::I(data_input_stream_read_int_value_10),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome =
            (|| -> JResult { Ok(V::I(host::data_input_stream_read_boolean(r, object_3)?)) })();
        (carry_0_r,) = (display,);
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let data_input_stream_read_boolean_value_7 = outcome.unwrap().i();
        outcome = (|| -> JResult {
            r.set(
                carry_0_r,
                field::canvas::STATE_X,
                V::I(data_input_stream_read_boolean_value_7),
            )?;
            Ok(V::I(0))
        })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        outcome = (|| -> JResult { Ok(V::I(r.get(display, field::canvas::UP_KEY)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let up_key_value = outcome.unwrap().i();
        if up_key_value == 0 {
            outcome = (|| -> JResult {
                r.set(display, field::canvas::UP_KEY, V::I(50))?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                r.set(display, field::canvas::UP_KEY, V::I(50))?;
                r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
                r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
                r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
                r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
                r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
                r.set(display, field::canvas::MENU_KEY, V::I(57))?;
                r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
                r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
                r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
                r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
                r.set(display, field::canvas::STATE_AK, V::I(1))?;
                r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
                r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
                let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
                if canvas_state_ap_upper_read != 0 {
                    host::canvas_set_full_screen_mode(r, canvas, 1)?;
                }
                r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
                return Ok(());
            }
        }
        outcome = (|| -> JResult { Ok(V::I(r.get(display, field::canvas::DOWN_KEY)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let down_key_value = outcome.unwrap().i();
        if down_key_value == 0 {
            outcome = (|| -> JResult {
                r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                r.set(display, field::canvas::UP_KEY, V::I(50))?;
                r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
                r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
                r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
                r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
                r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
                r.set(display, field::canvas::MENU_KEY, V::I(57))?;
                r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
                r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
                r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
                r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
                r.set(display, field::canvas::STATE_AK, V::I(1))?;
                r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
                r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
                let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
                if canvas_state_ap_upper_read != 0 {
                    host::canvas_set_full_screen_mode(r, canvas, 1)?;
                }
                r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
                return Ok(());
            }
        }
        outcome = (|| -> JResult { Ok(V::I(r.get(display, field::canvas::LEFT_KEY)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let left_key_value = outcome.unwrap().i();
        if left_key_value == 0 {
            outcome = (|| -> JResult {
                r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                r.set(display, field::canvas::UP_KEY, V::I(50))?;
                r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
                r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
                r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
                r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
                r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
                r.set(display, field::canvas::MENU_KEY, V::I(57))?;
                r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
                r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
                r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
                r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
                r.set(display, field::canvas::STATE_AK, V::I(1))?;
                r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
                r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
                let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
                if canvas_state_ap_upper_read != 0 {
                    host::canvas_set_full_screen_mode(r, canvas, 1)?;
                }
                r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
                return Ok(());
            }
        }
        outcome = (|| -> JResult { Ok(V::I(r.get(display, field::canvas::RIGHT_KEY)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let right_key_value = outcome.unwrap().i();
        if right_key_value == 0 {
            outcome = (|| -> JResult {
                r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                r.set(display, field::canvas::UP_KEY, V::I(50))?;
                r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
                r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
                r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
                r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
                r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
                r.set(display, field::canvas::MENU_KEY, V::I(57))?;
                r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
                r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
                r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
                r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
                r.set(display, field::canvas::STATE_AK, V::I(1))?;
                r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
                r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
                let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
                if canvas_state_ap_upper_read != 0 {
                    host::canvas_set_full_screen_mode(r, canvas, 1)?;
                }
                r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
                return Ok(());
            }
        }
        outcome = (|| -> JResult { Ok(V::I(r.get(display, field::canvas::PRIMARY_KEY)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let primary_key_value = outcome.unwrap().i();
        if primary_key_value == 0 {
            outcome = (|| -> JResult {
                r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                r.set(display, field::canvas::UP_KEY, V::I(50))?;
                r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
                r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
                r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
                r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
                r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
                r.set(display, field::canvas::MENU_KEY, V::I(57))?;
                r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
                r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
                r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
                r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
                r.set(display, field::canvas::STATE_AK, V::I(1))?;
                r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
                r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
                let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
                if canvas_state_ap_upper_read != 0 {
                    host::canvas_set_full_screen_mode(r, canvas, 1)?;
                }
                r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
                return Ok(());
            }
        }
        outcome = (|| -> JResult { Ok(V::I(r.get(display, field::canvas::MENU_KEY)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let menu_key_value = outcome.unwrap().i();
        if menu_key_value == 0 {
            outcome = (|| -> JResult {
                r.set(display, field::canvas::MENU_KEY, V::I(57))?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                r.set(display, field::canvas::UP_KEY, V::I(50))?;
                r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
                r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
                r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
                r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
                r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
                r.set(display, field::canvas::MENU_KEY, V::I(57))?;
                r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
                r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
                r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
                r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
                r.set(display, field::canvas::STATE_AK, V::I(1))?;
                r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
                r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
                let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
                if canvas_state_ap_upper_read != 0 {
                    host::canvas_set_full_screen_mode(r, canvas, 1)?;
                }
                r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
                return Ok(());
            }
        }
        outcome = (|| -> JResult { Ok(V::I(r.get(display, field::canvas::SECONDARY_KEY)?.i())) })();
        if outcome.is_err() {
            carry_0_r = outcome.unwrap_err();
            r.set(display, field::canvas::UP_KEY, V::I(50))?;
            r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
            r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
            r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
            r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
            r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
            r.set(display, field::canvas::MENU_KEY, V::I(57))?;
            r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
            r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
            r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
            r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
            r.set(display, field::canvas::STATE_AK, V::I(1))?;
            r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
            r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
            let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
            if canvas_state_ap_upper_read != 0 {
                host::canvas_set_full_screen_mode(r, canvas, 1)?;
            }
            r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
            return Ok(());
        }
        let secondary_key_value = outcome.unwrap().i();
        if secondary_key_value == 0 {
            outcome = (|| -> JResult {
                r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
                Ok(V::I(0))
            })();
            if outcome.is_err() {
                carry_0_r = outcome.unwrap_err();
                r.set(display, field::canvas::UP_KEY, V::I(50))?;
                r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
                r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
                r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
                r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
                r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
                r.set(display, field::canvas::MENU_KEY, V::I(57))?;
                r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
                r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
                r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
                r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
                r.set(display, field::canvas::STATE_AK, V::I(1))?;
                r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
                r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
                let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
                if canvas_state_ap_upper_read != 0 {
                    host::canvas_set_full_screen_mode(r, canvas, 1)?;
                }
                r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
                return Ok(());
            }
        }
    }
    outcome = (|| -> JResult { Ok(V::R(r.get(display, field::canvas::RECORD_STORE)?.r())) })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        r.set(display, field::canvas::UP_KEY, V::I(50))?;
        r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
        r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
        r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
        r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
        r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
        r.set(display, field::canvas::MENU_KEY, V::I(57))?;
        r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
        r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
        r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
        r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
        r.set(display, field::canvas::STATE_AK, V::I(1))?;
        r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
        r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
        let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
        if canvas_state_ap_upper_read != 0 {
            host::canvas_set_full_screen_mode(r, canvas, 1)?;
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
        return Ok(());
    }
    let record_store_value_2 = outcome.unwrap().r();
    outcome = (|| -> JResult {
        host::record_store_close_record_store(r, record_store_value_2)?;
        Ok(V::I(0))
    })();
    if outcome.is_err() {
        carry_0_r = outcome.unwrap_err();
        r.set(display, field::canvas::UP_KEY, V::I(50))?;
        r.set(display, field::canvas::DOWN_KEY, V::I(56))?;
        r.set(display, field::canvas::LEFT_KEY, V::I(52))?;
        r.set(display, field::canvas::RIGHT_KEY, V::I(54))?;
        r.set(display, field::canvas::PRIMARY_KEY, V::I(53))?;
        r.set(display, field::canvas::SECONDARY_KEY, V::I(55))?;
        r.set(display, field::canvas::MENU_KEY, V::I(57))?;
        r.set(display, field::canvas::MUSIC_VOLUME, V::I(50))?;
        r.set(display, field::canvas::EFFECT_VOLUME, V::I(50))?;
        r.set(display, field::canvas::STATE_AP_UPPER, V::I(0))?;
        r.set(display, field::canvas::USE_PHONE_BUTTONS, V::I(1))?;
        r.set(display, field::canvas::STATE_AK, V::I(1))?;
        r.set(display, field::canvas::STATE_AO_UPPER, V::I(1))?;
        r.set(display, field::canvas::VISUAL_EFFECTS, V::I(1))?;
        let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
        if canvas_state_ap_upper_read != 0 {
            host::canvas_set_full_screen_mode(r, canvas, 1)?;
        }
        r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
        return Ok(());
    }

    let canvas_state_ap_upper_read = r.get(canvas, field::canvas::STATE_AP_UPPER)?.i();
    if canvas_state_ap_upper_read != 0 {
        host::canvas_set_full_screen_mode(r, canvas, 1)?;
    }
    r.set(canvas, field::canvas::KEY_CONFIGURATION_STEP, V::I(0))?;
    return Ok(());
}
