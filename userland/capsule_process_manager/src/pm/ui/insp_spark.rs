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
use nonos_toolkit::font::ttf::line_height;

use crate::pm::state::Ring;
use crate::pm::theme::{ACCENT, MUTED, OK, TRACK_BG};

use super::metrics::{BODY_PX, CARD_LINE_GAP, INSP_SECTION_GAP, SPARK_H};
use super::{spark, text};

// Two trend panels over the same per-pid ring. A single sample has no shape and
// a flat line would read as a real idle stretch, so the caption says
// "collecting" and the plot stays empty until two points can be joined.
pub fn paint(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, ring: Option<&Ring>) -> u32 {
    let live = ring.map(|r| r.len() >= 2).unwrap_or(false);
    let mut y = head(fb, x, y, w, b"CPU", live);
    plot(fb, x, y, w, ring.filter(|_| live), true);
    y += SPARK_H + INSP_SECTION_GAP;
    y = head(fb, x, y, w, b"Memory", live);
    plot(fb, x, y, w, ring.filter(|_| live), false);
    y + SPARK_H + INSP_SECTION_GAP
}

fn head(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &[u8], live: bool) -> u32 {
    text::left(fb, x, y, label, MUTED, BODY_PX);
    let tail: &[u8] = if live { b"last 32s" } else { b"collecting" };
    text::right(fb, x + w, y, tail, MUTED, BODY_PX);
    y + line_height(BODY_PX).max(1) as u32 + CARD_LINE_GAP
}

// The baseline is drawn either way, so an empty panel still reads as a plot
// waiting for data rather than as a gap in the pane.
fn plot(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, ring: Option<&Ring>, cpu: bool) {
    fb.fill_rect(x, y + SPARK_H.saturating_sub(1), w, 1, TRACK_BG);
    let Some(ring) = ring else {
        return;
    };
    if cpu {
        spark::cpu(fb, x, y, w, SPARK_H, ring, ACCENT);
    } else {
        spark::mem(fb, x, y, w, SPARK_H, ring, OK);
    }
}
