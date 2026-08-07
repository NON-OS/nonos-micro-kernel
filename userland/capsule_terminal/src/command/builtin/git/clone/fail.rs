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
//! Saying why a transfer did not happen.

extern crate alloc;

use alloc::vec::Vec;

use nonos_git::TransportError;

use crate::command::output::Output;
use crate::term::state::State;

/// Report a failure in terms of what the user can do about it.
pub(in crate::command::builtin::git) fn fail_with(
    state: &mut State,
    command: &str,
    error: TransportError,
) {
    let reason: &[u8] = match error {
        TransportError::Unreachable => b"cannot reach the host, check the network",
        TransportError::Closed => b"the connection closed during the transfer",
        TransportError::Status(404) => b"no such repository, or it is private",
        TransportError::Status(401) | TransportError::Status(403) => {
            b"the server wants credentials, which this cannot send yet"
        }
        TransportError::Status(_) => b"the server refused the request",
        TransportError::Malformed => b"the server sent something unreadable",
        TransportError::Refused => b"the server refused the update",
    };
    let mut line = Vec::from(command.as_bytes());
    line.extend_from_slice(b": ");
    line.extend_from_slice(reason);
    Output::new(&mut state.scrollback).writeln(&line);
}
