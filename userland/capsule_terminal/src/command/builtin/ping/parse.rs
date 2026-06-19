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

pub fn parse_ipv4(s: &[u8]) -> Option<[u8; 4]> {
    let mut out = [0u8; 4];
    let mut idx = 0usize;
    let mut val: u32 = 0;
    let mut have = false;
    for &c in s {
        if c == b'.' {
            if !have || idx >= 3 {
                return None;
            }
            out[idx] = val as u8;
            idx += 1;
            val = 0;
            have = false;
        } else if c.is_ascii_digit() {
            val = val * 10 + (c - b'0') as u32;
            if val > 255 {
                return None;
            }
            have = true;
        } else {
            return None;
        }
    }
    if !have || idx != 3 {
        return None;
    }
    out[3] = val as u8;
    Some(out)
}
