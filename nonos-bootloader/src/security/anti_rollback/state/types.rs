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

use crate::security::anti_rollback::types::VersionState;

pub struct AntiRollbackState {
    pub(crate) state: VersionState,
    pub(crate) initialized: bool,
    pub(crate) tpm_available: bool,
}

impl AntiRollbackState {
    pub const fn new() -> Self {
        Self { state: VersionState::new(), initialized: false, tpm_available: false }
    }

    pub fn get_state(&self) -> &VersionState {
        &self.state
    }
}
