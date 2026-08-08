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

use super::super::aes::Aes256;

/// GCM-SIV's counter mode: a 32-bit little-endian counter in the first four
/// bytes, not the big-endian tail counter CTR normally uses.
pub fn apply_ctr32(enc_key: &[u8; 32], tag: &[u8; 16], data: &mut [u8]) {
    let cipher = Aes256::new(enc_key);
    let mut counter = *tag;
    counter[15] |= 0x80;
    for chunk in data.chunks_mut(16) {
        let mut keystream = counter;
        cipher.encrypt_block(&mut keystream);
        for (byte, key) in chunk.iter_mut().zip(keystream.iter()) {
            *byte ^= key;
        }
        let next =
            u32::from_le_bytes([counter[0], counter[1], counter[2], counter[3]]).wrapping_add(1);
        counter[..4].copy_from_slice(&next.to_le_bytes());
    }
}
