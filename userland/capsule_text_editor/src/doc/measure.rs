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

pub trait Measurer {
    fn advance(&self, text: &str, style: &crate::doc::style::RunStyle) -> f32;
    fn line_height(&self, style: &crate::doc::style::RunStyle) -> f32;
    fn ascent(&self, style: &crate::doc::style::RunStyle) -> f32;
}

pub struct FixedMeasurer;

impl Measurer for FixedMeasurer {
    fn advance(&self, text: &str, style: &crate::doc::style::RunStyle) -> f32 {
        text.len() as f32 * style.size_px * 0.5
    }

    fn line_height(&self, style: &crate::doc::style::RunStyle) -> f32 {
        style.size_px * 1.45
    }

    fn ascent(&self, style: &crate::doc::style::RunStyle) -> f32 {
        style.size_px * 1.1
    }
}
