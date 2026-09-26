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

//! Staging on a store that is never quiet.

use crate::store::Store;

use super::seeder::PackageSeeder;

/// The longest staging waits for a quiet moment before it takes one anyway.
const STARVE_MS: i64 = 1_000;

/*
 * The idle slot needs several receive timeouts in a row, and any caller
 * polling faster than that resets the count. A desktop reading the store
 * generation does, so staging never began and nothing written to the disk
 * came back on the next boot. A slice is a few milliseconds, so taking one
 * after a request, once a second at most, costs callers almost nothing.
 */
impl PackageSeeder {
    pub fn on_busy(&mut self, store: &mut Store) {
        let now = nonos_libc::mk_uptime_ms();
        if self.done || now.saturating_sub(self.last_slice_ms) < STARVE_MS {
            return;
        }
        self.advance(store);
    }
}
