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

use super::icon;
use super::layout::CARD_H;
use super::theme::{CY, INK, INK3, LINE, PANEL, RAISE};

// A card is its panel plus the gap below it, so a column advances by exactly
// what `card` returns and no caller repeats the arithmetic.
pub const CARD_GAP: u32 = 12;
const CARD_PAD: u32 = 16;
const CARD_ICON: u32 = 24;
const BAR_H: u32 = 6;

/// One card's content. `meta` is right-aligned on the title line, `bar` is an
/// optional 0..=100 fill drawn along the bottom of the panel.
pub struct Card<'a> {
    pub title: &'a str,
    pub sub: &'a str,
    pub meta: &'a str,
    pub tint: u32,
    pub dir: bool,
    pub bar: Option<u32>,
}

/// Draws the card at `(x, y)` across `w` and returns the height it consumed,
/// gap included, so a column of cards stacks by accumulating the return value.
pub fn card(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, c: &Card<'_>) -> u32 {
    fb.panel(x, y, w, CARD_H, 12, PANEL, LINE);
    let iy = y + (CARD_H - CARD_ICON) / 2;
    if c.dir {
        icon::folder(fb, x + CARD_PAD, iy, CARD_ICON, c.tint, PANEL);
    } else {
        icon::file(fb, x + CARD_PAD, iy, CARD_ICON, c.tint, PANEL);
    }
    let tx = (x + CARD_PAD * 2 + CARD_ICON) as i32;
    let _ = fb.text_ttf(tx, (y + 16) as i32, c.title, INK, 19.0);
    let _ = fb.text_ttf(tx, (y + 44) as i32, c.sub, INK3, 15.0);
    if !c.meta.is_empty() {
        let mw = fb.measure_ttf(c.meta, 15.0).max(0) as u32;
        let mx = (x + w).saturating_sub(CARD_PAD + mw);
        let _ = fb.text_ttf(mx as i32, (y + 16) as i32, c.meta, INK3, 15.0);
    }
    if let Some(pct) = c.bar {
        capacity_bar(fb, x + CARD_PAD, y + CARD_H - CARD_PAD, w.saturating_sub(CARD_PAD * 2), pct);
    }
    CARD_H + CARD_GAP
}

// The track is the raised surface and the fill the accent; both are opaque, so
// a raw rounded fill over the card panel is safe.
fn capacity_bar(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, pct: u32) {
    fb.fill_round(x, y, w, BAR_H, BAR_H / 2, RAISE);
    let filled = w * pct.min(100) / 100;
    if filled >= BAR_H {
        fb.fill_round(x, y, filled, BAR_H, BAR_H / 2, CY);
    }
}
