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
use crate::security::anti_rollback::nvram::write_to_nvram;
use crate::security::anti_rollback::types::RollbackError;

impl AntiRollbackState {
    pub fn update_kernel_version(
        &mut self,
        kernel_version: u64,
        timestamp: u64,
    ) -> Result<(), RollbackError> {
        if !self.initialized && !self.tpm_available {
            return Err(RollbackError::TpmNotAvailable);
        }
        self.check_kernel_version(kernel_version)?;
        if kernel_version > self.state.kernel_version {
            self.state.kernel_version = kernel_version;
        }
        if kernel_version > self.state.minimum_kernel {
            self.state.minimum_kernel = kernel_version;
        }
        self.state.last_boot_timestamp = timestamp;
        self.state.boot_count += 1;
        if self.tpm_available {
            write_to_nvram(&self.state)?;
        }
        Ok(())
    }

    pub fn set_minimum_kernel_version(&mut self, version: u64) -> Result<(), RollbackError> {
        if version > self.state.minimum_kernel {
            self.state.minimum_kernel = version;
            if self.tpm_available {
                write_to_nvram(&self.state)?;
            }
        }
        Ok(())
    }
}
