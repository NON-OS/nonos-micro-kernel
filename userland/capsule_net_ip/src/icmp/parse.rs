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

use crate::ipv4::fold;

use super::types::{IcmpHeader, HDR_LEN};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    TooShort,
    BadChecksum,
}

// Validate the ICMP checksum over the entire ICMP message (header
// + payload) and return the header plus payload slice on success.
pub fn parse(bytes: &[u8]) -> Result<(IcmpHeader, &[u8]), ParseError> {
    if bytes.len() < HDR_LEN {
        return Err(ParseError::TooShort);
    }
    if fold(bytes) != 0 {
        return Err(ParseError::BadChecksum);
    }
    let icmp_type = bytes[0];
    let code = bytes[1];
    let mut rest = [0u8; 4];
    rest.copy_from_slice(&bytes[4..8]);
    Ok((IcmpHeader { icmp_type, code, rest }, &bytes[HDR_LEN..]))
}
