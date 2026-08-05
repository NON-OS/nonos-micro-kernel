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

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::mk_uptime_ms;

use super::state::Server;
use crate::nym::{recv_once, send_through_mixnet, Delivery};
use crate::tunnel::{decode_response, encode_send};

/// How long one call waits on the exit before answering with nothing.
///
/// A mixnet round trip is seconds, far longer than this, and a client that
/// gets nothing simply asks again. Waiting out the whole round trip inside
/// one call would hold the capsule against every other connection for it.
const POLL_MS: i64 = 1_000;

/// Room for the largest client write plus the request that carries it.
const FRAME_MAX: usize = 34 * 1024;

/// Carry `data` to the exit and bring back whatever has come the other way.
///
/// An empty `data` is a read with nothing to send, which is how a client asks
/// whether the far end has answered yet.
pub fn relay(server: &mut Server, pid: u32, data: &[u8]) -> Vec<u8> {
    let Some(conn) = server.manager.id_of_socket(pid) else {
        return Vec::new();
    };
    if !data.is_empty() && !forward(server, conn, data) {
        return Vec::new();
    }
    collect(server, conn)
}

fn forward(server: &mut Server, conn: u64, data: &[u8]) -> bool {
    let Some(seq) = server.manager.take_seq(conn, data.len()) else {
        return false;
    };
    let mut buf = vec![0u8; FRAME_MAX];
    let Some(n) = encode_send(conn, seq, false, data, &mut buf) else {
        return false;
    };
    buf.truncate(n);
    send_through_mixnet(&buf).is_ok()
}

fn collect(server: &mut Server, conn: u64) -> Vec<u8> {
    let deadline = mk_uptime_ms().saturating_add(POLL_MS);
    let mut out = Vec::new();
    loop {
        let gone = match recv_once() {
            Delivery::Message(msg) => {
                take(server, &msg);
                false
            }
            Delivery::Empty => false,
            Delivery::Gone => true,
        };
        let (bytes, closed) = server.inbox.drain(conn);
        out.extend_from_slice(&bytes);
        if closed {
            server.inbox.forget(conn);
            server.manager.close(conn);
            break;
        }
        if gone || !out.is_empty() || mk_uptime_ms() >= deadline {
            break;
        }
    }
    out
}

/// File one delivered message against the connection it names.
///
/// A message that does not decode is dropped rather than guessed at. The
/// mixnet delivers whatever was addressed to us, and arriving is not evidence
/// that it belongs to a connection of ours.
fn take(server: &mut Server, msg: &[u8]) {
    crate::server::trace_reply_bytes(msg.len());
    let Some(response) = decode_response(msg) else {
        // It arrived and was not ours to read. Saying so separates a reply
        // the exit never sent from one sent in a shape we do not speak.
        crate::server::trace_reply_kind(msg);
        return;
    };
    server.inbox.accept(response.conn_id, response.seq, response.closed, response.data);
}
