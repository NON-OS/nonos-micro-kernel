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
use crate::regs::mmio_write32;
pub fn ring_doorbell(doorbell_base: u64, slot: u8, target: u8) {
    let addr = doorbell_base + (slot as u64) * 4;
    // The controller DMA-reads the ring's TRBs only after it sees the doorbell.
    // Order every prior TRB store ahead of this write so it never fetches a
    // stale or half-written TRB (a real concern on weakly ordered targets and
    // any write-combining grant; harmless on strongly ordered x86).
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    mmio_write32(addr, target as u32);
}
