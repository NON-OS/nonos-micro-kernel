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

//! Per-module message inbox.

extern crate alloc;

use core::sync::atomic::Ordering;
use spin::Mutex;

use super::stats::{InboxStats, InboxStatsSnapshot};
use crate::ipc::nonos_channel::IpcMessage;

/// Per-module message inbox with bounded capacity. `owner` is the
/// pid that registered the inbox; `0` is kernel-owned (the reply
/// inboxes that capsule_spawn pre-registers). A non-zero owner is
/// liveness-checked on every strict enqueue so the kernel cannot
/// route a message to a queue whose draining capsule has exited.
pub(super) struct Inbox {
    queue: Mutex<alloc::collections::VecDeque<IpcMessage>>,
    capacity: usize,
    owner: u32,
    stats: InboxStats,
}

impl Inbox {
    pub(super) fn new(capacity: usize, owner: u32) -> Self {
        let queue = alloc::collections::VecDeque::with_capacity(capacity);
        Self { queue: Mutex::new(queue), capacity, owner, stats: InboxStats::new() }
    }

    #[inline]
    pub(super) fn owner(&self) -> u32 {
        self.owner
    }

    /// Check if inbox is full
    #[inline]
    pub(super) fn is_full(&self) -> bool {
        self.queue.lock().len() >= self.capacity
    }

    /// Check if inbox is empty
    #[inline]
    pub(super) fn is_empty(&self) -> bool {
        self.queue.lock().is_empty()
    }

    /// Get current queue length
    #[inline]
    pub(super) fn len(&self) -> usize {
        self.queue.lock().len()
    }

    /// Get inbox capacity
    #[inline]
    pub(super) fn capacity(&self) -> usize {
        self.capacity
    }

    /// Try to enqueue without blocking
    pub(super) fn try_enqueue(&self, msg: IpcMessage) -> Result<(), IpcMessage> {
        let mut q = self.queue.lock();
        if q.len() < self.capacity {
            q.push_back(msg);
            let size = q.len();
            drop(q);
            self.stats.record_enqueue(size);
            Ok(())
        } else {
            self.stats.record_dropped();
            Err(msg)
        }
    }

    /// Dequeue next message
    #[inline]
    pub(super) fn dequeue(&self) -> Option<IpcMessage> {
        let msg = self.queue.lock().pop_front()?;
        self.stats.record_dequeue();
        Some(msg)
    }

    /// Peek at next message without removing
    pub(super) fn peek(&self) -> Option<IpcMessage> {
        self.queue.lock().front().cloned()
    }

    /// Get statistics snapshot
    pub(super) fn get_stats(&self) -> InboxStatsSnapshot {
        InboxStatsSnapshot {
            enqueued: self.stats.enqueued.load(Ordering::Relaxed),
            dequeued: self.stats.dequeued.load(Ordering::Relaxed),
            dropped_full: self.stats.dropped_full.load(Ordering::Relaxed),
            timeouts: self.stats.timeouts.load(Ordering::Relaxed),
            peak_size: self.stats.peak_size.load(Ordering::Relaxed),
            current_size: self.len(),
            capacity: self.capacity,
        }
    }

    /// Clear all messages from inbox
    pub(super) fn clear(&self) -> usize {
        let mut q = self.queue.lock();
        let count = q.len();
        q.clear();
        count
    }
}
