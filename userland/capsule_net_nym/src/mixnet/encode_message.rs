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
use crate::message::prepare;
use crate::payload::build_payload;
use crate::sphinx::constants::DESTINATION_ADDRESS_LENGTH;

/// Everything a message needs that is not the message.
pub struct Addressed<'a> {
    /// Where the packet goes, and what the message inside it is sealed to.
    pub destination: &'a [u8; DESTINATION_ADDRESS_LENGTH],
    pub destination_encryption: &'a [u8; 32],
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
    let mut set_seed = [0u8; 4];
    fill_random(&mut set_seed).ok()?;
    // The top bit is the header's own marker, so the id stays below it.
    let set_id = i32::from_be_bytes(set_seed) & 0x7fff_ffff;

    let prepared = prepare(addressed.sender_tag, addressed.reply_surbs, request, set_id)?;
    let mut out = Vec::with_capacity(prepared.fragments.len());

    for (index, fragment) in prepared.fragments.iter().enumerate() {
        let mut frag_id = [0u8; FRAG_ID_BYTES];
        frag_id[..4].copy_from_slice(&set_id.to_be_bytes());
        frag_id[4] = (index + 1) as u8;

        let home_delays = hop_delays_for(addressed.home.len())?;
        let ack = build_surb_ack(
            addressed.home,
            &home_delays,
            addressed.our_identity,
            addressed.ack_key,
            frag_id,
        )
        .ok()?;

        let payload = build_payload(&ack, addressed.destination_encryption, fragment).ok()?;
        out.push(seal_one(addressed.destination, &payload)?);
    }
    Some(out)
}
