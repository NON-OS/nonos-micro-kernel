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

use crate::term::util::format_u64;

pub fn fmt_dur(ms: u32) -> ([u8; 7], usize) {
    let mut o = [0u8; 7];
    if ms < 1000 {
        let n = format_u64(ms as u64, &mut o);
        o[n] = b'm';
        o[n + 1] = b's';
        (o, n + 2)
    } else if ms < 60_000 {
        let mut n = format_u64((ms / 1000) as u64, &mut o);
        o[n] = b'.';
        n += 1;
        o[n] = b'0' + ((ms % 1000) / 100) as u8;
        n += 1;
        o[n] = b's';
        n += 1;
        (o, n)
    } else {
        let mut n = format_u64((ms / 60_000).min(99) as u64, &mut o);
        o[n] = b'm';
        n += 1;
        let s = (ms % 60_000) / 1000;
        o[n] = b'0' + (s / 10) as u8;
        n += 1;
        o[n] = b'0' + (s % 10) as u8;
        n += 1;
        o[n] = b's';
        n += 1;
        (o, n)
    }
}
