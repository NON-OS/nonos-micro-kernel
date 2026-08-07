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

//! Turning the numbers a trade is made of into something readable.
//!
//! Every figure here is written into a caller's buffer rather than allocated,
//! because these run on the paint path and a swap screen redraws whenever a
//! digit is typed.

use super::token::Token;
use super::Quote;
use crate::wallet::state::State;

/// Write `v` as a decimal figure with `decimals` places, trimmed.
///
/// Trailing zeros are dropped because a price with six of them reads as
/// noise, and the digits after the point are capped so a long fraction
/// cannot push the symbol off the panel.
pub fn amount(v: u128, decimals: u8, out: &mut [u8]) -> usize {
    let scale = pow10(decimals);
    let whole = v / scale;
    let frac = v % scale;
    let mut n = u128_str(whole, out);
    if frac == 0 || n + 2 >= out.len() {
        return n;
    }
    out[n] = b'.';
    n += 1;
    // At most four places. Beyond that the figure is longer than it is
    // useful, and the exact value is what the chain enforces anyway.
    let keep = (decimals as usize).min(4);
    let mut rem = frac;
    let mut shown = 0;
    let mut trailing = 0;
    while shown < keep && n < out.len() {
        let step = scale / pow10(shown as u8 + 1);
        let digit = (rem / step) as u8;
        rem %= step;
        out[n] = b'0' + digit;
        trailing = if digit == 0 { trailing + 1 } else { 0 };
        n += 1;
        shown += 1;
    }
    n -= trailing;
    if n > 0 && out[n - 1] == b'.' {
        n -= 1;
    }
    n
}

/// The amount on one side of the trade.
pub fn amount_text(state: &State, pay: bool, out: &mut [u8]) -> usize {
    let (v, idx) = if pay {
        (state.swap_in, state.swap_from)
    } else {
        (state.swap_quote.out_amount, state.swap_to)
    };
    if v == 0 {
        out[0] = b'0';
        return 1;
    }
    amount(v, super::token(idx).decimals, out)
}

/// What one unit of the paying token buys, as `1 A = n B`.
pub fn rate_text(state: &State, out: &mut [u8]) -> usize {
    let from = super::token(state.swap_from);
    let to = super::token(state.swap_to);
    if state.swap_in == 0 || !state.swap_quote.ready {
        return copy(b"-", out);
    }
    // Scale one whole unit of the input through the same ratio the pool
    // quoted, so the rate shown is the rate this trade actually gets rather
    // than a mid price it will not receive.
    let one = pow10(from.decimals);
    let rate = state.swap_quote.out_amount.saturating_mul(one) / state.swap_in.max(1);
    let mut n = copy(b"1 ", out);
    n += copy(from.symbol.as_bytes(), &mut out[n..]);
    n += copy(b" = ", &mut out[n..]);
    n += amount(rate, to.decimals, &mut out[n..]);
    n += copy(b" ", &mut out[n..]);
    n + copy(to.symbol.as_bytes(), &mut out[n..])
}

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
    let to = super::token(state.swap_to);
    let mut n = amount(state.swap_quote.min_out, to.decimals, out);
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
pub fn route_text(from: &Token, to: &Token, out: &mut [u8]) -> usize {
    let mut n = copy(from.symbol.as_bytes(), out);
    n += copy(" -> ".as_bytes(), &mut out[n..]);
    n + copy(to.symbol.as_bytes(), &mut out[n..])
}

/// Fill in what a quote implies once the pool has answered.
pub fn settle(q: &mut Quote, out_amount: u128, spot_out: u128, in_amount: u128, bps: u32) {
    q.out_amount = out_amount;
    q.min_out = super::apply_slippage(out_amount, bps);
    q.impact_bps = super::impact_bps(in_amount, out_amount, spot_out);
    q.ready = true;
}

fn pow10(n: u8) -> u128 {
    let mut v = 1u128;
    let mut i = 0;
    while i < n {
        v = v.saturating_mul(10);
        i += 1;
    }
    v
}

fn u128_str(mut v: u128, out: &mut [u8]) -> usize {
    if v == 0 {
        out[0] = b'0';
        return 1;
    }
    let mut d = [0u8; 40];
    let mut n = 0;
    while v > 0 && n < d.len() {
        d[n] = b'0' + (v % 10) as u8;
        v /= 10;
        n += 1;
    }
    let n = n.min(out.len());
    for i in 0..n {
        out[i] = d[n - 1 - i];
    }
    n
}

fn copy(src: &[u8], out: &mut [u8]) -> usize {
    let n = src.len().min(out.len());
    out[..n].copy_from_slice(&src[..n]);
    n
}
