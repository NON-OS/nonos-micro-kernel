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

use crate::hardware::broker::claim;
use crate::hardware::broker::dma::limits::dma_page_limit_for_class;
use crate::hardware::broker::dma::types::{DmaMapError, DmaMapRequest, DMA_MAP_HIGH};
use crate::hardware::broker::table;

pub(super) const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: u64 = PAGE_SIZE - 1;
const FLAGS_KNOWN: u32 = DMA_MAP_HIGH;

// Returns the claim epoch on success so the caller can record it
// without a second lookup. All state is observed read-only here.
pub(super) fn validate(req: &DmaMapRequest, pid: u32) -> Result<u64, DmaMapError> {
    if req.flags & !FLAGS_KNOWN != 0 {
        return Err(DmaMapError::UnsupportedFlags);
    }
    if req.length == 0 || req.length & PAGE_MASK != 0 {
        return Err(DmaMapError::BadLength);
    }
    let claim = claim::lookup(req.device_id).ok_or(DmaMapError::NotClaimed)?;
    if claim.pid != pid {
        return Err(DmaMapError::NotClaimed);
    }
    if claim.epoch != req.claim_epoch {
        return Err(DmaMapError::StaleEpoch);
    }
    let class = table::class_of(req.device_id).ok_or(DmaMapError::UnknownDevice)?;
    let pages = req.length / PAGE_SIZE;
    if pages > dma_page_limit_for_class(class) {
        return Err(DmaMapError::BadLengthForClass);
    }
    Ok(claim.epoch)
}
