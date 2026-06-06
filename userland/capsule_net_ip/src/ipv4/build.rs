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

use super::addr::Ipv4Addr;
use super::checksum::seal_at;
use super::header::{CHECKSUM_OFFSET, DEFAULT_TTL, HDR_LEN_MIN, VERSION_4};

#[derive(Clone, Copy)]
pub struct BuildRequest<'a> {
    pub src: Ipv4Addr,
    pub dst: Ipv4Addr,
    pub protocol: u8,
    pub identification: u16,
    pub ttl: u8,
    pub payload: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    OutputTooSmall,
    PayloadTooLarge,
}

// Write a 20-byte IPv4 header followed by the payload into `out`.
// Returns the total wire length on success. The checksum is sealed
// over the resulting header before return.
pub fn build(req: &BuildRequest<'_>, out: &mut [u8]) -> Result<usize, BuildError> {
    let total_len = HDR_LEN_MIN + req.payload.len();
    if total_len > u16::MAX as usize {
        return Err(BuildError::PayloadTooLarge);
    }
    if out.len() < total_len {
        return Err(BuildError::OutputTooSmall);
    }
    out[0] = (VERSION_4 << 4) | 5;
    out[1] = 0;
    let total = (total_len as u16).to_be_bytes();
    out[2] = total[0];
    out[3] = total[1];
    let id = req.identification.to_be_bytes();
    out[4] = id[0];
    out[5] = id[1];
    out[6] = 0x40; // Don't Fragment
    out[7] = 0;
    out[8] = if req.ttl == 0 { DEFAULT_TTL } else { req.ttl };
    out[9] = req.protocol;
    out[10] = 0;
    out[11] = 0;
    out[12..16].copy_from_slice(&req.src);
    out[16..20].copy_from_slice(&req.dst);
    out[20..total_len].copy_from_slice(req.payload);
    seal_at(&mut out[..HDR_LEN_MIN], CHECKSUM_OFFSET);
    Ok(total_len)
}
