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

//! Prove a NOX payment from the transaction receipt the chain returns.
//!
//! `verify_payment` accepts a receipt only when it shows a successful
//! transaction carrying an ERC20 `Transfer` from the buyer to the treasury,
//! emitted by the NOX token, for at least the price. The scan reads only the
//! fields the decision needs and never allocates, so a truncated or hostile
//! receipt fails closed rather than reading as a payment.

#![cfg_attr(not(test), no_std)]

mod hex;
mod logs;
mod verify;

pub use verify::{verify_payment, Payment, ReceiptError, NOX_TOKEN, TRANSFER_TOPIC};

#[cfg(test)]
mod tests;

#[cfg(kani)]
mod kani_proofs;
