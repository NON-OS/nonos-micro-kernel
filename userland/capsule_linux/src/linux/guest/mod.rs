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

//! A hosted process: what it is, what it has open, and how this capsule
//! reaches into it.

mod fd;
mod fd_dup;
mod fd_empty;
mod fd_kind;
mod fd_make;
mod handle;
mod handle_new;
mod layout;
mod mem;
mod mem_copy;
mod mem_map;
mod mem_unmap;
mod region;
mod region_cut;
mod region_find;
mod threads;

pub use fd::Fd;
pub use fd_kind::Kind;
pub use handle::Guest;
pub use layout::{
    BRK_BASE, BRK_LIMIT, EXEC_BASE, INTERP_BASE, MMAP_BASE, MMAP_LIMIT, STACK_SIZE,
    STACK_TOP,
};
pub use mem::{page_down, page_up, span_within, MAX_SPAN, PAGE};
pub use region::Region;
