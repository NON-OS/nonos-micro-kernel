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
use nonos_libc::{mk_time_rtc, RtcTime};

use crate::term::state::State;

pub fn run(state: &mut State) -> bool {
    let mut t = RtcTime::default();
    if mk_time_rtc(&mut t as *mut RtcTime) < 0 {
        state.scrollback.push_error(b"date: clock unavailable");
        return false;
    }
    let mut s = Vec::with_capacity(19);
    push_u(&mut s, t.year as u64, 4);
    s.push(b'-');
    push_u(&mut s, t.month as u64, 2);
    s.push(b'-');
    push_u(&mut s, t.day as u64, 2);
    s.push(b' ');
    push_u(&mut s, t.hour as u64, 2);
    s.push(b':');
    push_u(&mut s, t.minute as u64, 2);
    s.push(b':');
    push_u(&mut s, t.second as u64, 2);
    state.scrollback.push_line(&s);
    true
}

fn push_u(out: &mut Vec<u8>, v: u64, width: usize) {
    let mut tmp = [0u8; 20];
    let mut n = v;
    let mut len = 0;
    loop {
        tmp[len] = b'0' + (n % 10) as u8;
        len += 1;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    for _ in len..width {
        out.push(b'0');
    }
    for j in (0..len).rev() {
        out.push(tmp[j]);
    }
}
