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

//! The 16-bit I/O port space, for the drivers still living in it.
//!
//! x86_64 has it in the instruction set. Nothing else does: on aarch64 a PCI
//! bridge's I/O window is a range of physical memory and a port is an offset
//! into it. A board with no window is normal, not an error, so reads there
//! answer `!0` the way an unclaimed bus cycle does and writes go nowhere.

mod wait;

#[cfg(target_arch = "aarch64")]
mod window;

#[cfg(target_arch = "aarch64")]
#[path = "aarch64.rs"]
mod backend;
#[cfg(target_arch = "x86_64")]
#[path = "x86_64/mod.rs"]
mod backend;
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[path = "unsupported.rs"]
mod backend;

#[cfg(target_arch = "aarch64")]
pub use window::set_io_window;

pub use backend::{inb, inl, inw, outb, outl, outw};
pub use wait::io_wait;
