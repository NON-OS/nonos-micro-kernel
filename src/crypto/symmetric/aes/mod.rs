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

mod core;
mod decrypt;
mod encrypt;
mod key_schedule;
mod modes;

pub use self::core::{INV_SBOX, SBOX};
pub use decrypt::Aes256;
pub use encrypt::Aes128;

pub const BLOCK_SIZE: usize = 16;

/// Standalone AES-128-CTR encryption/decryption function.
///
/// Applies AES-128 in counter mode to the provided data in-place.
/// The `counter` parameter specifies the starting block counter.
pub fn aes128_ctr_apply(key: &[u8; 16], iv: &[u8; 16], counter: u128, inout: &mut [u8]) {
    let cipher = Aes128::new(key);

    // Build initial nonce/counter block: IV XOR'd or combined with counter
    let mut nonce_counter = *iv;

    // Add counter to the last 16 bytes (big-endian)
    let counter_bytes = counter.to_be_bytes();
    for i in 0..16 {
        nonce_counter[i] ^= counter_bytes[i];
    }

    cipher.ctr_apply(&mut nonce_counter, inout);
}
pub const AES128_KEY_SIZE: usize = 16;
pub const AES256_KEY_SIZE: usize = 32;

pub(crate) const AES128_ROUNDS: usize = 10;
pub(crate) const AES256_ROUNDS: usize = 14;

pub(crate) const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];

pub fn aes_gcm_encrypt_in_place(
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    data: &mut [u8],
    tag: &mut [u8; 16],
) {
    let cipher = super::aes_gcm::Aes256Gcm::new(key);
    *tag = cipher.encrypt_in_place(nonce, aad, data);
}

pub fn aes_gcm_decrypt_in_place(
    key: &[u8; 32],
    nonce: &[u8; 12],
    aad: &[u8],
    data: &mut [u8],
    tag: &[u8; 16],
) -> bool {
    let cipher = super::aes_gcm::Aes256Gcm::new(key);
    cipher.decrypt_in_place(nonce, aad, data, tag).is_ok()
}
