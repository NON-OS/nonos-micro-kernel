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

//! What the chain will enforce, and when to stop the reader.

/// One whole percent, where a trade stops being unremarkable.
const WARN_BPS: u32 = 100;
/// Ten percent. Not a trade: an accident, or a pool with nothing in it.
const DANGER_BPS: u32 = 1_000;

/// Reduce a quoted amount by the slippage the reader accepts.
///
/// Computed on the way out rather than at signing, so what is displayed is
/// exactly what is submitted.
pub fn apply_slippage(out: u128, bps: u32) -> u128 {
    let bps = bps.min(10_000) as u128;
    out.saturating_mul(10_000u128.saturating_sub(bps)) / 10_000
}

/// Whether a quote is bad enough to stop rather than warn.
pub fn is_dangerous(impact_bps: u32) -> bool {
    impact_bps >= DANGER_BPS
}

/// Whether a quote is worth colouring as a caution.
pub fn is_warning(impact_bps: u32) -> bool {
    impact_bps >= WARN_BPS
}
