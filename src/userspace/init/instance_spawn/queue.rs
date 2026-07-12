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
    true
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
