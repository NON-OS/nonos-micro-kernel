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
use crate::constants::{
    VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_MAX_SIZE, VQ_STAGING_LEN, VQ_STAGING_OFFSET, VQ_USED_OFFSET,
};
#[derive(Clone, Copy)]
pub struct QueueLayout {
    pub queue_size: u16,
    pub region_user_va: u64,
    pub region_device_addr: u64,
}
impl QueueLayout {
    pub fn new(
        queue_size: u16,
        region_user_va: u64,
        region_device_addr: u64,
    ) -> Result<Self, &'static str> {
        if queue_size == 0 || queue_size > VQ_MAX_SIZE {
            return Err("virtio-gpu: queue size out of range");
        }
        if !queue_size.is_power_of_two() {
            return Err("virtio-gpu: queue size not power of two");
        }
        if region_user_va == 0 || region_device_addr == 0 {
            return Err("virtio-gpu: vq region not mapped");
        }
        Ok(Self { queue_size, region_user_va, region_device_addr })
    }
    #[inline]
    pub fn desc_va(self) -> *mut u8 {
        (self.region_user_va as usize + VQ_DESC_OFFSET) as *mut u8
    }
    #[inline]
    pub fn avail_va(self) -> *mut u8 {
        (self.region_user_va as usize + VQ_AVAIL_OFFSET) as *mut u8
    }
    #[inline]
    pub fn used_va(self) -> *mut u8 {
        (self.region_user_va as usize + VQ_USED_OFFSET) as *mut u8
    }
    #[inline]
    pub fn staging_va(self) -> *mut u8 {
        (self.region_user_va as usize + VQ_STAGING_OFFSET) as *mut u8
    }
    #[inline]
    pub fn staging_device_addr(self) -> u64 {
        self.region_device_addr + VQ_STAGING_OFFSET as u64
    }
    #[inline]
    pub const fn staging_len(self) -> usize {
        VQ_STAGING_LEN
    }
}
