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

use crate::constants::{LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS, STATUS_FAILED};
use crate::regs::Regs;

pub fn program_queue(
    regs: Regs,
    queue_index: u16,
    queue_phys: u64,
    queue_size_hint: u16,
) -> Result<u16, &'static str> {
    unsafe {
        if queue_phys & 0xFFF != 0 {
            regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
            return Err("virtio-net: unaligned queue");
        }
        regs.w16(LEG_QUEUE_SEL, queue_index);
        let qmax = regs.r16(LEG_QUEUE_NUM);
        if qmax == 0 {
            regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
            return Err("virtio-net: queue missing");
        }
        let qsize = core::cmp::min(qmax, queue_size_hint);
        let pfn64 = queue_phys >> 12;
        if pfn64 > u32::MAX as u64 {
            regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
            return Err("virtio-net: queue pfn overflow");
        }
        regs.w32(LEG_QUEUE_PFN, pfn64 as u32);
        Ok(qsize)
    }
}
