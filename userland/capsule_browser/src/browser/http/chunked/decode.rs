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

use alloc::vec::Vec;

pub fn decode(body: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < body.len() {
        let line_end = i + super::find_crlf::find_crlf(&body[i..])?;
        let size = super::parse_hex::parse_hex(&body[i..line_end])?;
        i = line_end + 2;
        if size == 0 {
            if body.get(i..i + 2) == Some(b"\r\n") {
                return Some(out);
            }
            return body[i..].windows(4).any(|w| w == b"\r\n\r\n").then_some(out);
        }
        if size > body.len().saturating_sub(i) {
            return None;
        }
        out.extend_from_slice(&body[i..i + size]);
        i += size;
        if body.get(i..i + 2) != Some(b"\r\n") {
            return None;
        }
        i += 2;
    }
    None
}
