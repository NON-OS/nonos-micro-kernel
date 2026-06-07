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

#[derive(Clone, Copy, Default)]
pub struct CursorState {
    pub x: u32,
    pub y: u32,
    pub visible: bool,
}

pub struct CursorTracker {
    state: CursorState,
}

impl CursorTracker {
    pub const fn at(x: u32, y: u32) -> Self {
        Self { state: CursorState { x, y, visible: true } }
    }

    pub fn update(&mut self, x: u32, y: u32, visible: bool) -> CursorState {
        let prev = self.state;
        self.state = CursorState { x, y, visible };
        prev
    }

    pub fn current(&self) -> CursorState {
        self.state
    }
}
