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

//! The calls that shape a guest's address space: `mmap`, `munmap`, `brk` and
//! `mprotect`.

mod map;
mod map_anon;
mod map_exec;
mod map_file;
mod map_req;
mod memory;
mod prot;
mod prot_span;

pub use map::mmap;
pub use map_req::MapReq;
pub use memory::{brk, munmap};
pub use prot::mprotect;
