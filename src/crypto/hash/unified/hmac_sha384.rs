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

//! HMAC-SHA-384 (RFC 2104 with SHA-384).
//! Block size is 128 bytes (same as SHA-512). Output is 48 bytes.

use crate::crypto::constant_time;
use crate::crypto::hash::sha384::{sha384, Hash384};
use alloc::vec::Vec;

/// SHA-384 block size (same as SHA-512).
const BLOCK_SIZE: usize = 128;

pub fn hmac_sha384(key: &[u8], message: &[u8]) -> Hash384 {
    let mut key_block = [0u8; BLOCK_SIZE];
    if key.len() > BLOCK_SIZE {
        let hk = sha384(key);
        key_block[..48].copy_from_slice(&hk);
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }

    let mut ipad = [0x36u8; BLOCK_SIZE];
    let mut opad = [0x5cu8; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        ipad[i] ^= key_block[i];
        opad[i] ^= key_block[i];
    }

    constant_time::secure_zero(&mut key_block);

    let mut inner = Vec::with_capacity(BLOCK_SIZE + message.len());
    inner.extend_from_slice(&ipad);
    inner.extend_from_slice(message);
    let inner_hash = sha384(&inner);

    constant_time::secure_zero(&mut ipad);

    let mut outer = Vec::with_capacity(BLOCK_SIZE + 48);
    outer.extend_from_slice(&opad);
    outer.extend_from_slice(&inner_hash);
    let result = sha384(&outer);

    constant_time::secure_zero(&mut opad);

    result
}

pub fn hmac_sha384_verify(key: &[u8], message: &[u8], mac: &[u8]) -> bool {
    let expect = hmac_sha384(key, message);
    constant_time::ct_eq(&expect, mac)
}
