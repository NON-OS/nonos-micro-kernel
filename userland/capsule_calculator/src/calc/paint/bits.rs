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
use nonos_toolkit::font::ttf::line_height;

use super::bits_num::NUM;
use crate::calc::hit::Hit;
use crate::calc::prog::BITS;
use crate::calc::state::State;
use crate::calc::theme::{CYAN, FAINT, KEY, KEY_HI, LINE, LINE_3};
use crate::calc::ui::bits_geom::{cell, BIT_H, LABEL_H};
use crate::calc::ui::metrics::{PX_MONO, R_KEY};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let win_w = fb.width as i32;
    let lh = line_height(PX_MONO).max(1);
    for bit in 0..BITS {
        let (x, y, w, h) = cell(win_w, bit);
        if w <= 0 || x < 0 || y < 0 {
            return;
        }
        let set = state.prog & (1i64 << bit) != 0;
        let (bg, ink) = if set { (KEY_HI, CYAN) } else { (KEY, FAINT) };
        let (ux, uy, uw, uh) = (x as u32, y as u32, w as u32, h as u32);
        let radius = R_KEY as u32;
        fb.fill_round(ux, uy, uw, uh, radius, bg);
        fb.stroke_round(ux, uy, uw, uh, radius, 1, LINE);
        if state.hover == Some(Hit::Bit(bit)) {
            fb.blend_rect(ux, uy, uw, uh, LINE);
            fb.stroke_round(ux, uy, uw, uh, radius, 1, LINE_3);
        }
        let glyph = if set { "1" } else { "0" };
        let gx = x + (w - fb.measure_ttf_mono(glyph, PX_MONO)) / 2;
        fb.text_ttf_mono(gx, y + (h - lh) / 2, glyph, ink, PX_MONO);
        let name = NUM[bit as usize];
        let nx = x + (w - fb.measure_ttf_mono(name, PX_MONO)) / 2;
        fb.text_ttf_mono(nx, y + BIT_H + (LABEL_H - lh) / 2, name, FAINT, PX_MONO);
    }
}
