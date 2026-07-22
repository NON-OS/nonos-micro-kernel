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

use crate::hmac512::HmacSha512;
use crate::wipe::wipe;

use super::scalar::{add_mod_n, is_valid_scalar};
use super::xprv::Xprv;
use super::HARDENED;

/// Hardened child: I = HMAC-SHA512(chain, 0x00 || parent_key || ser32(i')),
/// where i' carries the hardened offset. Needs no public key, which is why
/// the account levels of BIP44 are all hardened.
pub fn child_hardened(parent: &Xprv, index: u32) -> Option<Xprv> {
    let hardened_index = index.checked_add(HARDENED)?;
    let mut mac = HmacSha512::new(&parent.chain);
    mac.update(&[0x00]);
    mac.update(&parent.key);
    mac.update(&hardened_index.to_be_bytes());
    split(parent, mac.finalize())
}

/// Non-hardened child: I = HMAC-SHA512(chain, ser_P(parent_pub) || ser32(i)).
/// The compressed parent public key comes from the caller, computed by
/// whatever secp256k1 the platform trusts. Index must be below the hardened
/// offset.
pub fn child_normal(parent: &Xprv, parent_pub: &[u8; 33], index: u32) -> Option<Xprv> {
    if index >= HARDENED {
        return None;
    }
    let mut mac = HmacSha512::new(&parent.chain);
    mac.update(parent_pub);
    mac.update(&index.to_be_bytes());
    split(parent, mac.finalize())
}

fn split(parent: &Xprv, mut i: [u8; 64]) -> Option<Xprv> {
    let mut tweak = [0u8; 32];
    let mut chain = [0u8; 32];
    tweak.copy_from_slice(&i[..32]);
    chain.copy_from_slice(&i[32..]);
    wipe(&mut i);

    // BIP32: reject the derivation when the tweak is out of range or the sum
    // lands on zero; the caller moves to the next index. Never clamp.
    if !is_valid_scalar(&tweak) {
        wipe(&mut tweak);
        wipe(&mut chain);
        return None;
    }
    let mut key = add_mod_n(&parent.key, &tweak);
    wipe(&mut tweak);
    if !is_valid_scalar(&key) {
        wipe(&mut key);
        wipe(&mut chain);
        return None;
    }
    Some(Xprv { key, chain })
}
