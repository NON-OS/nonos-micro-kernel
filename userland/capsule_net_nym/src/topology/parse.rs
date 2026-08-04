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

use super::clock;
use super::directory::{DirectoryMeta, ParsedDirectory, Provenance};
use super::layout;
use super::node;
use super::store;
use super::types::{TopologyError, DIR_HEADER_LEN, DIR_MAGIC, DIR_VERSION, NODE_WIRE_LEN};
use super::verify;

pub fn install(body: &[u8]) -> Result<(), TopologyError> {
    let now = clock::now_ms()?;
    let parsed = parse(body, now)?;
    store::replace(parsed, now)
}

fn parse(body: &[u8], now: u64) -> Result<ParsedDirectory, TopologyError> {
    let count = layout::check_len(body)?;
    check_header(body, now)?;
    verify::check(body)?;
    let mut nodes = alloc::vec::Vec::with_capacity(count);
    for chunk in body[DIR_HEADER_LEN..].chunks(NODE_WIRE_LEN) {
        let Some(parsed) = node::parse(chunk) else {
            return Err(TopologyError::BadLength);
        };
        nodes.push(parsed);
    }
    Ok(ParsedDirectory { meta: meta(body), nodes })
}

fn check_header(body: &[u8], now: u64) -> Result<(), TopologyError> {
    if &body[..4] != DIR_MAGIC.as_ref() {
        return Err(TopologyError::BadMagic);
    }
    if body[4] != DIR_VERSION {
        return Err(TopologyError::BadVersion);
    }
    if body[5] != 0 {
        return Err(TopologyError::BadVersion);
    }
    let not_before = layout::u64_at(body, 16);
    let not_after = layout::u64_at(body, 24);
    if not_after <= not_before || now < not_before || now >= not_after {
        return Err(TopologyError::BadTime);
    }
    Ok(())
}

fn meta(body: &[u8]) -> DirectoryMeta {
    let mut issuer = [0u8; 32];
    issuer.copy_from_slice(&body[32..64]);
    DirectoryMeta {
        epoch: layout::u64_at(body, 8),
        not_before_ms: layout::u64_at(body, 16),
        not_after_ms: layout::u64_at(body, 24),
        issuer,
        provenance: Provenance::Signed,
    }
}
