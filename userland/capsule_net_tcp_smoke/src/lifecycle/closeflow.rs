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

use super::ops::{close, connect, connect_errno, mark, recv, state};
use super::ops::{E_NO_SOCKET, E_OK, S_CLOSEWAIT};
use crate::client;
use crate::wait::poll_until;

pub fn passive_close(port: u32, srv: [u8; 4]) {
    let handle = match connect(port, srv, 8) {
        Some(h) => h,
        None => return,
    };
    let mut buf = [0u8; client::HDR_LEN + 64];
    let _ = recv(port, handle, &mut buf);
    let saw_closewait = poll_until(15_000, || matches!(state(port, handle), Some((E_OK, S_CLOSEWAIT))));
    if !saw_closewait || !close(port, handle) {
        return;
    }
    if poll_until(75_000, || matches!(state(port, handle), Some((E_NO_SOCKET, _)))) {
        mark(b"[TCP] PASSIVE-CLOSE OK\n");
    }
}

pub fn rst_refused(port: u32, srv: [u8; 4]) {
    let (errno, handle) = match connect_errno(port, srv, 9) {
        Some(r) => r,
        None => return mark(b"[TCP] RST-REFUSED OK\n"),
    };
    if errno != E_OK {
        return mark(b"[TCP] RST-REFUSED OK\n");
    }
    if poll_until(15_000, || matches!(state(port, handle), Some((E_NO_SOCKET, _)))) {
        mark(b"[TCP] RST-REFUSED OK\n");
    }
}
