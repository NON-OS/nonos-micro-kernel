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
use crate::wallet::theme::{ACCENT, CYAN, DIM, FG, GREEN, GREEN_INK, INK, MUTED};

pub fn paint_portfolio(_state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    let col = (cw - 16) / 2;
    ui::card(fb, cx, 146, col, 120);
    let _ = fb.text_ttf((cx + 20) as i32, 166, "TRANSPARENT  \u{00b7}  ON-CHAIN", DIM(), 10.5);
    let px = fb.text_ttf((cx + 20) as i32, (188) as i32, "2.4091", FG(), 34.0);
    let _ = fb.text_ttf(px + 8, 202, "ETH", CYAN(), 16.0);
    let _ = fb.text_ttf((cx + 20) as i32, 240, "= $7,516.40  \u{00b7}  public", MUTED(), 13.0);

    let rx = cx + col + 16;
    ui::card(fb, rx, 146, col, 120);
    let _ = fb.text_ttf((rx + 20) as i32, 166, "SHIELDED  \u{00b7}  PRIVATE", DIM(), 10.5);
    ui::badge(fb, rx + col - 90, 162, b"6 notes", ACCENT(), INK());
    let sx = fb.text_ttf((rx + 20) as i32, 188, "0.842", GREEN(), 34.0);
    let _ = fb.text_ttf(sx + 8, 202, "ETH", GREEN(), 16.0);
    let _ = fb.text_ttf((rx + 20) as i32, 240, "unspent  \u{00b7}  view-tag scanned", MUTED(), 13.0);

    let _ = fb.text_ttf(cx as i32, 296, "SHIELDED NOTES", DIM(), 10.5);
    note(fb, cx, cw, 318, "note #1", "0.42 ETH");
    note(fb, cx, cw, 372, "note #2", "0.30 ETH");
    note(fb, cx, cw, 426, "note #3", "0.122 ETH");
}

fn note(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, name: &str, amt: &str) {
    ui::card(fb, x, y, w, 44);
    fb.fill_rect(x, y, 3, 44, GREEN());
    let _ = fb.text_ttf_mono((x + 20) as i32, (y + 13) as i32, name, FG(), 14.0);
    let aw = fb.measure_ttf(amt, 14.0).max(0) as u32;
    let _ = fb.text_ttf((x + w / 2 - aw / 2) as i32, (y + 13) as i32, amt, FG(), 14.0);
    ui::badge(fb, x + w - 88, y + 12, b"sealed", GREEN(), GREEN_INK());
}
