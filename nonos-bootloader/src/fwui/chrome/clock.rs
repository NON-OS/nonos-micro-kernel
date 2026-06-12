// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::fwui::data::{fmt_time, Sys};
use crate::fwui::draw::fill_rect;
use crate::fwui::layout::Layout;
use crate::fwui::metrics::{advance, glyph_h, pad};
use crate::fwui::text::text;
use crate::fwui::theme;
use alloc::format;
use uefi::table::runtime::Time;

pub fn clock(lay: &Layout, sys: &Sys, time: &Time) {
    let f = &lay.frame;
    let g = glyph_h();
    let ty = f.y.saturating_sub(g / 2);
    let s = format!("{} UTC    SECURE BOOT", fmt_time(time));
    let total = (s.len() as u32 + 3) * advance();
    let x = f.right().saturating_sub(pad() * 2 + total);
    fill_rect(
        x.saturating_sub(advance()),
        f.y.saturating_sub(g / 2),
        total + advance() * 3,
        g,
        theme::BG,
    );
    text(x, ty, s.as_bytes(), theme::DIM);
    let dx = x + (s.len() as u32 + 1) * advance();
    fill_rect(dx, ty + g / 4, g / 2, g / 2, if sys.secure_boot { theme::OK } else { theme::ERR });
}
