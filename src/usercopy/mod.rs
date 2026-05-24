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

mod bytes;
mod copy;
mod direct;
mod error;
mod policy;
mod string;
mod validate;
mod value;
mod walk;

#[cfg(test)]
pub mod tests;

pub use bytes::*;
pub use copy::*;
pub use error::*;
pub use string::*;
pub use validate::*;
pub use value::*;

/// Resolve a user virtual address to its physical frame base via the active
/// (capsule) page table. `None` if the page is unmapped or not user-accessible.
/// Unlike `paging::translate_address` (which walks the kernel's table), this
/// resolves addresses in the address space the capsule actually runs in, so it
/// works for capsule DMA / user buffers handed to the kernel by reference.
pub fn user_page_phys(va: u64) -> Option<u64> {
    walk::translate_read(va).ok().map(|leaf| leaf.phys_base)
}
