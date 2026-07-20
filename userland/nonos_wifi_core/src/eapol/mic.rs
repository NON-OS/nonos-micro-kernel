// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Verify and compute the MIC of an EAPOL-Key frame for key descriptor version
//! 2 (HMAC-SHA1). The MIC covers the whole frame with the 16-byte MIC field
//! zeroed, keyed by the KCK (the first 16 bytes of the PTK); the MIC is the
//! first 16 bytes of the HMAC-SHA1. The HMAC underneath is checked against RFC
//! vectors, so a round-trip and a tamper test pin the verify logic exactly.

use super::parse::{HEADER_LEN, MIC_LEN, MIC_OFFSET};
use crate::wpa::hmac::hmac_sha1_parts;

/// Compute the version-2 MIC of `frame` under `kck`.
pub fn compute_mic(kck: &[u8], frame: &[u8]) -> [u8; MIC_LEN] {
    let zeros = [0u8; MIC_LEN];
    let h = hmac_sha1_parts(kck, &[&frame[..MIC_OFFSET], &zeros, &frame[MIC_OFFSET + MIC_LEN..]]);
    let mut mic = [0u8; MIC_LEN];
    mic.copy_from_slice(&h[..MIC_LEN]);
    mic
}

/// Whether the MIC carried in `frame` matches the one computed under `kck`.
/// The comparison is constant time.
///
/// The MIC covers exactly the EAPOL-Key frame: the fixed header plus the key
/// data its length field announces. A received MPDU can carry a trailing FCS or
/// padding past that, which is not part of the MIC input, so the frame is bounded
/// to the announced length before hashing rather than trusting `frame.len()`.
pub fn verify_mic(kck: &[u8], frame: &[u8]) -> bool {
    if frame.len() < HEADER_LEN {
        return false;
    }
    let key_data_len = u16::from_be_bytes([frame[HEADER_LEN - 2], frame[HEADER_LEN - 1]]) as usize;
    let end = match HEADER_LEN.checked_add(key_data_len) {
        Some(e) if e <= frame.len() => e,
        _ => return false,
    };
    let computed = compute_mic(kck, &frame[..end]);
    let mut diff = 0u8;
    for (a, b) in frame[MIC_OFFSET..MIC_OFFSET + MIC_LEN].iter().zip(computed.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}
