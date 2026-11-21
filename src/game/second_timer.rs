//! The one-second simulation callback.
use super::{arithmetic, fields as field, host};
use crate::runtime::{JResult, Native, Runtime, V};

/// Initialize. Original reference: `a.<init>(LZelda;)V`.
pub(super) fn initialize(r: &mut Runtime, timer: u32, application: u32) -> Result<(), u32> {
    host::timer_task_initialize(r, timer)?;
    r.set(timer, field::second_timer::APPLICATION, V::R(application))?;
    return Ok(());
}

/// Tick. Original reference: `a.run()V`.
pub(super) fn tick(r: &mut Runtime, timer: u32) -> Result<(), u32> {
    let second_timer_application = r.get(timer, field::second_timer::APPLICATION)?.r();
    let canvas_value = super::lifecycle::canvas(r, second_timer_application)?;
    r.set(canvas_value, field::canvas::SECOND_ELAPSED, V::I(1))?;
    let second_timer_application_2 = r.get(timer, field::second_timer::APPLICATION)?.r();
    let canvas_value_2 = super::lifecycle::canvas(r, second_timer_application_2)?;
    r.set(canvas_value_2, field::canvas::REDRAW, V::I(1))?;
    return Ok(());
}
