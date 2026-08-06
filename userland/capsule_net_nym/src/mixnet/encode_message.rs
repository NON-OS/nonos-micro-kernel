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

use super::seal::{hop_delays_for, seal_one};
use crate::ack::{build_surb_ack, FRAG_ID_BYTES};
use crate::crypto::random::fill_random;
use crate::message::prepare_built;
use crate::payload::build_payload;
use crate::sphinx::constants::DESTINATION_ADDRESS_LENGTH;

/// Everything a message needs that is not the message.
pub struct Addressed<'a> {
    /// Where the packet goes, and what the message inside it is sealed to.
    pub destination: &'a [u8; DESTINATION_ADDRESS_LENGTH],
    pub destination_encryption: &'a [u8; 32],
    /// The gateway the recipient is reachable through. The route out has to
    /// end there: no other node holds a session with it, and one that does
    /// not will still answer every acknowledgement while dropping the
    /// message.
    pub destination_gateway: &'a [u8; 32],
    /// Our own address, for the acknowledgements that come back.
    pub our_identity: &'a [u8; DESTINATION_ADDRESS_LENGTH],
    pub ack_key: &'a [u8; 16],
    /// The route acknowledgements travel home by.
    pub home: &'a [crate::sphinx::node::Node],
    pub sender_tag: &'a [u8; 16],
    pub reply_surbs: &'a [Vec<u8>],
}

/// Turn one request into the packets that carry it.
///
/// A request is not a packet. It becomes a message that says who may reply
/// and hands over the routes to reply by, is padded so its size says nothing,
/// then split across as many packets as it needs. Each of those carries its
/// own acknowledgement and its own key agreement, so two packets of the same
/// message share nothing an observer could group them by.
pub fn encode_message(addressed: &Addressed<'_>, request: &[u8]) -> Option<Vec<Vec<u8>>> {
    let message =
        crate::message::repliable_data(addressed.sender_tag, addressed.reply_surbs, request);
    encode_built(addressed, message)
}

/// Turn a message that is already built into the packets that carry it.
///
/// A request is one kind of message. A top up of reply blocks is another, and
/// travels identically: same padding, same splitting, same per packet
/// acknowledgement and key agreement. Only the building differs.
pub fn encode_built(addressed: &Addressed<'_>, message: Vec<u8>) -> Option<Vec<Vec<u8>>> {
    let mut set_seed = [0u8; 4];
    fill_random(&mut set_seed).ok()?;
    // The top bit is the header's own marker, so the id stays below it.
    let set_id = i32::from_be_bytes(set_seed) & 0x7fff_ffff;

    let Some(prepared) = prepare_built(message, set_id) else {
        crate::trace::say(b"build: could not split the message into packets");
        return None;
    };
    crate::trace::say_num(b"build: fragments", prepared.fragments.len() as u64);
    let mut out = Vec::with_capacity(prepared.fragments.len());

    for (index, fragment) in prepared.fragments.iter().enumerate() {
        let mut frag_id = [0u8; FRAG_ID_BYTES];
        frag_id[..4].copy_from_slice(&set_id.to_be_bytes());
        frag_id[4] = (index + 1) as u8;

        let Some(home_delays) = hop_delays_for(addressed.home.len()) else {
            crate::trace::say(b"build: no delays for the route home");
            return None;
        };
        let ack = match build_surb_ack(
            addressed.home,
            &home_delays,
            addressed.our_identity,
            addressed.ack_key,
            frag_id,
        ) {
            Ok(ack) => ack,
            Err(_) => {
                crate::trace::say(b"build: could not build the acknowledgement");
                return None;
            }
        };

        let Ok(payload) = build_payload(&ack, addressed.destination_encryption, fragment) else {
            crate::trace::say(b"build: could not seal the payload");
            return None;
        };
        let Some(packet) = seal_one(addressed.destination, addressed.destination_gateway, &payload)
        else {
            crate::trace::say_num(b"build: could not seal the packet, bytes", payload.len() as u64);
            return None;
        };
        out.push(packet);
    }
    Some(out)
}
