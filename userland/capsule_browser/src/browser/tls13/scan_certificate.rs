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

pub fn scan(msgs: &[u8]) -> bool {
    let mut pos = 0usize;
    while pos + 4 <= msgs.len() {
        let len = ((msgs[pos + 1] as usize) << 16) | ((msgs[pos + 2] as usize) << 8) | msgs[pos + 3] as usize;
        let end = pos + 4 + len;
        if end > msgs.len() {
            return false;
        }
        if msgs[pos] == 11 && valid_body(&msgs[pos + 4..end]) {
            return true;
        }
        pos = end;
    }
    false
}

fn valid_body(body: &[u8]) -> bool {
    if body.len() < 7 {
        return false;
    }
    let ctx_len = body[0] as usize;
    let list_off = 1usize.saturating_add(ctx_len);
    if list_off + 3 > body.len() {
        return false;
    }
    let len = ((body[list_off] as usize) << 16) | ((body[list_off + 1] as usize) << 8) | body[list_off + 2] as usize;
    if list_off + 3 + len != body.len() || len < 8 {
        return false;
    }
    first_entry(&body[list_off + 3..])
}

fn first_entry(list: &[u8]) -> bool {
    let len = ((list[0] as usize) << 16) | ((list[1] as usize) << 8) | list[2] as usize;
    len > 128 && 5 + len <= list.len()
}
