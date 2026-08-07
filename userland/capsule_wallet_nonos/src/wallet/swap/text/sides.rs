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

//! The two amounts either side of the trade.

use super::amount::amount;
use crate::wallet::state::State;

/// The amount on one side, or a bare zero before anything is entered.
pub fn amount_text(state: &State, pay: bool, out: &mut [u8]) -> usize {
    let (v, idx) = if pay {
        // The typed figure scaled to the token's decimals, so what is shown
        // is the amount a contract would be given rather than the digits.
        (crate::wallet::swap::scaled(state), state.swap_from)
    } else {
        (state.swap_quote.out_amount, state.swap_to)
    };
    if v == 0 {
        out[0] = b'0';
        return 1;
    }
    amount(v, crate::wallet::swap::token(idx).decimals, out)
}
