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
//! A tool will not run without an entitlement the broker signed, and the broker
//! signs only against a confirmed, unspent NOX payment. So a grant cannot be
//! minted by the tool, forged without the broker key, or replayed onto a second
//! use. Signature verification is injected through `Verify` (kernel ed25519 in a
//! capsule, a host signer in tests), keeping this crate pure and host-provable.

#![cfg_attr(not(test), no_std)]

mod entitlement;
mod price;
mod verify;

pub use entitlement::{Entitlement, ParseError, BODY_LEN, ENTITLEMENT_LEN, MAGIC, SIG_LEN};
pub use price::{price_of, tool_name, ToolId, TREASURY};
pub use verify::{check, CheckError, Checked, Verify};

#[cfg(test)]
mod tests;

#[cfg(kani)]
mod kani_proofs;
