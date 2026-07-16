// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! AES-128 in CCM mode with an 8-byte tag and a 2-byte length field, exactly
//! as WPA2 CCMP protects a data frame: CBC-MAC over the additional data and the
//! payload for authentication, then CTR-mode encryption. Checked against the
//! RFC 3610 test vector in `iwlwifi_proofs`. no_std, streaming, no allocation.

use super::aes::Aes128;

const M: usize = 8; // tag length
const L: usize = 2; // length-field octets; nonce is 15 - L = 13 bytes

struct CbcMac<'a> {
    aes: &'a Aes128,
    x: [u8; 16],
    fill: usize,
}

impl<'a> CbcMac<'a> {
    fn new(aes: &'a Aes128) -> Self {
        Self { aes, x: [0u8; 16], fill: 0 }
    }
    fn update(&mut self, data: &[u8]) {
        for &b in data {
            self.x[self.fill] ^= b;
            self.fill += 1;
            if self.fill == 16 {
                self.aes.encrypt_block(&mut self.x);
                self.fill = 0;
            }
        }
    }
    fn close_block(&mut self) {
        if self.fill > 0 {
            self.aes.encrypt_block(&mut self.x);
            self.fill = 0;
        }
    }
}

fn ctr_block(nonce: &[u8; 13], counter: u16) -> [u8; 16] {
    let mut a = [0u8; 16];
    a[0] = (L - 1) as u8;
    a[1..14].copy_from_slice(nonce);
    a[14..16].copy_from_slice(&counter.to_be_bytes());
    a
}

/// Compute the CCM tag over the additional data and the payload (in the clear).
fn mac(aes: &Aes128, nonce: &[u8; 13], aad: &[u8], data: &[u8]) -> [u8; M] {
    let mut cbc = CbcMac::new(aes);
    let mut b0 = [0u8; 16];
    b0[0] = (if aad.is_empty() { 0 } else { 0x40 }) | ((((M - 2) / 2) as u8) << 3) | (L - 1) as u8;
    b0[1..14].copy_from_slice(nonce);
    b0[14..16].copy_from_slice(&(data.len() as u16).to_be_bytes());
    cbc.update(&b0);
    if !aad.is_empty() {
        cbc.update(&(aad.len() as u16).to_be_bytes());
        cbc.update(aad);
        cbc.close_block();
    }
    cbc.update(data);
    cbc.close_block();
    let mut t = [0u8; M];
    t.copy_from_slice(&cbc.x[..M]);
    t
}

fn apply_ctr(aes: &Aes128, nonce: &[u8; 13], input: &[u8], out: &mut [u8]) {
    let mut off = 0;
    let mut counter: u16 = 1;
    while off < input.len() {
        let mut s = ctr_block(nonce, counter);
        aes.encrypt_block(&mut s);
        let n = core::cmp::min(16, input.len() - off);
        for j in 0..n {
            out[off + j] = input[off + j] ^ s[j];
        }
        off += n;
        counter = counter.wrapping_add(1);
    }
}

/// Authenticate `aad` and encrypt `plaintext`, writing the ciphertext followed
/// by the 8-byte tag into `out`. Returns the written length.
pub fn ccm_encrypt(
    key: &[u8; 16],
    nonce: &[u8; 13],
    aad: &[u8],
    plaintext: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    if plaintext.len() > 0xFFFF || aad.len() >= 0xFF00 || out.len() < plaintext.len() + M {
        return None;
    }
    let aes = Aes128::new(key);
    let t = mac(&aes, nonce, aad, plaintext);
    apply_ctr(&aes, nonce, plaintext, &mut out[..plaintext.len()]);
    let mut s0 = ctr_block(nonce, 0);
    aes.encrypt_block(&mut s0);
    for i in 0..M {
        out[plaintext.len() + i] = t[i] ^ s0[i];
    }
    Some(plaintext.len() + M)
}

/// Decrypt `input` (ciphertext followed by the 8-byte tag) and verify the tag
/// over `aad`. Writes the plaintext into `out` and returns its length, or
/// `None` if the tag does not match. The tag compare is constant time.
pub fn ccm_decrypt(
    key: &[u8; 16],
    nonce: &[u8; 13],
    aad: &[u8],
    input: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    if input.len() < M || aad.len() >= 0xFF00 {
        return None;
    }
    let clen = input.len() - M;
    if out.len() < clen {
        return None;
    }
    let aes = Aes128::new(key);
    apply_ctr(&aes, nonce, &input[..clen], &mut out[..clen]);
    let t = mac(&aes, nonce, aad, &out[..clen]);
    let mut s0 = ctr_block(nonce, 0);
    aes.encrypt_block(&mut s0);
    let mut diff = 0u8;
    for i in 0..M {
        diff |= input[clen + i] ^ (t[i] ^ s0[i]);
    }
    if diff == 0 {
        Some(clen)
    } else {
        None
    }
}
