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

//! Table menu actions. Every row but the first needs a table under the caret,
//! so a miss reports why instead of failing quietly.

use super::app::Editor;
use super::unsupported::NO_TABLE_AT_CARET;

impl Editor {
    pub(super) fn table_menu(&mut self, op: u8) {
        let st = self.doc();
        let done = match op {
            0 => st.insert_table(3, 3),
            1 => st.insert_table_row(),
            2 => st.insert_table_col(),
            _ => st.delete_table(),
        };
        if !done {
            st.status = NO_TABLE_AT_CARET;
        }
    }
}
