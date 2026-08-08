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

use super::dest::Dest;
use crate::wire::REPLY_LEN;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Phase {
    Greeting,
    Request,
    Relaying,
    Closed,
}

/// What the serving loop should do after feeding the handshake more bytes.
///
/// The reply variant carries its bytes inline. The buffer is one short fixed
/// reply, and boxing it would allocate on every step of a handshake to save a
/// few bytes of enum width.
#[allow(clippy::large_enum_variant)]
pub enum Event {
    /// Not enough bytes yet; read more from the client.
    NeedMore,
    /// Send `buf[..len]`. Check [`super::Conn::is_closed`] afterwards, since a
    /// rejection replies and then closes.
    ToClient { buf: [u8; REPLY_LEN], len: usize },
    /// Open a tunnel to this destination, then report the result with
    /// [`super::Conn::opened`].
    Open(Dest),
    /// Carry these bytes to the exit and bring back whatever it answers.
    /// The handshake is over, so nothing here is parsed: the stream belongs
    /// to whatever protocol the client and the far end agreed on.
    Relay,
    /// Close without a reply: malformed, or not a SOCKS5 client at all.
    Close,
}
