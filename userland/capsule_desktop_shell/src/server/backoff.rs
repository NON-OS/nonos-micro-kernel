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

//! Widening the gap between attempts at a service that is not answering.
//!
//! vfs_pool spends its first seconds staging packages off the block device,
//! where it answers nobody. Anything that polls it through that window spends a
//! full IPC timeout per attempt, inside the shell's own loop, so the desktop
//! stalls behind each one. Asking harder does not make it answer sooner.
//!
//! Shared, because the shell now has two of these and they made the same
//! mistake independently.

use core::sync::atomic::{AtomicU64, Ordering};

use nonos_libc::mk_uptime_ms;

pub struct Backoff {
    next_ms: AtomicU64,
    gap_ms: AtomicU64,
    first_ms: u64,
    max_ms: u64,
}

impl Backoff {
    pub const fn new(first_ms: u64, max_ms: u64) -> Self {
        Backoff { next_ms: AtomicU64::new(0), gap_ms: AtomicU64::new(first_ms), first_ms, max_ms }
    }

    /// Whether enough time has passed to spend another call.
    pub fn due(&self) -> bool {
        now() >= self.next_ms.load(Ordering::Relaxed)
    }

    /// Record a miss and double the gap, to a ceiling.
    pub fn missed(&self) {
        let gap = self.gap_ms.load(Ordering::Relaxed);
        self.next_ms.store(now().saturating_add(gap), Ordering::Relaxed);
        self.gap_ms.store((gap * 2).min(self.max_ms), Ordering::Relaxed);
    }

    /// Record an answer. The gap resets so a service that goes quiet again is
    /// noticed promptly rather than at the ceiling it happened to reach before.
    pub fn answered(&self) {
        self.gap_ms.store(self.first_ms, Ordering::Relaxed);
        self.next_ms.store(0, Ordering::Relaxed);
    }
}

fn now() -> u64 {
    mk_uptime_ms().max(0) as u64
}
