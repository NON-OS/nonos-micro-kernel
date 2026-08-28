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

use alloc::vec::Vec;

use crate::doc::linebox::LineBox;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PageMetrics {
    pub width: f32,
    pub height: f32,
    pub margin: f32,
}

impl PageMetrics {
    pub fn content_width(&self) -> f32 {
        self.width - 2.0 * self.margin
    }

    pub fn content_height(&self) -> f32 {
        self.height - 2.0 * self.margin
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Page {
    pub lines: Vec<LineBox>,
}

impl Page {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}
