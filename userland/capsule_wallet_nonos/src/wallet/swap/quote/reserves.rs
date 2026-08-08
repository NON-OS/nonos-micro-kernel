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

/// What a pool holds of the two sides of a trade, and what it charges.
///
/// Ordered by the trade rather than by the pool: `in_amount` is the side
/// being paid in, whichever token that is. The caller orients the pair once,
/// so the arithmetic never has to ask which way round it is.
#[derive(Clone, Copy)]
pub struct Reserves {
    /// Held by the pool of the token being paid, in its smallest unit.
    pub in_amount: u128,
    /// Held by the pool of the token being received, in its smallest unit.
    pub out_amount: u128,
    /// The pool's fee on the input, in hundredths of a percent.
    pub fee_bps: u32,
}
