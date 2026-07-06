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

// Real AntiRollbackState plus the real check and update implementations.
#[allow(clippy::new_without_default)]
#[path = "../../../../../src/security/anti_rollback/state/types.rs"]
pub mod types;

#[path = "../../../../../src/security/anti_rollback/state/check.rs"]
mod check;

#[path = "../../../../../src/security/anti_rollback/state/update.rs"]
mod update;

pub use types::AntiRollbackState;
