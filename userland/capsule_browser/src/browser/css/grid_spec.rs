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

use alloc::string::String;
use alloc::vec::Vec;

// Named grid data the Copy style struct cannot hold, kept in a per-node side
// table like background images. Containers carry the named column lines and
// template areas; items carry their requested placement.
#[derive(Default)]
pub struct GridSpec {
    // (name, zero-based column line index) from [name] groups in the
    // grid-template-columns track list.
    pub col_lines: Vec<(String, u8)>,
    // grid-template-areas rows, each a list of cell tokens ("." is a hole).
    pub areas: Vec<Vec<String>>,
    // grid-area: <name> on an item.
    pub area: Option<String>,
    // grid-column / grid-row lines, raw: an integer or a line name. Rows are
    // numeric only; named row lines are rare enough to skip.
    pub col_start: Option<String>,
    pub col_end: Option<String>,
    pub row_start: Option<i16>,
    pub row_end: Option<i16>,
}

impl GridSpec {
    // True when this node requests an explicit item placement.
    pub fn places_item(&self) -> bool {
        self.area.is_some()
            || self.col_start.is_some()
            || self.col_end.is_some()
            || self.row_start.is_some()
            || self.row_end.is_some()
    }
}
