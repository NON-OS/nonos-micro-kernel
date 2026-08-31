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

//! Paragraph alignment. `line_offset` is the one place the horizontal shift of a
//! laid-out line is computed, so the painter and the caret hit-test stay on the
//! same pixels instead of drifting apart.

use crate::doc::block::Block;
use crate::doc::linebox::LineBox;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Align {
    Left,
    Center,
    Right,
}

pub fn line_offset(block: &Block, line: &LineBox, content_w: f32) -> f32 {
    let slack = content_w - line.width;
    if slack <= 0.0 {
        return 0.0;
    }
    match block.align {
        Align::Left => 0.0,
        Align::Center => slack * 0.5,
        Align::Right => slack,
    }
}
