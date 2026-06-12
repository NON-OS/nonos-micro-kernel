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

pub use super::super::application::certification;
pub use super::super::application::ethereum;
pub use super::super::application::nonos_signing;
pub use super::super::application::vault;

pub use super::super::core::aead::{
    aead_unwrap, aead_wrap, Aead, Aes256GcmAead, Chacha20Poly1305Aead,
};
pub use super::super::core::api::{
    ed25519_verify, estimate_entropy, feature_summary, fill_random, generate_keypair,
    generate_plonk_proof, generate_secure_key, hash_memory_region, hkdf_expand_labeled, init,
    init_crypto_subsystem, secure_erase_memory_region, secure_random_u32, secure_random_u64,
    secure_random_u8, secure_zero, sig, verify_plonk_proof, verify_signature, SignatureAlgorithm,
};
pub use super::super::core::syscall::{sign_message, verify_signature_syscall, SyscallCryptoError};
#[cfg(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024"))]
pub use super::super::core::traits::KyberKem;
#[cfg(any(feature = "mldsa2", feature = "mldsa3", feature = "mldsa5"))]
pub use super::super::core::traits::MlDsa65Sig;
pub use super::super::core::traits::{Ed25519Sig, Kem, Sig};
