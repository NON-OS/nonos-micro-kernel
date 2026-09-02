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

use super::feed::feed;
use super::request::{ask, Ask};
use super::state::reset;
use alloc::vec;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};

/// Largest SOCKS exchange worth buffering.
/// Largest exchange worth buffering from a client. A relayed write can be a
/// whole TLS record, so this is sized for the stream rather than for the
/// handshake that opens it.
const RX_MAX: usize = 34 * 1024;

/// Serve SOCKS clients over IPC.
///
/// The client speaks RFC 1928 as bytes; this capsule is the far end of that
/// conversation and the near end of a mixnet tunnel. Nothing here opens a
/// socket, which is what keeps a clearnet path from existing at all.
pub fn run() -> ! {
    reset();
    let mut rx = vec![0u8; RX_MAX];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), 0, &mut sender);
        if n < 0 || sender == 0 {
            continue;
        }
        // A caller with nothing to say is asking whether the far end has
        // answered yet, which is the only way to collect a reply that arrives
        // seconds after the request. It is a request like any other, and the
        // marker is what lets it be sent at all.
        // An unrecognized ask still gets an answer, for the same reason the
        // comment below gives: the caller is blocked on this reply, and a
        // silent continue here makes it wait out its whole timeout and tear
        // the session down for what was one malformed frame. Close it
        // explicitly instead, so the caller fails fast and reconnects.
        let out = match ask(&rx[..n as usize]) {
            Some(Ask::Stream(body)) => feed(sender, body),
            Some(Ask::Reset) => super::feed::reset_client(sender),
            None => super::feed::reset_client(sender),
        }
        .encode();
        // Every request is answered, including with nothing. A caller blocks
        // on its reply, so staying silent does not mean "no data", it means
        // the caller waits out its whole timeout for an answer already known.
        let _ = mk_ipc_reply(sender, out.as_ptr(), out.len());
    }
}
