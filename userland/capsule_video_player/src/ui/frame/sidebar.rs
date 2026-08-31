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

use super::geom::{brand, collapsed, nav_item, sidebar};
use super::storage::paint_storage;
use crate::ui::icon::nav;
use crate::ui::paint::{rrect, shape};
use crate::ui::screen::{Route, NAV};
use crate::ui::text::{BODY_PX, TITLE_PX};
use crate::ui::theme;

const ICON: u32 = 20;
const ICON_X: u32 = 11;
const LABEL_X: u32 = 42;

pub fn paint_sidebar(fb: &mut PaintBuffer, route: Route, items: usize) {
    let (w, h) = (fb.width, fb.height);
    let s = sidebar(w, h);
    fb.fill_rect(s.x, s.y, s.w, s.h, theme::SIDEBAR_BG);
    shape::vline(fb, s.x + s.w.saturating_sub(1), s.y, s.h, theme::BORDER);
    paint_brand(fb, w);
    for (index, item) in NAV.iter().enumerate() {
        paint_item(fb, w, index, *item, route);
    }
    paint_storage(fb, w, h, items);
}

fn paint_brand(fb: &mut PaintBuffer, w: u32) {
    let b = brand(w);
    rrect::fill_round(fb, b.x, b.y, 30, 30, 8, theme::SELECT);
    nav::video(fb, b.x + 6, b.y + 6, 18, theme::ACCENT);
    if !collapsed(w) {
        fb.text_ttf((b.x + LABEL_X) as i32, (b.y + 4) as i32, "Video", theme::TEXT, TITLE_PX);
    }
}

fn paint_item(fb: &mut PaintBuffer, w: u32, index: usize, item: Route, route: Route) {
    let r = nav_item(w, index);
    let active = item == route;
    if active {
        rrect::fill_round(fb, r.x, r.y, r.w, r.h, 9, theme::SELECT);
        fb.fill_rect(r.x, r.y + 9, 3, r.h.saturating_sub(18), theme::ACCENT);
    }
    let tint = if active { theme::ACCENT } else { theme::TEXT_DIM };
    glyph_for(item)(fb, r.x + ICON_X, r.y + (r.h.saturating_sub(ICON)) / 2, ICON, tint);
    if collapsed(w) {
        return;
    }
    let color = if active { theme::TEXT } else { theme::TEXT_DIM };
    fb.text_ttf((r.x + LABEL_X) as i32, (r.y + 10) as i32, item.label(), color, BODY_PX);
}

fn glyph_for(item: Route) -> fn(&mut PaintBuffer, u32, u32, u32, u32) {
    match item {
        Route::Library => nav::library,
        Route::Playlists => nav::playlist,
        Route::Files => nav::files,
        Route::Settings => nav::gear,
        _ => nav::home,
    }
}
