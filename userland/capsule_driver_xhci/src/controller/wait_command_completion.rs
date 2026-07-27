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
use nonos_libc::Deadline;

use crate::constants::{CC_SUCCESS, TRB_TYPE_CMD_COMPLETION_EVENT};
use crate::controller::park::{park_step, SPIN_BUDGET};
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::EventRing;

const COMPLETION_TIMEOUT_MS: u64 = 1_000;

#[derive(Clone, Copy)]
pub struct CommandCompletion {
    pub slot_id: u8,
}
pub fn wait_command_completion(
    intr_base: u64,
    issued_phys: u64,
    evt_ring: &mut EventRing,
) -> XhciResult<CommandCompletion> {
    let deadline = Deadline::after_ms(COMPLETION_TIMEOUT_MS);
    let mut spins = 0u32;
    loop {
        spins = spins.saturating_add(1);
        if spins > SPIN_BUDGET && deadline.expired() {
            return Err(XhciError::CommandCompletionTimeout);
        }
        if !evt_ring.has_event() {
            park_step(spins);
            continue;
        }
        let event = evt_ring.current_trb();
        evt_ring.advance();
        erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
        if event.get_type() != TRB_TYPE_CMD_COMPLETION_EVENT {
            continue;
        }
        if event.get_pointer() & !0xF != issued_phys & !0xF {
            continue;
        }
        let cc = event.completion_code();
        if cc != CC_SUCCESS {
            return Err(XhciError::CommandCompletionFailed(cc));
        }
        return Ok(CommandCompletion { slot_id: event.slot_id() });
    }
}
