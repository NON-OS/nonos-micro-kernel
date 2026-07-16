// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Derive the WPA2 pairwise transient key from the four-way handshake inputs.
//! The body is `[pp_len][passphrase][ssid_len][ssid][aa:6][spa:6][anonce:32]
//! [snonce:32]`; the handler returns the 48-byte PTK. All lengths are bounds
//! checked against the untrusted body before use.

use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;
use nonos_wifi_core::wpa::ptk::{pmk, ptk};

const TAIL: usize = 6 + 6 + 32 + 32;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    let Some((passphrase, ssid, aa, spa, anonce, snonce)) = parse(body) else {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    };
    let key = pmk(passphrase, ssid);
    let derived = ptk(&key, &aa, &spa, &anonce, &snonce);
    let _ = respond::send(sender_pid, req, E_OK, &derived, out);
}

type Parsed<'a> = (&'a [u8], &'a [u8], [u8; 6], [u8; 6], [u8; 32], [u8; 32]);

fn parse(body: &[u8]) -> Option<Parsed<'_>> {
    if body.is_empty() {
        return None;
    }
    let pp_len = body[0] as usize;
    let ssid_len_off = 1 + pp_len;
    if body.len() <= ssid_len_off {
        return None;
    }
    let passphrase = &body[1..ssid_len_off];
    let ssid_len = body[ssid_len_off] as usize;
    let ssid_off = ssid_len_off + 1;
    let tail_off = ssid_off + ssid_len;
    if body.len() < tail_off + TAIL {
        return None;
    }
    let ssid = &body[ssid_off..tail_off];
    let mut aa = [0u8; 6];
    let mut spa = [0u8; 6];
    let mut anonce = [0u8; 32];
    let mut snonce = [0u8; 32];
    aa.copy_from_slice(&body[tail_off..tail_off + 6]);
    spa.copy_from_slice(&body[tail_off + 6..tail_off + 12]);
    anonce.copy_from_slice(&body[tail_off + 12..tail_off + 44]);
    snonce.copy_from_slice(&body[tail_off + 44..tail_off + 76]);
    Some((passphrase, ssid, aa, spa, anonce, snonce))
}
