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

use nonos_toolkit::font::ttf::{ascent_with, builtin_face, line_height_with, measure_with};

use super::measure::Measurer;
use super::style::{Family, RunStyle};

pub struct TtfMeasurer;

impl Measurer for TtfMeasurer {
    fn advance(&self, text: &str, style: &RunStyle) -> f32 {
        let mono = style.family == Family::Mono;
        match builtin_face(mono, style.bold) {
            Some(f) => measure_with(f, text, style.size_px) as f32,
            None => 0.0,
        }
    }

    fn line_height(&self, style: &RunStyle) -> f32 {
        let mono = style.family == Family::Mono;
        match builtin_face(mono, style.bold) {
            Some(f) => line_height_with(f, style.size_px) as f32,
            None => style.size_px * 1.45,
        }
    }

    fn ascent(&self, style: &RunStyle) -> f32 {
        let mono = style.family == Family::Mono;
        match builtin_face(mono, style.bold) {
            Some(f) => ascent_with(f, style.size_px) as f32,
            None => style.size_px * 1.1,
        }
    }
}
