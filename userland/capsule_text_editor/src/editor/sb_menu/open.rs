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

use super::{MenuAction, SbMenu};
use crate::editor::tree::FileTree;

const DIR_ITEMS: &[MenuAction] =
    &[MenuAction::NewFile, MenuAction::NewFolder, MenuAction::Rename, MenuAction::Delete];
const FILE_ITEMS: &[MenuAction] = &[MenuAction::Rename, MenuAction::Delete];
const EMPTY_ITEMS: &[MenuAction] = &[MenuAction::NewFile, MenuAction::NewFolder];

// Build the menu for a right-click at (x, y): a directory row offers creation
// inside it, a file row only rename/delete, and the empty area creates at root.
pub(in crate::editor) fn open_menu(tree: &FileTree, x: u32, y: u32, row: Option<usize>) -> SbMenu {
    let items = match row.and_then(|r| tree.visible.get(r)) {
        Some(&ni) if tree.nodes[ni].is_dir => DIR_ITEMS,
        Some(_) => FILE_ITEMS,
        None => EMPTY_ITEMS,
    };
    SbMenu { x, y, target: row, items }
}
