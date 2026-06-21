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

use super::constants::{CQ_BYTES, DATA_BYTES, PRP_LIST_BYTES, SQ_BYTES};
use super::queue::IoQueue;
use crate::constants::REG_DOORBELL_BASE;
use crate::dma::DmaRegion;
use crate::error::NvmeResult;

impl IoQueue {
    pub(super) fn allocate(
        device_id: u64,
        epoch: u64,
        stride: u8,
        qid: u16,
        nsid: u32,
        capacity_sectors: u64,
    ) -> NvmeResult<Self> {
        let stride_bytes = 4u32 << stride;
        Ok(Self {
            sq: DmaRegion::map(device_id, epoch, SQ_BYTES)?,
            cq: DmaRegion::map(device_id, epoch, CQ_BYTES)?,
            prp_list: DmaRegion::map(device_id, epoch, PRP_LIST_BYTES)?,
            data: DmaRegion::map(device_id, epoch, DATA_BYTES)?,
            sq_tail: 0,
            cq_head: 0,
            phase: true,
            cid: 1,
            sq_db: REG_DOORBELL_BASE + (2 * qid as u32) * stride_bytes,
            cq_db: REG_DOORBELL_BASE + (2 * qid as u32 + 1) * stride_bytes,
            nsid,
            capacity_sectors,
        })
    }
}
