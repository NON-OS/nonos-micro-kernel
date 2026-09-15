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

//! The three things the included controller files take from `nonos_libc`.
//!
//! Standing in, not stubbing out: `Deadline` keeps real time so a wait that
//! the specification says must give up really gives up, `mk_yield` yields,
//! and `mk_irq_wait` answers that there is no interrupt grant, which is a
//! state the driver already handles by falling back to a yield.

use std::time::{Duration, Instant};

pub struct Deadline {
    end: Instant,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end: Instant::now() + Duration::from_millis(timeout_ms) }
    }
    pub fn expired(&self) -> bool {
        Instant::now() >= self.end
    }
}

pub fn mk_yield() -> i64 {
    std::thread::yield_now();
    0
}

/// No interrupt path exists on the host. A negative answer is what the driver
/// gets from a kernel that refused the grant, and it copes the same way.
pub extern "C" fn mk_irq_wait(_grant: u64, _last_seq: u64, _timeout_ms: u64, _out: *mut u64) -> i64 {
    -1
}
