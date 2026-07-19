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
use crate::wallet::theme::{ACCENT, CYAN, DIM, FG, GREEN};

const REV: [u8; 14] = [8, 12, 14, 16, 22, 26, 30, 36, 44, 52, 60, 72, 84, 96];
const APR: [u8; 10] = [38, 42, 40, 54, 58, 66, 70, 68, 80, 92];

pub fn paint_nox(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    let sw = (cw - 48) / 4;
    stat(fb, cx, 146, sw, "FEE (BPS)", "30", FG);
    stat(fb, cx + sw + 16, 146, sw, "STAKING APR (BPS)", "840", CYAN);
    stat(fb, cx + 2 * (sw + 16), 146, sw, "CUMUL. REVENUE", "112.4 ETH", FG);
    stat(fb, cx + 3 * (sw + 16), 146, sw, "YOUR STAKE", "4,000", CYAN);

    let col = (cw - 16) / 2;
    ui::card(fb, cx, 252, col, 170);
    let _ = fb.text_ttf((cx + 20) as i32, 270, "CUMULATIVE REVENUE", DIM, 10.5);
    ui::bars(fb, cx + 20, 296, col - 40, 108, &REV, ACCENT);
    let rx = cx + col + 16;
    ui::card(fb, rx, 252, col, 170);
    let _ = fb.text_ttf((rx + 20) as i32, 270, "APR HISTORY (BPS)", DIM, 10.5);
    ui::bars(fb, rx + 20, 296, col - 40, 108, &APR, GREEN);

    super::paint_nox_stake::paint_nox_stake(state, fb);
}

fn stat(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, val: &str, color: u32) {
    ui::card(fb, x, y, w, 90);
    let _ = fb.text_ttf((x + 18) as i32, (y + 18) as i32, label, DIM, 10.5);
    let _ = fb.text_ttf((x + 18) as i32, (y + 40) as i32, val, color, 28.0);
}
