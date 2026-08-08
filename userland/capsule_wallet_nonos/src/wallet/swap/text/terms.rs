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

//! The figures the terms panel reads.

use super::amount::amount;
use super::digits::{copy, u128_str};
use crate::wallet::state::State;
use crate::wallet::swap::apply_slippage;

/// A hundredths-of-a-percent figure as a percentage.
pub fn bps_text(bps: u32, out: &mut [u8]) -> usize {
    let mut n = u128_str((bps / 100) as u128, out);
    out[n] = b'.';
    n += 1;
    out[n] = b'0' + ((bps / 10) % 10) as u8;
    n += 1;
    out[n] = b'0' + (bps % 10) as u8;
    n += 1;
    n + copy(b" %", &mut out[n..])
}

/// The least the chain will let this trade return.
pub fn min_out_text(state: &State, out: &mut [u8]) -> usize {
    let to = crate::wallet::swap::token(state.swap_to);
    // Derived from the tolerance rather than stored, so the figure shown
    // and the figure enforced cannot drift apart.
    let least = apply_slippage(state.swap_quote.out_amount, state.swap_slippage_bps);
    let mut n = amount(least, to.decimals, out);
    n += copy(b" ", &mut out[n..]);
    n + copy(to.symbol.as_bytes(), &mut out[n..])
}

/// The tolerance the reader has set.
pub fn slippage_text(state: &State, out: &mut [u8]) -> usize {
    bps_text(state.swap_slippage_bps, out)
}

/// The gas the router is expected to want.
pub fn gas_text(gas: u64, out: &mut [u8]) -> usize {
    if gas == 0 {
        return copy(b"-", out);
    }
    let n = u128_str(gas as u128, out);
    n + copy(b" gas", &mut out[n..])
}

/// The path the trade takes, so a hop through a third asset is visible.
pub fn route_text(
    from: &crate::wallet::swap::Token,
    to: &crate::wallet::swap::Token,
    out: &mut [u8],
) -> usize {
    let mut n = copy(from.symbol.as_bytes(), out);
    n += copy(b" -> ", &mut out[n..]);
    n + copy(to.symbol.as_bytes(), &mut out[n..])
}
