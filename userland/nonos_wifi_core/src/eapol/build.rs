// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Build an EAPOL-Key frame, the send side of the four-way handshake. The
//! supplicant sends message two (its nonce) and message four (confirmation),
//! each carrying a MIC over the whole frame under the KCK. This lays out the
//! fixed header, appends any key data, and fills the MIC last.

use super::mic::compute_mic;
use super::parse::{EAPOL_TYPE_KEY, HEADER_LEN, MIC_LEN, MIC_OFFSET};

/// Build an EAPOL-Key frame into `out`, computing and inserting the MIC under
/// `kck`. Returns the frame length, or `None` if the buffer is too small. The
/// key-length field is left zero, as it is in the supplicant's messages.
pub fn build_key_frame(
    out: &mut [u8],
    key_info: u16,
    replay_counter: &[u8; 8],
    nonce: &[u8; 32],
    key_data: &[u8],
    kck: &[u8],
) -> Option<usize> {
    let total = HEADER_LEN.checked_add(key_data.len())?;
    if out.len() < total || key_data.len() > 0xFFFF {
        return None;
    }
    for b in out[..total].iter_mut() {
        *b = 0;
    }
    out[0] = 2;
    out[1] = EAPOL_TYPE_KEY;
    let plen = (HEADER_LEN - 4 + key_data.len()) as u16;
    out[2..4].copy_from_slice(&plen.to_be_bytes());
    out[4] = 2;
    out[5..7].copy_from_slice(&key_info.to_be_bytes());
    out[9..17].copy_from_slice(replay_counter);
    out[17..49].copy_from_slice(nonce);
    out[97..99].copy_from_slice(&(key_data.len() as u16).to_be_bytes());
    out[HEADER_LEN..total].copy_from_slice(key_data);
    let mic = compute_mic(kck, &out[..total]);
    out[MIC_OFFSET..MIC_OFFSET + MIC_LEN].copy_from_slice(&mic);
    Some(total)
}
