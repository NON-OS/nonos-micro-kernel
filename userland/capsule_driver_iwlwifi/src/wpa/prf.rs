// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! The IEEE 802.11i pseudo-random function over HMAC-SHA1. Used to expand the
//! pairwise master key into the pairwise transient key. `PRF(K, label, data)`
//! is HMAC-SHA1(K, label || 0x00 || data || i) concatenated over a counter i
//! until the output is filled. Checked structurally and through the PTK vector.

use super::hmac::hmac_sha1_parts;

/// Fill `out` with the 802.11i PRF of `data` under `key` with `label`.
pub fn prf(key: &[u8], label: &[u8], data: &[u8], out: &mut [u8]) {
    let mut counter: u8 = 0;
    let mut off = 0usize;
    while off < out.len() {
        let block = hmac_sha1_parts(key, &[label, &[0x00], data, &[counter]]);
        let n = core::cmp::min(20, out.len() - off);
        out[off..off + n].copy_from_slice(&block[..n]);
        off += n;
        counter = counter.wrapping_add(1);
    }
}
