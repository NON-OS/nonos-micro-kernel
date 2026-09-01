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
use super::line_text::text_parts;
use super::line_window::window;
use super::metrics::Metrics;
use super::prompt::draw_prompt;
use super::shade::elevate;
use super::suggestion::draw_suggestion;
use super::syntax::{classify, Part};

use crate::layout::Rect;
use crate::term::state::State;
use crate::term::theme::types::Theme;

pub fn draw_input_line(state: &State, fb: &mut PaintBuffer, r: Rect, m: Metrics, t: &Theme) {
    let (adv, px) = (m.adv, m.px);
    let y = r.y;
    let ox = r.x + TEXT_LEFT;
    // Warp-style input bar: an inset panel behind the prompt with a left
    // accent stripe, drawn first so the prompt and text land on top.
    let bar_x = r.x + TEXT_LEFT / 2;
    let bar_w = r.w.saturating_sub(TEXT_LEFT);
    let bar_y = y.saturating_sub(3);
    fb.fill_rect(bar_x, bar_y, bar_w, m.lh + 4, elevate(t.bg, 12));
    fb.fill_rect(bar_x, bar_y, 2, m.lh + 4, t.accent);
    // Character cells that fit between the left inset and an equal right margin.
    let total_cells = (r.w.saturating_sub(TEXT_LEFT * 2) / adv) as usize;
    // Prompt is glyph + path + trailing space; cap the path to a third of the
    // line so a deep cwd never starves the area left to type in.
    let prompt_cells = draw_prompt(state, fb, ox, y, adv, px, total_cells / 3, t);
    // Horizontal scroll: slide a body_cells-wide window so the cursor is always
    // on screen, showing the start of the line whenever it fits.
    let body = state.line.as_bytes();
    let cursor = state.line.cursor.min(body.len());
    let body_cells = total_cells.saturating_sub(prompt_cells).max(1);
    let (start, stop, scroll) = window(body, cursor, body_cells);
    let bx = ox + prompt_cells as u32 * adv;
    // Classify the whole line, not the visible window, so a word keeps its
    // colour when it scrolls in from either side.
    let mut parts = [Part::Plain; MAX_HIGHLIGHT];
    classify(body, &mut parts);
    let from = start.min(MAX_HIGHLIGHT);
    let to = stop.min(MAX_HIGHLIGHT);
    text_parts(fb, bx, y, &body[start..stop], &parts[from..to], adv, px, t);
    // The offer sits where the cursor is, so it has to be drawn before the
    // cursor goes on top of it.
    let typed_cells = stop.saturating_sub(start);
    let ghost_x = bx + typed_cells as u32 * adv;
    let room = body_cells.saturating_sub(typed_cells);
    draw_suggestion(state, fb, ghost_x, y, adv, px, room, t);
    let under = body.get(cursor).copied().unwrap_or(0);
    draw_cursor(fb, ox, prompt_cells, cursor - scroll, y + 1, under, m, t);
}

// The longest line that is coloured. Past it the tail is drawn plain rather
// than growing a buffer on every keystroke; a command that long is being
// pasted, not read.
const MAX_HIGHLIGHT: usize = 1024;
