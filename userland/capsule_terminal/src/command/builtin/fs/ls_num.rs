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

//! Number formatting for the `ls -l` columns: plain decimals, `-h` sizes, and
//! right alignment into a fixed column width.

use alloc::vec::Vec;

use crate::term::util::format_u64;

pub fn decimal(value: u64) -> Vec<u8> {
    let mut buf = [0u8; 24];
    let n = format_u64(value, &mut buf);
    buf[..n].to_vec()
}

pub fn human_size(value: u64) -> Vec<u8> {
    const UNITS: [u8; 4] = [b'B', b'K', b'M', b'G'];
    let mut idx = 0;
    let mut whole = value;
    let mut tenths = 0;
    while whole >= 1024 && idx + 1 < UNITS.len() {
        tenths = (whole % 1024) * 10 / 1024;
        whole /= 1024;
        idx += 1;
    }
    let mut out = decimal(whole);
    if idx > 0 && whole < 10 {
        out.push(b'.');
        out.push(b'0' + tenths as u8);
    }
    out.push(UNITS[idx]);
    out
}

pub fn pad_left(line: &mut Vec<u8>, text: &[u8], width: usize) {
    for _ in text.len()..width {
        line.push(b' ');
    }
    line.extend_from_slice(text);
}
