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

use super::constants::{CELL_WIDTH, HEADER_H};
use super::shade::elevate;
use crate::term::state::State;
use crate::term::theme::{ACCENT, PATH};

pub const TAB_W: u32 = 16 * CELL_WIDTH;
pub const STRIP_Y: u32 = HEADER_H;
pub const STRIP_H: u32 = 16;
pub const CLOSE_W: u32 = 14;
pub const PLUS_W: u32 = 2 * CELL_WIDTH;

// Toolbar feature buttons, right-aligned in the strip: cycle theme, zoom out,
// zoom in, clear. Indices match `feature_hit`.
pub const FEAT_W: u32 = 34;
pub const FEATURES: [&[u8]; 4] = [b"thm", b"A-", b"A+", b"clr"];

// Which feature button an x falls on, or None when the click is left of them.
pub fn feature_hit(x: u32, width: u32) -> Option<usize> {
    let base = width.saturating_sub(FEATURES.len() as u32 * FEAT_W);
    if x < base {
        return None;
    }
    let i = ((x - base) / FEAT_W) as usize;
    (i < FEATURES.len()).then_some(i)
}

pub fn draw_tabstrip(tabs: &[State], active: usize, fb: &mut PaintBuffer) {
    // The strip tracks the active tab's theme background, one shade above the
    // body, so the whole chrome stays consistent when the profile changes.
    let strip = elevate(tabs[active].bg, 12);
    fb.fill_rect(0, STRIP_Y, fb.width, STRIP_H, strip);
    for (i, tab) in tabs.iter().enumerate() {
        let x = i as u32 * TAB_W;
        if i == active {
            fb.fill_rect(x, STRIP_Y, TAB_W, STRIP_H, ACCENT);
        }
        let mut label = [b' '; 12];
        label[0] = b'1' + i as u8;
        label[1] = b':';
        let base = basename(tab.cwd.as_bytes());
        let take = base.len().min(9);
        label[3..3 + take].copy_from_slice(&base[base.len() - take..]);
        // Active tab: text reads as cut out from the accent fill.
        let fg = if i == active { strip } else { PATH };
        fb.text(x + 4, STRIP_Y + 2, &label[..3 + take], fg);
        fb.text(x + TAB_W - CLOSE_W, STRIP_Y + 2, b"x", fg);
    }
    let px = tabs.len() as u32 * TAB_W;
    // New-tab affordance drawn as a chip so it reads as a button, not stray text.
    chip(fb, px + 2, strip, b"+", true);
    // Feature toolbar on the right: each control gets button chrome (raised fill
    // plus an accent hairline) so it looks clickable instead of loose text.
    let fbase = fb.width.saturating_sub(FEATURES.len() as u32 * FEAT_W);
    for (i, label) in FEATURES.iter().enumerate() {
        let bx = fbase + i as u32 * FEAT_W;
        chip(fb, bx, strip, label, false);
    }
}

// A compact button chip within the strip: a raised fill, a 1px accent border,
// and centered label. `wide` centers a short glyph (the "+").
fn chip(fb: &mut PaintBuffer, x: u32, strip: u32, label: &[u8], wide: bool) {
    let (w, h) = (FEAT_W - 4, STRIP_H - 2);
    let (cx, cy) = (x + 2, STRIP_Y + 1);
    let border = elevate(strip, 26);
    fb.fill_rect(cx, cy, w, h, border);
    fb.fill_rect(cx + 1, cy + 1, w.saturating_sub(2), h.saturating_sub(2), elevate(strip, 10));
    let text_w = label.len() as u32 * 8;
    let pad = if wide { (w.saturating_sub(6)) / 2 } else { (w.saturating_sub(text_w)) / 2 };
    fb.text(cx + pad, STRIP_Y + 3, label, ACCENT);
}

fn basename(path: &[u8]) -> &[u8] {
    match path.iter().rposition(|&b| b == b'/') {
        Some(i) if i + 1 < path.len() => &path[i + 1..],
        _ => path,
    }
}
