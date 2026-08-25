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
use nonos_toolkit::icons::{draw, IconId};

use crate::pm::state::{Screen, State};
use crate::pm::theme::{ACCENT, FOREGROUND, MUTED, SEARCH_BG, SEARCH_BORDER};

use super::chips;
use super::metrics::{
    BODY_PX, HEAD_H, INSPECTOR_W, PANE_PAD_TOP, PANE_PAD_X, SEARCH_CARET_INSET, SEARCH_CARET_W,
    SEARCH_H, SEARCH_ICON, SEARCH_ICON_GAP, SEARCH_META_GAP, SEARCH_MIN_W, SEARCH_PAD_X,
    SEARCH_RADIUS, SEARCH_W,
};
use super::text;

const PLACEHOLDER: &[u8] = b"Filter processes";

// The one geometry the painter and the hit test both read. The field takes the
// far right of the head band, centred in it, so the meta line can be measured
// back from its left edge instead of the two fighting over the same pixels.
// The chip strip is measured text and cannot shrink, so the field is what gives
// way when the band is tight, down to SEARCH_MIN_W and no further.
pub fn rect(fb_w: u32, screen: Screen) -> (u32, u32, u32, u32) {
    let right = if screen.has_inspector() { fb_w.saturating_sub(INSPECTOR_W) } else { fb_w };
    let limit = right.saturating_sub(PANE_PAD_X);
    let chips_end = chips::origin(screen).map_or(0, |o| o + chips::width() + SEARCH_META_GAP);
    let w = limit.saturating_sub(chips_end).clamp(SEARCH_MIN_W, SEARCH_W);
    let y = PANE_PAD_TOP + HEAD_H.saturating_sub(SEARCH_H) / 2;
    (limit.saturating_sub(w), y, w, SEARCH_H)
}

// Everything here lands over paint the frame already laid down, so the fill,
// the border and the caret all go through blending primitives.
pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let (x, y, w, h) = rect(fb.width, state.screen);
    let focused = state.query_focused();
    let border = if focused { ACCENT } else { SEARCH_BORDER };
    fb.fill_round(x, y, w, h, SEARCH_RADIUS, SEARCH_BG);
    fb.stroke_round(x, y, w, h, SEARCH_RADIUS, 1, border);
    let icon_x = x + SEARCH_PAD_X;
    let icon_y = y + h.saturating_sub(SEARCH_ICON) / 2;
    draw(fb, IconId::SettingsSearch, icon_x, icon_y, SEARCH_ICON, MUTED);
    let text_x = icon_x + SEARCH_ICON + SEARCH_ICON_GAP;
    let avail = (x + w).saturating_sub(text_x + SEARCH_PAD_X + SEARCH_CARET_W);
    let query = state.query();
    let (body, ink) = if query.is_empty() { (PLACEHOLDER, MUTED) } else { (query, FOREGROUND) };
    let shown = text::fit(fb, body, BODY_PX, avail);
    let top = text::centred_top(y, h, BODY_PX);
    let end = text::left(fb, text_x, top, shown, ink, BODY_PX);
    if focused {
        let caret_x = if query.is_empty() { text_x } else { end.max(0) as u32 };
        let caret_h = h.saturating_sub(SEARCH_CARET_INSET * 2);
        fb.blend_rect(caret_x, y + SEARCH_CARET_INSET, SEARCH_CARET_W, caret_h, ACCENT);
    }
}
