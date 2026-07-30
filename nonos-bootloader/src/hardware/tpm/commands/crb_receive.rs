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

use super::crb_buffer::response_buffer;
use crate::hardware::tpm::constants::{
    TPM_CRB_CTRL_REQ, TPM_CRB_CTRL_START, TPM_CRB_REQ_GO_IDLE, TPM_CRB_START_GO,
};
use crate::hardware::tpm::state::TpmState;
use crate::hardware::tpm::types::TpmError;

/// Spins to wait for a command to finish. A key creation can take a while on a
/// firmware TPM, which is the slowest thing the bootloader asks for.
const COMPLETION_SPINS: u32 = 5_000_000;

/// Every TPM 2.0 response starts with a tag, a size and a return code, so a
/// reply shorter than this is not one.
const HEADER_LEN: usize = 10;

/// Collect the response a CRB part left in its buffer.
///
/// Completion is signalled by the part clearing `start`, not by a status bit in
/// a FIFO register. The response length comes from the header's own size field,
/// which is bounded by the advertised buffer before anything is copied.
pub(crate) fn crb_receive(state: &TpmState, buf: &mut [u8]) -> Result<usize, TpmError> {
    let buffer = response_buffer(state)?;

    let mut done = false;
    for _ in 0..COMPLETION_SPINS {
        if state.read_reg32(TPM_CRB_CTRL_START) & TPM_CRB_START_GO == 0 {
            done = true;
            break;
        }
        core::hint::spin_loop();
    }
    if !done {
        return Err(TpmError::Timeout);
    }

    // The part wrote the response before clearing start; this keeps the reads
    // below from being hoisted above that observation.
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);

    if buffer.size < HEADER_LEN {
        return Err(TpmError::InvalidResponse);
    }

    // Read the size field a byte at a time for the same reason the send side
    // writes one: a slice read over the device window is free to widen into a
    // vector load, which is not a defined device access and which a hypervisor
    // decoding the trap cannot handle.
    //
    // SAFETY: the address and size come from the part's control registers,
    // bounded by `crb_buffer`, and the bootloader is identity mapped.
    let mut header = [0u8; HEADER_LEN];
    for (offset, slot) in header.iter_mut().enumerate() {
        *slot = unsafe { core::ptr::read_volatile((buffer.addr as *const u8).add(offset)) };
    }
    let declared = u32::from_be_bytes([header[2], header[3], header[4], header[5]]) as usize;

    // A part that declares more than its own buffer holds, or less than a
    // header, is not to be trusted with the length of a copy.
    if declared < HEADER_LEN || declared > buffer.size {
        return Err(TpmError::InvalidResponse);
    }

    let take = declared.min(buf.len());
    // SAFETY: `take` is bounded by both the advertised buffer and the caller's
    // slice, so neither side can be overrun. Volatile and byte-wise for the
    // same reason as the header above.
    for (offset, slot) in buf.iter_mut().enumerate().take(take) {
        *slot = unsafe { core::ptr::read_volatile((buffer.addr as *const u8).add(offset)) };
    }

    // Let the part idle again so the next command starts from a known state.
    state.write_reg32(TPM_CRB_CTRL_REQ, TPM_CRB_REQ_GO_IDLE);
    Ok(take)
}
