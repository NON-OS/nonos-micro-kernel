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

//! Button in the three section-03 variants: solid, glow and ghost.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{pill, ITEM, R_CTL, S3};
use crate::ui::paint::{fill, panel, shadow, text, width};
use crate::ui::theme::{alpha, rgb, CYAN, EDGE, INK, VOID};

#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Solid,
    Glow,
    Ghost,
}

pub fn measure(label: &str, glyph: bool) -> i32 {
    let icon = if glyph { ITEM as i32 + S3 } else { 0 };
    width(label, ITEM) + icon + S3 * 4
}

pub fn button(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    label: &str,
    glyph: Option<Glyph>,
    v: Variant,
) {
    let rad = match v {
        Variant::Ghost => R_CTL,
        _ => pill(r.h),
    };
    let fg = match v {
        Variant::Ghost => INK,
        _ => VOID,
    };
    match v {
        Variant::Solid => fill(fb, r, rad, INK),
        Variant::Glow => {
            shadow(fb, r, rad, (r.h / 3) as u32, alpha(rgb(CYAN), 0x4C));
            fill(fb, r, rad, CYAN);
        }
        Variant::Ghost => panel(fb, r, rad, alpha(rgb(INK), 0x0F), EDGE),
    }
    let icon_w = if glyph.is_some() { ITEM as i32 + S3 } else { 0 };
    let run = width(label, ITEM) + icon_w;
    let mut x = r.x + (r.w - run) / 2;
    if let Some(g) = glyph {
        let s = ITEM as i32;
        icons.draw(fb, Rect::new(x, r.cy() - s / 2, s, s), g, fg);
        x += s + S3;
    }
    text(fb, x, r.cy() - (ITEM * 0.72) as i32, label, fg, ITEM);
}
