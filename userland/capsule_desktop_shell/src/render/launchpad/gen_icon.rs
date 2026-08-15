// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A generated tile for a command-line tool that ships no artwork: a coloured
//! rounded square, its hue derived from the name so each tool stays visually
//! distinct, with the tool's uppercase initial in the centre.

use crate::render::fill::fill_rect;
use crate::render::measure_aa::measure_aa_bytes;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font;
use crate::render::ui_font::{top_y_centered, TITLE_PX};
use crate::state::Context;

const TILE_BG: u32 = 0xFF0C_1016;

pub(super) fn draw(ctx: &Context, x: u32, y: u32, size: u32, label: &[u8]) {
    let (va, st, vw, vh) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    let sc = ui_font::scale();
    fill_rect(va, st, vw, vh, x, y, size, size, TILE_BG);
    let accent = hue(label);
    let inset = 4 * sc;
    fill_rect(va, st, vw, vh, x + inset, y + inset, size - 8 * sc, size - 8 * sc, accent);
    // Knock a small stair off each corner so the fill reads as rounded.
    let s2 = size.saturating_sub(6 * sc);
    for &(cx, cy) in
        &[(x + inset, y + inset), (x + s2, y + inset), (x + inset, y + s2), (x + s2, y + s2)]
    {
        fill_rect(va, st, vw, vh, cx, cy, 2 * sc, 2 * sc, TILE_BG);
    }
    let ch = [initial(label)];
    let gx = x + size.saturating_sub(measure_aa_bytes(&ch, TITLE_PX)) / 2;
    let gy = top_y_centered(y, size, TITLE_PX);
    text_aa_bytes(ctx, gx, gy, &ch, 0xFF0A_0E14, TITLE_PX);
}

// The tool's first letter, uppercased for the tile glyph.
fn initial(label: &[u8]) -> u8 {
    match label.first().copied() {
        Some(c @ b'a'..=b'z') => c - 32,
        Some(c) => c,
        None => b'?',
    }
}

// A stable, bright hue from the name so distinct tools never share a colour.
fn hue(label: &[u8]) -> u32 {
    let mut h: u32 = 2166136261;
    for &b in label {
        h = (h ^ b as u32).wrapping_mul(16777619);
    }
    let r = 0x60 | (h & 0x7F);
    let g = 0x60 | ((h >> 8) & 0x7F);
    let b = 0x60 | ((h >> 16) & 0x7F);
    0xFF00_0000 | (r << 16) | (g << 8) | b
}
