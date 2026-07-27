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

// `uptime`: how long the system has been up, from the monotonic clock.

use alloc::vec::Vec;
use nonos_libc::mk_uptime_ms;

use crate::term::state::State;
use crate::term::util::format_u64;

pub fn run(state: &mut State) -> bool {
    let ms = mk_uptime_ms();
    if ms < 0 {
        state.scrollback.push_error(b"uptime: unavailable");
        return false;
    }
    let secs = (ms as u64) / 1000;
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    let mut line: Vec<u8> = Vec::new();
    line.extend_from_slice(b"up ");
    append_u64(&mut line, h);
    line.extend_from_slice(b"h ");
    append_u64(&mut line, m);
    line.extend_from_slice(b"m ");
    append_u64(&mut line, s);
    line.push(b's');
    state.scrollback.push_line(&line);
    true
}

fn append_u64(out: &mut Vec<u8>, v: u64) {
    let mut buf = [0u8; 24];
    let k = format_u64(v, &mut buf);
    out.extend_from_slice(&buf[..k]);
}
