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

use super::transport::{transport, Transport, HEX_R};
use crate::ui::chrome::BarState;
use crate::ui::icon;
use crate::ui::layout::{Layout, Rect};
use crate::ui::paint::shape;
use crate::ui::text::{center_y, hhmmss, BODY_PX};
use crate::ui::theme;
use crate::ui::widget::progress::{paint_bar, paint_scrub};

const GLYPH: u32 = 18;
const SMALL: u32 = 16;

fn tint(on: bool) -> u32 {
    if on {
        theme::ACCENT
    } else {
        theme::TEXT_DIM
    }
}

fn center(r: Rect, size: u32) -> (u32, u32) {
    (r.x + r.w.saturating_sub(size) / 2, r.y + r.h.saturating_sub(size) / 2)
}

fn paint_play(fb: &mut PaintBuffer, t: &Transport, playing: bool) {
    let cx = t.play.x + HEX_R;
    let cy = t.play.y + HEX_R;
    shape::hex(fb, cx, cy, HEX_R, theme::ACCENT_DIM);
    let (gx, gy) = center(t.play, GLYPH);
    if playing {
        icon::transport::pause(fb, gx, gy, GLYPH, theme::APP_BG);
    } else {
        icon::transport::play(fb, gx, gy, GLYPH, theme::APP_BG);
    }
}

fn paint_sides(fb: &mut PaintBuffer, t: &Transport, muted: bool, volume: u32) {
    let (sx, sy) = center(t.shuffle, GLYPH);
    icon::transport::shuffle(fb, sx, sy, GLYPH, theme::TEXT_DIM);
    let (px, py) = center(t.prev, GLYPH);
    icon::transport::prev(fb, px, py, GLYPH, theme::TEXT);
    let (nx, ny) = center(t.next, GLYPH);
    icon::transport::next(fb, nx, ny, GLYPH, theme::TEXT);
    let (rx, ry) = center(t.repeat, GLYPH);
    icon::transport::repeat(fb, rx, ry, GLYPH, theme::TEXT_DIM);
    let (vx, vy) = center(t.cc, SMALL);
    icon::ui::cc(fb, vx, vy, SMALL, theme::TEXT_DIM);
    let (ix, iy) = center(t.pip, SMALL);
    icon::ui::pip(fb, ix, iy, SMALL, theme::TEXT_DIM);
    let (fx, fy) = center(t.full, SMALL);
    icon::ui::fullscreen(fb, fx, fy, SMALL, theme::TEXT_DIM);
    let mark = if muted { icon::transport::mute } else { icon::transport::volume };
    mark(fb, t.mute.x, t.mute.y, SMALL, tint(!muted));
    paint_bar(fb, t.volume, volume.min(100) * 10, theme::TEXT_DIM);
}

fn paint_times(fb: &mut PaintBuffer, l: &Layout, st: &BarState) {
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let y = center_y(l.scrub.y + 14, 20);
    fb.text_ttf(l.elapsed_x as i32, y, hhmmss(st.elapsed_ms, &mut a), theme::TEXT, BODY_PX);
    let total = hhmmss(st.total_ms, &mut b);
    let w = fb.measure_ttf(total, BODY_PX).max(0) as u32;
    fb.text_ttf(l.remain_x.saturating_sub(w) as i32, y, total, theme::TEXT_DIM, BODY_PX);
}

pub fn paint_transport(fb: &mut PaintBuffer, l: &Layout, st: &BarState, muted: bool, volume: u32) {
    fb.fill_rect(l.bar.x, l.bar.y, l.bar.w, l.bar.h, theme::PANEL);
    paint_scrub(fb, l.scrub, st.permille, st.permille);
    paint_times(fb, l, st);
    let t = transport(l, fb.width);
    paint_play(fb, &t, st.playing);
    paint_sides(fb, &t, muted, volume);
    fb.text_ttf(
        (t.lang.x + 10) as i32,
        center_y(t.lang.y, t.lang.h),
        "EN",
        theme::TEXT_DIM,
        BODY_PX,
    );
    icon::ui::chevron_down(fb, t.lang.x + t.lang.w - 20, t.lang.y + 9, 12, theme::TEXT_MUTED);
}
