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

use nonos_app_skeleton::{measure_ttf, PaintBuffer};

use super::theme::{CY, INK2, RAISE, VOID};

// A pill sized from its measured label, so a chip row and any per-row chip lay
// out to the same widths.
pub const CHIP_H: u32 = 28;
pub const CHIP_GAP: u32 = 8;
const CHIP_PAD: u32 = 12;
const CHIP_PX: f32 = 14.0;

/// The pill's own width, without the gap that follows it. Measured off the
/// free function rather than a surface, so the chip hit-test -- which has no
/// PaintBuffer to ask -- sizes every pill exactly as the painter did.
pub fn chip_w(label: &str) -> u32 {
    measure_ttf(label, CHIP_PX).max(0) as u32 + CHIP_PAD * 2
}

/// Draws one chip and returns its advance: the pill width plus the inter-chip
/// gap, so a row lays out by accumulating the return value.
pub fn chip(fb: &mut PaintBuffer, x: u32, y: u32, label: &str, active: bool) -> u32 {
    let w = chip_w(label);
    let (fill, ink) = if active { (CY, VOID) } else { (RAISE, INK2) };
    fb.fill_round(x, y, w, CHIP_H, CHIP_H / 2, fill);
    let _ = fb.text_ttf((x + CHIP_PAD) as i32, (y + 3) as i32, label, ink, CHIP_PX);
    w + CHIP_GAP
}
