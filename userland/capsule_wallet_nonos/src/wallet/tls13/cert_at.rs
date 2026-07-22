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

pub fn cert_at(body: &[u8], want: u8) -> Option<&[u8]> {
    let ctx_len = *body.first()? as usize;
    let off = 1usize.checked_add(ctx_len)?;
    if off + 3 > body.len() {
        return None;
    }
    let list_len =
        ((body[off] as usize) << 16) | ((body[off + 1] as usize) << 8) | body[off + 2] as usize;
    let end = off.checked_add(3)?.checked_add(list_len)?;
    let mut pos = off + 3;
    let mut index = 0u8;
    while pos + 5 <= end && end == body.len() {
        let len =
            ((body[pos] as usize) << 16) | ((body[pos + 1] as usize) << 8) | body[pos + 2] as usize;
        let cert = super::read::slice(body, pos + 3, len)?;
        let cert_end = pos + 3 + len;
        if cert_end + 2 > end {
            return None;
        }
        let ext_len = ((body[cert_end] as usize) << 8) | body[cert_end + 1] as usize;
        if index == want {
            return Some(cert);
        }
        pos = cert_end.checked_add(2)?.checked_add(ext_len)?;
        index = index.checked_add(1)?;
    }
    None
}
