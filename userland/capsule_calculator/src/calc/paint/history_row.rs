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

use crate::calc::format::{format, DISPLAY_MAX};
use crate::calc::history::Entry;
use crate::calc::hit::Hit;
use crate::calc::state::State;
use crate::calc::theme::{DIM, INK, KEY, LINE, LINE_3};
use crate::calc::ui::history_geom::row;
use crate::calc::ui::metrics::{PX_BODY, R_KEY};
use crate::calc::ui::trim::{trim, ELLIPSIS};

const PAD: i32 = 12;

pub fn paint(state: &State, fb: &mut PaintBuffer, index: usize, entry: &Entry) {
    let (x, y, w, h) = row(fb.width as i32, index);
    if w <= PAD * 3 || x < 0 || y < 0 {
        return;
    }
    let (ux, uy, uw, uh) = (x as u32, y as u32, w as u32, h as u32);
    let radius = R_KEY as u32;
    fb.fill_round(ux, uy, uw, uh, radius, KEY);
    fb.stroke_round(ux, uy, uw, uh, radius, 1, LINE);
    if state.hover == Some(Hit::Row(index)) {
        fb.blend_rect(ux, uy, uw, uh, LINE);
        fb.stroke_round(ux, uy, uw, uh, radius, 1, LINE_3);
    }
    let lh = line_height(PX_BODY).max(1);
    let ty = y + (h - lh) / 2;
    let mut buf = [0u8; DISPLAY_MAX];
    let n = format(entry.value, 0, &mut buf);
    let value = core::str::from_utf8(&buf[..n]).unwrap_or("");
    let vw = fb.measure_ttf(value, PX_BODY);
    fb.text_ttf(x + w - PAD - vw, ty, value, INK, PX_BODY);
    let (expr, cut) = trim(fb, entry.text(), w - PAD * 3 - vw, PX_BODY);
    let pen = fb.text_ttf(x + PAD, ty, expr, DIM, PX_BODY);
    if cut {
        fb.text_ttf(pen, ty, ELLIPSIS, DIM, PX_BODY);
    }
}
