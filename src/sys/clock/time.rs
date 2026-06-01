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

use super::core::unix_ms;
use crate::sys::policy;

pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

pub fn get_time() -> Time {
    let ms = unix_ms();
    let total_secs = ms / 1000;
    let tz_offset = policy::timezone_offset() as i64;
    let local_secs = (total_secs as i64 + tz_offset * 3600).max(0) as u64;
    let seconds_in_day = local_secs % (24 * 60 * 60);
    let hour = (seconds_in_day / 3600) as u8;
    let minute = ((seconds_in_day % 3600) / 60) as u8;
    let second = (seconds_in_day % 60) as u8;
    Time { hour, minute, second }
}
