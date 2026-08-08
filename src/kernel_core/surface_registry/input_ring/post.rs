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

use super::super::types::{InputEvent, RegistryError, INPUT_RING_CAP};
use super::ring::{DIAG, DROPPED, FIRST_INPUT_POST, KIND_DIAG_BASE, RING, SEQ, WAITER};

pub fn post_input(ev: InputEvent) -> Result<(), RegistryError> {
    if ev.kind >= KIND_DIAG_BASE {
        let slot = (ev.kind - KIND_DIAG_BASE) as usize;
        if let Some(c) = DIAG.get(slot) {
            c.fetch_add(1, Ordering::Relaxed);
        }
        return Ok(());
    }
    {
        let mut ring = RING.lock();
        let next = super::super::ring_math::wrap(ring.head, INPUT_RING_CAP);
        if next == ring.tail {
            DROPPED.fetch_add(1, Ordering::Relaxed);
            return Err(RegistryError::OutOfSlots);
        }
        let head = ring.head;
        ring.buf[head] = ev;
        ring.head = next;
    }
    SEQ.fetch_add(1, Ordering::Release);
    crate::sys::bench::mark_once(&FIRST_INPUT_POST, b"input_post_first");
    let waiter = WAITER.swap(0, Ordering::AcqRel);
    if waiter != 0 {
        crate::sched::wake_process(waiter as u32);
    }
    Ok(())
}
