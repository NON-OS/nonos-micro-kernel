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

use crate::gateway_client;
use crate::message::repliable_additional_surbs;
use crate::mixnet::{encode_built, Addressed};
use crate::reply::RECIPIENT_BYTES;
use crate::state::TABLE;
use crate::surb::build_supply;
use crate::trace;

use super::send_ready::ready;

/// Most reply blocks to send in answer to one request.
///
/// The far end names how many it wants and is not always modest about it.
/// Each one costs a route home to build and rides along in the packets that
/// carry it, so a request for hundreds is answered with what a transfer
/// actually needs rather than with what was asked for.
const MOST_AT_ONCE: usize = 40;

/// Send more reply blocks to a recipient that has run low.
///
/// A recipient keeps a reserve it will not spend, and once it is down to that
/// it stops answering and asks instead. Until it is topped up nothing further
/// arrives, which looks exactly like a transfer that stalled: the first part
/// of a page lands and the rest never does.
pub fn top_up(tcp_port: u32, recipient: &[u8; RECIPIENT_BYTES], amount: u32) {
    let mut identity = [0u8; 32];
    let mut encryption = [0u8; 32];
    let mut gateway = [0u8; 32];
    identity.copy_from_slice(&recipient[..32]);
    encryption.copy_from_slice(&recipient[32..64]);
    gateway.copy_from_slice(&recipient[64..96]);

    let wanted = (amount as usize).min(MOST_AT_ONCE);

    // Built while the table is held and sent once it is released. Sending
    // reaches for the table again if the link turns out to be gone, and a
    // session deliberately cannot be copied out, so the work is split either
    // side of the lock rather than the session being moved across it.
    let built = TABLE.lock().with_sphinx_session(|session| {
        let prepared = ready(session).ok()?;
        let surbs = build_supply_of(&session.gateway.identity, &prepared.identity, wanted)?;
        let addressed = Addressed {
            destination: &identity,
            destination_encryption: &encryption,
            destination_gateway: &gateway,
            our_identity: &prepared.identity,
            ack_key: &prepared.ack_key,
            home: &prepared.home,
            sender_tag: &session.sender_tag,
            reply_surbs: &surbs,
        };
        let message = repliable_additional_surbs(&session.sender_tag, &surbs);
        let packets = encode_built(&addressed, message)?;
        Some((session.gateway, surbs.len(), packets))
    });
    let Some(Some((via, sent, packets))) = built else {
        trace::say(b"top up: could not build the reply blocks");
        return;
    };
    trace::say_num(b"top up: sending reply blocks", sent as u64);

    for packet in &packets {
        let Ok(frame) =
            gateway_client::make_encrypted_blob(gateway_client::KIND_FORWARD_SPHINX, packet)
        else {
            return;
        };
        if gateway_client::send(tcp_port, via, &frame).is_err() {
            crate::server::gateway_lost();
            return;
        }
    }
}

/// Reply blocks for a top up, which asks for a count rather than taking the
/// fixed supply a request carries.
fn build_supply_of(
    gateway_identity: &[u8; 32],
    our_identity: &[u8; 32],
    wanted: usize,
) -> Option<Vec<Vec<u8>>> {
    let mut out = Vec::with_capacity(wanted);
    while out.len() < wanted {
        let batch = build_supply(gateway_identity, our_identity)?;
        for surb in batch {
            if out.len() == wanted {
                break;
            }
            out.push(surb);
        }
    }
    Some(out)
}
