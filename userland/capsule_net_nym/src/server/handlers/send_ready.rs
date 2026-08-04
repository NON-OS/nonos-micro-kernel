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
        return Err(E_CRYPTO);
    }
    let identity = client_identity().ok_or(E_CRYPTO)?;
    let key = ack_key().ok_or(E_CRYPTO)?;

    // Reply blocks and acknowledgements both need a route ending at the
    // gateway holding our session, which needs the directory's record for it.
    let gateway_identity = session.gateway.identity;
    let reply_surbs = build_supply(&gateway_identity, &identity.public).ok_or(E_NO_ROUTE)?;

    let mut seed = [0u8; 32];
    fill_random(&mut seed).map_err(|_| E_CRYPTO)?;
    let home = route_home(&seed, &gateway_identity).ok_or(E_NO_ROUTE)?;
    hop_delays_for(home.len()).ok_or(E_NO_ROUTE)?;

    Ok(Ready { identity: identity.public, ack_key: key, home, reply_surbs })
}
