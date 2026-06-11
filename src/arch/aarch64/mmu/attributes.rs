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

mod constructors;
mod descriptor;
mod kind;
mod page;
mod pte;

pub use kind::MemoryType;
pub use page::PageAttributes;
pub use pte::{
    PTE_ADDR_MASK, PTE_AF, PTE_AP_RO_ALL, PTE_AP_RO_EL1, PTE_AP_RW_ALL, PTE_AP_RW_EL1,
    PTE_ATTR_INDX_MASK, PTE_BLOCK, PTE_CONT, PTE_NG, PTE_NS, PTE_PAGE, PTE_PXN, PTE_SH_IS,
    PTE_SH_MASK, PTE_SH_NS, PTE_SH_OS, PTE_TABLE, PTE_UXN, PTE_VALID,
};
