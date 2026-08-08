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

//! Per-hop key material.
//!
//! One Diffie-Hellman result per hop is stretched to 288 bytes and cut into a
//! stream-cipher key, a header integrity key, the payload key, the blinding
//! factor for the next hop, and a replay tag.

mod blinding_factor;
mod derive_payload_key;
mod expand;
mod integrity_mac_key;
mod legacy_payload_key;
mod offsets;
mod payload_key_seed;
mod replay_tag;
mod stream_cipher_key;
mod types;

pub use derive_payload_key::derive_payload_key;
pub use expand::expand_shared_secret;
pub use types::ExpandedSharedSecret;
