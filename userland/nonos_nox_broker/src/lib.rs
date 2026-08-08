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

//! The license broker's decision, without the capsule around it.
//!
//! `issue` prices the tool, verifies the receipt pays the treasury at least
//! that price in NOX, refuses a transaction already redeemed, and returns the
//! grant the buyer earned, one use per price paid. It never signs: the capsule
//! holds the ed25519 key. Replay is blocked by a spent-set the caller persists;
//! `issue` records the funding hash only on success, so a failed attempt never
//! burns a payment and a repeat never mints a second grant.

#![cfg_attr(not(test), no_std)]

mod issue;
mod spent;

pub use issue::{issue, IssueError, Issued};
pub use spent::{SpentSet, SPENT_CAPACITY};

#[cfg(test)]
mod tests;

#[cfg(kani)]
mod kani_proofs;
