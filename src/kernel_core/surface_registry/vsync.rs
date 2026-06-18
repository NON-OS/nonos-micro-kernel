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

use core::sync::atomic::{AtomicU64, Ordering};

const TARGET_HZ_DEFAULT: u32 = 60;
const NS_PER_MS: u64 = 1_000_000;
const NS_PER_SEC: u64 = 1_000_000_000;

static LAST_VBLANK_NS: AtomicU64 = AtomicU64::new(0);
static TARGET_HZ: AtomicU64 = AtomicU64::new(TARGET_HZ_DEFAULT as u64);

pub fn vsync_period_ns(display_id: u32) -> u64 {
    if display_id != 0 {
        return 0;
    }
    let hz = TARGET_HZ.load(Ordering::Acquire).max(1);
    NS_PER_SEC / hz
}

fn deadline_ms(deadline_ns: u64) -> u64 {
    deadline_ns.saturating_add(NS_PER_MS - 1) / NS_PER_MS
}

pub fn wait_for_vsync(display_id: u32, pid: u32) -> Result<u64, super::types::RegistryError> {
    if display_id != 0 {
        return Err(super::types::RegistryError::InvalidArg);
    }
    let period = vsync_period_ns(display_id);
    let now = crate::time::now_ns();
    // Next vblank on a fixed phase grid derived from absolute time. Every
    // surface that waits within the same frame lands on the same boundary and
    // wakes together. The previous version advanced one shared running
    // deadline per waiter, so the compositor, wm, shell, and cursor were each
    // pushed a full period past the last caller and ended up refreshing at
    // target_hz / number_of_waiters, which is what made the desktop feel slow.
    let deadline = (now / period + 1) * period;
    while crate::time::now_ns() < deadline {
        crate::sched::sleep_until(pid, deadline_ms(deadline));
        crate::sched::yield_now();
    }
    // Published only as a monotonic vblank timestamp for readers; it no longer
    // feeds back into the next deadline, so waiters can never serialize.
    LAST_VBLANK_NS.fetch_max(deadline, Ordering::AcqRel);
    Ok(deadline)
}
