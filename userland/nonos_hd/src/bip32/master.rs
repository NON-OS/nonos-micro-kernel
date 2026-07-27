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

use crate::hmac512::hmac_sha512;
use crate::wipe::wipe;

use super::scalar::is_valid_scalar;
use super::xprv::Xprv;

/// BIP32 master key: HMAC-SHA512 keyed "Bitcoin seed" over the seed; the left
/// half is the key, the right half the chain code. None for a seed outside
/// 16..=64 bytes or the (astronomically unlikely) invalid scalar, which the
/// standard says to reject rather than clamp.
pub fn master_from_seed(seed: &[u8]) -> Option<Xprv> {
    if seed.len() < 16 || seed.len() > 64 {
        return None;
    }
    let mut i = hmac_sha512(b"Bitcoin seed", seed);

    let mut key = [0u8; 32];
    let mut chain = [0u8; 32];
    key.copy_from_slice(&i[..32]);
    chain.copy_from_slice(&i[32..]);
    wipe(&mut i);

    if !is_valid_scalar(&key) {
        wipe(&mut key);
        wipe(&mut chain);
        return None;
    }
    Some(Xprv { key, chain })
}
