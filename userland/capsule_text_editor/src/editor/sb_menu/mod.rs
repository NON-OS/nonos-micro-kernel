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

//! The explorer's right-click context menu: which actions apply to the clicked
//! row (or the empty area), where the menu sits, and its row metrics.

mod hit;
mod open;
mod paint;

pub(in crate::editor) use hit::menu_hit;
pub(in crate::editor) use open::open_menu;
pub(in crate::editor) use paint::paint_menu;

#[derive(Clone, Copy, PartialEq)]
pub enum MenuAction {
    NewFile,
    NewFolder,
    Rename,
    Delete,
}

impl MenuAction {
    pub fn label(self) -> &'static str {
        match self {
            MenuAction::NewFile => "New File",
            MenuAction::NewFolder => "New Folder",
            MenuAction::Rename => "Rename",
            MenuAction::Delete => "Delete",
        }
    }
}

pub struct SbMenu {
    pub x: u32,
    pub y: u32,
    // The visible tree row the menu acts on; None means the empty area, where
    // creation targets the root directory.
    pub target: Option<usize>,
    pub items: &'static [MenuAction],
}

pub(super) const MENU_W: u32 = 130;
pub(super) const MENU_ROW_H: u32 = 26;
