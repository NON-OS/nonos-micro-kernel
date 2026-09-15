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

//! Advancing the staging load from the receive loop's idle slot.

use crate::blk::load::{Load, Step};
use crate::store::Store;

use super::seeder::{note, PackageSeeder, MAX_ATTEMPTS, QUIET_POLLS, SLICE_MS};

impl PackageSeeder {
    pub fn on_idle(&mut self, store: &mut Store) {
        if self.done {
            return;
        }
        self.quiet += 1;
        if self.quiet < QUIET_POLLS + self.attempts {
            return;
        }
        self.quiet = 0;
        if self.load.is_none() {
            self.attempts += 1;
            match Load::begin() {
                Ok(load) => self.load = Some(load),
                Err(e) => {
                    crate::blk::status::record(&e);
                    if self.attempts >= MAX_ATTEMPTS {
                        self.done = true;
                        note(b"[VFSD] packages unavailable\n");
                    }
                    return;
                }
            }
        }
        let Some(load) = self.load.as_mut() else {
            return;
        };
        match load.step_for(SLICE_MS) {
            Step::More => {}
            Step::Done(staged) => {
                store.adopt_staged(staged);
                self.load = None;
                self.done = true;
                note(b"[VFSD] packages staged\n");
            }
            Step::Failed(e) => {
                crate::blk::status::record(&e);
                self.load = None;
                if self.attempts >= MAX_ATTEMPTS {
                    self.done = true;
                    note(b"[VFSD] packages unavailable\n");
                }
            }
        }
    }
}
