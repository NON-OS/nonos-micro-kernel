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

use crate::app::browse::Browse;
use crate::ui::frame::{geom, topbar};
use crate::ui::layout::Rect;
use crate::ui::screen::{Route, NAV};
use crate::ui::view::grid::grid_capacity;
use crate::ui::widget::card::tile_rect;
use crate::ui::widget::table::{row_rect, rows_visible, HEAD_H};

pub fn nav(w: u32, x: i32, y: i32) -> Option<Route> {
    geom::nav_hit(w, x, y).and_then(|i| NAV.get(i).copied())
}

pub fn view_mode(w: u32, h: u32, x: i32, y: i32) -> Option<bool> {
    let r = topbar::view_toggle(w, h);
    if !r.contains(x, y) {
        return None;
    }
    Some(topbar::grid_half(r).contains(x, y))
}

fn list_rows(body: Rect) -> Rect {
    Rect {
        x: body.x,
        y: body.y + HEAD_H + 4,
        w: body.w.saturating_sub(12),
        h: body.h.saturating_sub(HEAD_H + 4),
    }
}

pub fn item(browse: &Browse, body: Rect, x: i32, y: i32) -> Option<usize> {
    if browse.grid {
        let cap = grid_capacity(body);
        return (0..cap)
            .find(|&slot| tile_rect(body, slot).contains(x, y))
            .map(|slot| browse.scroll + slot)
            .filter(|&i| i < browse.len());
    }
    let rows = list_rows(body);
    (0..rows_visible(rows.h))
        .find(|&slot| row_rect(rows, slot).contains(x, y))
        .map(|slot| browse.scroll + slot)
        .filter(|&i| i < browse.len())
}

pub fn page_step(body: Rect, grid: bool) -> usize {
    if grid {
        return grid_capacity(body).max(1);
    }
    rows_visible(list_rows(body).h).max(1)
}
