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

use super::Kind;
use crate::geometry::Rect;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Minimized,
    Hidden,
}

#[derive(Clone, Copy)]
pub struct Window {
    pub owner_pid: u32,
    pub window_id: u32,
    pub rect: Rect,
    pub kind: Kind,
    pub visibility: Visibility,
    pub z: u32,
    pub in_use: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            owner_pid: 0,
            window_id: 0,
            rect: Rect::default(),
            kind: Kind::Normal,
            visibility: Visibility::Hidden,
            z: 0,
            in_use: false,
        }
    }
}

impl Window {
    pub fn matches(&self, owner_pid: u32, window_id: u32) -> bool {
        self.in_use && self.owner_pid == owner_pid && self.window_id == window_id
    }
}
