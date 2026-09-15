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

use super::chrome_card::Plate;
use super::measure_text::{truncate_to_width, width_of};
use super::sidebar_metrics::{line_box, rail_text_x, rail_w, RAIL_X};
use super::theme::{CY, INK, PILL_ON, PILL_ON_LINE, R_CARD};

const MARK_S: u32 = 34;
const MARK_TOP: u32 = 26;
const MARK: &str = "Ø";
const MARK_PX: f32 = 20.0;
const WORD: &str = "NØNOS Files";
const WORD_PX: f32 = 20.0;

/// The rail's masthead: the mark on its own cyan plate with the wordmark set
/// beside it. Both runs are placed by measurement, so the glyph centres in the
/// plate and the wordmark cuts at the rail edge rather than at a glyph count.
pub fn paint_brand(fb: &mut PaintBuffer) {
    Plate::new(PILL_ON)
        .radius(R_CARD)
        .line(PILL_ON_LINE)
        .glow(4)
        .draw(fb, RAIL_X, MARK_TOP, MARK_S, MARK_S);
    let gx = RAIL_X + MARK_S.saturating_sub(width_of(fb, MARK, MARK_PX)) / 2;
    let gy = MARK_TOP + MARK_S.saturating_sub(line_box(MARK_PX)) / 2;
    let _ = fb.text_ttf(gx as i32, gy as i32, MARK, CY, MARK_PX);
    let tx = rail_text_x();
    let avail = (RAIL_X + rail_w()).saturating_sub(tx);
    let word = truncate_to_width(fb, WORD, WORD_PX, avail);
    let wy = MARK_TOP + MARK_S.saturating_sub(line_box(WORD_PX)) / 2;
    let _ = fb.text_ttf(tx as i32, wy as i32, word, INK, WORD_PX);
}
