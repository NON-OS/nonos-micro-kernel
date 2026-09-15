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

// Package staging walks the block device, whose driver may not have bound its
// queues yet, so every request in that window burns the client timeout.
//
// Running it from the idle slot was not enough. One pass read the whole
// container: a capacity query, the header, the table of contents, then every
// payload in whole-sector chunks against a driver that caps a request, plus a
// digest each. Hundreds of block round trips with the receive loop suspended
// for all of them, which is why the desktop shell logged forty unanswered calls
// a boot and three separate clients grew backoffs to cope.
//
// The load is resumable now and runs on a time budget, so the receive path gets
// control back after a few milliseconds and the longest anyone waits is a single
// block request.
use nonos_libc::mk_debug;

use crate::blk::load::Load;

const POLL_MS: u64 = 250;

/// How long one staging slice may hold the receive loop.
///
/// Short enough that a caller arriving mid-slice waits less than a frame, long
/// enough that an idle machine stages at close to device speed rather than one
/// chunk per poll.
pub(super) const SLICE_MS: u64 = 8;
pub(super) const QUIET_POLLS: u32 = 2;
pub(super) const MAX_ATTEMPTS: u32 = 5;

pub struct PackageSeeder {
    pub(super) attempts: u32,
    pub(super) quiet: u32,
    pub(super) done: bool,
    /// The load in progress. Held across idle slots, which is the whole point:
    /// each slot advances it and hands the receive loop back.
    pub(super) load: Option<Load>,
}

impl PackageSeeder {
    pub fn new() -> Self {
        Self { attempts: 0, quiet: 0, done: false, load: None }
    }

    pub fn poll_ms(&self) -> u64 {
        if self.done {
            0
        } else {
            POLL_MS
        }
    }

    pub fn saw_request(&mut self) {
        self.quiet = 0;
    }
}

pub(super) fn note(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
