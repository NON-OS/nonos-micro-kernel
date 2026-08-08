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

use super::envelope::call;
use super::errno::{E_ERRNO, E_LEN, RX_EMPTY};

const OP_CONNECT: u16 = 3;
const OP_RECV: u16 = 6;
const OP_CLOSE: u16 = 7;

pub fn connect(port: u32, ip: [u8; 4], dst_port: u16) -> Result<u32, u16> {
    let mut body = [0u8; 6];
    body[0..4].copy_from_slice(&ip);
    body[4..6].copy_from_slice(&dst_port.to_le_bytes());
    let mut out = [0u8; 4];
    if call(port, OP_CONNECT, &body, &mut out)? != 4 {
        return Err(E_LEN);
    }
    Ok(u32::from_le_bytes(out))
}

pub fn recv(port: u32, handle: u32, out: &mut [u8]) -> Result<usize, u16> {
    // State how much can be held. A read takes bytes out of the socket, so
    // whatever net.tcp sends back that does not fit here is lost rather than
    // kept for the next call.
    let mut body = [0u8; 8];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..8].copy_from_slice(&(out.len() as u32).to_le_bytes());
    match call(port, OP_RECV, &body, out) {
        // Nothing has arrived yet. That is a state, not a failure: a peer
        // that has not answered is the normal case while waiting on one.
        Err(e) if e == E_ERRNO + RX_EMPTY => Ok(0),
        other => other,
    }
}

pub fn close(port: u32, handle: u32) -> Result<(), u16> {
    call(port, OP_CLOSE, &handle.to_le_bytes(), &mut []).map(|_| ())
}
