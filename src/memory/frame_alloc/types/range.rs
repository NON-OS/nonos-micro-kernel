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

use super::super::constants::FRAME_SIZE;
use super::super::error::{FrameAllocError, FrameResult};
use crate::memory::addr::PhysAddr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameRange {
    pub start: PhysAddr,
    pub end: PhysAddr,
}

impl FrameRange {
    pub fn new(start: PhysAddr, end: PhysAddr) -> FrameResult<Self> {
        if start >= end {
            return Err(FrameAllocError::InvalidRegion);
        }
        if !start.is_aligned(FRAME_SIZE) || !end.is_aligned(FRAME_SIZE) {
            return Err(FrameAllocError::RegionNotAligned);
        }
        Ok(Self { start, end })
    }

    pub fn frames_remaining(&self) -> usize {
        let aligned_start = self.start.align_up(FRAME_SIZE);
        if aligned_start >= self.end {
            0
        } else {
            ((self.end.as_u64() - aligned_start.as_u64()) / FRAME_SIZE) as usize
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.frames_remaining() == 0
    }
}
