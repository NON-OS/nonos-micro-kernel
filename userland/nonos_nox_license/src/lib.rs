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

//! Pay-per-use entitlements for NONOS tool capsules, settled in NOX.
//!
//! A tool refuses to run until it holds an entitlement the license broker
//! signed. The broker signs only after it has seen the buyer's NOX payment
//! confirmed on chain and recorded the funding transaction as spent, so an
//! entitlement cannot be minted by the tool, forged without the broker key,
//! or replayed from one payment onto two grants.
//!
//! This crate owns the wire format and the field rules and nothing else. It
//! never signs, never talks to the chain, and never calls a syscall: it hands
//! signature verification to a caller-supplied `Verify` so the same bytes are
//! checked by the kernel ed25519 primitive in a capsule and by a real host
//! signer under `cargo test`. Everything here is deterministic and pure, which
//! is why the format and the accept/reject boundary are proven on the host.

#![cfg_attr(not(test), no_std)]

mod entitlement;
mod price;
mod verify;

pub use entitlement::{Entitlement, ParseError, BODY_LEN, ENTITLEMENT_LEN, MAGIC, SIG_LEN};
pub use price::{price_of, tool_name, ToolId, TREASURY};
pub use verify::{check, CheckError, Checked, Verify};

#[cfg(test)]
mod tests;
