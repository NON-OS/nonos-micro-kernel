// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Build an EAPOL-Key frame the supplicant sends (message two or four). The
//! body is `[kck:16][key_info:2][replay:8][nonce:32][key_data...]`; the reply is
//! the frame with its MIC filled in.

use crate::eapol::build::build_key_frame;
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

const PREFIX: usize = 16 + 2 + 8 + 32;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    if body.len() < PREFIX {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let kck = &body[..16];
    let key_info = u16::from_be_bytes([body[16], body[17]]);
    let mut replay = [0u8; 8];
    let mut nonce = [0u8; 32];
    replay.copy_from_slice(&body[18..26]);
    nonce.copy_from_slice(&body[26..58]);
    let key_data = &body[PREFIX..];
    let mut frame = [0u8; 160];
    match build_key_frame(&mut frame, key_info, &replay, &nonce, key_data, kck) {
        Some(n) => {
            let _ = respond::send(sender_pid, req, E_OK, &frame[..n], out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        }
    }
}
