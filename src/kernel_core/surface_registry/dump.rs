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

use super::table::SLOTS;

pub fn dump_surface_accounting() {
    let slots = SLOTS.lock();
    let mut count = 0u64;
    let mut frames = 0u64;
    crate::sys::serial::println(b"[OOM] surfaces");
    for slot in slots.iter().flatten() {
        count = count.saturating_add(1);
        frames = frames.saturating_add(slot.frames.len() as u64);
        crate::sys::serial::print(b"[OOM] surface owner_pid=");
        crate::sys::serial::print_hex(slot.owner_pid as u64);
        crate::sys::serial::print(b" name=");
        if let Some(pcb) = crate::process::core::PROCESS_TABLE.find_by_pid(slot.owner_pid) {
            crate::sys::serial::print(pcb.name.lock().as_bytes());
        } else {
            crate::sys::serial::print(b"unknown");
        }
        crate::sys::serial::print(b" bytes=");
        crate::sys::serial::print_hex(slot.byte_len);
        crate::sys::serial::print(b" refs=");
        crate::sys::serial::print_hex(slot.refcount as u64);
        crate::sys::serial::println(b"");
    }
    crate::sys::serial::print(b"[OOM] surface_count=");
    crate::sys::serial::print_hex(count);
    crate::sys::serial::print(b" surface_frames=");
    crate::sys::serial::print_hex(frames);
    crate::sys::serial::println(b"");
    let (allocated, regions) = crate::memory::frame_alloc::get_stats();
    crate::sys::serial::print(b"[OOM] frame_allocated=");
    crate::sys::serial::print_hex(allocated as u64);
    crate::sys::serial::print(b" frame_regions=");
    crate::sys::serial::print_hex(regions as u64);
    crate::sys::serial::println(b"");
    let heap = crate::memory::heap::get_heap_stats();
    crate::sys::serial::print(b"[OOM] heap_current=");
    crate::sys::serial::print_hex(heap.current_usage as u64);
    crate::sys::serial::print(b" heap_peak=");
    crate::sys::serial::print_hex(heap.peak_usage as u64);
    crate::sys::serial::println(b"");
}
