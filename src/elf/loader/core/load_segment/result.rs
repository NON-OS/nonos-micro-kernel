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

use crate::arch::paging::descriptor::flags;

use crate::elf::loader::image::LoadedSegment;
use crate::elf::types::ProgramHeader;

use super::plan::SegmentPlan;

pub(super) fn loaded_segment(plan: &SegmentPlan, header: &ProgramHeader) -> LoadedSegment {
    let mut flags = flags::PRESENT | flags::USER;
    if header.is_writable() {
        flags |= flags::WRITABLE;
    }
    if !header.is_executable() {
        flags |= flags::NO_EXECUTE;
    }
    LoadedSegment { vaddr: plan.seg_va, size: plan.seg_size, flags, segment_type: header.p_type }
}
