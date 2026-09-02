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

pub mod constants;
pub mod error;
pub mod mmu;
mod types;

pub use constants::*;
pub use error::{MmuError, MmuResult};
pub use mmu::{
    current_cr3, get_mmu, init_mmu, invalidate_page, map_kernel_memory,
    mmu_is_initialized as is_initialized, protection_flags, report_protection, MMU,
};
pub use types::{PagePermissions, PageTableEntry, ProtectionFlags};
