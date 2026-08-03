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
//! Reading one packet.

use super::super::error::WireError;
use super::hex;

/// One packet off the wire.
pub enum Pkt<'a> {
    /// A data packet, without its length header.
    Data(&'a [u8]),
    /// The flush packet, `0000`, which ends a section.
    Flush,
}

/// Read the packet at the start of `input`, returning it and its total width
/// so the caller can advance. The length counts its own four bytes, so a data
/// packet of n bytes reads as n plus four.
pub fn read_pkt(input: &[u8]) -> Result<(Pkt<'_>, usize), WireError> {
    if input.len() < 4 {
        return Err(WireError::Truncated);
    }
    let len = hex::parse(&input[..4])?;
    if len == 0 {
        return Ok((Pkt::Flush, 4));
    }
    // 1 and 2 are the delimiter and response-end packets: neither carries data
    // and neither appears in a fetch response this reads.
    if len < 4 || len > input.len() {
        return Err(WireError::Truncated);
    }
    Ok((Pkt::Data(&input[4..len]), len))
}
