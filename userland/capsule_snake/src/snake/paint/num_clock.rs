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

use super::num::{tail, Digits, CAP};

// Minutes and seconds, because a run is measured in tens of seconds and a raw
// millisecond count reads as noise in a summary row.
pub fn clock(ms: i64) -> Digits {
    let total = (ms.max(0) / 1000) as u32;
    let mut buf = [b'0'; CAP];
    let mut at = CAP;
    let secs = total % 60;
    at -= 1;
    buf[at] = b'0' + (secs % 10) as u8;
    at -= 1;
    buf[at] = b'0' + (secs / 10) as u8;
    at -= 1;
    buf[at] = b':';
    let mut mins = total / 60;
    loop {
        at -= 1;
        buf[at] = b'0' + (mins % 10) as u8;
        mins /= 10;
        if mins == 0 {
            break;
        }
    }
    tail(buf, at)
}

pub fn hex(value: u32) -> Digits {
    const NIBBLE: [u8; 16] = *b"0123456789ABCDEF";
    let mut buf = [b'0'; CAP];
    for shift in 0..8 {
        buf[CAP - 1 - shift] = NIBBLE[((value >> (shift * 4)) & 0xF) as usize];
    }
    tail(buf, CAP - 8)
}
