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

use core::sync::atomic::{AtomicUsize, Ordering};

use super::directory_outcome::record;
use crate::directory_sync::sync_step;
use crate::setup;
use crate::state::directory_gateway_count;
use crate::trace;

/// Idle ticks to sit out before trying again, doubling on each failure.
pub(super) static SKIP: AtomicUsize = AtomicUsize::new(0);
/// Ticks to wait after the first failure. It starts high rather than at one
/// because every attempt costs the capsule a pause, and an attempt that just
/// failed is unlikely to succeed on the next tick.
pub(super) static BACKOFF: AtomicUsize = AtomicUsize::new(16);

/// Ceiling on the wait. The directory is what makes the mixnet usable, so
/// giving up on it entirely would leave the capsule permanently degraded, but
/// retrying at tick rate against an API that is down helps nobody.
pub(super) const MAX_BACKOFF: usize = 256;

/// Fetch the node list while nothing is asking to be served.
///
/// Until this runs there is no directory, and everything downstream is shaped
/// by that: gateways come from the compiled list, no exit can be found, and a
/// route home cannot be built because our own gateway is not described
/// anywhere. The fetch existed but nothing ever asked for it, so it never ran.
///
/// It is done on an idle tick because it takes a round trip and a handshake,
/// and a capsule that stops answering while it waits is indistinguishable
/// from one that died.
pub fn directory_tick() {
    // One good fetch is enough to unblock everything that needs a directory.
    if directory_gateway_count() > 0 {
        return;
    }
    let skip = SKIP.load(Ordering::Relaxed);
    if skip > 0 {
        SKIP.store(skip - 1, Ordering::Relaxed);
        return;
    }
    let tcp_port = setup::tcp_port();
    if tcp_port == 0 {
        return;
    }

    trace::say(b"directory: fetching");
    record(sync_step(tcp_port));
}
