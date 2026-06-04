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

use super::constants::{CELL_WIDTH, HEADER_H, TEXT_LEFT};
use crate::term::state::State;
use crate::term::theme::{ACCENT, HEADER_BG, HEADER_RULE, PATH};

pub fn draw_header(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(0, 0, fb.width, HEADER_H, HEADER_BG);
    fb.fill_rect(0, HEADER_H, fb.width, 1, HEADER_RULE);
    fb.text_scaled(TEXT_LEFT, 6, b"\xd8 NONOS", ACCENT, 2);
    let cwd = state.cwd.as_bytes();
    let take = cwd.len().min(34);
    let start = cwd.len() - take;
    let width = take as u32 * CELL_WIDTH;
    let x = fb.width.saturating_sub(width + TEXT_LEFT);
    fb.text(x, 10, &cwd[start..], PATH);
}
