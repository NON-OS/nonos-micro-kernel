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

use super::wire::{HandshakeError, Wire};
use crate::gateway_client::ws;
use alloc::vec;
use alloc::vec::Vec;

/// Largest handshake frame worth accepting: the longest legitimate one is the
/// gateway's 124-byte material as a JSON number array.
const MAX_FRAME: usize = 8192;

/// How long a handshake step waits on its answer. A gateway one round trip
/// away answers in well under this, and a gateway that does not answer at all
/// is one the caller should move on from.
const HANDSHAKE_WAIT_MS: i64 = 5_000;

pub struct WsWire {
    pub tcp_port: u32,
    pub stream: u32,
}

impl Wire for WsWire {
    fn send_text(&mut self, text: &str) -> Result<(), HandshakeError> {
        ws::send_text(self.tcp_port, self.stream, text.as_bytes())
            .map_err(|_| HandshakeError::Transport)
    }

    fn recv_text(&mut self) -> Result<Vec<u8>, HandshakeError> {
        let mut buf = vec![0u8; MAX_FRAME];
        let frame = ws::recv_binary(self.tcp_port, self.stream, &mut buf, HANDSHAKE_WAIT_MS)
            .map_err(|_| HandshakeError::Transport)?;
        buf.truncate(frame.len);
        Ok(buf)
    }
}
