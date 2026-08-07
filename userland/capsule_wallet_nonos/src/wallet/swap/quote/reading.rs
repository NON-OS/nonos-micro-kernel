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

//! The pool's own reading of a trade.

/// What a swap would give back, and what it costs to take it.
///
/// Every field is shown on screen rather than kept behind a control. A
/// reader who cannot see the price impact cannot tell a fair trade from one
/// that moves the pool against them.
#[derive(Clone, Copy, Default)]
pub struct Quote {
    /// What the pool says comes back, in the output token's smallest unit.
    pub out_amount: u128,
    /// How far this trade moves the price against itself, in hundredths of
    /// a percent.
    pub impact_bps: u32,
    /// Gas the router is expected to want.
    pub gas: u64,
    /// Whether these figures came from the pool rather than from nothing. A
    /// quote never fetched must not read as zero, since zero is also a real
    /// answer.
    pub ready: bool,
}
