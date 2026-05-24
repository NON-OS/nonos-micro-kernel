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

use core::arch::asm;

use super::mode::MmuMode;

pub fn read_satp() -> usize {
    let satp: usize;
    unsafe {
        asm!("csrr {}, satp", out(reg) satp, options(nostack));
    }
    satp
}

pub fn write_satp(satp: usize) {
    unsafe {
        asm!("csrw satp, {}", "sfence.vma", in(reg) satp, options(nostack));
    }
}

pub fn current_asid() -> u16 {
    ((read_satp() >> 44) & 0xffff) as u16
}

pub fn current_ppn() -> usize {
    read_satp() & ((1 << 44) - 1)
}

pub fn mmu_mode() -> MmuMode {
    match read_satp() >> 60 {
        0 => MmuMode::Bare,
        8 => MmuMode::Sv39,
        9 => MmuMode::Sv48,
        10 => MmuMode::Sv57,
        _ => MmuMode::Unknown,
    }
}

pub fn make_satp(mode: MmuMode, asid: u16, ppn: usize) -> usize {
    (mode.satp_mode() << 60) | ((asid as usize) << 44) | (ppn & ((1 << 44) - 1))
}
