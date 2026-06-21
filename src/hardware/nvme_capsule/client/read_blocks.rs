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

use alloc::vec::Vec;

use super::super::capability::gate_call;
use super::super::error::DriverNvmeError;
use super::super::protocol::{encode_request, MAX_RW_PAYLOAD_BYTES, OP_READ_BLOCKS, SECTOR_SIZE};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

pub fn read_blocks(lba: u64, out: &mut [u8]) -> Result<(), DriverNvmeError> {
    let _caller = gate_call()?;
    if out.is_empty() || out.len() % SECTOR_SIZE != 0 {
        return Err(DriverNvmeError::InvalidArgument);
    }
    if out.len() > MAX_RW_PAYLOAD_BYTES as usize {
        return Err(DriverNvmeError::OversizedRequest);
    }
    let nsectors = (out.len() / SECTOR_SIZE) as u32;
    let mut body: Vec<u8> = Vec::with_capacity(12);
    body.extend_from_slice(&lba.to_le_bytes());
    body.extend_from_slice(&nsectors.to_le_bytes());
    let request_id = next_request_id();
    let frame = encode_request(OP_READ_BLOCKS, 0, request_id, &body);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    if resp.body.len() != out.len() {
        return Err(DriverNvmeError::ProtocolMismatch);
    }
    out.copy_from_slice(&resp.body);
    Ok(())
}
