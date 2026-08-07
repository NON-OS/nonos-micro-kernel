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

use crate::wallet::swap::{Reserves, Token};

use super::seam::Seam;

/// Where a swap quote's figures come from.
///
/// Only the reading is behind the seam. The arithmetic that turns reserves
/// into an output, an impact and a minimum received is in `swap::quote` and
/// runs the same whatever provides the numbers, so wiring a live source
/// cannot quietly change what the screen computes.
///
/// A live implementation reads the pair's reserves and fee from the deployed
/// pool. Until one is wired this returns `NotWired` and the swap screen says
/// so rather than showing a figure it cannot stand behind.
pub trait QuoteProvider {
    /// The pool's holdings of `pay` and `receive`, oriented to the trade,
    /// with the fee it charges on the input.
    fn reserves(&self, pay: Token, receive: Token) -> Seam<Reserves>;

    /// Gas the router is expected to want for this trade.
    fn gas(&self, pay: Token, receive: Token) -> Seam<u64>;
}
