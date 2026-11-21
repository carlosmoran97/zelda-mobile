//! Shared portal state.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `e.<init>()V`.
pub(super) fn initialize(r: &mut Runtime, portal: u32) -> Result<(), u32> {
    host::object_initialize(r, portal)?;
    return Ok(());
}
