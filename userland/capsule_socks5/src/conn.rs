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

//! The per-connection SOCKS5 handshake state machine. It accumulates client bytes
//! (which may arrive split across reads), runs the method-selection then the
//! CONNECT exchange with the [`crate::wire`] codec, and tells the serving loop
//! what to do next: read more, send a reply, open a tunnel to a destination, or
//! close. It owns no transport, so it is driven the same way by a real socket and
//! by a proof. Allocation-free: a client cannot make it grow past one greeting or
//! one request.

use crate::wire::{self, Host, Parsed, REPLY_LEN};

/// The most bytes a handshake ever accumulates: a request with a full 255-byte
/// domain name (`ver+cmd+rsv+atyp+len` = 5, domain 255, port 2).
const ACC_MAX: usize = 262;
/// The largest domain name a destination carries.
const DOMAIN_MAX: usize = 255;

/// A destination for the tunnel, copied out of the (transient) request buffer so
/// it outlives the handshake bytes.
pub enum Dest {
    V4([u8; 4], u16),
    V6([u8; 16], u16),
    Domain { name: [u8; DOMAIN_MAX], len: u8, port: u16 },
}

impl Dest {
    fn from_host(host: &Host<'_>, port: u16) -> Self {
        match host {
            Host::V4(a) => Dest::V4(*a, port),
            Host::V6(a) => Dest::V6(*a, port),
            Host::Domain(d) => {
                let mut name = [0u8; DOMAIN_MAX];
                let len = d.len().min(DOMAIN_MAX);
                name[..len].copy_from_slice(&d[..len]);
                Dest::Domain { name, len: len as u8, port }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Greeting,
    Request,
    Relaying,
    Closed,
}

/// What the serving loop should do after feeding the handshake more bytes.
pub enum Event {
    /// Not enough bytes yet; read more from the client.
    NeedMore,
    /// Send `buf[..len]` to the client. Combine with [`Conn::is_closed`] to know
    /// whether to close afterwards (a rejection replies then closes).
    ToClient { buf: [u8; REPLY_LEN], len: usize },
    /// Open a tunnel to this destination, then report the result with
    /// [`Conn::opened`].
    Open(Dest),
    /// Close the connection without a reply (a malformed or non-SOCKS5 client).
    Close,
}

/// A SOCKS5 connection mid-handshake.
pub struct Conn {
    phase: Phase,
    acc: [u8; ACC_MAX],
    len: usize,
}

impl Default for Conn {
    fn default() -> Self {
        Self::new()
    }
}

impl Conn {
    pub fn new() -> Self {
        Self { phase: Phase::Greeting, acc: [0u8; ACC_MAX], len: 0 }
    }

    pub fn is_relaying(&self) -> bool {
        self.phase == Phase::Relaying
    }

    pub fn is_closed(&self) -> bool {
        self.phase == Phase::Closed
    }

    /// Feed handshake bytes from the client and advance. Extra bytes past the
    /// accumulator (a client flooding the handshake) close the connection rather
    /// than grow it.
    pub fn on_client(&mut self, data: &[u8]) -> Event {
        let room = ACC_MAX - self.len;
        if data.len() > room {
            self.phase = Phase::Closed;
            return Event::Close;
        }
        self.acc[self.len..self.len + data.len()].copy_from_slice(data);
        self.len += data.len();

        match self.phase {
            Phase::Greeting => self.greeting(),
            Phase::Request => self.request(),
            Phase::Relaying | Phase::Closed => Event::Close,
        }
    }

    // Method selection: accept no-auth, reject anything else, both with a two-byte
    // reply; a wrong version is not SOCKS5 and just closes.
    fn greeting(&mut self) -> Event {
        match wire::offers_no_auth(&self.acc[..self.len]) {
            None => {
                if self.len >= 1 && self.acc[0] != wire::VER {
                    self.phase = Phase::Closed;
                    return Event::Close;
                }
                Event::NeedMore
            }
            Some(no_auth) => {
                let consumed = 2 + self.acc[1] as usize;
                self.drain(consumed);
                let reply = wire::method_reply(no_auth);
                let mut buf = [0u8; REPLY_LEN];
                buf[..2].copy_from_slice(&reply);
                if no_auth {
                    self.phase = Phase::Request;
                } else {
                    self.phase = Phase::Closed;
                }
                Event::ToClient { buf, len: 2 }
            }
        }
    }

    // CONNECT: open a tunnel to the destination, or reply with the failure code
    // and close.
    fn request(&mut self) -> Event {
        match wire::parse_connect(&self.acc[..self.len]) {
            Parsed::Incomplete => Event::NeedMore,
            Parsed::Rejected(rep) => {
                self.phase = Phase::Closed;
                let mut buf = [0u8; REPLY_LEN];
                let len = wire::reply(rep, &mut buf);
                Event::ToClient { buf, len }
            }
            Parsed::Connect(c) => {
                let dest = Dest::from_host(&c.host, c.port);
                self.phase = Phase::Relaying;
                Event::Open(dest)
            }
        }
    }

    /// Report the tunnel-open result. On success returns the success reply to send
    /// the client, after which the connection relays; on failure returns the
    /// host-unreachable reply and closes.
    pub fn opened(&mut self, ok: bool) -> ([u8; REPLY_LEN], usize) {
        let mut buf = [0u8; REPLY_LEN];
        if ok {
            let len = wire::reply(wire::REP_OK, &mut buf);
            (buf, len)
        } else {
            self.phase = Phase::Closed;
            let len = wire::reply(wire::REP_HOST_UNREACH, &mut buf);
            (buf, len)
        }
    }

    // Drop the first `n` accumulated bytes, keeping any that belong to the next
    // phase (a client may pipeline the request behind the greeting).
    fn drain(&mut self, n: usize) {
        let n = n.min(self.len);
        self.acc.copy_within(n..self.len, 0);
        self.len -= n;
    }
}
