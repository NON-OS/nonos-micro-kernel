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

use crate::term::state::State;
use crate::term::util::format_u64;

// Operator-grade success line: "loaded <name> as pid <n>".
pub(super) fn emit_ok(state: &mut State, stem: &[u8], pid: u32) {
    let mut num = [0u8; 24];
    let k = format_u64(pid as u64, &mut num);
    let mut line = Vec::with_capacity(18 + stem.len() + k);
    line.extend_from_slice(b"loaded ");
    line.extend_from_slice(stem);
    line.extend_from_slice(b" as pid ");
    line.extend_from_slice(&num[..k]);
    state.scrollback.push_line(&line);
}

// Failure line. Map the load path's coarse errnos to a precise reason; a
// tampered artifact whose hash no longer matches its manifest lands here as
// a verification rejection, refused before execution.
pub(super) fn emit_err(state: &mut State, status: i32) {
    let reason: &[u8] = match status {
        -11 => b"install rejected: installer not ready, try again",
        -13 => b"install rejected: signature, manifest, or attestation failed verification",
        -16 => b"install rejected: installer busy",
        -17 => b"install rejected: already running, close it first",
        -22 => b"install rejected: malformed manifest or artifacts",
        _ => b"",
    };
    if !reason.is_empty() {
        state.scrollback.push_error(reason);
        return;
    }
    let mut num = [0u8; 24];
    let k = format_u64((-status) as u64, &mut num);
    let mut line = Vec::with_capacity(20 + k);
    line.extend_from_slice(b"install failed: -");
    line.extend_from_slice(&num[..k]);
    state.scrollback.push_error(&line);
}
