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

use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HandshakeError {
    Crypto,
    Transport,
    Malformed,
    Refused,
    BadSignature,
}

/// The socket, as the handshake needs to see it.
pub trait Wire {
    fn send_text(&mut self, text: &str) -> Result<(), HandshakeError>;
    fn recv_text(&mut self) -> Result<Vec<u8>, HandshakeError>;
}

/// The keys the exchange is anchored on.
pub struct Identity<'a> {
    pub own_seed: &'a [u8; 32],
    pub own_public: &'a [u8; 32],
    pub gateway_public: &'a [u8; 32],
}
