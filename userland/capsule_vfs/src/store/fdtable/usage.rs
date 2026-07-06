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

use super::types::{Store, MAX_FILES};

impl Store {
    // Store occupancy: current entry count, total bytes held, and the entry
    // ceiling, so a caller can show how full the filesystem is.
    pub fn usage(&self) -> (u32, u64, u32) {
        let bytes = self.files.iter().map(|f| f.data.len() as u64).sum();
        (self.files.len() as u32, bytes, MAX_FILES as u32)
    }
}
