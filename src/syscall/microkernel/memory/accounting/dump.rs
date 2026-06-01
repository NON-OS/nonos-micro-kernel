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

use core::sync::atomic::Ordering;

use super::state::{EVENTS, EVENT_CAP, EVENT_CURSOR, OWNERS, SYSTEM_BYTES};

pub fn dump_mmap_accounting() {
    crate::sys::serial::println(b"[OOM] mmap owners");
    for owner in OWNERS.lock().iter().copied().filter(|o| o.pid != 0 && o.bytes != 0) {
        crate::sys::serial::print(b"[OOM] mmap pid=");
        crate::sys::serial::print_hex(owner.pid as u64);
        crate::sys::serial::print(b" bytes=");
        crate::sys::serial::print_hex(owner.bytes);
        crate::sys::serial::println(b"");
    }
    crate::sys::serial::print(b"[OOM] mmap total_system=");
    crate::sys::serial::print_hex(SYSTEM_BYTES.load(Ordering::Relaxed));
    crate::sys::serial::println(b"");
    let events = EVENTS.lock();
    let start = EVENT_CURSOR.load(Ordering::Relaxed).saturating_sub(EVENT_CAP);
    for n in 0..EVENT_CAP {
        let event = events[(start + n) % EVENT_CAP];
        if event.pid == 0 {
            continue;
        }
        crate::sys::serial::print(b"[OOM] mmap_event kind=");
        crate::sys::serial::print_hex(event.kind as u64);
        crate::sys::serial::print(b" pid=");
        crate::sys::serial::print_hex(event.pid as u64);
        crate::sys::serial::print(b" size=");
        crate::sys::serial::print_hex(event.size);
        crate::sys::serial::print(b" va=");
        crate::sys::serial::print_hex(event.va);
        crate::sys::serial::print(b" owner=");
        crate::sys::serial::print_hex(event.owner);
        crate::sys::serial::print(b" system=");
        crate::sys::serial::print_hex(event.system);
        crate::sys::serial::println(b"");
    }
}
