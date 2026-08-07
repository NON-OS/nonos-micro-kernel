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

use super::amount_out::amount_out;
use super::impact::impact_bps;
use super::reading::Quote;
use super::reserves::Reserves;

/// Read a trade against a pool.
///
/// Returns nothing where any part of the reading is missing, so a quote is
/// either whole or absent. A half-filled quote would show a real output beside
/// an impact of zero, which reads as a free trade.
pub fn quote(amount_in: u128, r: &Reserves, gas: u64) -> Option<Quote> {
    let out = amount_out(amount_in, r.in_amount, r.out_amount, r.fee_bps)?;
    let impact = impact_bps(amount_in, r.in_amount, r.out_amount, out)?;
    Some(Quote { out_amount: out, impact_bps: impact, gas, ready: true })
}
