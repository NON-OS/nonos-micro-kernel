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

//! The left-to-right cell walk. Pill widths follow their measured labels, so a
//! longer style name pushes the toggles along and the hit-test follows, because
//! it reads back the very cells this produced.

use alloc::vec::Vec;
use nonos_app_skeleton::measure_ttf;

use super::items::{RibbonItem, ICON_COUNT, TOGGLES};
use super::metrics::{RibbonCell, BAR_PAD, CHEVRON_W, GAP, PILL_PAD, SEP_GAP, SQUARE_W};
use crate::editor::layout::{ACTIVITY_W, CHROME_PX};

pub(super) fn cells(labels: &[&str; 3]) -> Vec<RibbonCell> {
    let mut out = Vec::new();
    let mut x = ACTIVITY_W + BAR_PAD;
    for (p, label) in labels.iter().enumerate() {
        let w = measure_ttf(label, CHROME_PX).max(0) as u32 + PILL_PAD * 2 + CHEVRON_W;
        out.push(RibbonCell { x0: x, x1: x + w, item: RibbonItem::Pill(p) });
        x += w + GAP;
    }
    x += SEP_GAP;
    for t in 0..TOGGLES.len() {
        out.push(RibbonCell { x0: x, x1: x + SQUARE_W, item: RibbonItem::Toggle(t) });
        x += SQUARE_W + GAP;
    }
    x += SEP_GAP;
    for k in 0..ICON_COUNT {
        out.push(RibbonCell { x0: x, x1: x + SQUARE_W, item: RibbonItem::Icon(k) });
        x += SQUARE_W + GAP;
    }
    out
}
