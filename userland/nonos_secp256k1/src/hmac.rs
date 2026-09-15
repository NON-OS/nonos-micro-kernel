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

//! HMAC-SHA256, the one hash this curve needs.
//!
//! RFC 6979 derives the signing nonce with it. In the kernel this came from
//! `crypto::hmac_sha256`; here it is built on `nonos_hash::sha256`, which is
//! already in userland and already proven against the standard vectors.
//!
//! No allocation. The obvious implementation builds `ipad || message` in a
//! `Vec`, which puts a heap allocation on every step of nonce derivation and
//! leaves a copy of a buffer that has held key material somewhere the
//! allocator will hand out again. Both call sites in this crate pass a
//! message of a length known at compile time, so the working buffer is a
//! fixed array that is wiped before it goes out of scope.

use nonos_hash::sha256;

use crate::wipe::wipe;

pub(crate) const BLOCK: usize = 64;

/// The longest message this crate ever HMACs: RFC 6979's `V || 0x00 || key ||
/// hash` at 97 bytes. Anything longer is a caller this crate does not have.
pub(crate) const MAX_MESSAGE: usize = 97;

/// `None` when the message is longer than [`MAX_MESSAGE`], which cannot happen
/// from inside this crate and is refused rather than truncated if it ever does.
pub(crate) fn hmac_sha256(key: &[u8], message: &[u8]) -> Option<[u8; 32]> {
    if message.len() > MAX_MESSAGE {
        return None;
    }

    let mut block = crate::hmac_pad::block_key(key);

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

    /*
     * inner and outer are the key XORed with a constant, which is the key.
     * They are scrubbed here rather than left on the stack for whatever runs
     * next in this capsule to find.
     */
    wipe(&mut block);
    wipe(&mut inner);
    wipe(&mut outer);
    Some(mac)
}
