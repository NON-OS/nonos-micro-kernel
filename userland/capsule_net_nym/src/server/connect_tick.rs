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

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::gateway_client;
use crate::setup;

static CONNECTED: AtomicBool = AtomicBool::new(false);
static NEXT: AtomicUsize = AtomicUsize::new(0);
/// Idle ticks left to sit out before the next attempt.
static SKIP: AtomicUsize = AtomicUsize::new(0);
/// How many ticks the next failure will wait, doubling each time.
static BACKOFF: AtomicUsize = AtomicUsize::new(1);

/// Ceiling on the wait, in idle ticks. Retrying forever at tick rate would
/// put steady pressure on gateways run by other people, which is not a cost
/// this client gets to impose on them; giving up entirely would leave a
/// machine offline after one bad minute.
const MAX_BACKOFF: usize = 64;

/// Whether a gateway is bound.
pub fn connected() -> bool {
    CONNECTED.load(Ordering::Relaxed)
}

/// Try one bootstrap candidate, while nothing else is asking to be served.
///
/// One candidate per idle tick rather than the whole list at boot. Each stage
/// of a connection now waits a real round trip, so walking five gateways can
/// take tens of seconds, and a capsule cannot answer anyone while it does.
/// Everything downstream waits on this one, so blocking here stalls the
/// desktop on a handshake nobody asked for yet.
pub fn connect_tick() {
    if CONNECTED.load(Ordering::Relaxed) {
        return;
    }
    let skip = SKIP.load(Ordering::Relaxed);
    if skip > 0 {
        SKIP.store(skip - 1, Ordering::Relaxed);
        return;
    }
    let index = NEXT.fetch_add(1, Ordering::Relaxed);
    if gateway_client::connect_candidate(setup::tcp_port(), index) {
        CONNECTED.store(true, Ordering::Relaxed);
        return;
    }
    let wait = BACKOFF.load(Ordering::Relaxed);
    SKIP.store(wait, Ordering::Relaxed);
    BACKOFF.store((wait * 2).min(MAX_BACKOFF), Ordering::Relaxed);
}
