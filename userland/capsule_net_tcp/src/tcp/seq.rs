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

pub fn lt(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) < 0
}

pub fn leq(a: u32, b: u32) -> bool {
    (a.wrapping_sub(b) as i32) <= 0
}

pub fn gt(a: u32, b: u32) -> bool {
    lt(b, a)
}

pub fn between(x: u32, lo: u32, hi: u32) -> bool {
    leq(lo, x) && lt(x, hi)
}

pub fn acceptable(seg_seq: u32, seg_len: u32, rcv_nxt: u32, rcv_wnd: u16) -> bool {
    let wnd = rcv_wnd as u32;
    if seg_len == 0 {
        if wnd == 0 {
            seg_seq == rcv_nxt
        } else {
            between(seg_seq, rcv_nxt, rcv_nxt.wrapping_add(wnd))
        }
    } else if wnd == 0 {
        false
    } else {
        let end = rcv_nxt.wrapping_add(wnd);
        between(seg_seq, rcv_nxt, end)
            || between(seg_seq.wrapping_add(seg_len).wrapping_sub(1), rcv_nxt, end)
    }
}
