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

//! One file per family of Linux calls. A family gets its own file the
//! moment it has more than a handful, so the table stays readable as the
//! surface grows toward the whole of it.

mod io;
mod life;
mod ctl;
mod memory;
mod thread;
mod uname;
mod vector;

pub use io::{close, read, write};
pub use ctl::{fcntl, ioctl};
pub use life::exit;
pub use memory::{brk, mmap, munmap};
pub use thread::{arch_prctl, clock_gettime, getrandom};
pub use uname::uname;
pub use vector::writev;
