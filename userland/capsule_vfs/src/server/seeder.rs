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
// queues yet, so every request in that window burns the client timeout. Run
// before the receive loop it makes vfs_pool deaf for the whole stretch and each
// caller times out unanswered. Staging therefore runs from the loop's idle slot
// once the inbox is quiet, retried on a widening quiet period.
use nonos_libc::mk_debug;

use crate::store::Store;

const POLL_MS: u64 = 250;
const QUIET_POLLS: u32 = 2;
const MAX_ATTEMPTS: u32 = 5;

pub struct PackageSeeder {
    attempts: u32,
    quiet: u32,
    done: bool,
}

impl PackageSeeder {
    pub fn new() -> Self {
        Self { attempts: 0, quiet: 0, done: false }
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

    pub fn on_idle(&mut self, store: &mut Store) {
        if self.done {
            return;
        }
        self.quiet += 1;
        if self.quiet < QUIET_POLLS + self.attempts {
            return;
        }
        self.quiet = 0;
        self.attempts += 1;
        if store.seed_packages() {
            self.done = true;
            note(b"[VFSD] packages staged\n");
        } else if self.attempts >= MAX_ATTEMPTS {
            self.done = true;
            note(b"[VFSD] packages unavailable\n");
        }
    }
}

fn note(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
