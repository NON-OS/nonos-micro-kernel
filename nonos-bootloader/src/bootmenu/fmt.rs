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

pub(super) fn status_line(buf: &mut [u8; 64], secs: u32) -> &[u8] {
    if secs == 0 {
        let s = b"up/down select    enter boot";
        buf[..s.len()].copy_from_slice(s);
        return &buf[..s.len()];
    }
    let mut n = 0usize;
    for &b in b"Booting Hardened in " {
        buf[n] = b;
        n += 1;
    }
    if secs >= 10 {
        buf[n] = b'0' + (secs / 10 % 10) as u8;
        n += 1;
    }
    buf[n] = b'0' + (secs % 10) as u8;
    n += 1;
    for &b in b"s.  up/down, enter." {
        buf[n] = b;
        n += 1;
    }
    &buf[..n]
}
