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

//! Global inbox registry.
//!
//! Production routing rule: every inbox has an owner pid, recorded
//! at registration. `try_enqueue_strict` fails with `MissingInbox`
//! if no row exists, with `DeadOwner` if the owner pid has fallen
//! out of `PROCESS_TABLE`, and with `QueueFull` if the bounded
//! queue is full. There is no auto-registration on the send/recv
//! paths. The only path that creates an inbox without an explicit
//! pid is `register_or_get_bootstrap_inbox`, used by `capsule_spawn`
//! to set up the kernel's reply inboxes (owner = 0 = kernel).

extern crate alloc;

use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use spin::RwLock;

use super::error::{InboxError, StrictEnqueueError};
use super::inbox::Inbox;
use super::stats::InboxStatsSnapshot;
use crate::ipc::nonos_channel::IpcMessage;

pub const DEFAULT_INBOX_CAPACITY: usize = 1024;
pub const MIN_INBOX_CAPACITY: usize = 16;
pub const MAX_INBOX_CAPACITY: usize = 65536;

/// `0` marks an inbox owned by the kernel rather than a capsule.
/// Used for reply inboxes the spawn pipeline pre-registers; never
/// liveness-checked.
pub const KERNEL_OWNER: u32 = 0;

struct Registry {
    map: BTreeMap<String, Arc<Inbox>>,
}

impl Registry {
    const fn new() -> Self {
        Self { map: BTreeMap::new() }
    }
}

static REGISTRY: RwLock<Registry> = RwLock::new(Registry::new());
static DEFAULT_CAP: AtomicUsize = AtomicUsize::new(DEFAULT_INBOX_CAPACITY);
static GLOBAL_STATS: GlobalStats = GlobalStats::new();

struct GlobalStats {
    total_inboxes_created: AtomicU64,
    total_inboxes_removed: AtomicU64,
}

impl GlobalStats {
    const fn new() -> Self {
        Self { total_inboxes_created: AtomicU64::new(0), total_inboxes_removed: AtomicU64::new(0) }
    }
}

pub fn set_default_capacity(cap: usize) {
    let clamped = cap.clamp(MIN_INBOX_CAPACITY, MAX_INBOX_CAPACITY);
    DEFAULT_CAP.store(clamped, Ordering::Relaxed);
}

pub fn get_default_capacity() -> usize {
    DEFAULT_CAP.load(Ordering::Relaxed)
}

/// Register an inbox owned by `owner_pid`. Errors if the name is
/// empty or already registered. The default capacity applies; use
/// [`register_inbox_with_capacity`] for a custom bound.
pub fn register_inbox(module: &str, owner_pid: u32) -> Result<(), InboxError> {
    register_inbox_with_capacity(module, owner_pid, DEFAULT_CAP.load(Ordering::Relaxed))
}

/// Register an inbox with an explicit capacity and owner pid.
pub fn register_inbox_with_capacity(
    module: &str,
    owner_pid: u32,
    capacity: usize,
) -> Result<(), InboxError> {
    if module.is_empty() {
        return Err(InboxError::EmptyModuleName);
    }
    if !(MIN_INBOX_CAPACITY..=MAX_INBOX_CAPACITY).contains(&capacity) {
        return Err(InboxError::InvalidCapacity {
            value: capacity,
            min: MIN_INBOX_CAPACITY,
            max: MAX_INBOX_CAPACITY,
        });
    }
    let mut reg = REGISTRY.write();
    if reg.map.contains_key(module) {
        return Err(InboxError::AlreadyRegistered { module: module.into() });
    }
    reg.map.insert(module.into(), Arc::new(Inbox::new(capacity, owner_pid)));
    GLOBAL_STATS.total_inboxes_created.fetch_add(1, Ordering::Relaxed);
    Ok(())
}

/// Bootstrap-only: idempotently ensure a kernel-owned inbox exists.
/// Used by the spawn pipeline to register reply inboxes the kernel
/// itself will drain. Owner is `KERNEL_OWNER`; never liveness-checked.
/// Must NOT be called from normal IPC send/recv paths.
pub fn register_or_get_bootstrap_inbox(module: &str) {
    if module.is_empty() {
        return;
    }
    let mut reg = REGISTRY.write();
    if !reg.map.contains_key(module) {
        let cap = DEFAULT_CAP.load(Ordering::Relaxed);
        reg.map.insert(module.into(), Arc::new(Inbox::new(cap, KERNEL_OWNER)));
        GLOBAL_STATS.total_inboxes_created.fetch_add(1, Ordering::Relaxed);
    }
}

