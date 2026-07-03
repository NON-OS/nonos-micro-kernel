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

use crate::constants::{RX_BUFFER_LEN, RX_QUEUE_SIZE};

#[derive(Debug, Clone, Copy)]
pub struct RxQueue {
    pub region_va: *mut u8,
    pub region_phys: u64,
    pub buf_va: *mut u8,
    pub buf_phys: u64,
    pub buf_len: u32,
    pub buf_count: u16,
    pub last_used: u16,
    pub pending_refill: Option<u16>,
}

impl RxQueue {
    pub fn new(
        region_va: u64,
        region_phys: u64,
        buf_va: u64,
        buf_phys: u64,
        buf_count: u16,
    ) -> Self {
        Self {
            region_va: region_va as *mut u8,
            region_phys,
            buf_va: buf_va as *mut u8,
            buf_phys,
            buf_len: RX_BUFFER_LEN,
            buf_count,
            last_used: 0,
            pending_refill: None,
        }
    }

    pub const fn queue_size() -> u16 {
        RX_QUEUE_SIZE
    }

    pub fn region_phys(&self) -> u64 {
        self.region_phys
    }
}
