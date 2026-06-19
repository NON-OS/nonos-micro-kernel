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

use super::call::call_installer;
use super::emit::{emit_err, emit_ok};
use crate::term::state::State;

// install <name> [argv...]: ask the installer to verify, load, and spawn the
// capsule held at /capsules/<name>.*. The name must be a plain ascii token
// with no path separators; the installer reads the artifacts from the store.
pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.is_empty() {
        state.scrollback.push_error(b"usage: install <name> [argv...]");
        return false;
    }
    let stem = args[0];
    if !valid_name(stem) {
        state.scrollback.push_error(b"install: name must be ascii letters, digits, _ or -");
        return false;
    }
    let argv = argv_blob(stem, &args[1..]);
    match call_installer(stem, &argv) {
        Ok(new_pid) => {
            emit_ok(state, stem, new_pid);
            debug_marker(b"[TERMINAL-INSTALL] load ok\n");
            drain_output(state, new_pid);
            true
        }
        Err(status) => {
            emit_err(state, status);
            debug_marker(b"[TERMINAL-INSTALL] load failed\n");
            false
        }
    }
}

// Pull the loaded capsule's stdout out of its proc.<pid> inbox and into
// the terminal window. The kernel mirrors each MkDebug line there. We
// yield between polls so the child can run, stop when it exits, and cap
// the wait so a child that never exits cannot freeze the terminal.
fn drain_output(state: &mut State, pid: u32) {
    use nonos_libc::{mk_pid_alive, mk_proc_output, mk_time_millis, mk_yield};
    const DEADLINE_MS: i64 = 5000;
    let start = mk_time_millis();
    let mut buf = [0u8; 256];
    let mut saw_output = false;
    loop {
        let n = mk_proc_output(pid, buf.as_mut_ptr(), buf.len());
        if n > 0 {
            state.scrollback.push_line(&buf[..(n as usize).min(buf.len())]);
            mark_output_drained(&mut saw_output);
            continue;
        }
        if !mk_pid_alive(pid) {
            loop {
                let m = mk_proc_output(pid, buf.as_mut_ptr(), buf.len());
                if m <= 0 {
                    break;
                }
                state.scrollback.push_line(&buf[..(m as usize).min(buf.len())]);
                mark_output_drained(&mut saw_output);
            }
            break;
        }
        if mk_time_millis().wrapping_sub(start) > DEADLINE_MS {
            break;
        }
        mk_yield();
    }
}

fn mark_output_drained(saw_output: &mut bool) {
    if *saw_output {
        return;
    }
    *saw_output = true;
    debug_marker(b"[TERMINAL-MOUT] output drained\n");
}

fn debug_marker(msg: &[u8]) {
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
}

fn valid_name(stem: &[u8]) -> bool {
    !stem.is_empty()
        && stem.len() <= 64
        && stem.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

// argv[0] is the program name (the capsule stem), matching the POSIX
// convention upstream CLI parsers expect; the user args follow.
fn argv_blob(name: &[u8], args: &[&[u8]]) -> alloc::vec::Vec<u8> {
    let mut out = alloc::vec::Vec::new();
    out.extend_from_slice(name);
    for arg in args {
        out.push(0);
        out.extend_from_slice(arg);
    }
    out
}
