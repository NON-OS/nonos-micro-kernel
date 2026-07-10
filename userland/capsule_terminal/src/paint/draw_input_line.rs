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

use super::constants::TEXT_LEFT;
use super::draw_cursor::draw_cursor;
use super::metrics::Metrics;
use super::shade::elevate;
use crate::term::state::State;
use crate::term::theme::{ACCENT, FOREGROUND, PATH, PROMPT};

// Draw a byte slice as crisp monospace, one glyph per cell advance.
fn text(fb: &mut PaintBuffer, mut x: u32, y: u32, bytes: &[u8], argb: u32, adv: u32, px: f32) {
    let mut buf = [0u8; 4];
    for &b in bytes {
        if b > b' ' && b < 0x7f {
            let s = (b as char).encode_utf8(&mut buf);
            let _ = fb.text_ttf_mono(x as i32, y as i32, s, argb, px);
        }
        x += adv;
    }
}

pub fn draw_input_line(state: &State, fb: &mut PaintBuffer, y: u32, m: Metrics) {
    let (adv, px) = (m.adv, m.px);
    // Warp-style input bar: an inset panel behind the prompt with a left
    // accent stripe, drawn first so the prompt and text land on top.
    let bar_x = TEXT_LEFT / 2;
    let bar_w = fb.width.saturating_sub(TEXT_LEFT);
    let bar_y = y.saturating_sub(3);
    fb.fill_rect(bar_x, bar_y, bar_w, m.lh + 4, elevate(state.bg, 12));
    fb.fill_rect(bar_x, bar_y, 2, m.lh + 4, ACCENT);
    // Character cells that fit between the left inset and an equal right margin.
    let total_cells = (fb.width.saturating_sub(TEXT_LEFT * 2) / adv) as usize;
    // Prompt is glyph + path + trailing space; cap the path to a third of the
    // line so a deep cwd never starves the area left to type in.
    let cwd = state.cwd.as_bytes();
    let take = cwd.len().min((total_cells / 3).max(1));
    let prompt_cells = 1 + take + 1;
    text(fb, TEXT_LEFT, y, b">", PROMPT, adv, px);
    text(fb, TEXT_LEFT + adv, y, &cwd[cwd.len() - take..], PATH, adv, px);
    // Horizontal scroll: slide a body_cells-wide window so the cursor is always
    // on screen, showing the start of the line whenever it fits.
    let body = state.line.as_bytes();
    let cursor = state.line.cursor.min(body.len());
    let body_cells = total_cells.saturating_sub(prompt_cells).max(1);
    let scroll = if cursor < body_cells { 0 } else { cursor - body_cells + 1 };
    let end = (scroll + body_cells).min(body.len());
    let bx = TEXT_LEFT + prompt_cells as u32 * adv;
    text(fb, bx, y, &body[scroll..end], FOREGROUND, adv, px);
    let under = body.get(cursor).copied().unwrap_or(0);
    draw_cursor(fb, prompt_cells, cursor - scroll, y + 1, under, m);
}
