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
use crate::queue::layout::Queue;
impl Queue {
    pub(super) unsafe fn publish_avail(&self) {
        let avail = self.region_va.add(self.avail_offset).cast::<u16>();
        write_volatile(avail, 0u16);
        let idx = read_volatile(avail.add(1));
        let slot = (idx % self.queue_size) as usize;
        write_volatile(avail.add(2 + slot), 0u16);
        write_volatile(avail.add(1), idx.wrapping_add(1));
    }
}