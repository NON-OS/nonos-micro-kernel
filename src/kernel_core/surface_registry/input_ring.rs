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

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use spin::Mutex;

use super::types::{InputEvent, RegistryError, INPUT_RING_CAP};

// MPSC ring: many driver capsules post (kbd, mouse, touch); a single
// input_router capsule drains. Posts and drains both take the mutex
// during the short critical section held over the event ring stores.
// Per-source SPSC fanout lives inside the router capsule.

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

static DROPPED: AtomicU64 = AtomicU64::new(0);
static SEQ: AtomicU64 = AtomicU64::new(0);
static WAITER: AtomicU64 = AtomicU64::new(0);
static FIRST_INPUT_POST: AtomicBool = AtomicBool::new(false);
#[cfg(feature = "input-probe-inject")]
static INPUT_CONSUMER_READY: AtomicBool = AtomicBool::new(false);

pub fn post_input(ev: InputEvent) -> Result<(), RegistryError> {
    {
        let mut ring = RING.lock();
        let next = (ring.head + 1) % INPUT_RING_CAP;
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

pub fn input_seq() -> u64 {
    SEQ.load(Ordering::Acquire)
}

pub fn arm_input_waiter(pid: u32) {
    WAITER.store(pid as u64, Ordering::Release);
    #[cfg(feature = "input-probe-inject")]
    INPUT_CONSUMER_READY.store(true, Ordering::Release);
}

#[cfg(feature = "input-probe-inject")]
pub fn consumer_ready() -> bool {
    INPUT_CONSUMER_READY.load(Ordering::Acquire)
}

pub fn clear_input_waiter() {
    WAITER.store(0, Ordering::Release);
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
