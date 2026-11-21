//! Shared drawable state.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `f.<init>()V`.
pub(super) fn initialize(r: &mut Runtime, drawable: u32) -> Result<(), u32> {
    host::object_initialize(r, drawable)?;
    return Ok(());
}
