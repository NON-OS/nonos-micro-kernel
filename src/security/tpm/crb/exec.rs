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

use core::sync::atomic::{fence, Ordering};

use super::buffer::command_buffer;
use super::regs::{TPM_CRB_CTRL_REQ, TPM_CRB_CTRL_START, TPM_CRB_REQ_COMMAND_READY, TPM_CRB_START_GO};
use super::wait::{wait_complete, wait_ready};
use super::response::read_response;
use super::window::write32;
use crate::security::tpm::error::TpmError;

/// Run one command and copy the response back.
///
/// # Safety
/// Executing a TPM command changes the part's state. The caller owns what the
/// command means; this guarantees only the transport.
pub(super) unsafe fn execute(cmd: &[u8], out: &mut [u8]) -> Result<usize, TpmError> {
    let buffer = command_buffer()?;
    if cmd.len() > buffer.size {
        return Err(TpmError::InvalidResponse);
    }

    // SAFETY: eK@nonos.systems - asking the part to become ready is the
    // documented first step and cancels nothing already in flight.
    unsafe { write32(TPM_CRB_CTRL_REQ, TPM_CRB_REQ_COMMAND_READY)? };
    wait_ready()?;

    // Byte at a time, volatile, deliberately not `copy_nonoverlapping`: a
    // memcpy is free to widen into SSE, `movups` against device memory is not
    // a defined access, and a hypervisor decoding the trap aborts the guest.
    //
    // SAFETY: eK@nonos.systems - `buffer.virt` is the directmap view of the
    // buffer the part published, its length was checked above, and this module
    // is the only writer between readiness and the doorbell.
    for (offset, byte) in cmd.iter().enumerate() {
        unsafe {
            core::ptr::write_volatile((buffer.virt as *mut u8).add(offset), *byte);
        }
    }

    // The command must be entirely visible before the doorbell: the part may
    // begin reading the instant start is set.
    fence(Ordering::SeqCst);
    // SAFETY: eK@nonos.systems - the command is fully written above.
    unsafe { write32(TPM_CRB_CTRL_START, TPM_CRB_START_GO)? };

    wait_complete()?;
    read_response(out)
}
