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

//! The nonce composition, factored out of `next_nonce` so it holds no atomic
//! and can be included by the `mechanism_proofs` crate and checked against the
//! Lean `Nonos.Nonce` model. A nonce carries the monotonic counter in its low
//! 32 bits, so two calls with distinct counters within a timestamp never
//! collide: the counter is recoverable from the nonce.

/// Combine a millisecond timestamp with a monotonic counter. The counter
/// occupies the low 32 bits and the timestamp the high bits, so the counter is
/// recovered as `nonce & 0xFFFF_FFFF`.
pub(crate) const fn compose(timestamp: u64, counter: u64) -> u64 {
    (timestamp << 32) ^ (counter & 0xFFFF_FFFF)
}
