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

use core::ptr::read_volatile;

use super::{RxQueue, TxQueue};
use crate::constants::VQ_USED_OFFSET;

const USED_IDX_OFFSET: usize = 2;
const USED_RING_OFFSET: usize = 4;
const USED_ELEM_SIZE: usize = 8;

impl RxQueue {
    pub fn used_idx(&self) -> u16 {
        unsafe { read_volatile(self.region_va.add(VQ_USED_OFFSET + USED_IDX_OFFSET).cast()) }
    }

    pub fn used_elem_at(&self, ring_pos: u16) -> (u32, u32) {
        let off = VQ_USED_OFFSET + USED_RING_OFFSET + USED_ELEM_SIZE * (ring_pos as usize);
        unsafe {
            let id = read_volatile(self.region_va.add(off).cast::<u32>());
            let len = read_volatile(self.region_va.add(off + 4).cast::<u32>());
            (id, len)
        }
    }
}

impl TxQueue {
    pub fn used_idx(&self) -> u16 {
        unsafe { read_volatile(self.region_va.add(VQ_USED_OFFSET + USED_IDX_OFFSET).cast()) }
    }

    pub unsafe fn buffer_mut(&self, len: u32) -> &mut [u8] {
        let n = core::cmp::min(len, self.buf_len) as usize;
        core::slice::from_raw_parts_mut(self.buf_va, n)
    }
}
