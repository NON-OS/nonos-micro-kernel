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

pub fn parse_u64(resp: &[u8]) -> Option<u64> {
    let value = super::find_result::find_result(resp)?;
    let hex = value.strip_prefix(b"0x")?;
    let mut out = 0u64;
    for b in hex {
        let v = match *b {
            b'0'..=b'9' => *b - b'0',
            b'a'..=b'f' => *b - b'a' + 10,
            b'A'..=b'F' => *b - b'A' + 10,
            _ => return None,
        };
        out = out.checked_mul(16)?.checked_add(v as u64)?;
    }
    Some(out)
}
