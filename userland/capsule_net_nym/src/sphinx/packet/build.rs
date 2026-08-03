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

use super::types::SphinxPacket;
use crate::crypto::types::CryptoError;
use crate::sphinx::constants::VERSION_LENGTH;
use crate::sphinx::header::build_header;
use crate::sphinx::node::{Destination, Node};
use crate::sphinx::payload::{pad_payload, seal_payload};

/// Build a complete packet: header first, because laying the payload layers
/// needs the per-hop keys the header derivation produces.
pub fn build_packet(
    initial_secret: &[u8; 32],
    route: &[Node],
    destination: &Destination,
    delays: &[[u8; 8]],
    version: [u8; VERSION_LENGTH],
    message: &[u8],
) -> Result<SphinxPacket, CryptoError> {
    let built = build_header(initial_secret, route, destination, delays, version)?;
    let mut payload = pad_payload(message).ok_or(CryptoError::Kdf)?;
    seal_payload(&mut payload, &built.payload_keys).ok_or(CryptoError::Kdf)?;
    Ok(SphinxPacket { header: built.header, payload })
}
