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

extern crate alloc;

use alloc::vec::Vec;

use super::error::DecodeError;
use super::reader::Reader;
use super::strings::{bounded_count, bounded_string};
use super::{decode_price, decode_release, decode_token};
use crate::limits::{MAX_DESCRIPTION, MAX_NAME, MAX_PUBLISHER, MAX_RELEASES};
use crate::types::{CapsuleRelease, MarketplaceEntry};

pub(super) fn read(r: &mut Reader<'_>) -> Result<MarketplaceEntry, DecodeError> {
    let listing_id = bounded_string(r, MAX_NAME)?;
    let capsule_id = r.fixed::<32>()?;
    let name = bounded_string(r, MAX_NAME)?;
    let publisher_name = bounded_string(r, MAX_PUBLISHER)?;
    let publisher_pubkey = r.fixed::<32>()?;
    let publisher_eth_address = r.fixed::<20>()?;
    let description = bounded_string(r, MAX_DESCRIPTION)?;

    let price = decode_price::read(r)?;
    let token = decode_token::read(r)?;

    let rel_count = bounded_count(r, MAX_RELEASES)?;
    let mut releases: Vec<CapsuleRelease> = Vec::with_capacity(rel_count as usize);
    for _ in 0..rel_count {
        releases.push(decode_release::read(r)?);
    }

    Ok(MarketplaceEntry {
        listing_id,
        capsule_id,
        name,
        publisher_name,
        publisher_pubkey,
        publisher_eth_address,
        description,
        price,
        token,
        releases,
    })
}
