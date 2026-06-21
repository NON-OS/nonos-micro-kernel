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

use nonos_libc::{mk_time_rtc, RtcTime};

pub fn fmt_hms(h: u8, m: u8, s: u8) -> [u8; 8] {
    let mut out = *b"00:00:00";
    out[0] = b'0' + (h / 10) % 10;
    out[1] = b'0' + h % 10;
    out[3] = b'0' + (m / 10) % 10;
    out[4] = b'0' + m % 10;
    out[6] = b'0' + (s / 10) % 10;
    out[7] = b'0' + s % 10;
    out
}

pub fn rtc_hms() -> [u8; 8] {
    let mut t = RtcTime::default();
    if mk_time_rtc(&mut t as *mut RtcTime) == 0 {
        fmt_hms(t.hour, t.minute, t.second)
    } else {
        *b"--:--:--"
    }
}
