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

pub fn scan(msgs: &[u8], host: &[u8]) -> bool {
    let mut pos = 0usize;
    while pos + 4 <= msgs.len() {
        let len = ((msgs[pos + 1] as usize) << 16) | ((msgs[pos + 2] as usize) << 8) | msgs[pos + 3] as usize;
        let end = pos + 4 + len;
        if end > msgs.len() {
            return false;
        }
        if msgs[pos] == 11 && body_match(&msgs[pos + 4..end], host) {
            return true;
        }
        pos = end;
    }
    false
}

fn body_match(body: &[u8], host: &[u8]) -> bool {
    let Some(ctx_len) = body.first() else { return false };
    let Some(list_off) = 1usize.checked_add(*ctx_len as usize) else { return false };
    let Some(list) = cert_list(body, list_off) else { return false };
    let Some(cert) = first_cert(list) else { return false };
    super::cert_dns_match::matches(cert, host)
}

fn cert_list(body: &[u8], off: usize) -> Option<&[u8]> {
    if off + 3 > body.len() {
        return None;
    }
    let len = ((body[off] as usize) << 16) | ((body[off + 1] as usize) << 8) | body[off + 2] as usize;
    super::read::slice(body, off + 3, len)
}

fn first_cert(list: &[u8]) -> Option<&[u8]> {
    if list.len() < 5 {
        return None;
    }
    let len = ((list[0] as usize) << 16) | ((list[1] as usize) << 8) | list[2] as usize;
    super::read::slice(list, 3, len)
}
