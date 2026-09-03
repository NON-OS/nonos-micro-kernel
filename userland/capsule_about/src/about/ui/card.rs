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

use crate::about::theme::{BACKGROUND, CARD_BG, CARD_BORDER, LABEL, RULE_SOFT};

use super::metrics::{BODY_PX, CARD_HEAD_H, CARD_PAD, CARD_RADIUS};
use super::text;

// A titled card's first content row, and the height its frame costs on top of
// whatever it holds. Every screen sizes its cards from these two, so a change to
// the padding moves the layout and the scroll extent together.
pub const CONTENT_TOP: u32 = CARD_PAD * 2 + CARD_HEAD_H;
pub const OVERHEAD: u32 = CONTENT_TOP + CARD_PAD;

// The one panel shape, in a signed coordinate space because a scrolled card's top
// edge may already have passed the pane. What is gone is not drawn, so the
// surviving body is square where it was sliced and rounded only on the edges that
// are still real. The returned content top stays in i32 because the rows inside a
// sliced card sit above the pane too.
pub fn panel(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, h: u32) -> i32 {
    let content = y + CARD_PAD as i32;
    let bottom = y + h as i32;
    if bottom <= 0 || y >= fb.height as i32 || w == 0 {
        return content;
    }
    if y >= 0 {
        fb.fill_round(x, y as u32, w, h, CARD_RADIUS, CARD_BG);
        fb.stroke_round(x, y as u32, w, h, CARD_RADIUS, 1, CARD_BORDER);
        return content;
    }
    cut_top(fb, x, (bottom as u32).min(fb.height), w);
    content
}

// The sliced form. The foot is only rounded when the card's real bottom is inside
// the pane; stroke_round would leave a hairline across the body where its own top
// edge lands, so that one row is painted back out before the sides go down.
fn cut_top(fb: &mut PaintBuffer, x: u32, vis: u32, w: u32) {
    fb.fill_rect(x, 0, w, vis, CARD_BG);
    let r = CARD_RADIUS;
    let footed = vis > r * 2 && vis < fb.height;
    let sides = if footed { vis - r * 2 } else { vis };
    if footed {
        let foot = vis - r * 2;
        fb.fill_rect(x, foot, w, r * 2, BACKGROUND);
        fb.fill_round(x, foot, w, r * 2, r, CARD_BG);
        fb.stroke_round(x, foot, w, r * 2, r, 1, CARD_BORDER);
        fb.fill_rect(x + 1, foot, w.saturating_sub(2), 1, CARD_BG);
    }
    fb.fill_rect(x, 0, 1, sides, CARD_BORDER);
    fb.fill_rect(x + w.saturating_sub(1), 0, 1, sides, CARD_BORDER);
}

// The drawable width inside a card, which every screen needs and none should
// re-derive: the padding belongs to the card, not to the rows it holds.
pub fn inner(w: u32) -> u32 {
    w.saturating_sub(CARD_PAD * 2)
}

// A panel with a heading band and the hairline that closes it, in the same
// sliced-safe space as the panel under it. The returned value is the first
// content row, not the card top.
pub fn titled(fb: &mut PaintBuffer, x: u32, y: i32, w: u32, h: u32, title: &[u8]) -> i32 {
    let top = panel(fb, x, y, w, h);
    let inner_w = inner(w);
    let cut = text::fit(fb, title, BODY_PX, inner_w);
    text::line(fb, x + CARD_PAD, text::top_of(top, CARD_HEAD_H, BODY_PX), cut, LABEL, BODY_PX);
    let rule_y = top + CARD_HEAD_H as i32;
    text::rule(fb, x + CARD_PAD, rule_y, inner_w, RULE_SOFT);
    rule_y + CARD_PAD as i32
}
