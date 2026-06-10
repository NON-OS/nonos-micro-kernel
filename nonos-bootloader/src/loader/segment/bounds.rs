// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

extern crate alloc;

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::{memory, LoadedSegment};
use alloc::vec::Vec;

pub fn calculate_memory_bounds(
    segments: &[Option<LoadedSegment>],
) -> LoaderResult<(u64, u64, usize)> {
    let mut min_addr = u64::MAX;
    let mut max_addr = 0u64;

    for segment in segments.iter().flatten() {
        let start = segment.target_addr;
        let end = start.checked_add(segment.mem_size).ok_or(LoaderError::IntegerOverflow)?;

        min_addr = min_addr.min(start);
        max_addr = max_addr.max(end);
    }

    if min_addr == u64::MAX {
        return Err(LoaderError::NoLoadableSegments);
    }

    let total_size = max_addr.checked_sub(min_addr).ok_or(LoaderError::IntegerOverflow)? as usize;

    Ok((min_addr, max_addr, total_size))
}

pub fn check_segment_overlaps(segments: &[Option<LoadedSegment>]) -> LoaderResult<()> {
    let active_segments: Vec<_> = segments.iter().flatten().collect();

    for i in 0..active_segments.len() {
        for j in (i + 1)..active_segments.len() {
            let a = active_segments[i];
            let b = active_segments[j];

            let a_end = a.target_addr + a.mem_size;
            let b_end = b.target_addr + b.mem_size;

            if a.target_addr < b_end && b.target_addr < a_end {
                return Err(LoaderError::SegmentOverlap);
            }
        }
    }

    Ok(())
}

pub fn validate_segment_addresses(segments: &[Option<LoadedSegment>]) -> LoaderResult<()> {
    for segment in segments.iter().flatten() {
        if segment.target_addr != 0 && segment.target_addr < memory::MIN_LOAD_ADDRESS {
            return Err(LoaderError::AddressOutOfRange);
        }

        let end_addr = segment
            .target_addr
            .checked_add(segment.mem_size)
            .ok_or(LoaderError::IntegerOverflow)?;

        if end_addr > memory::MAX_LOAD_ADDRESS {
            return Err(LoaderError::AddressOutOfRange);
        }
    }

    Ok(())
}

pub fn count_wx_violations(segments: &[Option<LoadedSegment>]) -> usize {
    segments.iter().flatten().filter(|s| s.has_wx()).count()
}

pub fn total_file_size(segments: &[Option<LoadedSegment>]) -> u64 {
    segments.iter().flatten().map(|s| s.file_size).sum()
}

pub fn total_memory_size(segments: &[Option<LoadedSegment>]) -> u64 {
    segments.iter().flatten().map(|s| s.mem_size).sum()
}
