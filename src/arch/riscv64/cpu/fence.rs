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

pub fn fence() {
    unsafe {
        asm!("fence", options(nostack));
    }
}

pub fn fence_i() {
    unsafe {
        asm!("fence.i", options(nostack));
    }
}

pub fn sfence_vma() {
    unsafe {
        asm!("sfence.vma", options(nostack));
    }
}

pub fn sfence_vma_addr(addr: usize) {
    unsafe {
        asm!("sfence.vma {}, zero", in(reg) addr, options(nostack));
    }
}

pub fn sfence_vma_asid(asid: usize) {
    unsafe {
        asm!("sfence.vma zero, {}", in(reg) asid, options(nostack));
    }
}

pub fn sfence_vma_addr_asid(addr: usize, asid: usize) {
    unsafe {
        asm!("sfence.vma {}, {}", in(reg) addr, in(reg) asid, options(nostack));
    }
}
