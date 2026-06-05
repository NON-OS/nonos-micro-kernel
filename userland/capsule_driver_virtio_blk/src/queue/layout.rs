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
use crate::constants::{DATA_BUF_LEN, MAX_QUEUE_SIZE, VQ_DESC_OFFSET, VQ_REGION_SIZE};
use core::ptr::write_bytes;
#[derive(Clone, Copy)]
pub struct Queue {
    pub region_va: *mut u8,
    pub queue_size: u16,
    pub avail_offset: usize,
    pub used_offset: usize,
    pub header_va: *mut u8,
    pub header_phys: u64,
    pub data_va: *mut u8,
    pub data_phys: u64,
    pub data_len: u32,
    pub last_used: u16,
}
impl Queue {
    pub fn new(
        region_va: u64,
        _region_phys: u64,
        queue_size: u16,
        header_va: u64,
        header_phys: u64,
        data_va: u64,
        data_phys: u64,
    ) -> Self {
        let avail_offset = avail_offset(queue_size);
        let used_offset = used_offset(queue_size);
        unsafe {
            write_bytes(region_va as *mut u8, 0, VQ_REGION_SIZE);
        }
        Self {
            region_va: region_va as *mut u8,
            queue_size,
            avail_offset,
            used_offset,
            header_va: header_va as *mut u8,
            header_phys,
            data_va: data_va as *mut u8,
            data_phys,
            data_len: DATA_BUF_LEN as u32,
            last_used: 0,
        }
    }
    pub const fn max_supported_size() -> u16 {
        MAX_QUEUE_SIZE
    }
}
const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}
const fn avail_offset(queue_size: u16) -> usize {
    VQ_DESC_OFFSET + (queue_size as usize) * 16
}
const fn used_offset(queue_size: u16) -> usize {
    align_up(avail_offset(queue_size) + 6 + (queue_size as usize) * 2, 4096)
}
