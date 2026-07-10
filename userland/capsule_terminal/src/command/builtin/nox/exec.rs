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
use nonos_libc::mk_time_millis;

use super::install::call_installer;
use crate::jobs::{submit, JobWork};
use crate::term::state::State;

// exec <name> [argv...]: load the store capsule <name> through the installer
// (which parents it to this terminal) and drive it as an interactive
// foreground job: stdin forwarded from the keyboard, stdout drained into the
// block, Ctrl-C killing it. Unlike `run`/`open`, which focus an existing
// desktop app, this spawns a fresh child the terminal owns.
pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.is_empty() {
        state.scrollback.push_error(b"usage: exec <name> [argv...]");
        return false;
    }
    let stem = args[0];
    if !valid_name(stem) {
        state.scrollback.push_error(b"exec: name must be ascii letters, digits, _ or -");
        return false;
    }
    let argv = argv_blob(stem, &args[1..]);
    match call_installer(stem, &argv) {
        Ok(pid) => {
            let work = JobWork::ExternalStage { pid, in_buf: Vec::new(), in_cursor: 0 };
            let _ = submit(state, stem, false, work);
            state.fg_running = true;
            state.fg_started_ms = mk_time_millis();
            true
        }
        Err(_) => {
            state.scrollback.push_error(b"exec: load failed");
            false
        }
    }
}

fn valid_name(stem: &[u8]) -> bool {
    !stem.is_empty()
        && stem.len() <= 64
        && stem.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

fn argv_blob(name: &[u8], args: &[&[u8]]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(name);
    for arg in args {
        out.push(0);
        out.extend_from_slice(arg);
    }
    out
}
