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

mod api;
mod nvram;
mod state;
mod types;
mod util;

pub use api::{
    check_kernel_version, get_version_state, init_anti_rollback, update_kernel_version,
    ANTI_ROLLBACK,
};
pub use state::AntiRollbackState;
pub use types::{
    RollbackError, VersionState, DS_ROLLBACK, NVRAM_BOOTLOADER_INDEX, NVRAM_VERSION_INDEX,
};
