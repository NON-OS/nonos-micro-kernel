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

//! What to do with the result of one fetch step.

use core::sync::atomic::Ordering;

use super::directory_tick::{BACKOFF, MAX_BACKOFF, SKIP};
use crate::directory_sync::Step;
use crate::trace;

/// Idle ticks to wait before re-syncing when a directory arrived with gateways
/// but no exit. Long enough not to hammer the API, short enough that the
/// mixnet becomes usable within moments of the boot-time pressure clearing.
const RESYNC_FOR_EXIT: usize = 1;

/// Record a step, and hold off after one that did not arrive.
pub fn record(step: Step) {
    match step {
        Step::Progressed => {
            trace::say(b"directory: mix layers in hand, gateways next");
            BACKOFF.store(1, Ordering::Relaxed);
        }
        Step::Done(count) => {
            trace::say_num(b"directory synced, nodes", count as u64);
            // The gateway in hand was dialled before there was a directory,
            // so it is probably not in the one that just arrived.
            super::rebind::rebind_if_unknown();
            BACKOFF.store(1, Ordering::Relaxed);
            // A sync missing a gateway or an exit is not usable: the tick will
            // run again because one of those counts is still zero, but
            // re-fetching the whole list every tick would hammer the directory.
            // Sit out a fixed stretch first, long enough for the boot-time
            // crypto pressure that failed the handshake to clear, without giving
            // up on it.
            if crate::state::directory_gateway_count() == 0
                || crate::state::directory_exit_count() == 0
            {
                SKIP.store(RESYNC_FOR_EXIT, Ordering::Relaxed);
            }
        }
        Step::Failed(code) => {
            trace::say_num(b"directory sync failed, code", code as u64);
            let wait = BACKOFF.load(Ordering::Relaxed);
            SKIP.store(wait, Ordering::Relaxed);
            BACKOFF.store((wait + 1).min(MAX_BACKOFF), Ordering::Relaxed);
        }
    }
}
