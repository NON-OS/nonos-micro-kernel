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

use crate::crypto::random::fill_random;
use crate::mixnet::{route_home, seal::hop_delays_for};
use crate::protocol::{E_CRYPTO, E_NO_ROUTE};
use crate::sphinx::node::Node;
use crate::state::{ack_key, client_identity, Session};
use crate::surb::build_supply;
use crate::trace;

/// What a message needs before it can be built.
pub struct Ready {
    pub identity: [u8; 32],
    pub ack_key: [u8; 16],
    pub home: Vec<Node>,
    pub reply_surbs: Vec<Vec<u8>>,
}

/// Gather it, or say which piece is missing.
///
/// Each of these is a case where the message could still be put on the wire
/// but would arrive unanswerable or linkable, so it is refused instead. A
/// request that cannot be replied to is not a cheaper request; it is a
/// request whose answer is lost after the exit has already acted on it.
pub fn ready(session: &Session) -> Result<Ready, u16> {
    // Zeros mean the tag was never drawn. Sending it would give every such
    // session the same one, which is the link the tag exists to prevent.
    if session.sender_tag == [0u8; 16] {
        trace::say(b"send refused: no sender tag");
        return Err(E_CRYPTO);
    }
    let Some(identity) = client_identity() else {
        trace::say(b"send refused: no client identity");
        return Err(E_CRYPTO);
    };
    let Some(key) = ack_key() else {
        trace::say(b"send refused: no ack key");
        return Err(E_CRYPTO);
    };

    // Reply blocks and acknowledgements both need a route ending at the
    // gateway holding our session, which needs the directory's record for it.
    let gateway_identity = session.gateway.identity;
    let Some(reply_surbs) = build_supply(&gateway_identity, &identity.public) else {
        trace::say(b"send refused: no reply blocks, gateway not in directory yet");
        return Err(E_NO_ROUTE);
    };
    trace::say_num(b"reply blocks built", reply_surbs.len() as u64);

    let mut seed = [0u8; 32];
    fill_random(&mut seed).map_err(|_| E_CRYPTO)?;
    let Some(home) = route_home(&seed, &gateway_identity) else {
        trace::say(b"send refused: no route home");
        return Err(E_NO_ROUTE);
    };
    if hop_delays_for(home.len()).is_none() {
        trace::say(b"send refused: no delays for route home");
        return Err(E_NO_ROUTE);
    }
    trace::say_num(b"route home hops", home.len() as u64);

    Ok(Ready { identity: identity.public, ack_key: key, home, reply_surbs })
}
