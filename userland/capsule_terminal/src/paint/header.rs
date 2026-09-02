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

use super::constants::{HEADER_H, TEXT_LEFT};
use super::shade::elevate;
use crate::term::state::State;
use crate::term::theme::types::Theme;

pub fn draw_header(state: &State, fb: &mut PaintBuffer, t: &Theme) {
    fb.fill_rect(0, 0, fb.width, HEADER_H, elevate(t.bg, 10));
    // A hairline under the header, and a short accent run at the left edge of
    // it. The mark sits where the eye starts a line, so the window says whose
    // it is without a bar of colour across the whole width.
    fb.fill_rect(0, HEADER_H, fb.width, 1, elevate(t.bg, 22));
    fb.fill_rect(0, HEADER_H, TEXT_LEFT * 4, 1, t.accent);
    let _ = fb.text_ttf(TEXT_LEFT as i32, 5, "\u{00D8} NONOS", t.accent, 15.0);
    let cwd = state.cwd.as_bytes();
    let take = cwd.len().min(48);
    let start = cwd.len() - take;
    let cwd_str = core::str::from_utf8(&cwd[start..]).unwrap_or("");
    let width = fb.measure_ttf(cwd_str, 13.0).max(0) as u32;
    let x = fb.width.saturating_sub(width + TEXT_LEFT);
    let _ = fb.text_ttf(x as i32, 7, cwd_str, t.path, 13.0);
}
