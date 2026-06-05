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

//! Top-level marketplace index. The capsule receives a single
//! `MarketplaceIndex` blob over IPC; the structure is signed by the
//! marketplace operator's Ed25519 key and re-verified before any
//! entry is exposed.

extern crate alloc;

use alloc::vec::Vec;

use super::entry::MarketplaceEntry;

#[derive(Clone)]
pub struct MarketplaceIndex {
    /// Schema version; bump every time the wire format changes.
    pub schema_version: u32,
    /// Marketplace operator identifier ("nonos.marketplace.v1").
    /// Capped at `MAX_PUBLISHER` bytes.
    pub operator_id: alloc::string::String,
    /// Operator's Ed25519 verifier key. The capsule trusts an
    /// index only after `index_signature` checks against this key.
    pub operator_pubkey: [u8; 32],
    /// Unix-millis timestamp of the index snapshot.
    pub published_at_ms: u64,
    /// Strictly-increasing serial. The capsule refuses any index
    /// whose `serial` is less than the last one it accepted, so a
    /// rollback by a compromised mirror cannot revive a revoked
    /// listing.
    pub serial: u64,
    /// One entry per listing; capped at `MAX_ENTRIES`.
    pub entries: Vec<MarketplaceEntry>,
    /// Operator's signature over the canonical bytes of the index
    /// up to (and excluding) this field. 64 bytes for Ed25519.
    pub index_signature: Vec<u8>,
}
