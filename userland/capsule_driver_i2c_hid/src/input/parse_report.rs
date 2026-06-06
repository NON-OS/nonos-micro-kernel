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

use super::sample::MouseSample;

pub fn parse_report(buf: &[u8]) -> Option<MouseSample> {
    if buf.len() < 5 {
        return None;
    }
    let total = usize::from(u16::from_le_bytes([buf[0], buf[1]]));
    if total < 5 || total > buf.len() {
        return None;
    }
    let body = &buf[2..total];
    let start = if body.len() >= 4 && body[0] != 0 && body[1] & !0x1f == 0 { 1 } else { 0 };
    if body.len() < start + 3 {
        return None;
    }
    Some(MouseSample {
        buttons: body[start] & 0x1f,
        dx: body[start + 1] as i8,
        dy: body[start + 2] as i8,
        wheel: body.get(start + 3).copied().unwrap_or(0) as i8,
    })
}
