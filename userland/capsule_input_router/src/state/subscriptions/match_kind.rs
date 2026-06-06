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

use super::SubscriptionTable;

impl SubscriptionTable {
    pub fn match_kind(&self, kind: u16) -> impl Iterator<Item = u32> + '_ {
        let bit = 1u32.checked_shl(kind as u32).unwrap_or(0);
        self.entries.iter().filter(move |e| e.in_use && (e.kind_mask & bit) != 0).map(|e| e.pid)
    }
}
