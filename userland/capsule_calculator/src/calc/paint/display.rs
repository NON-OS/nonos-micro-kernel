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

use crate::calc::format::{format, DISPLAY_MAX, ERROR_TEXT};
use crate::calc::layout::{DISPLAY_H, PADDING};
use crate::calc::manifest::WIDTH;
use crate::calc::state::State;
use crate::calc::theme::{ERROR, INK, LINE_2, PANEL};

const CELL_WIDTH: u32 = 8;
const TEXT_RIGHT_PADDING: u32 = 16;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let display_top = 20;
    fb.fill_rect(PADDING, display_top, WIDTH - PADDING * 2, DISPLAY_H, PANEL);
    fb.blend_rect(PADDING, display_top, WIDTH - PADDING * 2, 1, LINE_2);
    fb.blend_rect(PADDING, display_top + DISPLAY_H - 1, WIDTH - PADDING * 2, 1, LINE_2);
    let mut buf = [0u8; DISPLAY_MAX];
    let (text, color): (&[u8], u32) = if state.is_error() {
        (ERROR_TEXT, ERROR)
    } else {
        let n = format(state.display, state.decimal_digits_typed, &mut buf);
        (&buf[..n], INK)
    };
    let text_width = (text.len() as u32) * CELL_WIDTH;
    let display_right = WIDTH - PADDING - TEXT_RIGHT_PADDING;
    let x = display_right.saturating_sub(text_width);
    let y = display_top + (DISPLAY_H - 8) / 2;
    fb.text(x, y, text, color);
}
