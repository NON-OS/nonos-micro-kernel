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
use crate::security::anti_rollback::nvram::read_from_nvram;
use crate::security::anti_rollback::types::{RollbackError, VersionState};
use crate::security::SecurityPolicy;

impl AntiRollbackState {
    pub fn init(&mut self, tpm_available: bool) -> Result<(), RollbackError> {
        self.tpm_available = tpm_available;
        if tpm_available {
            match read_from_nvram() {
                Ok(state) => self.state = state,
                Err(RollbackError::NvramReadFailed) => {
                    if SecurityPolicy::from_build() == SecurityPolicy::Development {
                        self.state = VersionState::new();
                    } else {
                        return Err(RollbackError::NvramReadFailed);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        self.initialized = true;
        Ok(())
    }
}
