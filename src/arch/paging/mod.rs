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

//! The paging hardware boundary. Everything the shared paging manager needs
//! from the CPU lives here as a small, arch-neutral surface: the page-table
//! root register, TLB invalidation, and the write-protect override. Each call
//! dispatches to the architecture that owns the instruction sequence, so the
//! manager above stays free of `asm!` and of any one CPU's register names.

pub mod descriptor;
mod enable_tagged;
mod root;
mod tagged_invalidate;
mod tagged_tlb;
mod tlb;
mod write_protect;

pub use enable_tagged::enable_tagged_invalidation;
pub use root::{read_root, write_root};
pub use tagged_invalidate::invalidate_tagged;
pub use tagged_tlb::supports_tagged_invalidation;
pub use tlb::{invalidate_all, invalidate_page};
pub use write_protect::{disable_write_protection, enable_write_protection};
