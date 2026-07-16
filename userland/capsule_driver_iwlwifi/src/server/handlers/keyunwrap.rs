// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Unwrap the group key (GTK) carried in EAPOL message three. The body is
//! `[kek:16][wrapped...]`; the reply is the recovered key, or an error if the
//! integrity check fails.

use nonos_wifi_core::ccmp::keywrap::aes_unwrap;
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    if body.len() < 16 {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let mut kek = [0u8; 16];
    kek.copy_from_slice(&body[..16]);
    let mut key = [0u8; 64];
    match aes_unwrap(&kek, &body[16..], &mut key) {
        Some(n) => {
            let _ = respond::send(sender_pid, req, E_OK, &key[..n], out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        }
    }
}
