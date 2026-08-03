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

use super::super::chacha20::{ChaCha20, KEY_BYTES as STREAM_KEY_BYTES, NONCE_BYTES};
use super::wipe::wipe;

/// Key the stream cipher with `half XOR left` and run it over the right half.
/// The nonce is zero: the key already varies with the block through that XOR.
pub(super) fn stream_into_right(left: &[u8], right: &mut [u8], half: &[u8; STREAM_KEY_BYTES]) {
    let mut key = *half;
    for (k, l) in key.iter_mut().zip(left.iter()) {
        *k ^= *l;
    }
    ChaCha20::new(&key, &[0u8; NONCE_BYTES]).apply(right);
    wipe(&mut key);
}
