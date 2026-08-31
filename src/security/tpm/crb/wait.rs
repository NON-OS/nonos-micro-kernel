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

use super::regs::{
    COMPLETE_SPINS, READY_SPINS, TPM_CRB_CTRL_START, TPM_CRB_CTRL_STS, TPM_CRB_START_GO,
    TPM_CRB_STS_TPM_IDLE,
};
use super::window::read32;
use crate::security::tpm::error::TpmError;

pub(super) fn wait_ready() -> Result<(), TpmError> {
    for _ in 0..READY_SPINS {
        if read32(TPM_CRB_CTRL_STS)? & TPM_CRB_STS_TPM_IDLE == 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(TpmError::Timeout)
}

/// The part clears the start bit when the response is ready. Waiting on that,
/// rather than on a status flag, is what the CRB interface defines.
pub(super) fn wait_complete() -> Result<(), TpmError> {
    for _ in 0..COMPLETE_SPINS {
        if read32(TPM_CRB_CTRL_START)? & TPM_CRB_START_GO == 0 {
            fence(Ordering::SeqCst);
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(TpmError::Timeout)
}
