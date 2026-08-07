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

use super::event::Phase;
use super::machine::Conn;
use crate::wire::{self, REPLY_LEN};

impl Conn {
    /// Report the tunnel-open result under the reply code the attempt earned:
    /// success replies and relays, anything else replies with the code that
    /// says which step refused, and closes.
    pub fn opened(&mut self, code: u8) -> ([u8; REPLY_LEN], usize) {
        let mut buf = [0u8; REPLY_LEN];
        if code == wire::REP_OK {
            let len = wire::reply(wire::REP_OK, &mut buf);
            return (buf, len);
        }
        self.phase = Phase::Closed;
        let len = wire::reply(code, &mut buf);
        (buf, len)
    }
}
