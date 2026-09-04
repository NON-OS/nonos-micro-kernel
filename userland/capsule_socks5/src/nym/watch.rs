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

//! Whether the current exit has earned trust, decided by delivery.
//!
//! A directory lists an exit because its operator registered one, not
//! because its requester serves traffic right now. Answering the lookup
//! proves reachability of the node; only a reply that actually comes back
//! proves the requester behind it works. This watch holds that distinction:
//! an exit is unproven until something is delivered through it, and an
//! unproven exit that stays silent past its budget is walked away from.

/// Silence budget for an unproven exit.
///
/// A mixnet round trip is seconds, so this is several round trips: an exit
/// that has answered nothing for this long after our first send is treated
/// as absent rather than slow. Proven exits are never timed: one slow page
/// must not abandon a requester that has already carried traffic.
pub const SILENCE_MS: i64 = 12_000;

/// Delivery record for the exit currently in use.
pub struct Watch {
    /// Uptime of the first send since this exit was chosen; 0 = none yet.
    pub first_send_ms: i64,
    /// A message came back through this exit.
    pub proven: bool,
    /// The operator chose this exit by hand. Their choice is never rotated
    /// away: routing traffic to a node they did not pick is worse than
    /// failing in plain sight.
    pub configured: bool,
}

impl Default for Watch {
    fn default() -> Self {
        Self::new()
    }
}

impl Watch {
    pub const fn new() -> Self {
        Self { first_send_ms: 0, proven: false, configured: false }
    }

    /// First send stamps the clock; later sends leave it alone, so the
    /// budget runs from the first unanswered request, not the newest.
    pub fn on_send(&mut self, now_ms: i64) {
        if self.first_send_ms == 0 {
            self.first_send_ms = now_ms;
        }
    }

    /// Anything delivered proves the exit and ends all rotation for it.
    pub fn on_delivered(&mut self) {
        self.proven = true;
    }

    /// Whether the exit has used up its silence budget.
    pub fn should_rotate(&self, now_ms: i64) -> bool {
        !self.configured
            && !self.proven
            && self.first_send_ms != 0
            && now_ms.saturating_sub(self.first_send_ms) >= SILENCE_MS
    }

    /// A new exit starts with a clean record.
    pub fn on_rotate(&mut self) {
        self.first_send_ms = 0;
        self.proven = false;
    }
}
