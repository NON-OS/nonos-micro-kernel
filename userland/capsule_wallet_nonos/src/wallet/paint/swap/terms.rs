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

//! What the trade will actually do, spelled out.
//!
//! Every other wallet folds this behind a control. Folding it away is what
//! lets a bad trade through: the reader sees a figure they like and signs,
//! and the number that would have stopped them was one tap out of sight. It
//! costs a panel to leave open, and the panel is the product.

use nonos_app_skeleton::PaintBuffer;

use super::super::ui;
use super::row::row;
use super::tone::impact_tone;
use crate::wallet::state::State;
use crate::wallet::swap::{self, token};
use crate::wallet::theme::{DIM, FG, MUTED};

const W: u32 = 340;

pub fn terms(state: &State, fb: &mut PaintBuffer, x: u32, y: u32) {
    ui::card(fb, x, y, W, 300);
    let _ = fb.text_ttf((x + 20) as i32, (y + 18) as i32, "TERMS", DIM(), 12.1);
    if !state.swap_quote.ready {
        let msg = "Enter an amount to price this trade.";
        let _ = fb.text_ttf((x + 20) as i32, (y + 56) as i32, msg, MUTED(), 13.8);
        return;
    }

    let q = &state.swap_quote;
    let mut b = [0u8; 48];

    let n = swap::rate_text(state, &mut b);
    row(fb, x, y + 52, W, "Rate", &b[..n], FG());

    // Coloured rather than merely printed: a number sitting in a list of
    // numbers is not a warning.
    let n = swap::bps_text(q.impact_bps, &mut b);
    row(fb, x, y + 92, W, "Price impact", &b[..n], impact_tone(q.impact_bps));

    // What the chain enforces while the transaction sits in the mempool,
    // shown as a figure rather than a tolerance to apply by hand.
    let n = swap::min_out_text(state, &mut b);
    row(fb, x, y + 132, W, "Minimum received", &b[..n], FG());

    let n = swap::slippage_text(state, &mut b);
    row(fb, x, y + 172, W, "Slippage", &b[..n], MUTED());

    let n = swap::gas_text(q.gas, &mut b);
    row(fb, x, y + 212, W, "Network fee", &b[..n], MUTED());

    // A swap that hops through a third asset pays two fees and carries two
    // impacts. The reader is entitled to know before signing.
    let n = swap::route_text(token(state.swap_from), token(state.swap_to), &mut b);
    row(fb, x, y + 252, W, "Route", &b[..n], MUTED());
}
