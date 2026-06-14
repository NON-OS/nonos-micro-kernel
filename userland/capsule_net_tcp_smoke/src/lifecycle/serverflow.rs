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

use super::ops::{accept, close, listen, mark, state};
use super::ops::{E_NO_SOCKET, E_OK, S_CLOSEWAIT, S_TIMEWAIT};
use crate::wait::poll_until;

const PASSIVE_PORT: u16 = 6000;
const ACTIVE_PORT: u16 = 6001;

fn accept_child(port: u32, listener: u32) -> Option<u32> {
    let mut child = None;
    poll_until(120_000, || {
        child = accept(port, listener);
        child.is_some()
    });
    child
}

fn accepted(port: u32, listen_port: u16, ready: &[u8]) -> Option<u32> {
    let listener = listen(port, listen_port)?;
    mark(ready);
    accept_child(port, listener)
}

pub fn passive_server(port: u32) {
    let child = match accepted(port, PASSIVE_PORT, b"[TCP] PLISTEN\n") {
        Some(c) => c,
        None => return,
    };
    let saw = poll_until(30_000, || matches!(state(port, child), Some((E_OK, S_CLOSEWAIT))));
    if !saw || !close(port, child) {
        return;
    }
    if poll_until(30_000, || matches!(state(port, child), Some((E_NO_SOCKET, _)))) {
        mark(b"[TCP] PASSIVE-CLOSE OK\n");
    }
}

pub fn active_server(port: u32) {
    let child = match accepted(port, ACTIVE_PORT, b"[TCP] ALISTEN\n") {
        Some(c) => c,
        None => return,
    };
    if !close(port, child) {
        return;
    }
    if poll_until(20_000, || matches!(state(port, child), Some((E_OK, S_TIMEWAIT)))) {
        mark(b"[TCP] TIMEWAIT\n");
    }
    if poll_until(90_000, || matches!(state(port, child), Some((E_NO_SOCKET, _)))) {
        mark(b"[TCP] CLOSED\n");
    }
}
