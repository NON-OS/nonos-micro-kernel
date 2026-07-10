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

use crate::dma::DmaRegion;

pub struct IoQueue {
    pub(super) sq: DmaRegion,
    pub(super) cq: DmaRegion,
    pub(super) prp_list: DmaRegion,
    pub(crate) data: DmaRegion,
    pub(super) sq_tail: u16,
    pub(super) cq_head: u16,
    pub(super) phase: bool,
    pub(super) cid: u16,
    pub(super) sq_db: u32,
    pub(super) cq_db: u32,
    pub(super) nsid: u32,
    pub(crate) capacity_sectors: u64,
    pub(crate) lba_size: u32,
}

impl IoQueue {
    /// Sectors that fit in the fixed DMA data buffer. The buffer is a constant
    /// byte budget, so the sector count scales inversely with the formatted LBA
    /// size (64 at 512-byte, 8 at 4096-byte). Bounding transfers by this keeps
    /// every read/write within the buffer regardless of the drive's format.
    pub(crate) fn max_sectors(&self) -> u32 {
        (super::constants::DATA_BYTES / self.lba_size.max(1) as u64) as u32
    }
}
