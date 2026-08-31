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

//! The rectangle an open pill's dropdown occupies, measured from the same row
//! labels the panel paints, so the panel and its hit-test cannot disagree.

use nonos_app_skeleton::measure_ttf;

use super::items::{pill_labels, RibbonItem};
use super::metrics::{band_top, row_h, RibbonCell, DROP_PAD_X, DROP_SLACK};
use crate::editor::layout::{ACTIVITY_W, CHROME_PX, RIBBON_H};

pub(super) fn panel_rect(cells: &[RibbonCell], pill: usize) -> (u32, u32, u32, u32) {
    let labels = pill_labels(pill);
    let x =
        cells.iter().find(|c| c.item == RibbonItem::Pill(pill)).map(|c| c.x0).unwrap_or(ACTIVITY_W);
    let mut text_w = 0u32;
    for label in labels.iter() {
        text_w = text_w.max(measure_ttf(label, CHROME_PX).max(0) as u32);
    }
    let w = text_w + DROP_PAD_X * 2 + DROP_SLACK;
    let h = labels.len() as u32 * row_h() + 2;
    (x, band_top() + RIBBON_H, w, h)
}
