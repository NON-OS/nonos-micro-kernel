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

use crate::calc::convert::{convert, list, Category};
use crate::calc::format::{format, DISPLAY_MAX};
use crate::calc::state::State;
use crate::calc::theme::{AMBER, CYAN, ERROR, FAINT, LINE_2, PANEL};
use crate::calc::ui::convert_geom::result;
use crate::calc::ui::metrics::{PX_BODY, R_PANEL};
use crate::calc::ui::trim::trim;

const LADDER: [f32; 3] = [34.0, 26.0, 17.0];
const CAPTION: &str = "static rates - not live";
const PAD: i32 = 14;

fn fit(fb: &PaintBuffer, text: &str, budget: i32) -> f32 {
    for px in LADDER.iter() {
        if fb.measure_ttf(text, *px) <= budget {
            return *px;
        }
    }
    LADDER[LADDER.len() - 1]
}

fn pair(state: &State, fb: &mut PaintBuffer, x: i32, y: i32, budget: i32) {
    let units = list(state.cat);
    let src = units.get(state.from).map(|unit| unit.name).unwrap_or("");
    let dst = units.get(state.to).map(|unit| unit.name).unwrap_or("");
    let (head, _) = trim(fb, src, budget, PX_BODY);
    let mut cursor = fb.text_ttf(x, y, head, FAINT, PX_BODY);
    cursor = fb.text_ttf(cursor, y, " to ", FAINT, PX_BODY);
    let (tail, _) = trim(fb, dst, budget, PX_BODY);
    fb.text_ttf(cursor, y, tail, FAINT, PX_BODY);
}

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let (x, y, w, h) = result(fb.width as i32, fb.height as i32);
    if w <= PAD * 2 || h <= 0 || x < 0 || y < 0 {
        return;
    }
    fb.panel(x as u32, y as u32, w as u32, h as u32, R_PANEL as u32, PANEL, LINE_2);
    let lh = line_height(PX_BODY).max(1);
    pair(state, fb, x + PAD, y + PAD, w / 3);
    if state.cat == Category::Currency {
        fb.text_ttf(x + PAD, y + h - PAD - lh, CAPTION, AMBER, PX_BODY);
    }
    let mut buf = [0u8; DISPLAY_MAX];
    let (text, ink) = match convert(state.cat, state.from, state.to, state.display) {
        Some(value) => {
            let n = format(value, 0, &mut buf);
            (core::str::from_utf8(&buf[..n]).unwrap_or("0"), CYAN)
        }
        None => ("Error", ERROR),
    };
    let px = fit(fb, text, w / 2);
    let tx = x + w - PAD - fb.measure_ttf(text, px);
    fb.text_ttf(tx, y + (h - line_height(px).max(1)) / 2, text, ink, px);
}
