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

//! Per-process IPC inboxes.
//!
//! Every inbox is registered with an explicit owner pid. Sends use
//! `try_enqueue_strict`, which fails closed when the inbox is
//! missing, when its owner pid has exited, or when the bounded queue
//! is full. There is no auto-registration on the IPC paths; the only
//! way to materialise an inbox is `register_inbox(name, owner_pid)`
//! (capsule-owned) or `register_or_get_bootstrap_inbox(name)`
//! (kernel-owned reply inbox, set up at spawn time).
//!
//! Capsule lifecycle integrates through `unregister_for_pid(pid)`,
//! called from `process::exit::teardown` to drop a dying capsule's
//! `proc.{pid}` inbox along with everything still queued in it.

mod error;
mod inbox;
mod registry;
mod stats;

pub use error::{InboxError, StrictEnqueueError};
pub use registry::{
    capacity, clear, exists, get_default_capacity, get_global_stats, get_inbox_stats, inbox_count,
    is_empty, is_full, len, list_inboxes, peek, register_inbox, register_inbox_with_capacity,
    register_or_get_bootstrap_inbox, set_default_capacity, try_dequeue_existing,
    try_enqueue_strict, unregister_for_pid, unregister_inbox, DEFAULT_INBOX_CAPACITY, KERNEL_OWNER,
    MAX_INBOX_CAPACITY, MIN_INBOX_CAPACITY,
};
pub use stats::InboxStatsSnapshot;
