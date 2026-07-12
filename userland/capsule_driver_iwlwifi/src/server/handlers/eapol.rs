// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Verify an EAPOL-Key frame of the four-way handshake: parse it and check its
//! MIC under the KCK (the first 16 bytes of the PTK). The body is
//! `[kck:16][eapol_frame...]`; the reply is `[valid:1][key_info:2][nonce:32]`.

use crate::eapol::mic::verify_mic;
use crate::eapol::parse::{parse, KEY_INFO_MIC, KEY_INFO_PAIRWISE};
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    if body.len() < 16 {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let kck = &body[..16];
    let frame = &body[16..];
    let Some(key) = parse(frame) else {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    };
    let valid = verify_mic(kck, frame);
    let pairwise = key.key_info & KEY_INFO_PAIRWISE != 0;
    let has_mic = key.key_info & KEY_INFO_MIC != 0;
    // valid, flags, descriptor, key info, key length, replay counter, nonce,
    // MIC, and the key-data length: the whole parsed frame the supplicant needs.
    let mut resp = [0u8; 65];
    resp[0] = valid as u8;
    resp[1] = (pairwise as u8) | ((has_mic as u8) << 1);
    resp[2] = key.descriptor_type;
    resp[3..5].copy_from_slice(&key.key_info.to_be_bytes());
    resp[5..7].copy_from_slice(&key.key_length.to_be_bytes());
    resp[7..15].copy_from_slice(&key.replay_counter);
    resp[15..47].copy_from_slice(&key.nonce);
    resp[47..63].copy_from_slice(&key.mic);
    resp[63..65].copy_from_slice(&(key.key_data.len() as u16).to_be_bytes());
    let _ = respond::send(sender_pid, req, E_OK, &resp, out);
}
