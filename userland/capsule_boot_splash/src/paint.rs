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

use nonos_toolkit::font::atlas::FontAtlas;
use nonos_toolkit::font::render::{draw_text, draw_text_scaled};

const BG: u32 = 0xFF00_0000;
const FG: u32 = 0xFFE8_F0F8;
const ACCENT: u32 = 0xFF00_D4AA;
const OK: u32 = 0xFF00_CC66;
const WARN: u32 = 0xFFFF_AA00;
const TITLE_SCALE: u32 = 5;

pub(crate) fn splash(base: u64, w: u32, h: u32, stride: u32, attested: Option<bool>) {
    let spx = stride as usize / 4;
    let buf = unsafe { core::slice::from_raw_parts_mut(base as *mut u32, spx * h as usize) };
    for p in buf.iter_mut() {
        *p = BG;
    }
    let atlas = FontAtlas::default();
    let title = b"NONOS";
    let tw = atlas.text_width(title) * TITLE_SCALE;
    draw_text_scaled(buf, spx, w, h, w.saturating_sub(tw) / 2, h / 2 - 48, title, ACCENT, TITLE_SCALE);
    let sub = b"initializing...";
    let sx = w.saturating_sub(atlas.text_width(sub)) / 2;
    draw_text(buf, spx, w, h, sx, h / 2 + 24, sub, FG);
    if let Some(ok) = attested {
        let (text, color): (&[u8], u32) =
            if ok { (b"ATTESTED", OK) } else { (b"UNVERIFIED", WARN) };
        let bx = w.saturating_sub(atlas.text_width(text)) / 2;
        draw_text(buf, spx, w, h, bx, h / 2 + 56, text, color);
    }
}
