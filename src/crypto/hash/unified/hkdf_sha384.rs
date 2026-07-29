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

//! HKDF-SHA-384 (RFC 5869 with SHA-384).
//! PRK size is 48 bytes. Max expand output is 255 * 48 = 12,240 bytes.

use super::hmac_sha384::hmac_sha384;
use crate::crypto::hash::sha384::Hash384;
use alloc::vec::Vec;

pub fn hkdf_extract_sha384(salt: Option<&[u8]>, ikm: &[u8]) -> Hash384 {
    let zero = [0u8; 48];
    let s = salt.unwrap_or(&zero);
    hmac_sha384(s, ikm)
}

pub fn hkdf_expand_sha384(prk: &[u8], info: &[u8], okm: &mut [u8]) -> Result<(), &'static str> {
    if okm.len() > 255 * 48 {
        return Err("hkdf-sha384: too large okm");
    }
    let mut t = [0u8; 48];
    let mut previous: Vec<u8> = Vec::new();
    let mut generated = 0usize;
    let mut counter = 1u8;
    while generated < okm.len() {
        let mut hmac_in = Vec::new();
        hmac_in.extend_from_slice(&previous);
        hmac_in.extend_from_slice(info);
        hmac_in.push(counter);
        t = hmac_sha384(prk, &hmac_in);
        let take = core::cmp::min(okm.len() - generated, 48);
        okm[generated..generated + take].copy_from_slice(&t[..take]);

        // SAFETY: write_volatile ensures the compiler cannot optimize away this
        // write. The pointer is valid from the mutable iterator over Vec<u8>.
        for b in hmac_in.iter_mut() {
            unsafe { core::ptr::write_volatile(b, 0) };
        }

        previous.clear();
        previous.extend_from_slice(&t);
        generated += take;
        counter = counter.wrapping_add(1);
    }

    // SAFETY: write_volatile ensures the compiler cannot optimize away these
    // writes. The pointers are valid from mutable references to local arrays/Vecs.
    for b in &mut t {
        unsafe { core::ptr::write_volatile(b, 0) };
    }
    for b in previous.iter_mut() {
        unsafe { core::ptr::write_volatile(b, 0) };
    }

    Ok(())
}
