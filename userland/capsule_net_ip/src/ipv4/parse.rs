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

use super::checksum;
use super::header::{Ipv4Header, HDR_LEN_MIN, VERSION_4};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    TooShort,
    BadVersion,
    BadIhl,
    TotalLengthMismatch,
    BadChecksum,
}

// Validate and parse an IPv4 packet. On success returns the header
// view (src, dst, protocol) and a slice of the payload (header
// trimmed to declared length, not raw frame length — caller must
// already have stripped any L2 padding). Options-bearing packets
// parse cleanly; we just don't decode the options.
pub fn parse(bytes: &[u8]) -> Result<(Ipv4Header, &[u8]), ParseError> {
    if bytes.len() < HDR_LEN_MIN {
        return Err(ParseError::TooShort);
    }
    let v = bytes[0] >> 4;
    if v != VERSION_4 {
        return Err(ParseError::BadVersion);
    }
    let ihl_words = bytes[0] & 0x0F;
    if (ihl_words as usize) < 5 {
        return Err(ParseError::BadIhl);
    }
    let header_len = (ihl_words as usize) * 4;
    if bytes.len() < header_len {
        return Err(ParseError::BadIhl);
    }
    let total_length = u16::from_be_bytes([bytes[2], bytes[3]]);
    if (total_length as usize) < header_len || (total_length as usize) > bytes.len() {
        return Err(ParseError::TotalLengthMismatch);
    }
    if checksum::fold(&bytes[..header_len]) != 0 {
        return Err(ParseError::BadChecksum);
    }
    let protocol = bytes[9];
    let mut src = [0u8; 4];
    let mut dst = [0u8; 4];
    src.copy_from_slice(&bytes[12..16]);
    dst.copy_from_slice(&bytes[16..20]);
    Ok((Ipv4Header { protocol, src, dst }, &bytes[header_len..total_length as usize]))
}
