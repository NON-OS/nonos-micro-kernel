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

use alloc::vec::Vec;
use spin::Mutex;

/// An app that declares instance-window endpoints in its signed manifest.
/// The syscall maps the requested handle to one of these before queueing.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PendingApp {
    Terminal,
    Browser,
    TextEditor,
    Settings,
    Calculator,
    Clock,
    About,
    Snake,
    WalletNonos,
    FileManager,
    ProcessManager,
    AudioPlayer,
    VideoPlayer,
}

// A click enqueues one request; a few in flight at once is the most a user
// produces, and the cap keeps a stuck drain from growing without bound.
const MAX_PENDING: usize = 8;

pub(super) static PENDING: Mutex<Vec<PendingApp>> = Mutex::new(Vec::new());

/// Record a spawn request. Returns false only when the queue is saturated,
/// which the caller treats as "try again", never as a hard failure.
pub(super) fn push(app: PendingApp) -> bool {
    let mut q = PENDING.lock();
    if q.len() >= MAX_PENDING {
        return false;
    }
    q.push(app);
    super::priority::raise();
    true
}

/// Return init to its idle band once nothing is left to spawn. The check and
/// the demotion happen under the queue lock, the same lock `push` raises
/// under, so a click landing here can never be left queued behind a
/// demotion it raced.
pub(super) fn settle() {
    let Some(q) = PENDING.try_lock() else {
        return;
    };
    if q.is_empty() {
        super::priority::restore();
    }
}

/// Take everything queued so far. Called from init's context only.
pub(super) fn take() -> Vec<PendingApp> {
    let mut q = match PENDING.try_lock() {
        Some(q) => q,
        None => return Vec::new(),
    };
    if q.is_empty() {
        return Vec::new();
    }
    core::mem::take(&mut *q)
}

/// Whether any window-instance request is waiting to be drained. The init loop
/// reads this to raise its priority only while there is deferred window work. A
/// contended lock means a push is in flight, which is itself pending work, so it
/// counts as pending rather than risking a missed boost.
pub(crate) fn has_pending() -> bool {
    match PENDING.try_lock() {
        Some(q) => !q.is_empty(),
        None => true,
    }
}
