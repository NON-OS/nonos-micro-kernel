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

use alloc::format;

use crate::loader::core::constants::elf_flags;
use crate::loader::core::types::ValidationResult;
use crate::loader::image::{KernelSegmentLayout, MAX_KERNEL_SEGMENTS};
use crate::log::logger::{log_info, log_warn};

use super::copy_payload::copy_payload;
use super::zero_tail::zero_tail;

// Walk every PT_LOAD recorded by the validator and stage it into
// the freshly allocated phys range. Each iteration:
//   1. computes the segment's phys destination as
//      `phys_base + (virt - virt_min)`;
//   2. copies `p_filesz` bytes from the payload;
//   3. zeros the BSS tail (`p_memsz - p_filesz`);
//   4. records (phys, virt, size, flags) into the layout array
//      for the paging stage to consume.
pub fn load_segments(
    payload: &[u8],
    v: &ValidationResult,
    phys_base: u64,
    virt_min: u64,
) -> ([KernelSegmentLayout; MAX_KERNEL_SEGMENTS], usize) {
    let mut out = [KernelSegmentLayout::default(); MAX_KERNEL_SEGMENTS];
    let mut count: usize = 0;

    for i in 0..v.load_count {
        let seg = &v.loads[i];
        let virt = seg.target;
        let dst_phys = phys_base + (virt - virt_min);

        if seg.p_align > 1 && (dst_phys as usize & (seg.p_align - 1)) != 0 {
            log_warn("loader", &format!("Segment misaligned at phys 0x{:x}", dst_phys));
        }

        let rwx = format!(
            "{}{}{}",
            if (seg.p_flags & elf_flags::PF_R) != 0 { "R" } else { "-" },
            if (seg.p_flags & elf_flags::PF_W) != 0 { "W" } else { "-" },
            if (seg.p_flags & elf_flags::PF_X) != 0 { "X" } else { "-" },
        );

        if seg.p_filesz > 0 {
            unsafe {
                copy_payload(payload, seg.p_offset, dst_phys, seg.p_filesz);
            }
            log_info(
                "loader",
                &format!(
                    "Loaded {} bytes phys=0x{:x} virt=0x{:x} [{}]",
                    seg.p_filesz, dst_phys, virt, rwx,
                ),
            );
        }

        unsafe {
            zero_tail(dst_phys, seg.p_filesz as u64, seg.p_memsz as u64);
        }
        if seg.p_memsz > seg.p_filesz {
            log_info(
                "loader",
                &format!(
                    "Zeroed {} bytes at phys 0x{:x}",
                    seg.p_memsz - seg.p_filesz,
                    dst_phys + seg.p_filesz as u64,
                ),
            );
        }

        if count < MAX_KERNEL_SEGMENTS {
            out[count] = KernelSegmentLayout {
                phys: dst_phys,
                virt,
                size: seg.p_memsz as u64,
                flags: seg.p_flags,
            };
            count += 1;
        } else {
            log_warn(
                "loader",
                "PT_LOAD count exceeds MAX_KERNEL_SEGMENTS; trailing segment dropped",
            );
        }
    }

    (out, count)
}
