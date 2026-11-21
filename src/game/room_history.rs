//! Shared room history state.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `i.<init>()V`.
pub(super) fn initialize(r: &mut Runtime, room_history: u32) -> Result<(), u32> {
    host::object_initialize(r, room_history)?;
    return Ok(());
}