/// Unregister by name, dropping all queued messages. Returns the
/// dropped count, or `None` if the inbox was not registered.
pub fn unregister_inbox(module: &str) -> Option<usize> {
    let mut reg = REGISTRY.write();
    if let Some(inbox) = reg.map.remove(module) {
        GLOBAL_STATS.total_inboxes_removed.fetch_add(1, Ordering::Relaxed);
        Some(inbox.len())
    } else {
        None
    }
}

/// Drop the canonical per-process inbox `proc.{pid}` for a dying
/// capsule. Called from `process::exit::teardown`. Reply inboxes
/// (`endpoint.<u64>`) are kernel-owned and intentionally left alone
/// so a respawn reuses them; stale replies are filtered by the
/// transport's generation re-check.
pub fn unregister_for_pid(pid: u32) -> Option<usize> {
    use alloc::format;
    let module = format!("proc.{}", pid);
    let mut reg = REGISTRY.write();
    if let Some(inbox) = reg.map.remove(module.as_str()) {
        GLOBAL_STATS.total_inboxes_removed.fetch_add(1, Ordering::Relaxed);
        Some(inbox.len())
    } else {
        None
    }
}

/// Strict enqueue. The inbox must exist; if its owner is not
/// `KERNEL_OWNER`, that pid must still be in `PROCESS_TABLE`. No
/// auto-registration. The owner liveness check covers the race
/// where exit teardown unregisters the endpoint+inbox between a
/// caller's `lookup_service` and the enqueue.
pub fn try_enqueue_strict(module: &str, msg: IpcMessage) -> Result<(), StrictEnqueueError> {
    let reg = REGISTRY.read();
    let inbox = reg.map.get(module).ok_or(StrictEnqueueError::MissingInbox)?;
    let owner = inbox.owner();
    if owner != KERNEL_OWNER && crate::process::get_process_table().find_by_pid(owner).is_none() {
        return Err(StrictEnqueueError::DeadOwner);
    }
    inbox.try_enqueue(msg).map_err(StrictEnqueueError::QueueFull)
}

/// Dequeue without auto-registration. Returns `None` if no inbox is
/// registered under that name. The dequeuing capsule must have been
/// pre-registered (the kernel for reply inboxes, `capsule_spawn` for
/// `proc.{pid}`).
pub fn try_dequeue_existing(module: &str) -> Option<IpcMessage> {
    let reg = REGISTRY.read();
    reg.map.get(module).and_then(|inbox| inbox.dequeue())
}

pub fn peek(module: &str) -> Option<IpcMessage> {
    REGISTRY.read().map.get(module).and_then(|i| i.peek())
}

pub fn len(module: &str) -> usize {
    REGISTRY.read().map.get(module).map(|i| i.len()).unwrap_or(0)
}

pub fn is_full(module: &str) -> bool {
    REGISTRY.read().map.get(module).map(|i| i.is_full()).unwrap_or(false)
}

pub fn is_empty(module: &str) -> bool {
    REGISTRY.read().map.get(module).map(|i| i.is_empty()).unwrap_or(true)
}

pub fn capacity(module: &str) -> Option<usize> {
    REGISTRY.read().map.get(module).map(|i| i.capacity())
}

pub fn exists(module: &str) -> bool {
    REGISTRY.read().map.contains_key(module)
}

pub fn get_inbox_stats(module: &str) -> Option<InboxStatsSnapshot> {
    REGISTRY.read().map.get(module).map(|i| i.get_stats())
}

pub fn clear(module: &str) -> usize {
    REGISTRY.read().map.get(module).map(|i| i.clear()).unwrap_or(0)
}

pub fn list_inboxes() -> Vec<String> {
    REGISTRY.read().map.keys().cloned().collect()
}

pub fn inbox_count() -> usize {
    REGISTRY.read().map.len()
}

pub fn get_global_stats() -> (u64, u64) {
    (
        GLOBAL_STATS.total_inboxes_created.load(Ordering::Relaxed),
        GLOBAL_STATS.total_inboxes_removed.load(Ordering::Relaxed),
    )
}
