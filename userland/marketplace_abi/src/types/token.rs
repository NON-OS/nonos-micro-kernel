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

//! Token metadata for the price field. The marketplace operator
//! decides which tokens it accepts; the capsule only carries the
//! identifier so a payment capsule can route the settlement.

extern crate alloc;

use alloc::string::String;

#[derive(Clone)]
pub struct TokenInfo {
    /// Human-facing symbol ("NOX", "ETH"). Capped at
    /// `MAX_TOKEN_SYMBOL` bytes; ASCII only on the wire.
    pub symbol: String,
    /// Number of fractional digits the token uses (18 for NOX/ETH,
    /// 6 for USDC, etc.). Carries through to UI formatting only.
    pub decimals: u8,
    /// Chain identifier as a stable u64. 1 = Ethereum mainnet.
    pub chain_id: u64,
    /// Token contract address as raw bytes (20 bytes for ERC-20,
    /// empty for native).
    pub contract_address: alloc::vec::Vec<u8>,
}
