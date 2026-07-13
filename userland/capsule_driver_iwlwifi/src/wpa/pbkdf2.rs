// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! PBKDF2-HMAC-SHA1 (RFC 2898). WPA2 derives the pairwise master key as
//! PBKDF2(passphrase, ssid, 4096, 32). Writes into a caller buffer, no
//! allocation. Checked against RFC 6070 vectors.

use super::hmac::{hmac_sha1, hmac_sha1_parts};

/// Fill `out` with PBKDF2-HMAC-SHA1(password, salt, iterations).
pub fn pbkdf2_sha1(password: &[u8], salt: &[u8], iterations: u32, out: &mut [u8]) {
    let mut block_index: u32 = 1;
    let mut off = 0usize;
    while off < out.len() {
        // T_i = U_1 xor U_2 xor ... xor U_c
        let mut u = hmac_sha1_parts(password, &[salt, &block_index.to_be_bytes()]);
        let mut t = u;
        for _ in 1..iterations {
            u = hmac_sha1(password, &u);
            for (tk, uk) in t.iter_mut().zip(u.iter()) {
                *tk ^= *uk;
            }
        }
        let n = core::cmp::min(20, out.len() - off);
        out[off..off + n].copy_from_slice(&t[..n]);
        off += n;
        block_index = block_index.wrapping_add(1);
    }
}
