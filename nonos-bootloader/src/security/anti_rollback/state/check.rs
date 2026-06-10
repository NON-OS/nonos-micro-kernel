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

use super::types::AntiRollbackState;
use crate::security::anti_rollback::types::RollbackError;

impl AntiRollbackState {
    pub fn check_kernel_version(&self, kernel_version: u64) -> Result<(), RollbackError> {
        if !self.initialized && !self.tpm_available {
            return Err(RollbackError::TpmNotAvailable);
        }
        if kernel_version == 0 {
            return Err(RollbackError::InvalidVersion);
        }
        if self.initialized && kernel_version < self.state.minimum_kernel {
            return Err(RollbackError::KernelVersionTooOld {
                kernel: kernel_version,
                minimum: self.state.minimum_kernel,
            });
        }
        Ok(())
    }

    pub fn check_bootloader_version(&self, bootloader_version: u64) -> Result<(), RollbackError> {
        if !self.initialized && !self.tpm_available {
            return Err(RollbackError::TpmNotAvailable);
        }
        if bootloader_version == 0 {
            return Err(RollbackError::InvalidVersion);
        }
        if self.initialized && bootloader_version < self.state.minimum_bootloader {
            return Err(RollbackError::BootloaderVersionTooOld {
                current: bootloader_version,
                minimum: self.state.minimum_bootloader,
            });
        }
        Ok(())
    }
}
