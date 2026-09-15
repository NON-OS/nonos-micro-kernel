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

//! HMAC-SHA256, which HKDF is built from.
//!
//! Same construction as the one in `nonos_secp256k1`, on the same
//! `nonos_hd::sha256`. It is duplicated rather than shared because the two
//! crates have different message bounds and neither should carry the other's:
//! there, the longest message is RFC 6979's 97 bytes; here it is an HKDF info
//! string.
//!
//! No allocation. The working buffer is a fixed array wiped before it goes
//! out of scope, since it holds the key XORed with a constant, which is the
//! key.

use nonos_hd::sha256;

use crate::wipe::wipe;

const BLOCK: usize = 64;

/// The longest message this crate HMACs: the HKDF info string, which is the
/// prefix, a record name and the counter byte.
pub(crate) const MAX_MESSAGE: usize = 128;

/// `None` when the message is longer than [`MAX_MESSAGE`], which cannot
/// happen from inside this crate and is refused rather than truncated if it
/// ever does.
pub(crate) fn hmac_sha256(key: &[u8], message: &[u8]) -> Option<[u8; 32]> {
    if message.len() > MAX_MESSAGE {
        return None;
    }
    let mut block = [0u8; BLOCK];
    if key.len() > BLOCK {
        block[..32].copy_from_slice(&sha256(key));
    } else {
        block[..key.len()].copy_from_slice(key);
    }

    let mut inner = [0u8; BLOCK + MAX_MESSAGE];
    let mut outer = [0u8; BLOCK + 32];
    for i in 0..BLOCK {
        inner[i] = 0x36 ^ block[i];
        outer[i] = 0x5c ^ block[i];
    }
    inner[BLOCK..BLOCK + message.len()].copy_from_slice(message);
    let inner_hash = sha256(&inner[..BLOCK + message.len()]);
    outer[BLOCK..].copy_from_slice(&inner_hash);
    let mac = sha256(&outer);

    wipe(&mut block);
    wipe(&mut inner);
    wipe(&mut outer);
    Some(mac)
}
