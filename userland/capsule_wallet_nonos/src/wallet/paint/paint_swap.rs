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

use super::swap_mark::mark;
use super::swap_terms::terms;
use super::ui;
use crate::wallet::state::State;
use crate::wallet::swap::token;
use crate::wallet::theme::{CYAN, DIM, FG, INK, LINE2, MUTED, PANEL_2};

const CARD_W: u32 = 560;

/// The trade: what goes in, what comes back, and what it costs to find out.
///
/// The two sides are the same shape on purpose. A reader checking a trade is
/// comparing two numbers, and anything that makes one side look different
/// from the other makes that comparison slower than it needs to be.
pub fn paint_swap(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    ui::card(fb, cx, 150, CARD_W, 300);
    let _ = fb.text_ttf((cx + 20) as i32, 168, "SWAP", DIM(), 12.1);

    side(fb, state, cx, 196, "You pay", state.swap_from, true);
    arrow(fb, cx + CARD_W / 2, 300);
    side(fb, state, cx, 320, "You receive", state.swap_to, false);

    // The action names the next step rather than the whole journey. A token
    // the router has never been allowed to move has to be approved first,
    // and a reader told only "Swap" would sign twice without being told why.
    let from = token(state.swap_from);
    let label: &[u8] = if !state.swap_quote.ready {
        b"Enter an amount"
    } else if !from.is_native() && state.swap_step == 0 {
        b"Approve"
    } else {
        b"Swap"
    };
    ui::primary(fb, cx + 20, 396, CARD_W - 40, label);

    terms(state, fb, cx + CARD_W + 16, 150);
}

/// One half of the trade.
fn side(fb: &mut PaintBuffer, state: &State, x: u32, y: u32, cap: &str, idx: u8, pay: bool) {
    let t = token(idx);
    ui::bordered(fb, x + 20, y, CARD_W - 40, 88, PANEL_2(), LINE2());
    let _ = fb.text_ttf((x + 36) as i32, (y + 12) as i32, cap, DIM(), 12.1);

    // The amount is the largest thing on the panel because it is what the
    // reader is actually deciding.
    let mut buf = [0u8; 40];
    let n = crate::wallet::swap::amount_text(state, pay, &mut buf);
    let text = core::str::from_utf8(&buf[..n]).unwrap_or("0");
    let colour = if pay { FG() } else { CYAN() };
    let _ = fb.text_ttf((x + 36) as i32, (y + 36) as i32, text, colour, 30.0);

    // The mark and symbol sit together on the right, where a reader's eye
    // lands after reading the number.
    let sw = fb.measure_ttf(t.symbol, 17.0).max(0) as u32;
    let right = x + CARD_W - 40;
    let _ = fb.text_ttf((right - sw) as i32, (y + 40) as i32, t.symbol, FG(), 17.0);
    mark(fb, right - sw - 34, y + 30, t);
}

/// The direction of the trade, drawn between the two sides.
fn arrow(fb: &mut PaintBuffer, cx: u32, y: u32) {
    ui::bordered(fb, cx - 14, y, 28, 28, PANEL_2(), LINE2());
    let _ = fb.text_ttf((cx - 5) as i32, (y + 6) as i32, "\u{2193}", MUTED(), 15.0);
    let _ = INK();
}
