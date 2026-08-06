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

use nonos_libc::{mk_uptime_ms, mk_yield};

use super::carry;
use super::parse;
use super::send;
use super::types::FrameKind;
use crate::tcp_client;

/// Nothing arrived inside the caller's budget. Not a failure of the link.
pub const E_TIMEOUT: u16 = 8;
/// The peer sent a close frame, so the session is over.
pub const E_CLOSED: u16 = 9;

/// One frame off the link: its bytes, and whether the gateway sent it as
/// text. The two are not interchangeable. A binary frame is an encrypted
/// blob under the session key; a text frame is a control message in the
/// clear, and running one through the other's parser only ever fails.
pub struct Frame {
    pub len: usize,
    pub text: bool,
}

pub fn recv_binary(tcp_port: u32, stream: u32, out: &mut [u8], wait_ms: i64) -> Result<Frame, u16> {
    let mut buf = carry::take(stream);
    let mut chunk = [0u8; 1536];
    let mut ctrl = [0u8; 125];
    let deadline = mk_uptime_ms().saturating_add(wait_ms);
    loop {
        // Whatever is already held is parsed before the socket is read again,
        // so a burst is drained frame by frame across successive calls.
        while let Some(frame) = parse::next(&buf, out, &mut ctrl)? {
            match frame.kind {
                FrameKind::Binary | FrameKind::Text => {
                    carry::keep(stream, &buf[frame.consumed..]);
                    return Ok(Frame { len: frame.len, text: frame.kind == FrameKind::Text });
                }
                FrameKind::Ping => send::send_pong(tcp_port, stream, &ctrl[..frame.len])?,
                FrameKind::Pong => {}
                FrameKind::Close => {
                    carry::keep(stream, &[]);
                    return Err(E_CLOSED);
                }
            }
            buf.drain(0..frame.consumed);
        }
        let n = tcp_client::recv(tcp_port, stream, &mut chunk)?;
        if n == 0 {
            if mk_uptime_ms() >= deadline {
                carry::keep(stream, &buf);
                return Err(E_TIMEOUT);
            }
            mk_yield();
            continue;
        }
        buf.extend_from_slice(&chunk[..n]);
    }
}
