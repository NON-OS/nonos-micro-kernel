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

use nonos_tls::{Io, SessionError};

use super::budget::Budget;
use crate::tcp_client;

/// The gateway transport, seen as the byte stream a TLS session drives.
///
/// The directory is fetched over the same `net.tcp` client that reaches
/// gateways, which is already proven against real peers. Nothing about the
/// fetch is anonymous: it happens before there is a mixnet to be anonymous
/// over, which is the bootstrap problem every mixnet client has.
pub struct TcpIo {
    pub tcp_port: u32,
    pub stream: u32,
    budget: Budget,
}

impl TcpIo {
    pub fn new(tcp_port: u32, stream: u32) -> Self {
        Self { tcp_port, stream, budget: Budget::new() }
    }

    pub fn overran(&self) -> bool {
        self.budget.overran()
    }
}

impl Io for TcpIo {
    fn write_all(&mut self, data: &[u8]) -> Result<(), SessionError> {
        if self.budget.spent() {
            return Err(SessionError::Io);
        }
        tcp_client::send_all(self.tcp_port, self.stream, data).map_err(|_| SessionError::Io)
    }

    fn read(&mut self, into: &mut [u8]) -> Result<usize, SessionError> {
        // A peer with nothing to say reads as zero bytes, not as an error, so
        // a session waiting on a handshake that never comes keeps asking. The
        // budget is what ends that, rather than the session's own bound.
        if self.budget.spent() {
            return Err(SessionError::Io);
        }
        tcp_client::recv(self.tcp_port, self.stream, into).map_err(|_| SessionError::Io)
    }
}
