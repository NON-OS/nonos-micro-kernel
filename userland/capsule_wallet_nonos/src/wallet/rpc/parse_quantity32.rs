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

pub fn parse_quantity32(resp: &[u8]) -> Option<[u8; 32]> {
    let value = super::find_result::find_result(resp)?;
    let hex = value.strip_prefix(b"0x")?;
    if hex.len() > 64 {
        return None;
    }
    let mut out = [0u8; 32];
    let mut nibble = (64usize).saturating_sub(hex.len());
    for b in hex {
        let v = match *b {
            b'0'..=b'9' => *b - b'0',
            b'a'..=b'f' => *b - b'a' + 10,
            b'A'..=b'F' => *b - b'A' + 10,
            _ => return None,
        };
        let slot = nibble / 2;
        out[slot] |= if nibble & 1 == 0 { v << 4 } else { v };
        nibble += 1;
    }
    Some(out)
}
