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

use crate::app::prefs::SECTION_SPAN;
use crate::ui::layout::Rect;
use crate::ui::widget::section::CARD_HEAD;

pub const RAIL_W: u32 = 200;
pub const ROW_H: u32 = 38;
pub const ACTIONS_H: u32 = 38;
pub const TOGGLE_ROW_H: u32 = 56;

pub fn rail(body: Rect) -> Rect {
    Rect { x: body.x, y: body.y, w: RAIL_W, h: body.h.saturating_sub(ACTIONS_H + 16) }
}

pub fn section_row(body: Rect, index: usize) -> Rect {
    let r = rail(body);
    Rect { x: r.x, y: r.y + index as u32 * ROW_H, w: r.w.saturating_sub(12), h: ROW_H - 4 }
}

pub fn card(body: Rect) -> Rect {
    let r = rail(body);
    let x = r.x + r.w + 24;
    Rect { x, y: body.y, w: (body.x + body.w).saturating_sub(x), h: r.h }
}

pub fn content(body: Rect) -> Rect {
    let c = card(body);
    Rect {
        x: c.x + 18,
        y: c.y + CARD_HEAD,
        w: c.w.saturating_sub(36),
        h: c.h.saturating_sub(CARD_HEAD),
    }
}

pub fn toggle_row(body: Rect, slot: usize) -> Rect {
    let c = content(body);
    Rect { y: c.y + slot as u32 * TOGGLE_ROW_H, h: TOGGLE_ROW_H, ..c }
}

pub fn reset_button(body: Rect) -> Rect {
    Rect { x: body.x, y: body.y + body.h.saturating_sub(ACTIONS_H), w: 170, h: ACTIONS_H }
}

pub fn save_button(body: Rect) -> Rect {
    let x = (body.x + body.w).saturating_sub(150);
    Rect { x, y: body.y + body.h.saturating_sub(ACTIONS_H), w: 150, h: ACTIONS_H }
}

pub fn cancel_button(body: Rect) -> Rect {
    let s = save_button(body);
    Rect { x: s.x.saturating_sub(120), w: 110, ..s }
}

pub fn section_at(body: Rect, x: i32, y: i32) -> Option<usize> {
    (0..SECTION_SPAN.len()).find(|&i| section_row(body, i).contains(x, y))
}

pub fn toggle_at(body: Rect, section: usize, x: i32, y: i32) -> Option<usize> {
    let (base, len) = SECTION_SPAN[section];
    (0..len).find(|&slot| toggle_row(body, slot).contains(x, y)).map(|slot| base + slot)
}
