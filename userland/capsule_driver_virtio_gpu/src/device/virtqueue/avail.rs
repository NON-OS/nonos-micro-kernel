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
use super::layout::QueueLayout;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{fence, Ordering};
#[inline]
fn avail_idx_ptr(layout: QueueLayout) -> *mut u16 {
    (layout.avail_va() as usize + 2) as *mut u16
}
#[inline]
fn avail_ring_ptr(layout: QueueLayout, slot: u16) -> *mut u16 {
    (layout.avail_va() as usize + 4 + (slot as usize) * 2) as *mut u16
}
pub fn read_idx(layout: QueueLayout) -> u16 {
    unsafe { read_volatile(avail_idx_ptr(layout)) }
}
pub fn publish(layout: QueueLayout, desc_head: u16) {
    let idx = read_idx(layout);
    let slot = idx % layout.queue_size;
    unsafe {
        write_volatile(avail_ring_ptr(layout, slot), desc_head);
    }
    fence(Ordering::Release);
    unsafe {
        write_volatile(avail_idx_ptr(layout), idx.wrapping_add(1));
    }
}
