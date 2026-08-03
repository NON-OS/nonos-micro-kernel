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

use super::decimal::{render_u32, DIGITS};
use super::manifest::WIDTH;

const BG: u32 = 0xFF10161C;
const ACCENT: u32 = 0xFF00D4AA;
const TEXT: u32 = 0xFFE8F0F8;
const DIM: u32 = 0xFF667788;
const PANEL: u32 = 0xFF1A2430;

pub fn paint(fb: &mut PaintBuffer, clicks: u32) {
    fb.clear(BG);
    fb.fill_rect(0, 0, WIDTH, 4, ACCENT);
    fb.text_scaled(24, 36, b"gui demo", ACCENT, 2);
    fb.text(24, 76, b"click the window to count", TEXT);
    fb.fill_rect(24, 100, 332, 60, PANEL);
    fb.text(40, 122, b"clicks", DIM);
    let mut buf = [0u8; DIGITS];
    fb.text_scaled(140, 112, render_u32(clicks, &mut buf), TEXT, 3);
    fb.text(24, 186, b"press Esc to close", DIM);
}
