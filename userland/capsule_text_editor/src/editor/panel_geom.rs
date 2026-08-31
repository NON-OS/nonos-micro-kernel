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

//! One source of truth for panel geometry: the painter and the hit-test both
//! derive the box and its row strip from these measured functions, so a click
//! lands on the row that was drawn.

use nonos_app_skeleton::measure_ttf;

use super::layout::{CHROME_PX, TITLEBAR_H};
use super::panel::{PANEL_PAD, PANEL_SLACK};
use super::widget::nav_row_h;

pub(in crate::editor) fn panel_rect(
    win_w: u32,
    win_h: u32,
    title: &str,
    labels: &[&str],
) -> (u32, u32, u32, u32) {
    let mut text_w = measure_ttf(title, CHROME_PX).max(0) as u32;
    for label in labels {
        text_w = text_w.max(measure_ttf(label, CHROME_PX).max(0) as u32);
    }
    let w = (text_w + PANEL_PAD * 2 + PANEL_SLACK).min(win_w);
    let h = (labels.len() as u32 + 1) * nav_row_h(CHROME_PX) + PANEL_PAD;
    let x = win_w.saturating_sub(w) / 2;
    let y = (win_h.saturating_sub(h) / 2).max(TITLEBAR_H);
    (x, y, w, h)
}

pub(in crate::editor) fn panel_list(rect: (u32, u32, u32, u32)) -> (u32, u32, u32) {
    let (x, y, w, _) = rect;
    let inset = PANEL_PAD / 2;
    (x + inset, y + inset + nav_row_h(CHROME_PX), w.saturating_sub(inset * 2))
}
