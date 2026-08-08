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

pub fn cert_count(body: &[u8]) -> u8 {
    let Some(ctx_len) = body.first() else { return 0 };
    let Some(off) = 1usize.checked_add(*ctx_len as usize) else { return 0 };
    if off + 3 > body.len() {
        return 0;
    }
    let list_len =
        ((body[off] as usize) << 16) | ((body[off + 1] as usize) << 8) | body[off + 2] as usize;
    if off + 3 + list_len != body.len() {
        return 0;
    }
    let mut pos = off + 3;
    let mut count = 0u8;
    while pos + 5 <= body.len() {
        let cert_len =
            ((body[pos] as usize) << 16) | ((body[pos + 1] as usize) << 8) | body[pos + 2] as usize;
        let Some(cert_end) = pos.checked_add(3).and_then(|v| v.checked_add(cert_len)) else {
            return 0;
        };
        if cert_end + 2 > body.len() {
            return 0;
        }
        let ext_len = ((body[cert_end] as usize) << 8) | body[cert_end + 1] as usize;
        let Some(next) = cert_end.checked_add(2).and_then(|v| v.checked_add(ext_len)) else {
            return 0;
        };
        if next > body.len() || count == u8::MAX {
            return 0;
        }
        count += 1;
        pos = next;
    }
    if pos == body.len() {
        count
    } else {
        0
    }
}
