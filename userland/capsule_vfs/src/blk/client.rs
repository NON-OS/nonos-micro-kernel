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

use alloc::vec;

use super::error::BlkError;
use super::reply::decode_reply;
use super::transport::call;
use super::wire::{
    CAPACITY_BODY_LEN, HDR_LEN, MAX_READ_BYTES, OP_CAPACITY, OP_READ_BLOCKS, RW_REQ_LEN,
    SECTOR_SIZE, STATUS_LEN,
};

pub fn capacity() -> Result<u64, BlkError> {
    let mut rx = [0u8; HDR_LEN + STATUS_LEN + CAPACITY_BODY_LEN];
    let (n, request_id) = call(OP_CAPACITY, &[], &mut rx)?;
    let body = decode_reply(&rx, n, OP_CAPACITY, request_id)?;
    if body.len() < CAPACITY_BODY_LEN {
        return Err(BlkError::BadLength);
    }
    Ok(u64::from_le_bytes([
        body[0], body[1], body[2], body[3], body[4], body[5], body[6], body[7],
    ]))
}

// `out` sizes the request: it must be a non-zero whole number of sectors and no
// larger than the driver's per-request ceiling, since anything else is rejected
// on the far side as E_INVAL or E_MSGSIZE after a pointless round-trip.
pub fn read_blocks(lba: u64, out: &mut [u8]) -> Result<(), BlkError> {
    if out.is_empty() || out.len() % SECTOR_SIZE != 0 || out.len() > MAX_READ_BYTES {
        return Err(BlkError::Inval);
    }
    let nsectors = (out.len() / SECTOR_SIZE) as u32;
    let mut body = [0u8; RW_REQ_LEN];
    body[0..8].copy_from_slice(&lba.to_le_bytes());
    body[8..12].copy_from_slice(&nsectors.to_le_bytes());
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN + out.len()];
    let (n, request_id) = call(OP_READ_BLOCKS, &body, &mut rx)?;
    let payload = decode_reply(&rx, n, OP_READ_BLOCKS, request_id)?;
    if payload.len() != out.len() {
        return Err(BlkError::BadLength);
    }
    out.copy_from_slice(payload);
    Ok(())
}
