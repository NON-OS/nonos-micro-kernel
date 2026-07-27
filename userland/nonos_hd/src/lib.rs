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

//! BIP39 mnemonics and BIP32/BIP44 hierarchical key derivation for the NONOS
//! keyring capsule. Everything is `no_std`, allocation-free and from scratch:
//! SHA-256, SHA-512, HMAC-SHA512 and PBKDF2-HMAC-SHA512 (the real 2048 rounds)
//! live in this crate, proven on the host against the official BIP39 and BIP32
//! test vectors. The one primitive not implemented here is secp256k1 scalar
//! multiplication: non-hardened derivation takes the parent public key from a
//! caller-supplied provider, which inside the capsule is the kernel's proven
//! `crypto_secp256k1_pubkey` syscall and in host tests is the audited k256
//! crate. Secret intermediates are wiped with volatile writes on every path.

#![no_std]

mod hmac512;
mod pbkdf2;
mod sha256;
mod sha512;
mod wipe;
mod wordlist;

pub mod bip32;
pub mod bip39;
mod path;

pub use hmac512::{hmac_sha512, HmacSha512};
pub use path::derive_eth_key;
pub use pbkdf2::pbkdf2_hmac_sha512;
pub use sha256::sha256;
pub use sha512::{sha512, Sha512};
pub use wipe::wipe;
pub use wordlist::ENGLISH_WORDLIST;
