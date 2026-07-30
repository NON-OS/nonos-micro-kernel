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

//! ARM PrimeCell PL031, the real-time clock on QEMU's virt board and most
//! ARM reference platforms.
//!
//! Simpler than a PC's CMOS clock: `DR` is a 32-bit count of seconds since the
//! Unix epoch, readable in one access with no BCD, no update-in-progress flag
//! and no register index to select first.

mod read;
mod state;

pub use read::unix_timestamp;
pub use state::set_base;
