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

pub mod civil;
pub mod core;
pub mod format;
mod resolve;
pub mod time;

pub use core::*;
pub use format::*;
pub use time::*;

pub fn get_unix_time() -> u64 {
    unix_ms() / 1000
}

pub fn uptime_seconds() -> u64 {
    crate::sys::timer::uptime::uptime_seconds()
}

pub fn unix_timestamp() -> u64 {
    crate::sys::timer::uptime::uptime_seconds() + 1704067200
}

pub fn boot_time_secs() -> u64 {
    1704067200
}

pub fn system_time_secs() -> u64 {
    boot_time_secs() + uptime_seconds()
}

pub fn uptime_ns() -> u64 {
    crate::sys::timer::uptime::uptime_seconds() * 1_000_000_000
}

pub fn uptime_ms() -> u64 {
    crate::sys::timer::uptime::uptime_seconds() * 1000
}

pub fn get_ticks() -> u64 {
    crate::interrupts::timer::state::get_ticks()
}
