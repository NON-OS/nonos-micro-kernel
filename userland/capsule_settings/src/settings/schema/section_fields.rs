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

use nonos_policy_proto::Field;

use crate::settings::section::Section;

use super::blocks_for::blocks_for;
use super::rows::Row;

/// How many editable rows a section has. The cursor indexes this list, and the
/// pane walks the same block tables, so the two cannot disagree about which row
/// is which.
pub fn field_count(section: Section) -> usize {
    let mut n = 0;
    for b in blocks_for(section) {
        for r in b.rows {
            if matches!(r, Row::Field(_)) {
                n += 1;
            }
        }
    }
    n
}

pub fn field_at(section: Section, index: usize) -> Option<Field> {
    let mut n = 0;
    for b in blocks_for(section) {
        for r in b.rows {
            if let Row::Field(f) = r {
                if n == index {
                    return Some(*f);
                }
                n += 1;
            }
        }
    }
    None
}
