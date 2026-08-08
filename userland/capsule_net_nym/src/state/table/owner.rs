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

use super::types::Table;

impl Table {
    pub fn has_session(&self, owner: u32, id: u32) -> bool {
        self.sessions.iter().any(|s| s.owner == owner && s.id == id)
    }

    pub fn ids_for_owner(&self, owner: u32) -> Vec<u32> {
        self.sessions.iter().filter(|s| s.owner == owner).map(|s| s.id).collect()
    }
}
