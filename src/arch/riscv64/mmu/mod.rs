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

pub mod attributes;
mod boot_map;
mod branch;
mod constants;
mod map;
mod mode;
mod satp;
mod state;
pub mod sv39;
pub mod sv48;
pub mod table;
mod tlb;
mod unmap;

pub use attributes::{PageAttributes, PteFlags};
pub use boot_map::init_mmu;
pub use constants::{PAGE_SHIFT, PAGE_SIZE};
pub use map::map_page;
pub use mode::MmuMode;
pub use satp::{current_asid, current_ppn, make_satp, mmu_mode, read_satp, write_satp};
pub use sv39::{Sv39, VA_BITS_39};
pub use sv48::{Sv48, VA_BITS_48};
pub use table::PageTable;
pub use tlb::{flush_tlb_all, flush_tlb_asid, flush_tlb_page};
pub use unmap::unmap_page;
