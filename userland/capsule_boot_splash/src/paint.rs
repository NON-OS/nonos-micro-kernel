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

use crate::chrome::panel;

const DIM: u32 = 0xFF5B_6B78;
const ACCENT: u32 = 0xFF00_D4AA;
const ACCENT_DIM: u32 = 0xFF00_5544;
const OK: u32 = 0xFF00_CC66;
const WARN: u32 = 0xFFFF_AA00;
const TITLE_SCALE: u32 = 5;
const SPIN: &[u8] = b"|/-\\";

pub(crate) fn splash(base: u64, w: u32, h: u32, stride: u32, attested: Option<bool>) {
    let spx = stride as usize / 4;
    let buf = unsafe { core::slice::from_raw_parts_mut(base as *mut u32, spx * h as usize) };
    crate::vignette::fill(buf, spx, w, h);
    let atlas = FontAtlas::default();
    let title = b"N\xd8NOS";
    let tw = atlas.text_width(title) * TITLE_SCALE;
    let tx = w.saturating_sub(tw) / 2;
    let ty = h / 2 - 96;
    draw_text_scaled(buf, spx, w, h, tx.saturating_sub(1), ty, title, ACCENT_DIM, TITLE_SCALE);
    draw_text_scaled(buf, spx, w, h, tx, ty, title, ACCENT, TITLE_SCALE);
    attest_panel(buf, spx, w, h, attested);
    status(buf, spx, w, h, 0);
}

fn attest_panel(buf: &mut [u32], spx: usize, w: u32, h: u32, attested: Option<bool>) {
    let pw = 300;
    let px = w.saturating_sub(pw) / 2;
    let py = h / 2;
    panel(buf, spx, w, h, px, py, pw, 86, b"boot-chain attestation");
    draw_text(buf, spx, w, h, px + 10, py + 30, b"[+] kernel  blake3 ok", OK);
    draw_text(buf, spx, w, h, px + 10, py + 48, b"[#] zk      groth16 ok", ACCENT);
    if let Some(ok) = attested {
        let (t, c): (&[u8], u32) = if ok { (b"ATTESTED", OK) } else { (b"UNVERIFIED", WARN) };
        draw_text(buf, spx, w, h, px + 10, py + 66, t, c);
    }
}

pub(crate) fn status(buf: &mut [u32], spx: usize, w: u32, h: u32, frame: u32) {
    let y = h / 2 + 116;
    crate::vignette::fill_band(buf, spx, w, h, y, 16);
    let atlas = FontAtlas::default();
    let sp = [SPIN[(frame % 4) as usize]];
    let cur: &[u8] = if frame & 1 == 0 { b"_" } else { b" " };
    let label = b"initializing";
    let x = w.saturating_sub(atlas.text_width(label) + 24) / 2;
    draw_text(buf, spx, w, h, x, y, &sp, ACCENT);
    draw_text(buf, spx, w, h, x + 16, y, label, DIM);
    draw_text(buf, spx, w, h, x + 16 + atlas.text_width(label) + 4, y, cur, ACCENT);
}
