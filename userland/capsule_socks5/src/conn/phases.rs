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
use super::event::{Event, Phase};
use super::machine::Conn;
use crate::wire::{self, Parsed, REPLY_LEN};

impl Conn {
    // Method selection: accept no-auth, reject anything else, both with a
    // two-byte reply. A wrong version is not SOCKS5 and just closes.
    pub(super) fn greeting(&mut self) -> Event {
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
                self.phase = if no_auth { Phase::Request } else { Phase::Closed };
                Event::ToClient { buf, len: 2 }
            }
        }
    }

    // CONNECT: open a tunnel to the destination, or reply with the failure
    // code and close.
    pub(super) fn request(&mut self) -> Event {
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
}
