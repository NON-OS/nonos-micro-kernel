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

use crate::wallet::pool::{quote as quote_source, QuoteProvider};
use crate::wallet::state::State;
use crate::wallet::swap::{quote, token, Quote};

/// Re-read the pool for the amount and pair now on screen.
///
/// Called wherever the trade changes, since a quote belongs to the amount it
/// was fetched for. The reading is dropped first and only replaced once a
/// whole one is in hand, so a stale figure can never sit under a new amount.
/// Where the source has nothing to say the quote stays unready and the screen
/// shows that it has no price, rather than the last one it had.
pub fn refresh(state: &mut State) {
    state.swap_quote = Quote::default();
    if state.swap_in == 0 {
        return;
    }
    let source = quote_source();
    let pay = *token(state.swap_from);
    let receive = *token(state.swap_to);
    let Some(reserves) = source.reserves(pay, receive).as_ready().copied() else {
        return;
    };
    let Some(gas) = source.gas(pay, receive).as_ready().copied() else {
        return;
    };
    if let Some(read) = quote(state.swap_in, &reserves, gas) {
        state.swap_quote = read;
    }
}
