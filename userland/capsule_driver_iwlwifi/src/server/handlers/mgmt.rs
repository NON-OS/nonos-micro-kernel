// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Build an 802.11 management frame on request: probe request, open-system
//! authentication or association request. A supplicant hands the driver the
//! parameters and receives the frame bytes to transmit.

use crate::dot11::header::{SUBTYPE_ASSOC_REQ, SUBTYPE_AUTH, SUBTYPE_PROBE_REQ};
use crate::dot11::mgmt::{assoc_request, auth_open, probe_request};
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

const DEFAULT_RATES: &[u8] = &[0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24];
const DEFAULT_CAP: u16 = 0x0431;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    if body.len() < 13 {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    let subtype = body[0];
    let mut src = [0u8; 6];
    src.copy_from_slice(&body[1..7]);
    let mut bssid = [0u8; 6];
    bssid.copy_from_slice(&body[7..13]);
    let ssid = &body[13..];
    let mut frame = [0u8; 256];
    let built = match subtype {
        SUBTYPE_PROBE_REQ => probe_request(&mut frame, src, ssid, DEFAULT_RATES, 0),
        SUBTYPE_AUTH => auth_open(&mut frame, src, bssid, 0),
        SUBTYPE_ASSOC_REQ => {
            assoc_request(&mut frame, src, bssid, ssid, DEFAULT_RATES, DEFAULT_CAP, 0)
        }
        _ => None,
    };
    match built {
        Some(n) => {
            let _ = respond::send(sender_pid, req, E_OK, &frame[..n], out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        }
    }
}
