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

//! How much a guest actually holds at an address.

use super::handle::Guest;

impl Guest {
    /// Bytes mapped contiguously from `addr`, zero when nothing is.
    pub fn mapped_from(&self, addr: u64) -> u64 {
        let mut reach = addr;
        loop {
            let Some(r) = self.regions.iter().find(|r| r.at <= reach && reach < r.at + r.len)
            else {
                break;
            };
            /*
             * Walked rather than answered from the one region, because a guest
             * that maps twice at adjoining addresses holds one run of pages
             * and the list remembers two.
             */
            reach = r.at + r.len;
        }
        reach - addr
    }
}
