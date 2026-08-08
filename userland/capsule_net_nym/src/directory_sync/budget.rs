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

//! The wall clock an attempt runs against.

use nonos_libc::mk_uptime_ms;

/// How long a whole fetch may take before it is abandoned.
///
/// This capsule serves from one thread, so every millisecond spent here is a
/// millisecond it is not answering anyone. That argues for a small bound, and
/// the lists argue against one: the gateway list is seventy five kilobytes
/// over a fresh TLS session, and a bound that cannot carry it means a
/// directory with mix layers and no gateways, which is worse than a pause.
///
/// So it is sized to complete rather than to stay quick, and the cost is paid
/// where it is cheapest: the three fetches happen once, at startup, before
/// anything is browsing, and never again once a directory is in hand. What it
/// still rules out is the case it was added for, a session waiting forever on
/// a peer that has stopped talking.
const FETCH_BUDGET_MS: i64 = 12_000;

/// The budget one attempt runs against.
pub struct Budget {
    deadline_ms: i64,
    out_of_time: bool,
}

impl Budget {
    pub fn new() -> Self {
        Self { deadline_ms: mk_uptime_ms().saturating_add(FETCH_BUDGET_MS), out_of_time: false }
    }

    /// Whether the time is up. Records it, so the caller can tell afterwards
    /// why the attempt ended.
    pub fn spent(&mut self) -> bool {
        if mk_uptime_ms() >= self.deadline_ms {
            self.out_of_time = true;
            return true;
        }
        false
    }

    /// Whether this attempt ended because it ran out of time rather than
    /// because the far end said something wrong. The two need different
    /// answers: one is a budget to widen, the other is a peer to stop asking.
    pub fn overran(&self) -> bool {
        self.out_of_time
    }
}
