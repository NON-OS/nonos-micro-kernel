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
use crate::constants::{EVENT_RING_SEGMENT_TRBS, TRB_BYTES};
use crate::dma::{DmaPool, DmaRegion};
use crate::error::XhciResult;
const ERST_ENTRY_BYTES: u64 = 16;
pub struct EventRing {
    pub(super) segment: DmaRegion,
    pub(super) erst: DmaRegion,
    pub(super) consumer_cycle: u8,
    pub(super) dequeue_index: usize,
    pub(super) drained_total: u64,
}
impl EventRing {
    pub fn new(pool: &DmaPool) -> XhciResult<Self> {
        let segment_bytes = (EVENT_RING_SEGMENT_TRBS as u64) * (TRB_BYTES as u64);
        let segment = pool.alloc(segment_bytes)?;
        segment.zero();
        let erst = pool.alloc(ERST_ENTRY_BYTES)?;
        erst.zero();
        let segment_phys = segment.phys();
        let erst_va = erst.as_mut_ptr::<u32>();
        unsafe {
            core::ptr::write_volatile(erst_va.add(0), (segment_phys & 0xFFFF_FFFF) as u32);
            core::ptr::write_volatile(erst_va.add(1), (segment_phys >> 32) as u32);
            core::ptr::write_volatile(erst_va.add(2), EVENT_RING_SEGMENT_TRBS as u32);
            core::ptr::write_volatile(erst_va.add(3), 0);
        }
        Ok(Self { segment, erst, consumer_cycle: 1, dequeue_index: 0, drained_total: 0 })
    }
}
