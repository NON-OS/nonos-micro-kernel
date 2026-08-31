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

use crate::ui::fit::fit;
use crate::ui::icon::ui::chevron_down;
use crate::ui::layout::Rect;
use crate::ui::paint::rrect;
use crate::ui::text::{center_y, BODY_PX};
use crate::ui::theme;

const CHEV: u32 = 13;
const PAD: u32 = 12;

pub fn paint_dropdown(fb: &mut PaintBuffer, r: Rect, label: &str) {
    rrect::panel(fb, r.x, r.y, r.w, r.h, 8, theme::PANEL, theme::BORDER);
    let room = r.w.saturating_sub(PAD * 2 + CHEV + 8);
    let shown = fit(label, room, BODY_PX);
    fb.text_ttf((r.x + PAD) as i32, center_y(r.y, r.h), shown, theme::TEXT, BODY_PX);
    let cx = (r.x + r.w).saturating_sub(PAD + CHEV);
    chevron_down(fb, cx, r.y + r.h.saturating_sub(CHEV) / 2, CHEV, theme::TEXT_DIM);
}

pub fn paint_labelled(fb: &mut PaintBuffer, r: Rect, caption: &str, label: &str) {
    fb.text_ttf(r.x as i32, r.y as i32, caption, theme::TEXT_DIM, BODY_PX);
    let box_y = r.y + 24;
    let h = r.h.saturating_sub(24);
    paint_dropdown(fb, Rect { x: r.x, y: box_y, w: r.w, h }, label);
}
