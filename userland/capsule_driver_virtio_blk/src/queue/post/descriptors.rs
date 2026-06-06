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
use super::direction::Direction;
use crate::constants::{
    HEADER_OFFSET, SECTOR_SIZE, STATUS_OFFSET, VQ_DESC_OFFSET, VRING_DESC_F_NEXT,
    VRING_DESC_F_WRITE,
};
use crate::queue::layout::Queue;
use core::ptr::write_volatile;
const DESC_SIZE: usize = 16;
const HEADER_LEN: u32 = 16;
const STATUS_LEN: u32 = 1;
impl Queue {
    pub(super) unsafe fn write_descriptor_chain(&self, dir: Direction, nsectors: u32) {
        let desc_base = self.region_va.add(VQ_DESC_OFFSET);
        let header_phys = self.header_phys + HEADER_OFFSET as u64;
        let status_phys = self.header_phys + STATUS_OFFSET as u64;
        self.write_header_descriptor(desc_base, header_phys);
        if dir == Direction::Flush {
            self.write_status_descriptor(desc_base.add(DESC_SIZE), status_phys);
            return;
        }
        self.write_data_descriptor(desc_base.add(DESC_SIZE), dir, nsectors);
        self.write_status_descriptor(desc_base.add(DESC_SIZE * 2), status_phys);
    }
    unsafe fn write_header_descriptor(&self, desc: *mut u8, header_phys: u64) {
        write_volatile(desc.cast::<u64>(), header_phys);
        write_volatile(desc.add(8).cast::<u32>(), HEADER_LEN);
        write_volatile(desc.add(12).cast::<u16>(), VRING_DESC_F_NEXT);
        write_volatile(desc.add(14).cast::<u16>(), 1u16);
    }
    unsafe fn write_data_descriptor(&self, desc: *mut u8, dir: Direction, nsectors: u32) {
        let data_len = nsectors.saturating_mul(SECTOR_SIZE as u32);
        let flags = match dir {
            Direction::Read => VRING_DESC_F_NEXT | VRING_DESC_F_WRITE,
            Direction::Write => VRING_DESC_F_NEXT,
            Direction::Flush => VRING_DESC_F_NEXT,
        };
        write_volatile(desc.cast::<u64>(), self.data_phys);
        write_volatile(desc.add(8).cast::<u32>(), data_len);
        write_volatile(desc.add(12).cast::<u16>(), flags);
        write_volatile(desc.add(14).cast::<u16>(), 2u16);
    }
    unsafe fn write_status_descriptor(&self, desc: *mut u8, status_phys: u64) {
        write_volatile(desc.cast::<u64>(), status_phys);
        write_volatile(desc.add(8).cast::<u32>(), STATUS_LEN);
        write_volatile(desc.add(12).cast::<u16>(), VRING_DESC_F_WRITE);
        write_volatile(desc.add(14).cast::<u16>(), 0u16);
    }
}
