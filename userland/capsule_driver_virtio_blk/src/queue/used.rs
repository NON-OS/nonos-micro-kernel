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
use super::layout::Queue;
use crate::constants::STATUS_OFFSET;
const USED_IDX_OFFSET: usize = 2;
impl Queue {
    pub fn used_idx(&self) -> u16 {
        unsafe { read_volatile(self.region_va.add(self.used_offset + USED_IDX_OFFSET).cast()) }
    }
    pub fn status_byte(&self) -> u8 {
        unsafe { read_volatile(self.header_va.add(STATUS_OFFSET)) }
    }
    pub unsafe fn data(&self, len: u32) -> &[u8] {
        let n = core::cmp::min(len, self.data_len) as usize;
        core::slice::from_raw_parts(self.data_va, n)
    }
    pub unsafe fn data_mut(&self, len: u32) -> &mut [u8] {
        let n = core::cmp::min(len, self.data_len) as usize;
        core::slice::from_raw_parts_mut(self.data_va, n)
    }
}