// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A generated tile for a command-line tool that ships no artwork: a coloured
//! rounded square, its hue derived from the name so each tool stays visually
//! distinct, with the tool's uppercase initial in the centre.

use crate::render::measure_aa::measure_aa_bytes;
use crate::render::palette;
use crate::render::surface::surface;
use crate::render::text_aa::text_aa_bytes;
use crate::render::ui_font;
use crate::render::ui_font::{top_y_centered, TITLE_PX};
use crate::state::Context;

pub(super) fn draw(ctx: &Context, x: u32, y: u32, size: u32, label: &[u8]) {
    {
        let mut buf = surface(ctx);
        let r = (size * 10 / 46).max(2);
        buf.panel(x, y, size, size, r, palette::TILE_FILL, palette::LINE_SOFT);
        let inset = 4 * ui_font::scale();
        let inner = size.saturating_sub(2 * inset);
        buf.fill_round(
            x + inset,
            y + inset,
            inner,
            inner,
            r.saturating_sub(inset / 2).max(1),
            hue_of(label),
        );
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

// A stable, bright hue from the name so distinct tools never share a colour,
// masked to a translucent alpha so the accent sits inside the glass plate
// rather than reading as a solid block.
fn hue_of(label: &[u8]) -> u32 {
    let mut h: u32 = 2166136261;
    for &b in label {
        h = (h ^ b as u32).wrapping_mul(16777619);
    }
    let r = 0x60 | (h & 0x7F);
    let g = 0x60 | ((h >> 8) & 0x7F);
    let b = 0x60 | ((h >> 16) & 0x7F);
    let base = 0xFF00_0000 | (r << 16) | (g << 8) | b;
    (base & 0x00FF_FFFF) | 0x6600_0000
}
