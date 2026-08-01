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
mod control;
pub mod granule;
mod map;
mod state;
pub mod table;
mod tlb;
pub mod translation;
mod ttbr;
mod unmap;

pub use attributes::{MemoryType, PageAttributes};
pub use boot_map::{init_mmu, KERNEL_SPACE_START};
pub use granule::{Granule, GRANULE_16K, GRANULE_4K, GRANULE_64K};
pub use map::map_page;
pub use table::PageTable;
pub use tlb::{flush_tlb_all, flush_tlb_asid, flush_tlb_page};
pub use translation::{phys_to_virt, virt_to_phys};
pub use ttbr::{read_ttbr0, read_ttbr1, set_ttbr0, set_ttbr1};
pub use unmap::unmap_page;
