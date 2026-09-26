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

//! The parked guests: one entry per thread waiting inside a syscall this
//! kernel refused, holding its frame until its supervisor answers.

use alloc::vec::Vec;

use spin::Mutex;

use super::frame::ForeignFrame;
use super::registry;

pub(super) struct Parked {
    pub frame: ForeignFrame,
    /// Set by the supervisor's reply, read by the guest on wake.
    pub answer: Option<u64>,
    /// Taken by the first supervisor wait that claims it.
    pub claimed: bool,
}

pub(super) static PARKED: Mutex<Vec<Parked>> = Mutex::new(Vec::new());

/// Park `frame`, unless its supervisor has gone in the meantime. False
/// says nothing was parked and the guest must be refused instead.
pub(super) fn park(frame: ForeignFrame) -> bool {
    let pid = frame.pid;
    let mut parked = PARKED.lock();
    parked.push(Parked { frame, answer: None, claimed: false });
    // Checked with the table held, and after the push rather than before.
    if registry::is_foreign(pid) {
        return true;
    }
    parked.pop();
    false
}

/// The answer for `pid`, removing the entry once it is taken.
pub(super) fn take_answer(pid: u32) -> Option<u64> {
    let mut parked = PARKED.lock();
    let at = parked.iter().position(|p| p.frame.pid == pid)?;
    let value = parked[at].answer?;
    parked.remove(at);
    Some(value)
}
