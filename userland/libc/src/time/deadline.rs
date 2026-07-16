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

use super::mk_uptime_ms;

/// A monotonic timeout deadline measured against the boot uptime clock, so it is
/// immune to wall-clock adjustments and usable before the wall clock is set. It
/// gives drivers a CPU-speed-independent timeout in place of a raw spin count.
#[derive(Clone, Copy)]
pub struct Deadline {
    end_ms: u64,
}

impl Deadline {
    /// A deadline `timeout_ms` milliseconds from the current uptime.
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self::at(read_uptime_ms().saturating_add(timeout_ms))
    }

    /// A deadline at an explicit monotonic timestamp (milliseconds since boot).
    pub const fn at(end_ms: u64) -> Self {
        Self { end_ms }
    }

    /// True once the given uptime has reached the deadline.
    pub const fn is_past(&self, now_ms: u64) -> bool {
        now_ms >= self.end_ms
    }

    /// True once the current uptime has reached the deadline.
    pub fn expired(&self) -> bool {
        self.is_past(read_uptime_ms())
    }
}

fn read_uptime_ms() -> u64 {
    let t = mk_uptime_ms();
    if t < 0 {
        0
    } else {
        t as u64
    }
}
