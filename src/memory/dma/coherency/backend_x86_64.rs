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

use core::sync::atomic::{compiler_fence, Ordering};

use crate::memory::addr::VirtAddr;
use crate::memory::dma::types::DmaDirection;

use super::super::Coherency;

pub(in crate::memory::dma::coherency) fn sync_for_device(
    _cpu_addr: VirtAddr,
    _size: usize,
    _direction: DmaDirection,
    _coherency: Coherency,
) {
    bus_fence();
}

pub(in crate::memory::dma::coherency) fn sync_for_cpu(
    _cpu_addr: VirtAddr,
    _size: usize,
    _direction: DmaDirection,
    _coherency: Coherency,
) {
    bus_fence();
}

#[inline(always)]
fn bus_fence() {
    compiler_fence(Ordering::SeqCst);
    // SAFETY: ek@nonos.systems — every supported x86_64 host bus is
    // hardware-coherent, so the sync windows reduce to a fence; `mfence`
    // gives StoreLoad ordering between the buffer accesses and the
    // device-side transactions on either side of the window.
    unsafe {
        core::arch::asm!("mfence", options(nostack, preserves_flags));
    }
}
