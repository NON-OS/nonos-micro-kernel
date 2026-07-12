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

//! The elapsed-tick test of the load balancer, factored out of
//! `LoadBalanceState::should_balance` so it holds no atomic and can be included
//! by the `mechanism_proofs` crate and checked against the Lean `Nonos.Timer`
//! model.

/// Whether at least `interval` ticks have elapsed since `last` at `current`.
/// Saturating, so a tick-counter wraparound reads as no time elapsed rather than
/// a spuriously huge span.
pub(crate) const fn elapsed_reached(current: u64, last: u64, interval: u64) -> bool {
    current.saturating_sub(last) >= interval
}
