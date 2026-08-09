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

use crate::layout::{px, Line};

// Stand-in for the TTF advance the capsule measures with on target. Only the
// wrap decisions are under test here, so a deterministic per-character advance
// is enough and keeps the expected line breaks stable across font revisions.
pub fn measure(text: &str, size: f32, mono: bool) -> i32 {
    let ratio = if mono { 0.60 } else { 0.52 };
    (text.chars().count() as f32 * size * ratio).round() as i32
}

pub fn line_width(line: &Line) -> i32 {
    line.spans
        .iter()
        .map(|span| measure(&span.text, px(line.style), span.mono))
        .sum()
}

pub fn plain(line: &Line) -> String {
    line.spans.iter().map(|span| span.text.as_str()).collect()
}

pub fn words(line: &Line) -> usize {
    plain(line).split_whitespace().count()
}
