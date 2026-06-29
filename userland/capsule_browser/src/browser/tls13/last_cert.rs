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

pub fn last_cert(body: &[u8]) -> Option<&[u8]> {
    let ctx_len = *body.first()? as usize;
    let off = 1usize.checked_add(ctx_len)?;
    if off + 3 > body.len() {
        return None;
    }
    let list_len = ((body[off] as usize) << 16) | ((body[off + 1] as usize) << 8) | body[off + 2] as usize;
    let end = off.checked_add(3)?.checked_add(list_len)?;
    if end != body.len() {
        return None;
    }
    let mut pos = off + 3;
    let mut last = None;
    while pos + 5 <= end {
        let len = ((body[pos] as usize) << 16) | ((body[pos + 1] as usize) << 8) | body[pos + 2] as usize;
        let cert = super::read::slice(body, pos + 3, len)?;
        let cert_end = pos + 3 + len;
        let ext_len = ((body[cert_end] as usize) << 8) | body[cert_end + 1] as usize;
        pos = cert_end.checked_add(2)?.checked_add(ext_len)?;
        last = Some(cert);
    }
    if pos == end { last } else { None }
}
