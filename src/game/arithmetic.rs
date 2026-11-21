//! Integer arithmetic preserves the original overflow and division behavior.
use crate::runtime::Runtime;
pub(super) fn divide_int(r: &mut Runtime, numerator: i32, denominator: i32) -> Result<i32, u32> {
    if denominator == 0 {
        return Err(r.exception("java/lang/ArithmeticException", "division by zero"));
    }
    Ok(numerator.wrapping_div(denominator))
}
pub(super) fn remainder_int(r: &mut Runtime, numerator: i32, denominator: i32) -> Result<i32, u32> {
    if denominator == 0 {
        return Err(r.exception("java/lang/ArithmeticException", "division by zero"));
    }
    Ok(numerator.wrapping_rem(denominator))
}
pub(super) fn divide_long(r: &mut Runtime, numerator: i64, denominator: i64) -> Result<i64, u32> {
    if denominator == 0 {
        return Err(r.exception("java/lang/ArithmeticException", "division by zero"));
    }
    Ok(numerator.wrapping_div(denominator))
}
pub(super) fn remainder_long(
    r: &mut Runtime,
    numerator: i64,
    denominator: i64,
) -> Result<i64, u32> {
    if denominator == 0 {
        return Err(r.exception("java/lang/ArithmeticException", "division by zero"));
    }
    Ok(numerator.wrapping_rem(denominator))
}
