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
use super::header::{CHECKSUM_OFFSET, HDR_LEN_MIN};

#[derive(Clone, Copy)]
pub struct BuildRequest<'a> {
    pub src: [u8; 4],
    pub dst: [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: u8,
    pub window: u16,
    pub payload: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildError {
    OutputTooSmall,
}

pub fn build(req: &BuildRequest<'_>, out: &mut [u8]) -> Result<usize, BuildError> {
    let total = HDR_LEN_MIN + req.payload.len();
    if out.len() < total {
        return Err(BuildError::OutputTooSmall);
    }
    out[0..2].copy_from_slice(&req.src_port.to_be_bytes());
    out[2..4].copy_from_slice(&req.dst_port.to_be_bytes());
    out[4..8].copy_from_slice(&req.seq.to_be_bytes());
    out[8..12].copy_from_slice(&req.ack.to_be_bytes());
    out[12] = 5 << 4;
    out[13] = req.flags;
    out[14..16].copy_from_slice(&req.window.to_be_bytes());
    out[CHECKSUM_OFFSET] = 0;
    out[CHECKSUM_OFFSET + 1] = 0;
    out[18..20].copy_from_slice(&0u16.to_be_bytes());
    out[HDR_LEN_MIN..total].copy_from_slice(req.payload);
    let cksum = compute(&req.src, &req.dst, &out[..total]);
    out[CHECKSUM_OFFSET..CHECKSUM_OFFSET + 2].copy_from_slice(&cksum.to_be_bytes());
    Ok(total)
}
