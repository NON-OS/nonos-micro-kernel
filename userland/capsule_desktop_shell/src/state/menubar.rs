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

//! Which menu-bar title is open and which of its rows the pointer is over.
//! Both are indices into the tables in `render::menubar_menu`, so the painter
//! and the hit test agree without carrying geometry in state.

pub struct MenubarState {
    pub open: Option<usize>,
    pub hover: Option<usize>,
}

pub fn new_menubar_state() -> MenubarState {
    MenubarState { open: None, hover: None }
}
