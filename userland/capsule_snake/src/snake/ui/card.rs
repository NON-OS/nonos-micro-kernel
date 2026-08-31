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
use nonos_toolkit::icons::{draw, IconId};

use crate::snake::theme::{LABEL, MUTED, PANEL_BG, PANEL_BORDER, TITLE};

use super::metrics::{GAP_TIGHT, ICON_SM, PAD_TIGHT, PX_BODY, PX_LABEL, PX_STAT, RADIUS_CARD};
use super::rect::Rect;
use super::text;

pub fn panel(fb: &mut PaintBuffer, r: Rect, radius: u32) {
    fb.fill_round(r.0, r.1, r.2, r.3, radius, PANEL_BG);
    fb.stroke_round(r.0, r.1, r.2, r.3, radius, 1, PANEL_BORDER);
}

// Caption with its mark, the number under it, then the subcaption. Every string
// is cut with the measured fit; nothing here counts glyphs.
pub fn stat(fb: &mut PaintBuffer, r: Rect, icon: IconId, caption: &[u8], value: &[u8], sub: &[u8]) {
    panel(fb, r, RADIUS_CARD);
    let left = r.0 + PAD_TIGHT;
    let max_w = r.2.saturating_sub(PAD_TIGHT * 2);
    let cap_h = line_height(PX_LABEL).max(1) as u32;
    let cap_top = r.1 + PAD_TIGHT;
    let mark_y = cap_top + cap_h.saturating_sub(ICON_SM) / 2;
    draw(fb, icon, left, mark_y, ICON_SM, MUTED);
    let cap_x = left + ICON_SM + GAP_TIGHT;
    let cap_w = max_w.saturating_sub(ICON_SM + GAP_TIGHT);
    let cut = text::fit(caption, PX_LABEL, cap_w);
    text::left(fb, cap_x, cap_top, cut, MUTED, PX_LABEL);
    let value_top = cap_top + cap_h + GAP_TIGHT;
    text::mono(fb, left, value_top, value, TITLE, PX_STAT);
    let sub_top = value_top + line_height(PX_STAT).max(1) as u32;
    text::left(fb, left, sub_top, text::fit(sub, PX_BODY, max_w), LABEL, PX_BODY);
}
