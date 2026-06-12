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

use super::layout::Layout;

const SCRIM: u32 = 0xFF20_262E;
const TEXT: u32 = 0xFFEC_EFF4;
const SUB: u32 = 0xFF9A_A4B2;

pub fn ready(layout: &Layout, fb: &mut PaintBuffer) {
    banner(layout, fb);
    centered(layout, fb, b"PRESS A DIRECTION KEY", layout.y + layout.h / 2 - 8, SUB, 2);
}

pub fn paused(layout: &Layout, fb: &mut PaintBuffer) {
    banner(layout, fb);
    centered(layout, fb, b"PAUSED", layout.y + layout.h / 2 - 12, TEXT, 3);
}

pub fn game_over(layout: &Layout, fb: &mut PaintBuffer) {
    banner(layout, fb);
    centered(layout, fb, b"GAME OVER", layout.y + layout.h / 2 - 32, TEXT, 3);
    centered(layout, fb, b"ENTER TO RESTART", layout.y + layout.h / 2 + 8, SUB, 2);
}

fn banner(layout: &Layout, fb: &mut PaintBuffer) {
    fb.fill_rect(layout.x, layout.y + layout.h / 2 - 48, layout.w, 96, SCRIM);
}

fn centered(layout: &Layout, fb: &mut PaintBuffer, text: &[u8], y: u32, argb: u32, scale: u32) {
    let w = text.len() as u32 * 8 * scale;
    let x = layout.x + layout.w.saturating_sub(w) / 2;
    fb.text_scaled(x, y, text, argb, scale);
}
