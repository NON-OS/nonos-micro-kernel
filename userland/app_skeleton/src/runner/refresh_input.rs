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

use crate::app::App;
use crate::discover::Peers;
use crate::setup::ensure_input_subscription;

use super::boot::BootedApp;

// Frames between unconditional re-subscribe heartbeats. At one service frame
// per vsync this is on the order of two seconds, cheap enough to run forever
// and short enough that input recovers quickly.
const RESUBSCRIBE_FRAMES: u32 = 120;

pub(super) fn refresh_input<A: App>(
    booted: &mut BootedApp<A>,
    peers: &Peers,
    request_id: &mut u32,
) {
    // Re-assert the subscription on a heartbeat, not only after a failed
    // first attempt. The input router can drop a subscription out from under
    // us (dead-pid purge, router restart, table churn); without a periodic
    // re-assert the app would go input-deaf for the rest of the session.
    booted.input_beat = booted.input_beat.wrapping_add(1);
    let heartbeat_due = booted.input_beat % RESUBSCRIBE_FRAMES == 0;
    if !booted.input_ready || heartbeat_due {
        booted.input_ready =
            ensure_input_subscription(peers.input_router, &booted.manifest, request_id);
    }
}
