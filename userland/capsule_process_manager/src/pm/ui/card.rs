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

use crate::pm::theme::{CARD_BG, CARD_BORDER, MUTED, TITLE, WARNING};

use super::metrics::{
    BODY_PX, CARD_H, CARD_ICON, CARD_LABEL_GAP, CARD_LINE_GAP, CARD_PAD, CARD_RADIUS, CARD_VALUE_PX,
};
use super::text;

// A stat card: icon and caption on the first line, the number in mono with its
// unit and a right-aligned subcaption on the second, and whatever height is left
// as a band at the bottom. The band's y is returned rather than assumed, so a
// caller drawing a sparkline into it never re-derives this stack.
pub fn paint(
    fb: &mut PaintBuffer,
    x: u32,
    y: u32,
    w: u32,
    icon: IconId,
    caption: &[u8],
    value: &[u8],
    unit: &[u8],
    sub: &[u8],
) -> u32 {
    fb.fill_round(x, y, w, CARD_H, CARD_RADIUS, CARD_BG);
    fb.stroke_round(x, y, w, CARD_H, CARD_RADIUS, 1, CARD_BORDER);
    let left = x + CARD_PAD;
    let cap_top = y + CARD_PAD;
    let body_h = line_height(BODY_PX).max(1) as u32;
    draw(fb, icon, left, cap_top + body_h.saturating_sub(CARD_ICON) / 2, CARD_ICON, MUTED);
    text::left(fb, left + CARD_ICON + CARD_LABEL_GAP, cap_top, caption, WARNING, BODY_PX);
    let value_top = cap_top + body_h + CARD_LINE_GAP;
    let value_h = line_height(CARD_VALUE_PX).max(1) as u32;
    let after = text::mono(fb, left, value_top, value, TITLE, CARD_VALUE_PX).max(0) as u32;
    let base = value_top + value_h.saturating_sub(body_h);
    text::left(fb, after + CARD_LABEL_GAP, base, unit, WARNING, BODY_PX);
    text::right(fb, x + w - CARD_PAD, base, sub, MUTED, BODY_PX);
    value_top + value_h + CARD_LINE_GAP
}
