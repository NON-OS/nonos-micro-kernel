// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::core::TpmState;
use crate::hardware::tpm::constants::{
    TPM_ACCESS, TPM_ACCESS_ACTIVE, TPM_ACCESS_REQUEST, TPM_LOC_CTRL, TPM_LOC_CTRL_RELINQUISH,
    TPM_LOC_CTRL_REQUEST, TPM_LOC_STS, TPM_LOC_STS_GRANTED, TPM_STS,
};
use crate::hardware::tpm::types::TpmError;

/// How long to wait for a locality grant. Both files answer in microseconds
/// when they answer at all, so this only needs to outlast a slow firmware TPM
/// coming out of idle.
const LOCALITY_SPINS: u32 = 1000;

impl TpmState {
    /// Take locality 0 so commands can be submitted.
    ///
    /// The two register files ask differently. FIFO writes `requestUse` into
    /// `TPM_ACCESS` at 0x00 and waits for `activeLocality` in the same
    /// register. CRB writes `requestAccess` into `TPM_LOC_CTRL` at 0x08 and
    /// reads `granted` from `TPM_LOC_STS` at 0x0C. On CRB, offset 0x00 is the
    /// read-only `TPM_LOC_STATE`, so the FIFO sequence wrote to a register
    /// that cannot grant anything and then spun waiting on a bit that never
    /// sets.
    pub fn request_locality(&self) -> Result<(), TpmError> {
        if !self.initialized {
            return Err(TpmError::NotPresent);
        }

        if self.is_crb {
            self.write_reg8(TPM_LOC_CTRL, TPM_LOC_CTRL_REQUEST);
            for _ in 0..LOCALITY_SPINS {
                if (self.read_reg8(TPM_LOC_STS) & TPM_LOC_STS_GRANTED) != 0 {
                    return Ok(());
                }
                core::hint::spin_loop();
            }
            return Err(TpmError::Timeout);
        }

        self.write_reg8(TPM_ACCESS, TPM_ACCESS_REQUEST);
        for _ in 0..LOCALITY_SPINS {
            if (self.read_reg8(TPM_ACCESS) & TPM_ACCESS_ACTIVE) != 0 {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(TpmError::Timeout)
    }

    /// Give locality back. Holding it locks out every other agent on the
    /// platform, so this runs whether the command succeeded or not.
    pub fn release_locality(&self) {
        if !self.initialized {
            return;
        }
        if self.is_crb {
            self.write_reg8(TPM_LOC_CTRL, TPM_LOC_CTRL_RELINQUISH);
        } else {
            self.write_reg8(TPM_ACCESS, TPM_ACCESS_ACTIVE);
        }
    }

    pub(crate) fn wait_for_status(&self, mask: u8, expected: u8) -> Result<(), TpmError> {
        for _ in 0..10000 {
            if (self.read_reg8(TPM_STS) & mask) == expected {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        Err(TpmError::Timeout)
    }
}
