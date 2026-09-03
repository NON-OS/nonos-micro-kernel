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

use nonos_libc::mk_uptime_ms;

pub fn read_millis() -> Option<u64> {
    let raw = mk_uptime_ms();
    if raw < 0 {
        return None;
    }
    Some(raw as u64)
}

pub fn split_dhms(total_ms: u64) -> (u64, u64, u64, u64) {
    let total_s = total_ms / 1000;
    let days = total_s / 86_400;
    let hours = (total_s % 86_400) / 3600;
    let minutes = (total_s % 3600) / 60;
    let seconds = total_s % 60;
    (days, hours, minutes, seconds)
}
