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

use nonos_libc::mk_debug;

use crate::client;

pub const E_OK: u16 = 0;
pub const E_NO_SOCKET: u16 = 5;
pub const S_CLOSEWAIT: u8 = 4;
pub const S_TIMEWAIT: u8 = 8;

pub fn mark(msg: &[u8]) {
    let _ = mk_debug(msg.as_ptr(), msg.len());
}

#[cfg(feature = "tcp-selftest")]
const OP_SELFTEST: u16 = 0x7F;

#[cfg(feature = "tcp-selftest")]
pub fn selftest(port: u32) {
    let mut resp = [0u8; client::HDR_LEN + 4];
    let Some((errno, n)) = client::call(port, OP_SELFTEST, &[], &mut resp) else {
        return;
    };
    if errno != E_OK || n < 4 {
        return;
    }
    let bits = u32::from_le_bytes([
        resp[client::HDR_LEN],
        resp[client::HDR_LEN + 1],
        resp[client::HDR_LEN + 2],
        resp[client::HDR_LEN + 3],
    ]);
    if bits & 1 != 0 {
        mark(b"[TCP] SEQ-KAT OK\n");
    }
    if bits & 2 != 0 {
        mark(b"[TCP] ACCEPT-KAT OK\n");
    }
    if bits & 4 != 0 {
        mark(b"[TCP] RTT-KAT OK\n");
    }
    if bits & 8 != 0 {
        mark(b"[TCP] WND-KAT OK\n");
    }
    if bits & 16 != 0 {
        mark(b"[TCP] RETX-KAT OK\n");
    }
    if bits & 32 != 0 {
        mark(b"[TCP] REASM-KAT OK\n");
    }
    if bits & 64 != 0 {
        mark(b"[TCP] CC-KAT OK\n");
    }
}

pub fn connect_errno(port: u32, dst: [u8; 4], dport: u16) -> Option<(u16, u32)> {
    let mut body = [0u8; 6];
    body[0..4].copy_from_slice(&dst);
    body[4..6].copy_from_slice(&dport.to_le_bytes());
    let mut resp = [0u8; client::HDR_LEN + 4];
    let (errno, n) = client::call(port, 3, &body, &mut resp)?;
    let handle = if n >= 4 {
        u32::from_le_bytes([
            resp[client::HDR_LEN],
            resp[client::HDR_LEN + 1],
            resp[client::HDR_LEN + 2],
            resp[client::HDR_LEN + 3],
        ])
    } else {
        0
    };
    Some((errno, handle))
}

pub fn connect(port: u32, dst: [u8; 4], dport: u16) -> Option<u32> {
    connect_errno(port, dst, dport).and_then(|(e, h)| if e == E_OK { Some(h) } else { None })
}

pub fn recv(port: u32, handle: u32, buf: &mut [u8]) -> Option<(u16, usize)> {
    let mut body = [0u8; 4];
    body.copy_from_slice(&handle.to_le_bytes());
    client::call(port, 6, &body, buf)
}

pub fn state(port: u32, handle: u32) -> Option<(u16, u8)> {
    let mut body = [0u8; 4];
    body.copy_from_slice(&handle.to_le_bytes());
    let mut resp = [0u8; client::HDR_LEN + 1];
    let (errno, n) = client::call(port, 9, &body, &mut resp)?;
    let st = if n >= 1 { resp[client::HDR_LEN] } else { 0 };
    Some((errno, st))
}

pub fn close(port: u32, handle: u32) -> bool {
    let mut b = [0u8; 4];
    b.copy_from_slice(&handle.to_le_bytes());
    let mut resp = [0u8; client::HDR_LEN];
    client::call(port, 7, &b, &mut resp) == Some((E_OK, 0))
}

fn handle_at(resp: &[u8], n: usize) -> Option<u32> {
    if n < 4 {
        return None;
    }
    Some(u32::from_le_bytes([
        resp[client::HDR_LEN],
        resp[client::HDR_LEN + 1],
        resp[client::HDR_LEN + 2],
        resp[client::HDR_LEN + 3],
    ]))
}

pub fn listen(port: u32, listen_port: u16) -> Option<u32> {
    let body = listen_port.to_le_bytes();
    let mut resp = [0u8; client::HDR_LEN + 4];
    let (errno, n) = client::call(port, 2, &body, &mut resp)?;
    if errno != E_OK {
        return None;
    }
    handle_at(&resp, n)
}

pub fn accept(port: u32, listener: u32) -> Option<u32> {
    let body = listener.to_le_bytes();
    let mut resp = [0u8; client::HDR_LEN + 4];
    let (errno, n) = client::call(port, 4, &body, &mut resp)?;
    if errno != E_OK {
        return None;
    }
    handle_at(&resp, n)
}
