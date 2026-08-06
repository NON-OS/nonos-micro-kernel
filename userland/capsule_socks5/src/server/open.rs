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

use crate::conn::Dest;
use crate::nym::{connect_request, open_session, SendError};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OpenOutcome {
    Opened,
    NoRoute,
}

/// Ask the mixnet to open a tunnel to `dest`.
///
/// Returns whether the request left. It does not wait for the exit to answer:
/// a mixnet round trip is deliberately slow, and holding the SOCKS handshake
/// open across it would stall the client on a path built to add delay.
pub fn open_tunnel(conn_id: u64, dest: &Dest) -> OpenOutcome {
    super::trace::destination(dest);
    if open_session().is_none() {
        super::trace::open_failed(b"no session", 0);
        return OpenOutcome::NoRoute;
    }
    match connect_request(conn_id, dest) {
        Ok(frame) => match crate::nym::send_through_mixnet(&frame) {
            Ok(()) => OpenOutcome::Opened,
            Err(SendError::Remote(code)) => {
                super::trace::open_failed(b"send", code);
                OpenOutcome::NoRoute
            }
            Err(_) => {
                super::trace::open_failed(b"send", 0);
                OpenOutcome::NoRoute
            }
        },
        Err(SendError::NoExit) => {
            super::trace::open_failed(b"no exit", 0);
            OpenOutcome::NoRoute
        }
        Err(_) => {
            super::trace::open_failed(b"request", 0);
            OpenOutcome::NoRoute
        }
    }
}
