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

mod cleanup;
mod error;
mod init;
mod query;
mod security;

pub use cleanup::wipe_sensitive_handoff_data;
pub use error::{FbGeometryReason, HandoffError};
pub use init::init_handoff;
pub use query::{get_handoff, is_initialized, total_memory};
// Re-exported only for the gated boot handoff tests; the production
// caller in `init` imports it directly from the `security` submodule.
#[cfg(any(test, feature = "nonos-selftest"))]
pub(crate) use security::validate_security;
