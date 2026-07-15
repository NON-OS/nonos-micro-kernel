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

use crate::constants::{CC_SHORT_PACKET, CC_SUCCESS, TRB_TYPE_TRANSFER_EVENT};
use crate::error::{XhciError, XhciResult};
use crate::regs::runtime::erdp_program;
use crate::rings::event::EventRing;

const TRANSFER_TIMEOUT_MS: u64 = 1_000;
// Check the wall-time deadline only every this many idle spins so the transfer
// poll stays a tight loop and does not make a syscall per iteration.
const DEADLINE_CHECK_SPINS: u32 = 1024;

pub fn wait_transfer_completion(
    intr_base: u64,
    issued_phys: u64,
    evt_ring: &mut EventRing,
) -> XhciResult<()> {
    let deadline = Deadline::after_ms(TRANSFER_TIMEOUT_MS);
    let mut spins = 0u32;
    loop {
        if !evt_ring.has_event() {
            spins = spins.wrapping_add(1);
            if spins.is_multiple_of(DEADLINE_CHECK_SPINS) && deadline.expired() {
                return Err(XhciError::TransferCompletionTimeout);
            }
            core::hint::spin_loop();
            continue;
        }
        let event = evt_ring.current_trb();
        evt_ring.advance();
        erdp_program(intr_base, evt_ring.current_dequeue_phys(), true, 0);
        if event.get_type() != TRB_TYPE_TRANSFER_EVENT {
            continue;
        }
        if event.get_pointer() & !0xF != issued_phys & !0xF {
            continue;
        }
        return complete(event.completion_code());
    }
}
fn complete(code: u8) -> XhciResult<()> {
    match code {
        CC_SUCCESS | CC_SHORT_PACKET => Ok(()),
        other => Err(XhciError::TransferCompletionFailed(other)),
    }
}
