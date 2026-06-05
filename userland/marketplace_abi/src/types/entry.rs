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

//! A single marketplace listing. Carries presentation metadata, the
//! publisher pubkey, the price contract, and one or more releases.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use super::price::PriceModel;
use super::release::CapsuleRelease;
use super::token::TokenInfo;

#[derive(Clone)]
pub struct MarketplaceEntry {
    /// Listing identifier the marketplace assigns. Independent of
    /// `capsule_id` so the same capsule can be listed twice (e.g.
    /// commercial vs. open licence) under different terms.
    pub listing_id: String,
    /// Capsule identity hash; matches the kernel's eventual capsule
    /// table entry once installed.
    pub capsule_id: [u8; 32],
    /// Display name; UTF-8, capped at `MAX_NAME` bytes.
    pub name: String,
    /// Publisher organisation as displayed; UTF-8, capped at
    /// `MAX_PUBLISHER` bytes.
    pub publisher_name: String,
    /// Publisher's Ed25519 verifier key. Must match the signing
    /// key used on every release the listing exposes.
    pub publisher_pubkey: [u8; 32],
    /// Publisher's Ethereum address (20 bytes). On-chain settlement
    /// authority for paid installs; covered by the operator index
    /// signature and cross-checked against CapsuleRegistry on chain.
    pub publisher_eth_address: [u8; 20],
    /// Free-form description; UTF-8, capped at `MAX_DESCRIPTION`.
    pub description: String,
    /// Price contract for the listing.
    pub price: PriceModel,
    /// Token the price is denominated in.
    pub token: TokenInfo,
    /// One or more releases. The first entry is the default.
    pub releases: Vec<CapsuleRelease>,
}
