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

use super::constants::{CQ_BYTES, IDENTIFY_BYTES, SQ_BYTES};
use super::types::AdminQueue;
use crate::dma::DmaRegion;
use crate::error::NvmeResult;

impl AdminQueue {
    pub fn allocate(device_id: u64, epoch: u64) -> NvmeResult<Self> {
        Ok(Self {
            sq: DmaRegion::map(device_id, epoch, SQ_BYTES)?,
            cq: DmaRegion::map(device_id, epoch, CQ_BYTES)?,
            identify: DmaRegion::map(device_id, epoch, IDENTIFY_BYTES)?,
            tail: 0,
            head: 0,
            phase: true,
            cid: 1,
        })
    }
}
