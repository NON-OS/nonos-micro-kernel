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

use crate::security::anti_rollback::types::{RollbackError, VersionState};

// The real implementation persists the state to a TPM NV index. The proofs
// target the decision logic that runs before this point, so the write is a
// no-op that always succeeds; it never masks a check.
pub fn write_to_nvram(_state: &VersionState) -> Result<(), RollbackError> {
    Ok(())
}
