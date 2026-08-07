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

//! What one unit of the paying token buys.

use super::amount::amount;
use super::digits::{copy, pow10};
use crate::wallet::state::State;

/// `1 A = n B`, at the rate this trade actually gets.
///
/// Scaled through the ratio the pool quoted rather than a mid price, because
/// a mid price is not what the reader will receive and showing it would
/// flatter every trade.
pub fn rate_text(state: &State, out: &mut [u8]) -> usize {
    let from = crate::wallet::swap::token(state.swap_from);
    let to = crate::wallet::swap::token(state.swap_to);
    if state.swap_in == 0 || !state.swap_quote.ready {
        return copy(b"-", out);
    }
    let one = pow10(from.decimals);
    let rate = state.swap_quote.out_amount.saturating_mul(one) / state.swap_in.max(1);
    let mut n = copy(b"1 ", out);
    n += copy(from.symbol.as_bytes(), &mut out[n..]);
    n += copy(b" = ", &mut out[n..]);
    n += amount(rate, to.decimals, &mut out[n..]);
    n += copy(b" ", &mut out[n..]);
    n + copy(to.symbol.as_bytes(), &mut out[n..])
}
