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

/// Diagnostic sentinel events. A driver posts one of these to report a milestone
/// the on-screen bars display; they never enter the ring or route anywhere.
///   slot 0: a ps2 port read succeeded (the PIO grant is live)
///   slot 1: reached serving loop      slot 2: i2c-HID touchpad found
///   slot 3: i8042 produced a byte     slot 4: bound i2c controller (index+1)
///   slot 5: the i2c bind was probe-confirmed by the touchpad's ACK
///   slot 6: raw non-empty touchpad report read (pre-decode, grows per report)
///   slot 7: the touchpad acknowledged RESET (wake handshake confirmed)
pub const KIND_DIAG_BASE: u16 = 0xFE00;
const DIAG_SLOTS: usize = 8;
#[allow(clippy::declare_interior_mutable_const)]
const DIAG_INIT: AtomicU64 = AtomicU64::new(0);
static DIAG: [AtomicU64; DIAG_SLOTS] = [DIAG_INIT; DIAG_SLOTS];

/// Value of a diagnostic counter slot (0 if out of range).
pub fn input_diag(slot: usize) -> u64 {
    DIAG.get(slot).map_or(0, |c| c.load(Ordering::Acquire))
}

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

static DRAINED: AtomicU64 = AtomicU64::new(0);

/// Total input events the router has drained. Paired with `input_seq` (events
/// posted), this locates a stall: posts climbing with drains flat means the
/// router is not consuming; both flat means no driver is producing.
pub fn input_drains() -> u64 {
    DRAINED.load(Ordering::Acquire)
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
    DRAINED.fetch_add(n as u64, Ordering::Relaxed);
    n
}
