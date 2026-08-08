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

use super::super::super::Coherency;
use super::maintain::{barrier, range, Op};
use crate::memory::addr::VirtAddr;
use crate::memory::dma::types::DmaDirection;

pub(in crate::memory::dma::coherency) fn sync_for_device(
    cpu_addr: VirtAddr,
    size: usize,
    direction: DmaDirection,
    coherency: Coherency,
) {
    compiler_fence(Ordering::SeqCst);
    if coherency.requires_cache_maintenance() && direction.writes_to_device() {
        range(cpu_addr, size, Op::Clean);
    }
    barrier();
}

pub(in crate::memory::dma::coherency) fn sync_for_cpu(
    cpu_addr: VirtAddr,
    size: usize,
    direction: DmaDirection,
    coherency: Coherency,
) {
    if coherency.requires_cache_maintenance() && direction.reads_from_device() {
        range(cpu_addr, size, Op::CleanAndInvalidate);
    }
    barrier();
    compiler_fence(Ordering::SeqCst);
}
