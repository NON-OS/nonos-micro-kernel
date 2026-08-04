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

extern crate alloc;

use alloc::vec::Vec;

use crate::crypto::fill_random;
use crate::tcp_client;

pub fn send_binary(tcp_port: u32, stream: u32, payload: &[u8]) -> Result<(), u16> {
    send_frame(tcp_port, stream, 0x82, payload)
}

pub fn send_text(tcp_port: u32, stream: u32, payload: &[u8]) -> Result<(), u16> {
    send_frame(tcp_port, stream, 0x81, payload)
}

pub fn send_close(tcp_port: u32, stream: u32) -> Result<(), u16> {
    send_frame(tcp_port, stream, 0x88, &[])
}

pub fn send_pong(tcp_port: u32, stream: u32, payload: &[u8]) -> Result<(), u16> {
    send_frame(tcp_port, stream, 0x8a, payload)
}

/// A ping, which keeps a link that carries nothing between packets from being
/// closed as idle. A gateway that drops the connection takes the session with
/// it, and the client finds out only by having a packet refused.
pub fn send_ping(tcp_port: u32, stream: u32) -> Result<(), u16> {
    send_frame(tcp_port, stream, 0x89, &[])
}

fn send_frame(tcp_port: u32, stream: u32, op: u8, payload: &[u8]) -> Result<(), u16> {
    if payload.len() > u16::MAX as usize {
        return Err(4);
    }
    let mut mask = [0u8; 4];
    fill_random(&mut mask).map_err(|_| 9u16)?;
    let mut frame = Vec::with_capacity(payload.len() + 8);
    frame.push(op);
    write_len(&mut frame, payload.len());
    frame.extend_from_slice(&mask);
    for i in 0..payload.len() {
        frame.push(payload[i] ^ mask[i % 4]);
    }
    tcp_client::send_all(tcp_port, stream, &frame)
}

fn write_len(frame: &mut Vec<u8>, len: usize) {
    if len < 126 {
        frame.push(0x80 | len as u8);
    } else {
        frame.push(0x80 | 126);
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    }
}
