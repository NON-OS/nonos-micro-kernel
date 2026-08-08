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

//! One half of the trade.

use nonos_app_skeleton::PaintBuffer;

use super::super::ui;
use super::mark::mark;
use crate::wallet::state::State;
use crate::wallet::swap::token;
use crate::wallet::theme::{CYAN, DIM, FG, LINE2, PANEL_2};

/// The panel a reader reads a figure off.
///
/// Both sides are drawn the same on purpose. Checking a trade means
/// comparing two numbers, and anything that makes one side look unlike the
/// other slows that comparison down.
pub fn side(fb: &mut PaintBuffer, state: &State, x: u32, y: u32, w: u32, cap: &str, pay: bool) {
    let idx = if pay { state.swap_from } else { state.swap_to };
    let t = token(idx);
    ui::bordered(fb, x + 20, y, w - 40, 88, PANEL_2(), LINE2());
    let _ = fb.text_ttf((x + 36) as i32, (y + 12) as i32, cap, DIM(), 12.1);

    // The amount is the largest thing here because it is what the reader is
    // actually deciding.
    let mut buf = [0u8; 40];
    let n = crate::wallet::swap::amount_text(state, pay, &mut buf);
    let text = core::str::from_utf8(&buf[..n]).unwrap_or("0");
    let colour = if pay { FG() } else { CYAN() };
    let _ = fb.text_ttf((x + 36) as i32, (y + 36) as i32, text, colour, 30.0);

    // Mark and symbol sit right, where the eye lands after the number.
    let sw = fb.measure_ttf(t.symbol, 17.0).max(0) as u32;
    let right = x + w - 40;
    let _ = fb.text_ttf((right - sw) as i32, (y + 40) as i32, t.symbol, FG(), 17.0);
    mark(fb, right - sw - 34, y + 30, t);
}
