//! Shared trail point state.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `g.<init>()V`.
pub(super) fn initialize(r: &mut Runtime, trail_point: u32) -> Result<(), u32> {
    host::object_initialize(r, trail_point)?;
    return Ok(());
}
