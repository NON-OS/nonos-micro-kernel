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
use crate::wallet::theme::{DIM, FG, MUTED};

pub fn paint_nox(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    let sw = (cw - 48) / 4;
    // These come from the NOX market and staking contracts on chain; until
    // those reads are wired the wallet shows a dash, never an invented figure.
    stat(fb, cx, 146, sw, "FEE (BPS)", "\u{2014}");
    stat(fb, cx + sw + 16, 146, sw, "STAKING APR", "\u{2014}");
    stat(fb, cx + 2 * (sw + 16), 146, sw, "CUMUL. REVENUE", "\u{2014}");
    stat(fb, cx + 3 * (sw + 16), 146, sw, "YOUR STAKE", "\u{2014}");

    ui::card(fb, cx, 252, cw, 150);
    let _ = fb.text_ttf((cx + 20) as i32, 270, "NOX ON-CHAIN STATE", DIM(), 10.5);
    let _ = fb.text_ttf(
        (cx + 20) as i32,
        304,
        "Live NOX market and staking figures are read from the",
        MUTED(),
        13.0,
    );
    let _ = fb.text_ttf(
        (cx + 20) as i32,
        326,
        "mainnet contracts through your RPC. Reading them into",
        MUTED(),
        13.0,
    );
    let _ = fb.text_ttf((cx + 20) as i32, 348, "the wallet is the next step.", MUTED(), 13.0);

    super::paint_nox_stake::paint_nox_stake(state, fb);
}

fn stat(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, val: &str) {
    ui::card(fb, x, y, w, 90);
    let _ = fb.text_ttf((x + 18) as i32, (y + 18) as i32, label, DIM(), 10.5);
    let _ = fb.text_ttf((x + 18) as i32, (y + 40) as i32, val, FG(), 28.0);
}
