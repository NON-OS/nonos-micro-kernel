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

use super::state::{Event, EVENTS, EVENT_CAP, EVENT_CURSOR, OWNERS, SYSTEM_BYTES};

pub(crate) fn record_munmap(pid: u32, size: usize, va: u64) {
    let size = size as u64;
    let old = SYSTEM_BYTES
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some(v.saturating_sub(size)))
        .unwrap_or_else(|v| v);
    let system = old.saturating_sub(size);
    let mut total = 0;
    {
        let mut owners = OWNERS.lock();
        if let Some(owner) = owners.iter_mut().find(|o| o.pid == pid) {
            owner.bytes = owner.bytes.saturating_sub(size);
            total = owner.bytes;
        }
    }
    let idx = EVENT_CURSOR.fetch_add(1, Ordering::Relaxed) % EVENT_CAP;
    EVENTS.lock()[idx] = Event { pid, kind: b'U', size, va, owner: total, system };
    crate::sys::serial::print(b"[MUNMAP] pid=");
    crate::sys::serial::print_hex(pid as u64);
    crate::sys::serial::print(b" name=");
    if let Some(pcb) = crate::process::core::PROCESS_TABLE.find_by_pid(pid) {
        crate::sys::serial::print(pcb.name.lock().as_bytes());
    } else {
        crate::sys::serial::print(b"unknown");
    }
    crate::sys::serial::print(b" size=");
    crate::sys::serial::print_hex(size);
    crate::sys::serial::print(b" va=");
    crate::sys::serial::print_hex(va);
    crate::sys::serial::print(b" total_pid=");
    crate::sys::serial::print_hex(total);
    crate::sys::serial::print(b" total_system=");
    crate::sys::serial::print_hex(system);
    crate::sys::serial::println(b"");
}
