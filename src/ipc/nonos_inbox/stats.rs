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

//! Inbox statistics types.

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Statistics for a single inbox
#[derive(Debug, Default)]
pub(super) struct InboxStats {
    pub enqueued: AtomicU64,
    pub dequeued: AtomicU64,
    pub dropped_full: AtomicU64,
    pub timeouts: AtomicU64,
    pub peak_size: AtomicUsize,
}

impl InboxStats {
    pub(super) const fn new() -> Self {
        Self {
            enqueued: AtomicU64::new(0),
            dequeued: AtomicU64::new(0),
            dropped_full: AtomicU64::new(0),
            timeouts: AtomicU64::new(0),
            peak_size: AtomicUsize::new(0),
        }
    }

    pub(super) fn record_enqueue(&self, current_size: usize) {
        self.enqueued.fetch_add(1, Ordering::Relaxed);
        // Update peak if current size is higher
        let mut peak = self.peak_size.load(Ordering::Relaxed);
        while current_size > peak {
            match self.peak_size.compare_exchange_weak(
                peak,
                current_size,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(p) => peak = p,
            }
        }
    }

    pub(super) fn record_dequeue(&self) {
        self.dequeued.fetch_add(1, Ordering::Relaxed);
    }

    pub(super) fn record_dropped(&self) {
        self.dropped_full.fetch_add(1, Ordering::Relaxed);
    }
}

/// Snapshot of inbox statistics
#[derive(Debug, Clone, Copy)]
pub struct InboxStatsSnapshot {
    /// Total messages enqueued
    pub enqueued: u64,
    /// Total messages dequeued
    pub dequeued: u64,
    /// Messages dropped due to full inbox
    pub dropped_full: u64,
    /// Enqueue timeouts
    pub timeouts: u64,
    /// Peak queue size observed
    pub peak_size: usize,
    /// Current queue size
    pub current_size: usize,
    /// Inbox capacity
    pub capacity: usize,
}

impl core::fmt::Display for InboxStatsSnapshot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Inbox[enq:{} deq:{} drop:{} timeout:{} size:{}/{} peak:{}]",
            self.enqueued,
            self.dequeued,
            self.dropped_full,
            self.timeouts,
            self.current_size,
            self.capacity,
            self.peak_size
        )
    }
}
