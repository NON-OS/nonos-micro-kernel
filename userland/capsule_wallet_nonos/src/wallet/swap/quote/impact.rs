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

use super::amount_out::BPS;

/// How far a trade moves the price against itself, in hundredths of a
/// percent.
///
/// Measured against the pool's own mid price, the rate an infinitely small
/// trade would get. The gap between that and what this trade actually
/// returns is what the trader gives up for taking the whole size at once.
///
/// The fee is deliberately not counted here. A fee is a known, quoted cost
/// shown on its own line; folding it into impact would make every trade look
/// as though it were moving the pool when it is not, and a reader who cannot
/// separate the two cannot tell a thin pool from an expensive one.
///
/// `None` where the mid price does not exist or the figures will not divide.
pub fn impact_bps(amount_in: u128, reserve_in: u128, reserve_out: u128, out: u128) -> Option<u32> {
    if amount_in == 0 || reserve_in == 0 || reserve_out == 0 {
        return None;
    }
    // What the mid price alone would have returned, ignoring both the curve
    // and the fee.
    let ideal = mul_div(amount_in, reserve_out, reserve_in)?;
    if ideal == 0 {
        return None;
    }
    // A quote at or above the mid price has no impact to report rather than
    // a negative one.
    let lost = ideal.saturating_sub(out);
    let bps = mul_div(lost, BPS, ideal)?;
    Some(bps.min(BPS) as u32)
}
