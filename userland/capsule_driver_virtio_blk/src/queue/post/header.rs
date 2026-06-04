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
use crate::constants::{HEADER_OFFSET, STATUS_OFFSET};
use crate::queue::layout::Queue;
use core::ptr::write_volatile;
impl Queue {
    pub(super) unsafe fn write_header(&self, dir: Direction, lba: u64) {
        let hdr = self.header_va.add(HEADER_OFFSET);
        write_volatile(hdr.cast::<u32>(), dir.req_type());
        write_volatile(hdr.add(4).cast::<u32>(), 0u32);
        write_volatile(hdr.add(8).cast::<u64>(), lba);
        write_volatile(self.header_va.add(STATUS_OFFSET), 0xFFu8);
    }
}
