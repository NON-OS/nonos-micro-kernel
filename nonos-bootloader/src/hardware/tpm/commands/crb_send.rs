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

use super::crb_buffer::command_buffer;
use crate::hardware::tpm::constants::{
    TPM_CRB_CTRL_REQ, TPM_CRB_CTRL_START, TPM_CRB_CTRL_STS, TPM_CRB_REQ_COMMAND_READY,
    TPM_CRB_START_GO, TPM_CRB_STS_TPM_IDLE,
};
use crate::hardware::tpm::state::TpmState;
use crate::hardware::tpm::types::TpmError;

/// Spins to wait for the part to leave idle. Generous: a firmware TPM waking
/// from a low power state is slower than a discrete one on the LPC bus.
const READY_SPINS: u32 = 200_000;

/// Submit a command through the CRB buffer.
///
/// Nothing like the FIFO sequence. There is no byte port: the part publishes a
/// buffer in memory, the driver writes the whole command there, then rings
/// `TPM_CRB_CTRL_START`. Writing command bytes to `TPM_DATA_FIFO` on a CRB part
/// lands on `TPM_CRB_CTRL_CANCEL` and neighbouring control registers, which is
/// why the FIFO path came back with an invalid response rather than an error.
pub(crate) fn crb_send(state: &TpmState, cmd: &[u8]) -> Result<(), TpmError> {
    let buffer = command_buffer(state)?;
    if cmd.len() > buffer.size {
        return Err(TpmError::InvalidResponse);
    }

    // Ask the part to become ready, then wait for it to drop out of idle.
    state.write_reg32(TPM_CRB_CTRL_REQ, TPM_CRB_REQ_COMMAND_READY);
    let mut ready = false;
    for _ in 0..READY_SPINS {
        if state.read_reg32(TPM_CRB_CTRL_STS) & TPM_CRB_STS_TPM_IDLE == 0 {
            ready = true;
            break;
        }
        core::hint::spin_loop();
    }
    if !ready {
        return Err(TpmError::Timeout);
    }

    // Byte at a time, volatile, and deliberately not `copy_nonoverlapping`.
    // The command buffer is inside the device's window, and a memcpy is free to
    // widen into SSE: `movups` against MMIO is not a defined device access, and
    // a hypervisor decoding the trap sees an instruction it has no rule for and
    // gives up. QEMU aborts the guest outright.
    //
    // SAFETY: `buffer.addr` and its size come from the part's own control
    // registers, bounded by `crb_buffer`, the bootloader is identity mapped so
    // the physical address is directly writable, and the length was checked
    // against the buffer above.
    for (offset, byte) in cmd.iter().enumerate() {
        unsafe {
            core::ptr::write_volatile((buffer.addr as *mut u8).add(offset), *byte);
        }
    }

    // The command has to be fully visible before the doorbell, because the
    // part may start reading the moment start is set.
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    state.write_reg32(TPM_CRB_CTRL_START, TPM_CRB_START_GO);
    Ok(())
}
