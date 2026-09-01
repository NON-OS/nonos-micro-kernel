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

use super::constants::TEXT_LEFT;
use super::metrics::Metrics;
use crate::term::theme::types::Theme;

pub fn draw_cursor(
    fb: &mut PaintBuffer,
    prompt_cells: usize,
    cursor_cell: usize,
    baseline_y: u32,
    under: u8,
    m: Metrics,
    t: &Theme,
) {
    let (adv, px) = (m.adv, m.px);
    let x = TEXT_LEFT + (prompt_cells as u32 + cursor_cell as u32) * adv;
    fb.fill_rect(x, baseline_y, adv, m.lh.saturating_sub(2), t.accent);
    // Inverse block: when the cursor sits on a printable glyph, repaint it in
    // the background colour so the character reads through the block.
    if under > b' ' && under < 0x7f {
        let mut buf = [0u8; 4];
        let s = (under as char).encode_utf8(&mut buf);
        let _ = fb.text_ttf_mono(x as i32, baseline_y.saturating_sub(1) as i32, s, t.bg, px);
    }
}
