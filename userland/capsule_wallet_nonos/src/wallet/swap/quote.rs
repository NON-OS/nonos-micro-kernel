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

//! What a swap would give back, and what it would cost to take it.

/// The pool's own reading of a trade.
///
/// Every field here is shown on screen rather than kept behind a disclosure.
/// A reader who cannot see the price impact cannot tell a fair trade from
/// one that moves the pool against them, and hiding it is what makes a swap
/// feel safe rather than be safe.
#[derive(Clone, Copy, Default)]
pub struct Quote {
    /// What the pool says comes back, in the output token's smallest unit.
    pub out_amount: u128,
    /// The least that may arrive before the swap is abandoned, after
    /// slippage is allowed for.
    pub min_out: u128,
    /// How far this trade moves the price against itself, in hundredths of
    /// a percent.
    pub impact_bps: u32,
    /// Gas the router is expected to want.
    pub gas: u64,
    /// Whether the figures above came from the pool rather than from
    /// nothing. A quote that was never fetched must not be shown as zero,
    /// because zero is also a real answer.
    pub ready: bool,
}

/// Reduce a quoted amount by the slippage the reader is willing to accept.
///
/// The reader sets a tolerance and this is where it becomes a number the
/// chain enforces. Computing it on the way out rather than at signing time
/// means what is displayed is exactly what is submitted.
pub fn apply_slippage(out: u128, bps: u32) -> u128 {
    let bps = bps.min(10_000) as u128;
    out.saturating_mul(10_000u128.saturating_sub(bps)) / 10_000
}

/// How far a trade moves the pool, in hundredths of a percent.
///
/// Taken as the shortfall against what the trade would have returned at the
/// pool's current price with no depth cost at all. A trade small enough to
/// move nothing reports zero rather than a rounding artefact.
pub fn impact_bps(in_amount: u128, out_amount: u128, spot_out: u128) -> u32 {
    if spot_out == 0 || in_amount == 0 || out_amount >= spot_out {
        return 0;
    }
    let lost = spot_out - out_amount;
    let bps = lost.saturating_mul(10_000) / spot_out;
    bps.min(10_000) as u32
}

/// Whether a quote is bad enough that the reader should be stopped rather
/// than warned.
///
/// Ten percent is not a trade, it is an accident or a pool with nothing in
/// it. Below that the screen colours the figure and lets the reader decide.
pub fn is_dangerous(impact_bps: u32) -> bool {
    impact_bps >= 1_000
}

/// Whether a quote is worth colouring as a caution.
pub fn is_warning(impact_bps: u32) -> bool {
    impact_bps >= 100
}
