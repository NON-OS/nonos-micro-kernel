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

use spin::Mutex;

use super::types::{InputEvent, RegistryError, INPUT_RING_CAP};

// MPSC ring: many driver capsules post (kbd, mouse, touch); a single
// input_router capsule drains. Posts and drains both take the mutex
// for now; lock is held only over a few u64 stores so contention stays
// bounded. Per-source SPSC fanout lives inside the router capsule.

struct Ring {
    head: usize,
    tail: usize,
    buf: [InputEvent; INPUT_RING_CAP],
}

static RING: Mutex<Ring> = Mutex::new(Ring {
    head: 0,
    tail: 0,
    buf: [InputEvent {
        kind: 0,
        flags: 0,
        code: 0,
        x: 0,
        y: 0,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    }; INPUT_RING_CAP],
});

pub fn post_input(ev: InputEvent) -> Result<(), RegistryError> {
    let mut ring = RING.lock();
    let next = (ring.head + 1) % INPUT_RING_CAP;
    if next == ring.tail {
        return Err(RegistryError::OutOfSlots);
    }
    let head = ring.head;
    ring.buf[head] = ev;
    ring.head = next;
    Ok(())
}

pub fn drain_input(out: &mut [InputEvent]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut ring = RING.lock();
    let mut n = 0usize;
    while n < out.len() && ring.tail != ring.head {
        out[n] = ring.buf[ring.tail];
        ring.tail = (ring.tail + 1) % INPUT_RING_CAP;
        n += 1;
    }
    n
}
