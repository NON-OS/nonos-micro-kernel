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

mod aead;
mod error;
mod hash;
mod keccak;
mod primitives;
mod random;
mod secp256k1_pubkey;
mod ed25519_pubkey;
mod ed25519_sign;
mod secp256k1_sign;
mod verify;

pub use aead::{
    handle_crypto_decrypt, handle_crypto_decrypt_aad, handle_crypto_encrypt,
    handle_crypto_encrypt_aad,
};
pub use hash::handle_crypto_hash;
pub use keccak::handle_crypto_keccak256;
pub use primitives::{
    handle_hkdf_sha256, handle_hmac_sha256, handle_x25519_public, handle_x25519_shared,
};
pub use random::handle_crypto_random;
pub use secp256k1_pubkey::handle_crypto_secp256k1_pubkey;
pub use ed25519_pubkey::handle_crypto_ed25519_pubkey;
pub use ed25519_sign::handle_crypto_ed25519_sign;
pub use secp256k1_sign::handle_crypto_secp256k1_sign;
pub use verify::handle_crypto_ed25519_verify;
