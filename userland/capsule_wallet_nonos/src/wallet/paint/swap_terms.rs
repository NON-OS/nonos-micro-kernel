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
use crate::wallet::swap::{is_dangerous, is_warning, token};
use crate::wallet::theme::{CYAN, DIM, FG, GREEN, LINE2, MUTED, WARN};

/// What the trade will actually do, spelled out.
///
/// Every other wallet puts this behind a chevron. Folding it away is what
/// lets a bad trade through: the reader sees a number they like and signs,
/// and the figure that would have stopped them was one tap out of sight.
/// It costs a panel to leave it open, and the panel is the product.
pub fn terms(state: &State, fb: &mut PaintBuffer, x: u32, y: u32) {
    let w = 340u32;
    ui::card(fb, x, y, w, 300);
    let _ = fb.text_ttf((x + 20) as i32, (y + 18) as i32, "TERMS", DIM(), 12.1);

    if !state.swap_quote.ready {
        let _ = fb.text_ttf(
            (x + 20) as i32,
            (y + 56) as i32,
            "Enter an amount to price this trade.",
            MUTED(),
            13.8,
        );
        return;
    }

    let q = &state.swap_quote;
    let mut b = [0u8; 48];

    let n = crate::wallet::swap::rate_text(state, &mut b);
    row(fb, x, y + 52, w, "Rate", &b[..n], FG());

    // The figure that decides whether a trade is worth taking. Coloured
    // rather than merely printed, because a number in a list of numbers is
    // not a warning.
    let n = crate::wallet::swap::bps_text(q.impact_bps, &mut b);
    let tone = if is_dangerous(q.impact_bps) {
        WARN()
    } else if is_warning(q.impact_bps) {
        CYAN()
    } else {
        GREEN()
    };
    row(fb, x, y + 92, w, "Price impact", &b[..n], tone);

    // What the chain will enforce. This is the number that protects the
    // reader while the transaction sits in the mempool, so it is shown as a
    // figure and not as a tolerance they have to apply themselves.
    let n = crate::wallet::swap::min_out_text(state, &mut b);
    row(fb, x, y + 132, w, "Minimum received", &b[..n], FG());

    let n = crate::wallet::swap::slippage_text(state, &mut b);
    row(fb, x, y + 172, w, "Slippage", &b[..n], MUTED());

    let n = crate::wallet::swap::gas_text(q.gas, &mut b);
    row(fb, x, y + 212, w, "Network fee", &b[..n], MUTED());

    // The route is shown because a swap that hops through a third asset
    // pays two fees and carries two impacts, and the reader is entitled to
    // know that before signing rather than after.
    let from = token(state.swap_from);
    let to = token(state.swap_to);
    let mut r = [0u8; 32];
    let n = crate::wallet::swap::route_text(from, to, &mut r);
    row(fb, x, y + 252, w, "Route", &r[..n], MUTED());
    let _ = LINE2();
}

/// One labelled figure, label left and value right.
fn row(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, value: &[u8], tone: u32) {
    let _ = fb.text_ttf((x + 20) as i32, y as i32, label, MUTED(), 13.4);
    let v = core::str::from_utf8(value).unwrap_or("");
    let vw = fb.measure_ttf(v, 14.2).max(0) as u32;
    let _ = fb.text_ttf((x + w - 20 - vw) as i32, (y - 1) as i32, v, tone, 14.2);
}
