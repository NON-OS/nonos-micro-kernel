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

pub mod aes;
pub mod blake2b;
pub mod chacha20;
pub mod gcm_siv;
pub mod hkdf_blake3;
pub mod lioness;
pub mod polyval;

mod aead;
pub mod ecdh;
pub mod hash;
pub mod kdf;
pub mod random;
pub mod types;

pub use aead::{open, seal};
pub use ecdh::{x25519_public, x25519_shared};
pub use hash::blake3;
pub use kdf::{hkdf_sha256, hmac_sha256};
pub use random::fill_random;
pub use types::{Key, Nonce, TAG_BYTES};
