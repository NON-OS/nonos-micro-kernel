// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::intent::SwitchIntent;
use super::lease::SwitchLease;
use super::outcome::{SwitchError, SwitchOutcome};
use crate::process::core::api::current_pid;
use crate::process::scheduler::preemption::{perform_yield_inline, preempt_current_process};

/// Whether the caller can be switched away from right now.
pub(super) fn interrupts_enabled() -> bool {
    crate::arch::cpu::interrupts_enabled()
}

/// Give up the CPU the way `intent` asks for.
///
/// This used to live behind a per architecture backend, but only the mask
/// check underneath it was ever architecture specific, and the arch layer
/// already answers that one. What is left decides on the current task and
/// hands off to preemption, which is the same work on any CPU.
pub(super) fn perform(
    _lease: SwitchLease,
    intent: SwitchIntent,
) -> Result<SwitchOutcome, SwitchError> {
    if current_pid().is_none() {
        return Err(SwitchError::NoCurrentTask);
    }
    match intent {
        SwitchIntent::Preempt => preempt_current_process(),
        SwitchIntent::Yield => perform_yield_inline(),
    }
    Ok(SwitchOutcome::Returned)
}
