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
//! The license broker will not sign an entitlement on the buyer's say-so. It
//! asks the node for the funding transaction's receipt and hands the bytes to
//! `verify_payment` here, which accepts only when the receipt shows a
//! successful transaction carrying an ERC20 `Transfer` from the buyer to the
//! treasury, emitted by the NOX token contract, for at least the tool's price.
//! Anything less, a failed transaction, a transfer of the wrong token, to the
//! wrong address, or for too little, is refused.
//!
//! The scan is deliberately narrow: it reads only the fields the decision
//! needs out of the receipt JSON and matches them against fixed expectations.
//! It never allocates and never trusts a field it did not find, so a truncated
//! or hostile receipt fails closed rather than reading as a payment.

#![cfg_attr(not(test), no_std)]

mod hex;
mod logs;
mod verify;

pub use verify::{verify_payment, Payment, ReceiptError, NOX_TOKEN, TRANSFER_TOPIC};

#[cfg(test)]
mod tests;
