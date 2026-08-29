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

//! One source of truth for menu geometry: the painter and the hit-test both
//! derive title cells and panel rows from these measured functions.

use alloc::vec::Vec;
use nonos_app_skeleton::measure_ttf;
use nonos_toolkit::ttf::line_height;

use super::items::{rows, TITLES};
use crate::editor::layout::{CHROME_PX, TITLEBAR_H};

pub(in crate::editor) struct TitleSpan {
    pub x0: u32,
    pub x1: u32,
}

pub(super) const BAR_X0: u32 = 8;
pub(super) const TITLE_PAD: u32 = 12;
pub(super) const DROP_PAD_X: u32 = 14;
pub(super) const DROP_SLACK: u32 = 28;

pub(super) fn row_h() -> u32 {
    line_height(CHROME_PX).max(14) as u32 + 8
}

pub(super) fn text_top(box_h: u32) -> u32 {
    let lh = line_height(CHROME_PX).max(1) as u32;
    box_h.saturating_sub(lh) / 2
}

pub(super) fn title_spans() -> Vec<TitleSpan> {
    let mut out = Vec::new();
    let mut x = BAR_X0;
    for title in TITLES.iter() {
        let w = measure_ttf(title, CHROME_PX).max(0) as u32 + TITLE_PAD * 2;
        out.push(TitleSpan { x0: x, x1: x + w });
        x += w;
    }
    out
}

pub(super) fn panel_rect(spans: &[TitleSpan], open: usize) -> (u32, u32, u32, u32) {
    let items = rows(open);
    let x = spans.get(open).map(|s| s.x0).unwrap_or(BAR_X0);
    let mut text_w = 0u32;
    for (label, _) in items {
        text_w = text_w.max(measure_ttf(label, CHROME_PX).max(0) as u32);
    }
    let w = text_w + DROP_PAD_X * 2 + DROP_SLACK;
    let h = items.len() as u32 * row_h() + 2;
    (x, TITLEBAR_H, w, h)
}
