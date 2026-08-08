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

pub fn u16_at(buf: &[u8], off: usize) -> Option<u16> {
    // checked_add so a near-usize::MAX offset cannot wrap the bound check.
    if off.checked_add(2)? > buf.len() {
        return None;
    }
    Some(u16::from_be_bytes([buf[off], buf[off + 1]]))
}

pub fn slice(buf: &[u8], off: usize, len: usize) -> Option<&[u8]> {
    if off.checked_add(len)? > buf.len() {
        return None;
    }
    Some(&buf[off..off + len])
}
