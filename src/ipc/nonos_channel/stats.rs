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

//! Bus statistics types.

use core::sync::atomic::AtomicU64;

/// Global bus statistics
pub(super) struct BusStats {
    pub messages_enqueued: AtomicU64,
    pub messages_dequeued: AtomicU64,
    pub messages_timed_out: AtomicU64,
    pub channels_opened: AtomicU64,
    pub channels_closed: AtomicU64,
    pub bytes_transferred: AtomicU64,
    pub queue_full_rejections: AtomicU64,
}

impl BusStats {
    pub(super) const fn new() -> Self {
        Self {
            messages_enqueued: AtomicU64::new(0),
            messages_dequeued: AtomicU64::new(0),
            messages_timed_out: AtomicU64::new(0),
            channels_opened: AtomicU64::new(0),
            channels_closed: AtomicU64::new(0),
            bytes_transferred: AtomicU64::new(0),
            queue_full_rejections: AtomicU64::new(0),
        }
    }
}

/// Snapshot of bus statistics
#[derive(Debug, Clone, Copy)]
pub struct BusStatsSnapshot {
    /// Total messages enqueued
    pub messages_enqueued: u64,
    /// Total messages dequeued
    pub messages_dequeued: u64,
    /// Total messages that timed out
    pub messages_timed_out: u64,
    /// Total channels opened
    pub channels_opened: u64,
    /// Total channels closed
    pub channels_closed: u64,
    /// Total bytes transferred
    pub bytes_transferred: u64,
    /// Queue full rejections
    pub queue_full_rejections: u64,
    /// Current queue depth
    pub current_queue_depth: usize,
    /// Current channel count
    pub current_channel_count: usize,
}

impl core::fmt::Display for BusStatsSnapshot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Bus[enq:{} deq:{} timeout:{} ch:{}/{} bytes:{} reject:{}]",
            self.messages_enqueued,
            self.messages_dequeued,
            self.messages_timed_out,
            self.current_channel_count,
            self.channels_opened,
            self.bytes_transferred,
            self.queue_full_rejections
        )
    }
}

