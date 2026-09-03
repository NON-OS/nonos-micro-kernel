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

use crate::about::data::{abi, build, product, trust};
use crate::about::format::trimmed;
use crate::about::theme::{ACCENT, FOREGROUND, MUTED, TITLE};

use super::super::card;
use super::super::chip::chip;
use super::super::metrics::{
    BODY_PX, CARD_PAD, CHIP_H, HERO_H, HERO_META_TOP, HERO_SUB_TOP, HERO_TEXT_X,
    HERO_TITLE_TOP, VALUE_PX,
};
use super::super::text::{self, line};
use super::overview_mark::mark;

const BADGE: &[u8] = b"Verified";

// The identity band: the mark, the product, and the four facts that name this
// exact image. The badge on the right is a passive label, not a control: reaching
// _start is what proves it, and nothing here can toggle that.
pub fn paint(fb: &mut PaintBuffer, y: i32, w: u32) {
    card::panel(fb, 0, y, w, HERO_H);
    mark(fb, y);
    line(fb, HERO_TEXT_X, y + HERO_TITLE_TOP as i32, product::NAME, TITLE, VALUE_PX);
    let sub = y + HERO_SUB_TOP as i32;
    let cut = text::fit(fb, product::TAGLINE, BODY_PX, w.saturating_sub(HERO_TEXT_X + CARD_PAD));
    line(fb, HERO_TEXT_X, sub, cut, FOREGROUND, BODY_PX);
    meta(fb, y + HERO_META_TOP as i32);
    badge(fb, y, w);
}

fn meta(fb: &mut PaintBuffer, y: i32) {
    let mut x = line(fb, HERO_TEXT_X, y, trimmed(build::VERSION), FOREGROUND, BODY_PX);
    for (value, accent) in [(build::GIT_SHA, false), (build::ARCH, false), (abi::NAME, true)] {
        x = line(fb, x.max(0) as u32, y, b"   /   ", MUTED, BODY_PX);
        let fg = if accent { ACCENT } else { FOREGROUND };
        x = line(fb, x.max(0) as u32, y, value, fg, BODY_PX);
    }
}

fn badge(fb: &mut PaintBuffer, y: i32, w: u32) {
    let top = y + HERO_TITLE_TOP as i32;
    let bw = super::super::chip::width_of(BADGE);
    let x = w.saturating_sub(CARD_PAD + bw);
    if top >= 0 && top + CHIP_H as i32 <= fb.height as i32 {
        chip(fb, x, top as u32, BADGE, true);
    }
    let sw = text::width_of(trust::HYBRID_SCHEME, BODY_PX);
    let sx = w.saturating_sub(CARD_PAD + sw);
    line(fb, sx, top + (CHIP_H + 10) as i32, trust::HYBRID_SCHEME, MUTED, BODY_PX);
}
