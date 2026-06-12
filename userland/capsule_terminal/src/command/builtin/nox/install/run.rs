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

use super::artifacts::read_artifacts;
use super::call::call_installer;
use super::emit::{emit_err, emit_ok};
use super::super::ensure_pid::ensure_pid;
use crate::term::state::State;

// install <name>: read a capsule's artifacts from the store and ask the
// installer to verify, load, and spawn it. The name selects
// /capsules/<name>.* and must be a plain ascii token with no path separators.
pub fn run(state: &mut State, args: &[&[u8]]) -> bool {
    if args.is_empty() {
        state.scrollback.push_error(b"usage: install <name>");
        return false;
    }
    let stem = args[0];
    if !valid_name(stem) {
        state.scrollback.push_error(b"install: name must be ascii letters, digits, _ or -");
        return false;
    }
    let pid = ensure_pid(state);
    let artifacts = match read_artifacts(pid, stem) {
        Ok(a) => a,
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            return false;
        }
    };
    match call_installer(&artifacts) {
        Ok(new_pid) => {
            emit_ok(state, stem, new_pid);
            true
        }
        Err(status) => {
            emit_err(state, status);
            false
        }
    }
}

fn valid_name(stem: &[u8]) -> bool {
    !stem.is_empty()
        && stem.len() <= 64
        && stem
            .iter()
            .all(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}
