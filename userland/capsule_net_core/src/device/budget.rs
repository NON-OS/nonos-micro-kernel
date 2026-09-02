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


//! Time budgets for the driver calls the serve loop makes.
//!
//! `net_core` is single threaded: the same loop that answers `net.dhcp.client`
//! and `net.dns` is the loop that talks to the NIC driver. A driver call is a
//! synchronous IPC round trip, so its timeout is time the loop cannot spend on
//! a client. Clients allow 64 ms for a reply, which is the ceiling every budget
//! here has to fit under together.
//!
//! [`DEVICE_CALL_MS`] bounds one call. [`POLL_WINDOW_MS`] bounds a whole
//! `iface.poll()`, which issues one call per frame and would otherwise be
//! bounded only by how much traffic the card has queued.

use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::mk_time_millis;

/// How long one driver round trip may take before it is written off.
///
/// A healthy driver answers a register read in about a millisecond; this is
/// sized for a stalled one. Writing a call off is never fatal — a link probe
/// reads as "no carrier" and retries next second, a frame read as "nothing
/// queued" and retries next poll.
pub const DEVICE_CALL_MS: u64 = 8;

/// How long the device may be polled for before the loop owes clients a turn.
pub const POLL_WINDOW_MS: i64 = 8;

static POLL_DEADLINE: AtomicI64 = AtomicI64::new(0);

/// Open a polling window. Driver traffic is only allowed inside one.
pub fn open_poll() {
    POLL_DEADLINE.store(mk_time_millis() + POLL_WINDOW_MS, Ordering::Relaxed);
}

/// Close the polling window, so nothing reaches the driver off the poll path.
pub fn close_poll() {
    POLL_DEADLINE.store(0, Ordering::Relaxed);
}

/// Whether there is still budget to spend on the device this poll.
pub fn poll_open() -> bool {
    mk_time_millis() < POLL_DEADLINE.load(Ordering::Relaxed)
}
