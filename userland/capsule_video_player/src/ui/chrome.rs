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

use nonos_app_skeleton::paint::PaintBuffer;

use super::glyph;
use super::layout::Layout;
use super::text::{hhmmss, BODY_PX};
use super::theme;

const KNOB_W: u32 = 10;
const KNOB_H: u32 = 14;

pub struct BarState {
    pub playing: bool,
    pub elapsed_ms: i64,
    pub total_ms: i64,
    pub permille: u32,
}

pub fn paint_bar(fb: &mut PaintBuffer, l: &Layout, st: &BarState) {
    fb.fill_rect(l.bar.x, l.bar.y, l.bar.w, l.bar.h, theme::BAR_BG);
    fb.fill_rect(l.scrub.x, l.scrub.y, l.scrub.w, l.scrub.h, theme::TRACK);
    let filled = (l.scrub.w as u64 * st.permille as u64 / 1000) as u32;
    fb.fill_rect(l.scrub.x, l.scrub.y, filled, l.scrub.h, theme::FILL);
    let knob_x = l.scrub.x + filled.min(l.scrub.w.saturating_sub(1));
    let knob_y = l.scrub.y.saturating_sub((KNOB_H - l.scrub.h) / 2);
    fb.fill_rect(knob_x.saturating_sub(KNOB_W / 2), knob_y, KNOB_W, KNOB_H, theme::FILL);
    paint_buttons(fb, l, st.playing);
    paint_times(fb, l, st);
}

fn paint_buttons(fb: &mut PaintBuffer, l: &Layout, playing: bool) {
    if playing {
        glyph::pause(fb, l.play.x + 11, l.play.y + 9, 5, 18, 4, theme::GLYPH);
    } else {
        glyph::triangle_right(fb, l.play.x + 13, l.play.y + 9, 18, theme::GLYPH);
    }
    glyph::triangle_left(fb, l.back.x + 11, l.back.y + 11, 14, theme::GLYPH);
    glyph::triangle_right(fb, l.fwd.x + 11, l.fwd.y + 11, 14, theme::GLYPH);
}

fn paint_times(fb: &mut PaintBuffer, l: &Layout, st: &BarState) {
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let elapsed = hhmmss(st.elapsed_ms, &mut a);
    let total = hhmmss(st.total_ms, &mut b);
    let y = (l.bar.y + l.bar.h).saturating_sub(26) as i32;
    fb.text_ttf(l.elapsed_x as i32, y, elapsed, theme::TEXT, BODY_PX);
    let w = fb.measure_ttf(total, BODY_PX).max(0) as u32;
    fb.text_ttf(l.remain_x.saturating_sub(w) as i32, y, total, theme::TEXT_DIM, BODY_PX);
}
