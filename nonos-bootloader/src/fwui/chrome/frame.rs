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

use crate::fwui::draw::{draw_rect, fill_rect, line};
use crate::fwui::layout::Layout;
use crate::fwui::metrics::{advance, glyph_h, pad};
use crate::fwui::text::text;
use crate::fwui::theme;

pub fn frame(lay: &Layout) {
    let f = &lay.frame;
    let (g, adv) = (glyph_h(), advance());
    fill_rect(0, 0, lay.screen_w, lay.screen_h, theme::BG);
    draw_rect(f.x, f.y, f.w, f.h, theme::FRAME);
    let title = b"NONOS  FIRMWARE SETUP";
    let tx = f.x + pad() * 2;
    fill_rect(
        tx.saturating_sub(adv),
        f.y.saturating_sub(g / 2),
        (title.len() as u32 + 2) * adv,
        g,
        theme::BG,
    );
    text(tx, f.y.saturating_sub(g / 2), title, theme::TEXT);
    line(
        (tx + adv + 1) as i32,
        (f.y + g / 2 - 2) as i32,
        (tx + 2 * adv - 2) as i32,
        (f.y.saturating_sub(g / 2) + 1) as i32,
        theme::ACCENT,
    );
    let by = f.bottom().saturating_sub(g / 2);
    let left = b"NO OS PRESENT    UEFI x86_64";
    fill_rect(f.x + pad(), by, (left.len() as u32 + 2) * adv, g, theme::BG);
    text(f.x + pad() * 2, by, left, theme::MUTE);
    let help = b"UP/DN ITEM    LF/RT SCREEN    ENTER SELECT    F10 BOOT";
    let hx = f.right().saturating_sub(pad() * 2 + help.len() as u32 * adv);
    fill_rect(hx.saturating_sub(adv), by, (help.len() as u32 + 2) * adv, g, theme::BG);
    text(hx, by, help, theme::MUTE);
}
