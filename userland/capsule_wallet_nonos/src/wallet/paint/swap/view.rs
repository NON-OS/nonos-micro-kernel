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

//! What goes in, what comes back, and the terms alongside.

use nonos_app_skeleton::PaintBuffer;

use super::super::ui;
use super::side::side;
use super::terms::terms;
use crate::wallet::state::State;
use crate::wallet::swap::token;
use crate::wallet::theme::{DIM, LINE2, MUTED, PANEL_2};

const CARD_W: u32 = 560;

pub fn paint_swap(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    ui::card(fb, cx, 150, CARD_W, 300);
    let _ = fb.text_ttf((cx + 20) as i32, 168, "SWAP", DIM(), 12.1);

    side(fb, state, cx, 196, CARD_W, "You pay", true);
    arrow(fb, cx + CARD_W / 2, 300);
    side(fb, state, cx, 320, CARD_W, "You receive", false);

    ui::primary(fb, cx + 20, 396, CARD_W - 40, action(state));
    terms(state, fb, cx + CARD_W + 16, 150);
}

/// The next step, not the whole journey.
///
/// A token the router has never been allowed to move needs an approval
/// first, and a reader told only "Swap" would sign twice without being told
/// why the first signature was not the trade.
fn action(state: &State) -> &'static [u8] {
    if !state.swap_quote.ready {
        b"Enter an amount"
    } else if !token(state.swap_from).is_native() && state.swap_step == 0 {
        b"Approve"
    } else {
        b"Swap"
    }
}

/// The direction of the trade, between the two sides.
fn arrow(fb: &mut PaintBuffer, cx: u32, y: u32) {
    ui::bordered(fb, cx - 14, y, 28, 28, PANEL_2(), LINE2());
    let _ = fb.text_ttf((cx - 4) as i32, (y + 6) as i32, "v", MUTED(), 14.0);
}
