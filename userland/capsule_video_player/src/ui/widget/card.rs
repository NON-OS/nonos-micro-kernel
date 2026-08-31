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

use super::{poster::paint_poster, progress::paint_bar};
use crate::catalog::media::MediaItem;
use crate::ui::fit::fit;
use crate::ui::format::duration;
use crate::ui::layout::Rect;
use crate::ui::paint::rrect;
use crate::ui::text::BODY_PX;
use crate::ui::theme;

pub const TILE_W: u32 = 214;
pub const TILE_H: u32 = 186;
pub const GAP: u32 = 16;
const ART_H: u32 = 118;
const BAR_H: u32 = 4;

pub fn columns(w: u32) -> usize {
    (((w + GAP) / (TILE_W + GAP)) as usize).max(1)
}

pub fn tile_rect(body: Rect, slot: usize) -> Rect {
    let cols = columns(body.w);
    let col = (slot % cols) as u32;
    let row = (slot / cols) as u32;
    Rect {
        x: body.x + col * (TILE_W + GAP),
        y: body.y + row * (TILE_H + GAP),
        w: TILE_W,
        h: TILE_H,
    }
}

pub fn paint_tile(fb: &mut PaintBuffer, r: Rect, item: &MediaItem, selected: bool) {
    let border = if selected { theme::ACCENT_DIM } else { theme::BORDER };
    rrect::panel(fb, r.x, r.y, r.w, r.h, 12, theme::PANEL, border);
    let art = Rect { x: r.x + 8, y: r.y + 8, w: r.w - 16, h: ART_H };
    paint_poster(fb, art, item.kind);
    let permille = item.permille();
    if permille > 0 {
        let bar = Rect { x: art.x, y: art.y + art.h - BAR_H, w: art.w, h: BAR_H };
        paint_bar(fb, bar, permille, theme::ACCENT);
    }
    let room = r.w - 24;
    let title = fit(item.title(), room, BODY_PX);
    fb.text_ttf((r.x + 12) as i32, (art.y + art.h + 12) as i32, title, theme::TEXT, BODY_PX);
    let meta = duration(item.duration_ms);
    let sub = fit(&meta, room, BODY_PX);
    let sy = (art.y + art.h + 34) as i32;
    fb.text_ttf((r.x + 12) as i32, sy, sub, theme::TEXT_MUTED, BODY_PX);
}
