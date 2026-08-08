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

use crate::crypto::hkdf_blake3::{expand, extract};
use crate::crypto::types::CryptoError;
use crate::crypto::{fill_random, x25519_public, x25519_shared};

/// Bytes of the key a packet's message is sealed under. Counter mode over a
/// 128 bit block cipher, so half the width used elsewhere.
pub const PACKET_KEY_BYTES: usize = 16;

/// Agree a key with the recipient for this packet alone.
///
/// The public half travels with the packet so the recipient can repeat the
/// agreement; the private half is thrown away here. A fresh pair per packet
/// is what keeps two packets to the same recipient from sharing a key, which
/// is what would otherwise link them.
pub fn packet_shared_key(
    recipient_encryption_key: &[u8; 32],
) -> Result<([u8; 32], [u8; PACKET_KEY_BYTES]), CryptoError> {
    let mut private = [0u8; 32];
    fill_random(&mut private)?;
    let mut public = [0u8; 32];
    x25519_public(&private, &mut public)?;

    let mut shared = [0u8; 32];
    x25519_shared(&private, recipient_encryption_key, &mut shared)?;

    let mut prk = [0u8; 32];
    extract(&[], &shared, &mut prk)?;
    let mut key = [0u8; PACKET_KEY_BYTES];
    expand(&prk, &[], &mut key)?;
    Ok((public, key))
}
