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

use super::fmt::{caps_line, digest_prefix, tier_word};
use super::summary::{slug, PkgSummary};
use crate::term::state::State;
use crate::term::util::format_u64;

// The consent prompt. `name` is the slug the package installs under, which
// is what the user will later launch and remove; the endpoint name in the
// reply is an internal detail and is deliberately not shown.
pub(super) fn summary(state: &mut State, s: &PkgSummary) {
    field(state, b"name       ", slug(&s.namespace));
    field(state, b"namespace  ", &s.namespace);
    field(state, b"tier       ", tier_word(s.tier));
    field(state, b"caps       ", &caps_line(s.caps));
    field(state, b"digest     ", &digest_prefix(&s.digest));
}

// Map the installer's coarse errnos to a precise reason. A package whose
// digest moved between the query and the commit lands on the verification
// line, refused before anything reaches the store.
pub(super) fn error(state: &mut State, status: i32) {
    let reason: &[u8] = match status {
        -2 => b"pkg: not installed",
        -5 => b"pkg: store write failed",
        -11 => b"pkg: installer not ready, try again",
        -13 => b"pkg: signature or digest failed verification",
        -17 => b"pkg: already installed, remove it first",
        -22 => b"pkg: malformed package or path",
        _ => b"",
    };
    if !reason.is_empty() {
        state.scrollback.push_error(reason);
        return;
    }
    let mut num = [0u8; 24];
    let k = format_u64((-status) as u64, &mut num);
    let mut line = Vec::with_capacity(14 + k);
    line.extend_from_slice(b"pkg failed: -");
    line.extend_from_slice(&num[..k]);
    state.scrollback.push_error(&line);
}

fn field(state: &mut State, label: &[u8], value: &[u8]) {
    let mut line = Vec::with_capacity(label.len() + value.len());
    line.extend_from_slice(label);
    line.extend_from_slice(value);
    state.scrollback.push_line(&line);
}
