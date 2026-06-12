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

pub use core::time::Duration;

fn now_millis() -> u64 {
    let t = nonos_libc::mk_time_millis();
    if t < 0 {
        0
    } else {
        t as u64
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(u64);

impl Instant {
    pub fn now() -> Self {
        Instant(now_millis())
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_millis(self.0.saturating_sub(earlier.0))
    }

    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SystemTime(u64);

pub const UNIX_EPOCH: SystemTime = SystemTime(0);

impl SystemTime {
    pub fn now() -> Self {
        SystemTime(now_millis())
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, Duration> {
        if self.0 >= earlier.0 {
            Ok(Duration::from_millis(self.0 - earlier.0))
        } else {
            Err(Duration::from_millis(earlier.0 - self.0))
        }
    }
}
