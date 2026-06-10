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

use crate::loader::errors::{LoaderError, LoaderResult};
use crate::loader::types::LoadedSegment;
use crate::log::logger::log_debug;

use super::types::SegmentLoadInfo;

pub unsafe fn load_segment(source: &[u8], info: &SegmentLoadInfo) -> LoaderResult<()> {
    let source_end =
        info.source_offset.checked_add(info.source_size).ok_or(LoaderError::IntegerOverflow)?;

    if source_end > source.len() {
        return Err(LoaderError::SegmentOutOfBounds);
    }

    if info.source_size > 0 {
        let src_ptr = source.as_ptr().add(info.source_offset);
        let dst_ptr = info.dest_addr as *mut u8;

        core::ptr::copy_nonoverlapping(src_ptr, dst_ptr, info.source_size);

        log_debug("segment", "Copied segment data");
    }

    if let Some((bss_start, bss_size)) = info.bss_region() {
        let bss_ptr = bss_start as *mut u8;
        core::ptr::write_bytes(bss_ptr, 0, bss_size);

        log_debug("segment", "Zeroed BSS region");
    }

    Ok(())
}

pub unsafe fn load_all_segments(
    source: &[u8],
    segments: &[Option<LoadedSegment>],
    base_addr: u64,
    virt_base: u64,
) -> LoaderResult<usize> {
    let mut loaded_count = 0;

    for segment in segments.iter().flatten() {
        let info = SegmentLoadInfo::from_segment(segment, base_addr, virt_base);
        load_segment(source, &info)?;
        loaded_count += 1;
    }

    Ok(loaded_count)
}
