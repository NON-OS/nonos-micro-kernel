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

use super::parse;
use super::send;
use super::types::FrameKind;
use crate::tcp_client;

pub fn recv_binary(tcp_port: u32, stream: u32, out: &mut [u8]) -> Result<usize, u16> {
    let mut buf = Vec::with_capacity(out.len() + 32);
    let mut chunk = [0u8; 1536];
    let mut ctrl = [0u8; 125];
    for _ in 0..16 {
        let n = tcp_client::recv(tcp_port, stream, &mut chunk)?;
        if n == 0 {
            continue;
        }
        buf.extend_from_slice(&chunk[..n]);
        while let Some(frame) = parse::next(&buf, out, &mut ctrl)? {
            match frame.kind {
                FrameKind::Binary | FrameKind::Text => return Ok(frame.len),
                FrameKind::Ping => send::send_pong(tcp_port, stream, &ctrl[..frame.len])?,
                FrameKind::Pong => {}
                FrameKind::Close => return Err(9),
            }
            buf.drain(0..frame.consumed);
        }
    }
    Err(8)
}
