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

use super::checksum::compute;
use super::header::{UdpHeader, HDR_LEN};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    TooShort,
    LengthMismatch,
    BadChecksum,
}

// Validate the UDP segment and return the parsed header plus the
// payload slice. The pseudo-header is reconstructed from the
// caller-supplied IPv4 addresses; the checksum is verified unless
// the wire value is zero, which RFC 768 reserves as "no checksum".
pub fn parse<'a>(
    src: &[u8; 4],
    dst: &[u8; 4],
    segment: &'a [u8],
) -> Result<(UdpHeader, &'a [u8]), ParseError> {
    if segment.len() < HDR_LEN {
        return Err(ParseError::TooShort);
    }
    let src_port = u16::from_be_bytes([segment[0], segment[1]]);
    let dst_port = u16::from_be_bytes([segment[2], segment[3]]);
    let length = u16::from_be_bytes([segment[4], segment[5]]) as usize;
    let checksum = u16::from_be_bytes([segment[6], segment[7]]);
    if length < HDR_LEN || length > segment.len() {
        return Err(ParseError::LengthMismatch);
    }
    if checksum != 0 {
        let observed = compute(src, dst, &segment[..length]);
        if observed != 0xFFFF && observed != 0 {
            return Err(ParseError::BadChecksum);
        }
    }
    Ok((UdpHeader { src_port, dst_port }, &segment[HDR_LEN..length]))
}
