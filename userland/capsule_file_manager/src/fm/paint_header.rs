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

use nonos_app_skeleton::PaintBuffer;

use super::manifest::WIDTH;
use super::paint_metrics::{GLYPH_W, TEXT_LEFT};
use super::state::State;
use super::theme::MUTED;

pub fn paint_header(state: &State, fb: &mut PaintBuffer) {
    let mut tag = alloc::string::String::from("sort:");
    tag.push_str(core::str::from_utf8(state.sort_mode.label()).unwrap_or("?"));
    if !state.filter.is_empty() {
        tag.push_str("  /");
        tag.push_str(&state.filter);
    }
    let x = WIDTH.saturating_sub(TEXT_LEFT + GLYPH_W * tag.len() as u32);
    fb.text(x, 18, tag.as_bytes(), MUTED);
}
