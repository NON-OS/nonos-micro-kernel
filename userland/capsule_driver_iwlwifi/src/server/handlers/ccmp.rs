// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Protect or unprotect a frame with WPA2 CCMP (AES-128-CCM). The body is
//! `[dir][key:16][nonce:13][aad_len][aad...][data...]`: dir 0 encrypts and
//! authenticates, dir 1 decrypts and verifies. The reply is the result, or an
//! error if authentication fails. All lengths are bounds checked.

use crate::ccmp::ccm::{ccm_decrypt, ccm_encrypt};
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

const PREFIX: usize = 1 + 16 + 13 + 1;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    if body.len() < PREFIX {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let dir = body[0];
    let mut key = [0u8; 16];
    let mut nonce = [0u8; 13];
    key.copy_from_slice(&body[1..17]);
    nonce.copy_from_slice(&body[17..30]);
    let aad_len = body[30] as usize;
    let aad_end = PREFIX + aad_len;
    if body.len() < aad_end {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let aad = &body[PREFIX..aad_end];
    let data = &body[aad_end..];
    let mut scratch = [0u8; 256];
    let result = if dir == 0 {
        ccm_encrypt(&key, &nonce, aad, data, &mut scratch)
    } else {
        ccm_decrypt(&key, &nonce, aad, data, &mut scratch)
    };
    match result {
        Some(n) => {
            let _ = respond::send(sender_pid, req, E_OK, &scratch[..n], out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        }
    }
}
