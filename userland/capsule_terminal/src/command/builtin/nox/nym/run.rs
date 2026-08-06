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
use nonos_libc::mk_service_lookup;

use super::gateway::push_gateway;
use super::status::status;
use super::topology::topology_line;
use super::wire::{OP_HEALTHCHECK, OP_TOPOLOGY_STATUS};
use crate::term::state::State;

const SERVICE: &[u8] = b"net.nym";

/// Report the mixnet client's state without leaving the machine.
///
/// Until now the only way to see whether the Nym path was alive was the host
/// serial log, which is no use to somebody running the OS.
pub fn run(state: &mut State) -> bool {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        state.scrollback.push_error(b"nym: capsule not running");
        return false;
    }
    let Some(health) = status(port, OP_HEALTHCHECK, 6) else {
        state.scrollback.push_error(b"nym: capsule not answering");
        return false;
    };

    let mut line: Vec<u8> = Vec::new();
    if health[..4] == [0, 0, 0, 0] {
        line.extend_from_slice(b"nym: client up, no gateway yet");
    } else {
        line.extend_from_slice(b"nym: gateway ");
        push_gateway(&mut line, &health);
    }
    state.scrollback.push_line(&line);

    match status(port, OP_TOPOLOGY_STATUS, 28) {
        Some(body) => state.scrollback.push_line(&topology_line(&body)),
        None => state.scrollback.push_line(b"topology: unavailable"),
    }
    true
}
