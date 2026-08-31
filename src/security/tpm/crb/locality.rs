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

use super::regs::{
    READY_SPINS, TPM_INTERFACE_ID, TPM_INTF_TYPE_CRB, TPM_INTF_TYPE_MASK, TPM_LOC_CTRL,
    TPM_LOC_CTRL_RELINQUISH, TPM_LOC_CTRL_REQUEST, TPM_LOC_STS, TPM_LOC_STS_GRANTED,
};
use super::window::{init_window, read32, write32};
use crate::security::tpm::error::TpmError;

/// Map the window and confirm a CRB part is behind it.
///
/// Refusing a FIFO part is not a limitation to work around: the two register
/// files overlap, so driving a FIFO part through CRB offsets writes command
/// bytes into control registers and produces a garbled response rather than an
/// error.
pub(super) fn probe() -> Result<(), TpmError> {
    init_window()?;
    let intf = read32(TPM_INTERFACE_ID)?;
    if intf == u32::MAX {
        return Err(TpmError::NotPresent);
    }
    if intf & TPM_INTF_TYPE_MASK != TPM_INTF_TYPE_CRB {
        return Err(TpmError::NotPresent);
    }
    Ok(())
}

/// Take locality 0. Every command runs inside a granted locality; issuing one
/// without it is answered by the part, not by this driver.
pub(super) fn acquire() -> Result<(), TpmError> {
    if read32(TPM_LOC_STS)? & TPM_LOC_STS_GRANTED != 0 {
        return Ok(());
    }
    // SAFETY: eK@nonos.systems - a locality request changes no key state and
    // is the documented way to begin using the part.
    unsafe { write32(TPM_LOC_CTRL, TPM_LOC_CTRL_REQUEST)? };
    for _ in 0..READY_SPINS {
        if read32(TPM_LOC_STS)? & TPM_LOC_STS_GRANTED != 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(TpmError::Timeout)
}

/// Give locality back. Best effort: a part that will not release it is not a
/// reason to fail a command that already succeeded.
pub(super) fn release() {
    // SAFETY: eK@nonos.systems - relinquishing only narrows what this driver
    // may do next.
    let _ = unsafe { write32(TPM_LOC_CTRL, TPM_LOC_CTRL_RELINQUISH) };
}
