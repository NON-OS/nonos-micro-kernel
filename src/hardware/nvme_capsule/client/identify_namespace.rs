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

use super::super::capability::gate_call;
use super::super::error::DriverNvmeError;
use super::super::protocol::{
    encode_request, IDENTIFY_NAMESPACE_PAYLOAD_LEN, OP_IDENTIFY_NAMESPACE,
};
use super::read::{u16_at, u32_at, u64_at};
use super::seq::next_request_id;
use super::status_map::lift;
use super::transport::round_trip;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NvmeNamespaceIdentity {
    pub nsid: u32,
    pub size_lba: u64,
    pub capacity_lba: u64,
    pub used_lba: u64,
    pub lba_size: u32,
    pub metadata_size: u16,
    pub format_index: u8,
    pub formatted_lba_count: u8,
}

pub fn identify_namespace() -> Result<NvmeNamespaceIdentity, DriverNvmeError> {
    let _caller = gate_call()?;
    let request_id = next_request_id();
    let frame = encode_request(OP_IDENTIFY_NAMESPACE, 0, request_id, &[]);
    let resp = round_trip(request_id, frame)?;
    if resp.status != 0 {
        return Err(lift(resp.status));
    }
    if resp.body.len() < IDENTIFY_NAMESPACE_PAYLOAD_LEN {
        return Err(DriverNvmeError::ProtocolMismatch);
    }
    Ok(NvmeNamespaceIdentity {
        nsid: u32_at(&resp.body, 0),
        size_lba: u64_at(&resp.body, 4),
        capacity_lba: u64_at(&resp.body, 12),
        used_lba: u64_at(&resp.body, 20),
        lba_size: u32_at(&resp.body, 28),
        metadata_size: u16_at(&resp.body, 32),
        format_index: resp.body[34],
        formatted_lba_count: resp.body[35],
    })
}
