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

use super::build::build_surb;
use super::bytes::surb_bytes;
use super::keys::remember;
use crate::crypto::random::fill_random;
use crate::mixnet::{route_home, seal::hop_delays_for};
use crate::sphinx::constants::DESTINATION_ADDRESS_LENGTH;

/// How many reply blocks a request carries.
///
/// Each one is a single use route home, so this is a budget for how much the
/// far end may say back before it has to ask for more. Too few and a reply
/// larger than the budget cannot be delivered at all; too many and every
/// request pays for capacity it will not use, in packets that all have to be
/// built and sent.
pub const SURBS_PER_REQUEST: usize = 8;

/// Build the reply blocks that travel with a request.
///
/// Each block gets its own route, so the far end cannot tell from two blocks
/// that they lead to the same place. The keys are kept here because a reply
/// arrives sealed under one of them and there is nothing in the packet that
/// says which; matching is by trying what we hold.
pub fn build_supply(
    gateway_identity: &[u8; 32],
    our_identity: &[u8; DESTINATION_ADDRESS_LENGTH],
) -> Option<Vec<Vec<u8>>> {
    let mut out = Vec::with_capacity(SURBS_PER_REQUEST);
    for _ in 0..SURBS_PER_REQUEST {
        let mut seed = [0u8; 32];
        fill_random(&mut seed).ok()?;
        let home = route_home(&seed, gateway_identity)?;
        let delays = hop_delays_for(home.len())?;
        let surb = build_surb(&home, &delays, our_identity).ok()?;
        out.push(surb_bytes(&surb));
        remember(surb.key);
    }
    Some(out)
}
