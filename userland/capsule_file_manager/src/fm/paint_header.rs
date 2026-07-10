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

extern crate alloc;

use nonos_app_skeleton::PaintBuffer;

use super::layout::{CONTENT_X, HEADER_H, PAD_X};
use super::state::State;
use super::theme::{ACCENT, FOREGROUND, HEADER_BG, LINE, MUTED};

pub fn paint_header(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width;
    let cw = w.saturating_sub(CONTENT_X);
    fb.fill_rect(CONTENT_X, 0, cw, HEADER_H, HEADER_BG);
    fb.fill_rect(CONTENT_X, HEADER_H - 1, cw, 1, LINE);

    // accent tick + current path as the title (below the 26px window titlebar)
    fb.fill_rect(CONTENT_X + PAD_X, 32, 4, 22, ACCENT);
    let path = if state.prefix.is_empty() { "/" } else { state.prefix.as_str() };
    let _ = fb.text_ttf((CONTENT_X + PAD_X + 16) as i32, 28, path, FOREGROUND, 26.0);

    // right side: item count + active sort + filter, small and muted
    let mut meta = alloc::format!("{} items", state.entries.len());
    meta.push_str("   sort ");
    meta.push_str(core::str::from_utf8(state.sort_mode.label()).unwrap_or("?"));
    if !state.filter.is_empty() {
        meta.push_str("   /");
        meta.push_str(&state.filter);
    }
    let mw = fb.measure_ttf(&meta, 16.0).max(0) as u32;
    let mx = w.saturating_sub(PAD_X + mw);
    let _ = fb.text_ttf(mx as i32, 34, &meta, MUTED, 16.0);
}
