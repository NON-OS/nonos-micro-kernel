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

use crate::ui::fit::right_x;
use crate::ui::layout::Rect;
use crate::ui::paint::rrect;
use crate::ui::text::{BODY_PX, TITLE_PX};
use crate::ui::theme;

pub const HEAD_H: u32 = 34;
pub const CARD_HEAD: u32 = 52;

pub fn paint_head(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, title: &str, action: &str) {
    fb.text_ttf(x as i32, y as i32, title, theme::TEXT, TITLE_PX);
    if !action.is_empty() {
        let ax = right_x(action, x + w, BODY_PX);
        fb.text_ttf(ax, (y + 4) as i32, action, theme::ACCENT, BODY_PX);
    }
}

pub fn paint_label(fb: &mut PaintBuffer, x: u32, y: u32, label: &str) {
    fb.text_ttf(x as i32, y as i32, label, theme::LABEL, BODY_PX);
}

pub fn paint_card(fb: &mut PaintBuffer, r: Rect) {
    rrect::panel(fb, r.x, r.y, r.w, r.h, 12, theme::PANEL, theme::BORDER);
}

pub fn paint_titled_card(fb: &mut PaintBuffer, r: Rect, title: &str) -> u32 {
    paint_card(fb, r);
    fb.text_ttf((r.x + 18) as i32, (r.y + 16) as i32, title, theme::TEXT, TITLE_PX);
    r.y + CARD_HEAD
}
