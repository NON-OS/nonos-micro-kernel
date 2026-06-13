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

use nonos_libc::mk_yield;

use super::ops::{close, connect, connect_errno, mark, recv, state};
use super::ops::{E_NO_SOCKET, E_OK, S_CLOSEWAIT};
use crate::client;

pub fn passive_close(port: u32, srv: [u8; 4]) {
    let handle = match connect(port, srv, 8) {
        Some(h) => h,
        None => return,
    };
    let mut buf = [0u8; client::HDR_LEN + 64];
    let _ = recv(port, handle, &mut buf);
    let mut saw_closewait = false;
    for _ in 0..6000 {
        if let Some((E_OK, S_CLOSEWAIT)) = state(port, handle) {
            saw_closewait = true;
            break;
        }
        mk_yield();
    }
    if !saw_closewait || !close(port, handle) {
        return;
    }
    for _ in 0..12000 {
        if let Some((E_NO_SOCKET, _)) = state(port, handle) {
            mark(b"[TCP] PASSIVE-CLOSE OK\n");
            return;
        }
        mk_yield();
    }
}

pub fn rst_refused(port: u32, srv: [u8; 4]) {
    let (errno, handle) = match connect_errno(port, srv, 9) {
        Some(r) => r,
        None => {
            mark(b"[TCP] RST-REFUSED OK\n");
            return;
        }
    };
    if errno != E_OK {
        mark(b"[TCP] RST-REFUSED OK\n");
        return;
    }
    for _ in 0..12000 {
        if let Some((E_NO_SOCKET, _)) = state(port, handle) {
            mark(b"[TCP] RST-REFUSED OK\n");
            return;
        }
        mk_yield();
    }
}
