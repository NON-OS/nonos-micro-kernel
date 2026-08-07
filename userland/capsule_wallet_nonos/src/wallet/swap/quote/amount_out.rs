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

use crate::wallet::num::mul_div;

/// Basis points in the whole.
pub const BPS: u128 = 10_000;

/// What a constant-product pool returns for `amount_in`, after its fee.
///
/// The pool holds `reserve_in` of what is paid and `reserve_out` of what is
/// received, and keeps their product at least what it was. Writing it as
/// `in_after_fee * reserve_out / (reserve_in + in_after_fee)` is the same
/// figure the pool computes itself, so what the screen shows is what the
/// chain will do rather than an estimate of it.
///
/// `None` where there is no answer to give: an empty pool, a fee that is not
/// a fee, or figures too large to divide back into 128 bits. Each is a
/// refusal, since a zero would read as a real quote of nothing.
pub fn amount_out(
    amount_in: u128,
    reserve_in: u128,
    reserve_out: u128,
    fee_bps: u32,
) -> Option<u128> {
    if amount_in == 0 || reserve_in == 0 || reserve_out == 0 {
        return None;
    }
    let fee = fee_bps as u128;
    if fee >= BPS {
        return None;
    }
    // The fee is taken off the input, which is what keeps the invariant on
    // the pool's side of the trade rather than the trader's.
    let in_after_fee = amount_in.checked_mul(BPS - fee)?;
    let denominator = reserve_in.checked_mul(BPS)?.checked_add(in_after_fee)?;
    let out = mul_div(in_after_fee, reserve_out, denominator)?;
    // A pool can never pay out everything it holds; the formula already
    // guarantees this, and the check states it where a reader can see it.
    if out >= reserve_out {
        return None;
    }
    Some(out)
}
