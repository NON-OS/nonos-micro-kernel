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

use core::sync::atomic::{AtomicBool, AtomicU64};
use spin::Mutex;

use super::super::types::{InputEvent, INPUT_RING_CAP};

// MPSC ring: many driver capsules post (kbd, mouse, touch); a single
// input_router capsule drains. Posts and drains both take the mutex
// during the short critical section held over the event ring stores.
// Per-source SPSC fanout lives inside the router capsule.

pub(super) struct Ring {
    pub(super) head: usize,
    pub(super) tail: usize,
    pub(super) buf: [InputEvent; INPUT_RING_CAP],
}

pub(super) static RING: Mutex<Ring> = Mutex::new(Ring {
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

pub(super) static DROPPED: AtomicU64 = AtomicU64::new(0);
pub(super) static SEQ: AtomicU64 = AtomicU64::new(0);
pub(super) static WAITER: AtomicU64 = AtomicU64::new(0);
pub(super) static FIRST_INPUT_POST: AtomicBool = AtomicBool::new(false);
#[cfg(feature = "input-probe-inject")]
pub(super) static INPUT_CONSUMER_READY: AtomicBool = AtomicBool::new(false);

/// Diagnostic sentinel events. A driver posts one of these to report a milestone
/// the on-screen bars display; they never enter the ring or route anywhere.
///   slot 0: a ps2 port read succeeded (the PIO grant is live)
///   slot 1: reached serving loop      slot 2: i2c-HID touchpad found
///   slot 3: i8042 produced a byte     slot 4: bound i2c controller (index+1)
///   slot 5: the i2c bind was probe-confirmed by the touchpad's ACK
///   slot 6: raw non-empty touchpad report read (pre-decode, grows per report)
///   slot 7: the touchpad acknowledged RESET (wake handshake confirmed)
pub const KIND_DIAG_BASE: u16 = 0xFE00;
pub(super) const DIAG_SLOTS: usize = 8;
#[allow(clippy::declare_interior_mutable_const)]
const DIAG_INIT: AtomicU64 = AtomicU64::new(0);
pub(super) static DIAG: [AtomicU64; DIAG_SLOTS] = [DIAG_INIT; DIAG_SLOTS];

pub(super) static DRAINED: AtomicU64 = AtomicU64::new(0);
