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

use crate::crypto::aes::Ctr64Be;
use crate::sphinx::constants::{STREAM_CIPHER_KEY_SIZE, STREAM_CIPHER_OUTPUT_LENGTH};
use alloc::vec;
use alloc::vec::Vec;

/// One hop's pseudorandom bytes. The IV is all zeros: every hop uses a key
/// derived from its own shared secret, so the keystream never repeats across
/// hops and a counter here would buy nothing.
pub(super) fn pseudorandom_bytes(key: &[u8; STREAM_CIPHER_KEY_SIZE]) -> Vec<u8> {
    let mut out = vec![0u8; STREAM_CIPHER_OUTPUT_LENGTH];
    Ctr64Be::new(key, &[0u8; 16]).keystream(&mut out);
    out
}
