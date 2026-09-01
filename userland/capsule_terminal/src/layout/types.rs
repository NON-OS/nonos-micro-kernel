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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }

    pub fn contains(&self, px: u32, py: u32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w && py < self.y + self.h
    }
}

#[derive(Clone, Copy)]
pub struct Chrome {
    pub titlebar_h: u32,
    pub tabstrip_h: u32,
    pub body_pad_top: u32,
    pub footer_h: u32,
    pub text_left: u32,
    pub row_h: u32,
}

#[derive(Clone, Copy)]
pub struct Rails {
    pub left: u32,
    pub right: u32,
}

#[derive(Clone, Copy)]
pub struct Layout {
    pub titlebar: Rect,
    pub tabstrip: Rect,
    pub left_rail: Rect,
    pub right_rail: Rect,
    pub body: Rect,
    pub input: Rect,
    pub footer: Rect,
}
