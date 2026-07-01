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

pub fn response_len(buf: &[u8]) -> Option<usize> {
    if buf.len() < 5 || buf[0] != 0x05 {
        return None;
    }
    match buf[3] {
        0x01 => Some(10),
        0x03 => buf.get(4).map(|&n| 7 + n as usize),
        0x04 => Some(22),
        _ => None,
    }
}
