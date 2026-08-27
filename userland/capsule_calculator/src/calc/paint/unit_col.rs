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

use crate::calc::convert::list;
use crate::calc::hit::Hit;
use crate::calc::state::State;
use crate::calc::theme::{CYAN, DIM, FAINT, GLOW, KEY, LINE, LINE_3};
use crate::calc::ui::convert_geom::{col_top, row, HEAD_H};
use crate::calc::ui::convert_hit::ConvertHit;
use crate::calc::ui::metrics::{PX_BODY, R_KEY};
use crate::calc::ui::trim::trim;

const PAD: i32 = 12;

fn probe(from: bool, i: usize) -> Hit {
    if from {
        Hit::Convert(ConvertHit::From(i))
    } else {
        Hit::Convert(ConvertHit::To(i))
    }
}

pub fn paint(state: &State, fb: &mut PaintBuffer, from: bool) {
    let win_w = fb.width as i32;
    let lh = line_height(PX_BODY).max(1);
    let head = if from { "From" } else { "To" };
    let selected = if from { state.from } else { state.to };
    let (hx, _, _, _) = row(win_w, from, 0);
    fb.text_ttf(hx, col_top() + (HEAD_H - lh) / 2, head, FAINT, PX_BODY);
    for (i, unit) in list(state.cat).iter().enumerate() {
        let (x, y, w, h) = row(win_w, from, i);
        if w <= PAD * 2 || x < 0 || y < 0 {
            return;
        }
        let active = i == selected;
        let (ux, uy, uw, uh) = (x as u32, y as u32, w as u32, h as u32);
        let radius = R_KEY as u32;
        fb.fill_round(ux, uy, uw, uh, radius, if active { GLOW } else { KEY });
        fb.stroke_round(ux, uy, uw, uh, radius, 1, if active { LINE_3 } else { LINE });
        if !active && state.hover == Some(probe(from, i)) {
            fb.blend_rect(ux, uy, uw, uh, LINE);
        }
        let (label, _) = trim(fb, unit.name, w - PAD * 2, PX_BODY);
        let ink = if active { CYAN } else { DIM };
        fb.text_ttf(x + PAD, y + (h - lh) / 2, label, ink, PX_BODY);
    }
}
