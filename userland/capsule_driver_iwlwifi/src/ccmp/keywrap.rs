// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! AES Key Unwrap (RFC 3394). WPA2 carries the group key (GTK) in EAPOL message
//! three wrapped with this under the KEK (the second sixteen bytes of the PTK).
//! Unwrap recovers the key blocks and checks the fixed integrity value, so a
//! forged or wrong-key wrap is rejected. Checked against the RFC 3394 vector.

use super::aes::Aes128;

const IV: [u8; 8] = [0xA6, 0xA6, 0xA6, 0xA6, 0xA6, 0xA6, 0xA6, 0xA6];
const MAX_BLOCKS: usize = 8;

fn xor_counter(a: &mut [u8; 8], t: u64) {
    let tb = t.to_be_bytes();
    for (x, y) in a.iter_mut().zip(tb.iter()) {
        *x ^= *y;
    }
}

/// Unwrap `wrapped` (n+1 eight-byte blocks) under `kek`, writing the n recovered
/// key blocks into `out`. Returns the number of bytes written, or `None` if the
/// input is malformed or the integrity check fails.
pub fn aes_unwrap(kek: &[u8; 16], wrapped: &[u8], out: &mut [u8]) -> Option<usize> {
    if wrapped.len() < 16 || !wrapped.len().is_multiple_of(8) {
        return None;
    }
    let n = wrapped.len() / 8 - 1;
    if n == 0 || n > MAX_BLOCKS || out.len() < n * 8 {
        return None;
    }
    let aes = Aes128::new(kek);
    let mut a = [0u8; 8];
    a.copy_from_slice(&wrapped[..8]);
    let mut r = [[0u8; 8]; MAX_BLOCKS];
    for (i, block) in r.iter_mut().take(n).enumerate() {
        block.copy_from_slice(&wrapped[(i + 1) * 8..(i + 2) * 8]);
    }
    for j in (0..6).rev() {
        for i in (1..=n).rev() {
            xor_counter(&mut a, (n * j + i) as u64);
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(&a);
            block[8..].copy_from_slice(&r[i - 1]);
            aes.decrypt_block(&mut block);
            a.copy_from_slice(&block[..8]);
            r[i - 1].copy_from_slice(&block[8..]);
        }
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(IV.iter()) {
        diff |= x ^ y;
    }
    if diff != 0 {
        return None;
    }
    for (i, block) in r.iter().take(n).enumerate() {
        out[i * 8..(i + 1) * 8].copy_from_slice(block);
    }
    Some(n * 8)
}
