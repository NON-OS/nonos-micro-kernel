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

mod decrypt;
mod decrypt_aad;
mod ed25519_pubkey;
mod ed25519_sign;
mod ed25519_verify;
mod encrypt;
mod encrypt_aad;
mod hash;
mod keccak256;
mod prf;
mod random;
mod secp256k1_pubkey;
mod secp256k1_sign;
mod x25519;

pub use decrypt::crypto_decrypt;
pub use decrypt_aad::crypto_decrypt_aad;
pub use ed25519_pubkey::crypto_ed25519_pubkey;
pub use ed25519_sign::crypto_ed25519_sign;
pub use ed25519_verify::crypto_ed25519_verify;
pub use encrypt::crypto_encrypt;
pub use encrypt_aad::crypto_encrypt_aad;
pub use hash::crypto_hash;
pub use keccak256::crypto_keccak256;
pub use prf::{crypto_hkdf_sha256, crypto_hmac_sha256};
pub use random::crypto_random;
pub use secp256k1_pubkey::crypto_secp256k1_pubkey;
pub use secp256k1_sign::crypto_secp256k1_sign;
pub use x25519::{crypto_x25519_public, crypto_x25519_shared};
