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

// The real version/error types (pure: plain u64 fields and an error enum).
// `new_without_default` is a style choice in the real source, not restyled here.
#[allow(clippy::new_without_default)]
#[path = "../../../../src/security/anti_rollback/types/mod.rs"]
pub mod types;

// The real check/update decision logic, over the real AntiRollbackState.
pub mod state;

// TPM/NVRAM write shim: the real path persists to hardware NV storage; the
// proofs exercise the decision logic, so the write is a no-op here.
pub mod nvram;

pub use state::AntiRollbackState;
