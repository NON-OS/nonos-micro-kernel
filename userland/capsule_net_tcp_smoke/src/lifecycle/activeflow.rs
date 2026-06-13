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

use super::ops::{close, mark, recv, state, E_NO_SOCKET, E_OK, S_TIMEWAIT};
use crate::client;

pub fn echo(port: u32, handle: u32) {
    let mut body = [0u8; 8];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..8].copy_from_slice(b"ping");
    let mut resp = [0u8; client::HDR_LEN];
    if client::call(port, 5, &body, &mut resp) != Some((E_OK, 0)) {
        return;
    }
    let mut buf = [0u8; client::HDR_LEN + 64];
    for _ in 0..64 {
        if let Some((E_OK, n)) = recv(port, handle, &mut buf) {
            if n >= 4 && &buf[client::HDR_LEN..client::HDR_LEN + 4] == b"ping" {
                mark(b"[TCP] ECHO OK\n");
                return;
            }
        }
        mk_yield();
    }
}

pub fn close_active(port: u32, handle: u32) -> bool {
    if close(port, handle) {
        mark(b"[TCP] CLOSE OK\n");
        true
    } else {
        false
    }
}

pub fn timewait(port: u32, handle: u32) {
    for _ in 0..6000 {
        if let Some((E_OK, S_TIMEWAIT)) = state(port, handle) {
            mark(b"[TCP] TIMEWAIT\n");
            return;
        }
        mk_yield();
    }
}

pub fn closed(port: u32, handle: u32) {
    for _ in 0..12000 {
        if let Some((E_NO_SOCKET, _)) = state(port, handle) {
            mark(b"[TCP] CLOSED\n");
            return;
        }
        mk_yield();
    }
}
