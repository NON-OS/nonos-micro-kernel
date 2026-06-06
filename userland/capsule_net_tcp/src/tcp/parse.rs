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
use super::header::{TcpHeader, HDR_LEN_MIN};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    TooShort,
    BadDataOffset,
    BadChecksum,
}

pub fn parse<'a>(
    src: &[u8; 4],
    dst: &[u8; 4],
    segment: &'a [u8],
) -> Result<(TcpHeader, &'a [u8]), ParseError> {
    if segment.len() < HDR_LEN_MIN {
        return Err(ParseError::TooShort);
    }
    let data_offset_words = segment[12] >> 4;
    if data_offset_words < 5 {
        return Err(ParseError::BadDataOffset);
    }
    let header_len = (data_offset_words as usize) * 4;
    if segment.len() < header_len {
        return Err(ParseError::BadDataOffset);
    }
    if compute(src, dst, segment) != 0 {
        return Err(ParseError::BadChecksum);
    }
    let hdr = TcpHeader {
        src_port: u16::from_be_bytes([segment[0], segment[1]]),
        dst_port: u16::from_be_bytes([segment[2], segment[3]]),
        seq: be32(segment, 4),
        ack: be32(segment, 8),
        flags: segment[13],
    };
    Ok((hdr, &segment[header_len..]))
}

fn be32(buf: &[u8], off: usize) -> u32 {
    u32::from_be_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}
