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

mod consts;
mod init;
mod rdtsc;
mod time;

pub use consts::{BOOT_TSC, BOOT_UNIX_MS, NTP_OFFSET_MS, TSC_HZ};
pub use init::init;
pub use rdtsc::rdtsc;
pub use time::{base_unix_ms, set_ntp_offset_ms, since_boot_ms, unix_ms};
