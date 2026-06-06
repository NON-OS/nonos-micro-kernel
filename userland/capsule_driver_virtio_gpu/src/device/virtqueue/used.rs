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
use core::ptr::read_volatile;
use core::sync::atomic::{fence, Ordering};
#[inline]
fn used_idx_ptr(layout: QueueLayout) -> *const u16 {
    (layout.used_va() as usize + 2) as *const u16
}
#[inline]
fn used_ring_entry(layout: QueueLayout, slot: u16) -> *const u32 {
    (layout.used_va() as usize + 4 + (slot as usize) * 8) as *const u32
}
pub fn read_idx(layout: QueueLayout) -> u16 {
    unsafe { read_volatile(used_idx_ptr(layout)) }
}
pub struct UsedEntry {
    pub id: u32,
    pub len: u32,
}
pub fn read_entry(layout: QueueLayout, ring_slot: u16) -> UsedEntry {
    let slot = ring_slot % layout.queue_size;
    let p = used_ring_entry(layout, slot);
    unsafe {
        let id = read_volatile(p);
        let len = read_volatile(p.add(1));
        fence(Ordering::Acquire);
        UsedEntry { id, len }
    }
}
