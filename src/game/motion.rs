//! Shared motion state.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `d.<init>()V`.
pub(super) fn initialize(r: &mut Runtime, motion: u32) -> Result<(), u32> {
    host::object_initialize(r, motion)?;
    return Ok(());
}
