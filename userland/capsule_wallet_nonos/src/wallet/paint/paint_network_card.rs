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

use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, DIM, FG, GREEN, LINE, MUTED};

pub fn paint_network_card(_state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    ui::card(fb, x, y, w, 200);
    let _ = fb.text_ttf((x + 20) as i32, (y + 18) as i32, "GAS  \u{00b7}  ETHEREUM L1", DIM(), 10.5);
    let gx = fb.text_ttf((x + 20) as i32, (y + 38) as i32, "16", FG(), 30.0);
    let _ = fb.text_ttf(gx + 8, (y + 50) as i32, "gwei base", MUTED(), 13.0);
    row(fb, x, w, y + 84, "Slow", FG(), "12 gwei", "~2m");
    row(fb, x, w, y + 122, "Avg", ACCENT(), "18 gwei", "~30s");
    row(fb, x, w, y + 160, "Fast", GREEN(), "26 gwei", "~12s");
}

fn row(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, label: &str, lc: u32, val: &str, t: &str) {
    fb.fill_rect(x + 20, y + 30, w - 40, 1, LINE());
    let _ = fb.text_ttf((x + 20) as i32, (y + 6) as i32, label, lc, 14.0);
    let vw = fb.measure_ttf(val, 14.0).max(0) as u32;
    let _ = fb.text_ttf((x + w - 96 - vw) as i32, (y + 6) as i32, val, FG(), 14.0);
    let tw = fb.measure_ttf(t, 13.0).max(0) as u32;
    let _ = fb.text_ttf((x + w - 20 - tw) as i32, (y + 7) as i32, t, DIM(), 13.0);
}
