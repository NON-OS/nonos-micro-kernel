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

#![allow(clippy::too_many_arguments)]

extern crate alloc;

pub mod application;
pub mod asymmetric;
pub mod base64;
pub mod core;
pub mod error;
pub mod exports;
pub mod hardware_accel;
pub mod hash;
pub mod kernel_keys;
pub mod pqc;
pub mod pqclean_support;
pub mod random_api;
pub mod stark;
pub mod symmetric;
pub mod util;
pub mod zk;
pub mod zk_kernel;

pub use asymmetric::ed25519;
pub use asymmetric::secp256k1;
pub use error::{CryptoError, CryptoResult};
pub use hash::blake3;
pub use hash::sha3;
pub use hash::sha3 as keccak;
pub use hash::sha512;
pub use random_api as random;
pub use symmetric::aes;
pub use symmetric::aes_gcm;
pub use symmetric::chacha20poly1305;
pub use util::bigint;
pub use util::constant_time;
pub use util::entropy;
pub use util::hmac;
pub use util::rng;

pub mod sha256 {
    pub fn hash(data: &[u8]) -> [u8; 32] {
        super::hash::sha256(data)
    }
}
pub use exports::*;
pub use rng::{fill_random_bytes, get_random_bytes, random_u32};

pub fn fnv1a_u32(value: u32) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    for byte in value.to_le_bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    hash
}
