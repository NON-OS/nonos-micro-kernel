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

use alloc::vec;

use super::envelope::call;

const OP_CONNECT: u16 = 3;
const OP_SEND: u16 = 5;
const OP_RECV: u16 = 6;
const OP_CLOSE: u16 = 7;
const SEGMENT_MAX: usize = 1460;
/// net.tcp took fewer bytes than were offered.
const E_SHORT_WRITE: u16 = 5;

pub fn connect(port: u32, ip: [u8; 4], dst_port: u16) -> Result<u32, u16> {
    let mut body = [0u8; 6];
    body[0..4].copy_from_slice(&ip);
    body[4..6].copy_from_slice(&dst_port.to_le_bytes());
    let mut out = [0u8; 4];
    if call(port, OP_CONNECT, &body, &mut out)? != 4 {
        return Err(4);
    }
    Ok(u32::from_le_bytes(out))
}

pub fn send_all(port: u32, handle: u32, payload: &[u8]) -> Result<(), u16> {
    for chunk in payload.chunks(SEGMENT_MAX) {
        send_chunk(port, handle, chunk)?;
    }
    Ok(())
}

pub fn recv(port: u32, handle: u32, out: &mut [u8]) -> Result<usize, u16> {
    call(port, OP_RECV, &handle.to_le_bytes(), out)
}

pub fn close(port: u32, handle: u32) -> Result<(), u16> {
    call(port, OP_CLOSE, &handle.to_le_bytes(), &mut []).map(|_| ())
}

fn send_chunk(port: u32, handle: u32, chunk: &[u8]) -> Result<(), u16> {
    let mut body = vec![0u8; 4 + chunk.len()];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..].copy_from_slice(chunk);
    // net.tcp answers a send with the count it accepted. Room has to be made
    // for it, or a short reply buffer turns a delivered write into an error.
    let mut out = [0u8; 4];
    if call(port, OP_SEND, &body, &mut out)? != 4 {
        return Err(4);
    }
    // A partial write would leave the peer reading a truncated request, so it
    // is reported rather than passed off as success.
    if u32::from_le_bytes(out) as usize != chunk.len() {
        return Err(E_SHORT_WRITE);
    }
    Ok(())
}
