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

//! What a menu row does when it is chosen. `Todo` marks a row the capsule has
//! no implementation for yet; those draw dimmed and report themselves.

use super::tables;

#[derive(Clone, Copy, PartialEq)]
pub(in crate::editor) enum MenuCmd {
    Ctrl(u32, bool),
    CloseTab,
    ToggleSidebar,
    Todo,
}

pub(in crate::editor) type MenuRow = (&'static str, MenuCmd);

pub(in crate::editor) const TITLES: [&str; 8] =
    ["File", "Edit", "View", "Insert", "Format", "Tools", "Table", "Help"];

pub(in crate::editor) fn rows(title: usize) -> &'static [MenuRow] {
    match title {
        0 => &tables::FILE,
        1 => &tables::EDIT,
        2 => &tables::VIEW,
        3 => &tables::INSERT,
        4 => &tables::FORMAT,
        5 => &tables::TOOLS,
        6 => &tables::TABLE,
        7 => &tables::HELP,
        _ => &[],
    }
}
