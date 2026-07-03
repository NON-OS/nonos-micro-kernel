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

use core::ptr::{read_volatile, write_volatile};

use super::TxQueue;
use crate::constants::{RING_SLOTS, VQ_AVAIL_OFFSET, VQ_DESC_OFFSET};

const DESC_SIZE: usize = 16;
const AVAIL_RING_OFFSET: usize = 4;

impl TxQueue {
    pub fn post_packet(&self, slot: u16, length: u32) {
        unsafe {
            let desc = self.region_va.add(VQ_DESC_OFFSET).add(DESC_SIZE * slot as usize);
            write_volatile(desc.cast::<u64>(), self.buf_phys + self.buf_len as u64 * slot as u64);
            write_volatile(desc.add(8).cast::<u32>(), length);
            write_volatile(desc.add(12).cast::<u16>(), 0u16);
            write_volatile(desc.add(14).cast::<u16>(), 0u16);
            let avail = self.region_va.add(VQ_AVAIL_OFFSET).cast::<u16>();
            let idx = read_volatile(avail.add(1));
            let pos = (idx % RING_SLOTS) as usize;
            write_volatile(avail.add(AVAIL_RING_OFFSET / 2 + pos), slot);
            write_volatile(avail.add(1), idx.wrapping_add(1));
        }
    }
}
