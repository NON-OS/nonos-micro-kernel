// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! HMAC-SHA1 (RFC 2104). The keyed MAC WPA2 builds PBKDF2 and the 802.11i PRF
//! on. The core takes the message as a list of parts so callers can MAC a
//! concatenation without allocating. Checked against RFC 2202 vectors.

use super::sha1::{sha1, Sha1};

const BLOCK: usize = 64;

/// HMAC-SHA1 of the concatenation of `parts` under `key`.
pub fn hmac_sha1_parts(key: &[u8], parts: &[&[u8]]) -> [u8; 20] {
    let mut k = [0u8; BLOCK];
    if key.len() > BLOCK {
        k[..20].copy_from_slice(&sha1(key));
    } else {
        k[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; BLOCK];
    let mut opad = [0x5cu8; BLOCK];
    for ((ip, op), kk) in ipad.iter_mut().zip(opad.iter_mut()).zip(k.iter()) {
        *ip ^= *kk;
        *op ^= *kk;
    }
    let mut inner = Sha1::new();
    inner.update(&ipad);
    for p in parts {
        inner.update(p);
    }
    let inner_hash = inner.finalize();
    let mut outer = Sha1::new();
    outer.update(&opad);
    outer.update(&inner_hash);
    outer.finalize()
}

/// HMAC-SHA1 of a single message.
pub fn hmac_sha1(key: &[u8], msg: &[u8]) -> [u8; 20] {
    hmac_sha1_parts(key, &[msg])
}
