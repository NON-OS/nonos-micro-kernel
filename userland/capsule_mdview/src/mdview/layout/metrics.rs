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

use super::block::Style;

pub fn px(style: Style) -> f32 {
    match style {
        Style::H1 => 26.0,
        Style::H2 => 21.0,
        Style::H3 => 17.0,
        Style::Code => 14.0,
        _ => 15.0,
    }
}

pub fn line_height(style: Style) -> i32 {
    (px(style) * 1.45) as i32
}

pub fn gap(style: Style) -> i32 {
    match style {
        Style::H1 => 16,
        Style::H2 | Style::H3 => 12,
        Style::Bullet => 4,
        _ => 10,
    }
}

pub fn indent(style: Style) -> i32 {
    match style {
        Style::Bullet => 22,
        Style::Code => 10,
        _ => 0,
    }
}
