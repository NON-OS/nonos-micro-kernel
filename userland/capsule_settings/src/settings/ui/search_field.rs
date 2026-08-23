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

use crate::settings::state::State;

use super::bytes::as_str;
use super::metrics::BODY_PX;
use super::text;
use super::theme::{ACCENT, SEARCH_BG, SEARCH_BORDER, SEARCH_FG, SEARCH_PLACEHOLDER};

pub const WIDTH: u32 = 236;
const PAD_X: u32 = 10;
const ICON: u32 = 14;
const GAP: u32 = 8;

pub fn paint(fb: &mut PaintBuffer, state: &State) {
    let (w, h) = (fb.width, fb.height);
    if w <= PAD_X * 2 || h <= ICON {
        return;
    }
    let r = h / 2;
    fb.fill_round(0, 0, w, h, r, SEARCH_BG);
    let border = if state.search_focused { ACCENT } else { SEARCH_BORDER };
    fb.stroke_round(0, 0, w, h, r, 1, border);
    draw(fb, IconId::SettingsSearch, PAD_X, (h - ICON) / 2, ICON, SEARCH_PLACEHOLDER);
    let x = PAD_X + ICON + GAP;
    let top = text::centred_top(0, h, BODY_PX);
    let query = as_str(state.search.as_slice());
    if query.is_empty() {
        text::left(fb, x, top, "Search settings", SEARCH_PLACEHOLDER, BODY_PX);
        return;
    }
    let end = text::left(fb, x, top, query, SEARCH_FG, BODY_PX).max(0) as u32;
    if state.search_focused {
        caret(fb, end + 2, h);
    }
}

fn caret(fb: &mut PaintBuffer, x: u32, h: u32) {
    if x + 1 >= fb.width {
        return;
    }
    let inset = h / 5;
    fb.fill_rect(x, inset, 1, h.saturating_sub(inset * 2), ACCENT);
}
