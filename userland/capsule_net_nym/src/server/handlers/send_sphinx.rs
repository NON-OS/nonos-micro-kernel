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

use crate::gateway_client;
use crate::mixnet::{encode_message, Addressed};
use crate::protocol::{E_CRYPTO, E_NO_ROUTE, E_NO_TCP, E_OK};
use crate::server::handlers::send_ready::ready;
use crate::state::Session;
use crate::trace;

/// Send a request through the mixnet as a real message.
///
/// Refuses rather than degrades: a message that cannot be answered or that
/// would be linkable is not sent at all.
pub fn send_sphinx(tcp_port: u32, session: &Session, payload: &[u8]) -> u16 {
    let prepared = match ready(session) {
        Ok(prepared) => prepared,
        Err(errno) => return errno,
    };

    let addressed = Addressed {
        destination: &session.dest,
        destination_encryption: &session.dest_encryption,
        destination_gateway: &session.dest_gateway,
        our_identity: &prepared.identity,
        ack_key: &prepared.ack_key,
        home: &prepared.home,
        sender_tag: &session.sender_tag,
        reply_surbs: &prepared.reply_surbs,
    };
    let Some(packets) = encode_message(&addressed, payload) else {
        trace::say_num(b"send refused: could not build packets for bytes", payload.len() as u64);
        return E_NO_ROUTE;
    };
    trace::say_two(b"sending packets for bytes", packets.len() as u64, payload.len() as u64);

    for packet in &packets {
        let Ok(frame) =
            gateway_client::make_encrypted_blob(gateway_client::KIND_FORWARD_SPHINX, packet)
        else {
            trace::say(b"send failed: could not seal frame for gateway");
            return E_CRYPTO;
        };
        if let Err(code) = gateway_client::send(tcp_port, session.gateway, &frame) {
            // Callers see one code for a write that did not land, so the
            // reason is only recoverable from the log. A closed socket and a
            // socket that never drained need different fixes.
            gateway_client::trace::fail(b"send", code);
            crate::server::gateway_lost();
            return E_NO_TCP;
        }
    }
    trace::say(b"sent");
    E_OK
}
