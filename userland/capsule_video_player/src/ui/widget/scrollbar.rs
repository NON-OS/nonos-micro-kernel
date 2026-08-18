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

use crate::ui::layout::Rect;
use crate::ui::paint::rrect;
use crate::ui::theme;

pub const WIDTH: u32 = 6;
const MIN_THUMB: u32 = 28;

pub fn clamp_scroll(scroll: usize, visible: usize, total: usize) -> usize {
    let max = total.saturating_sub(visible);
    scroll.min(max)
}

pub fn thumb(r: Rect, scroll: usize, visible: usize, total: usize) -> Rect {
    if total <= visible || total == 0 {
        return Rect { x: r.x, y: r.y, w: r.w, h: r.h };
    }
    let h = (r.h as u64 * visible as u64 / total as u64) as u32;
    let h = h.max(MIN_THUMB).min(r.h);
    let span = r.h - h;
    let max = total.saturating_sub(visible).max(1);
    let y = r.y + (span as u64 * scroll.min(max) as u64 / max as u64) as u32;
    Rect { x: r.x, y, w: r.w, h }
}

pub fn paint_scrollbar(fb: &mut PaintBuffer, r: Rect, scroll: usize, visible: usize, total: usize) {
    if total <= visible {
        return;
    }
    rrect::fill_round(fb, r.x, r.y, r.w, r.h, r.w / 2, theme::PANEL_ALT);
    let t = thumb(r, scroll, visible, total);
    rrect::fill_round(fb, t.x, t.y, t.w, t.h, t.w / 2, theme::TRACK);
}

pub fn track(body: Rect) -> Rect {
    Rect { x: body.x + body.w.saturating_sub(WIDTH), y: body.y, w: WIDTH, h: body.h }
}
