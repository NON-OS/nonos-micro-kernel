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

//! Handing a waiting supervisor its next piece of work.

use super::frame::ForeignFrame;
use super::registry;
use super::trap_table::PARKED;

/// The next unclaimed frame for `supervisor`, if one is waiting.
pub(super) fn claim_next(supervisor: u32) -> Option<ForeignFrame> {
    let mut parked = PARKED.lock();
    for entry in parked.iter_mut() {
        if entry.claimed || entry.answer.is_some() {
            continue;
        }
        if registry::supervisor_of(entry.frame.pid) == Some(supervisor) {
            entry.claimed = true;
            return Some(entry.frame);
        }
    }
    None
}

/// Give a frame back after a delivery that did not happen.
pub(super) fn unclaim(pid: u32) {
    let mut parked = PARKED.lock();
    if let Some(entry) = parked.iter_mut().find(|p| p.frame.pid == pid) {
        entry.claimed = false;
    }
}
