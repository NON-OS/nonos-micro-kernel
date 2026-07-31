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

// Terminate a capsule by pid: `kill <pid> [signal]` (defaults to 9). Routed
// through the capability-checked MkKill syscall, so it only reaches a pid the
// terminal is allowed to signal. `caps`/`ps` lists the pids.

use alloc::vec::Vec;
use nonos_libc::mk_kill;

use crate::term::state::State;
use crate::term::util::format_u64;

pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(&pid_arg) = args.first() else {
        state.scrollback.push_error(b"usage: kill <pid> [signal]");
        return false;
    };
    let Some(pid) = parse_u64(pid_arg) else {
        state.scrollback.push_error(b"kill: pid must be a number");
        return false;
    };
    let sig = args.get(1).and_then(|a| parse_u64(a)).unwrap_or(9);
    if mk_kill(pid, sig) < 0 {
        state.scrollback.push_error(b"kill: no such pid, or not permitted");
        return false;
    }
    let mut line: Vec<u8> = Vec::new();
    line.extend_from_slice(b"killed pid ");
    append_u64(&mut line, pid);
    state.scrollback.push_line(&line);
    true
}

fn parse_u64(a: &[u8]) -> Option<u64> {
    if a.is_empty() {
        return None;
    }
    let mut v: u64 = 0;
    for &b in a {
        if !b.is_ascii_digit() {
            return None;
        }
        v = v.checked_mul(10)?.checked_add((b - b'0') as u64)?;
    }
    Some(v)
}

fn append_u64(out: &mut Vec<u8>, v: u64) {
    let mut buf = [0u8; 24];
    let k = format_u64(v, &mut buf);
    out.extend_from_slice(&buf[..k]);
}
