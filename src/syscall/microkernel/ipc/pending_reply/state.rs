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

extern crate alloc;

use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use spin::Mutex;

// Each pending entry is (caller pid, caller reply inbox, correlation token).
// Carrying the caller's per-call token lets the redirect-reply path (a service
// replying via `mk_ipc_send` to its own fixed reply endpoint) stamp the reply
// with the correlation the caller waits on, so the caller can reject a forged
// reply (which can only ever carry correlation 0). The pid rides alongside so
// that same path can wake the caller once the reply is enqueued: a service's
// reply inbox is owned by pid 0, so the router's own wake is skipped for it.
pub(super) static PENDING: Mutex<BTreeMap<u32, VecDeque<(u32, String, u64)>>> =
    Mutex::new(BTreeMap::new());

pub(super) const MAX_PER_SERVICE: usize = 64;

// The correlation of the request each server dequeued last, recorded by the
// receive path and consumed by the reply redirect. Position in the queue
// cannot pair a reply with its caller: kernel-side senders push no pending
// entry but their requests still occupy the server's inbox, so every one of
// them shifted the FIFO and handed some capsule caller's reply to the wrong
// call. The token names the request outright. Lock-free fixed table because
// this is touched on every dequeue, including under held IPC locks.
const SERVED_SLOTS: usize = 1024;
#[allow(clippy::declare_interior_mutable_const)]
const SERVED_SLOT_INIT: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
static LAST_SERVED: [core::sync::atomic::AtomicU64; SERVED_SLOTS] =
    [SERVED_SLOT_INIT; SERVED_SLOTS];

pub(crate) fn record_served(server_pid: u32, correlation: u64) {
    LAST_SERVED[server_pid as usize % SERVED_SLOTS]
        .store(correlation, core::sync::atomic::Ordering::Release);
}

pub(super) fn last_served(server_pid: u32) -> u64 {
    LAST_SERVED[server_pid as usize % SERVED_SLOTS].load(core::sync::atomic::Ordering::Acquire)
}
