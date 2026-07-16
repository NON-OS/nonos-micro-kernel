// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Parse a received beacon or probe-response frame into a network summary: the
//! BSSID, operating channel, whether it advertises RSN (WPA2), the capability
//! field, and the SSID. This is how a scan turns air into a network list.

use crate::dot11::parse::parse_beacon;
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

pub fn handle(sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    match parse_beacon(body) {
        Some(info) => {
            let mut resp = [0u8; 96];
            resp[0..6].copy_from_slice(&info.bssid);
            resp[6] = info.channel;
            resp[7] = info.rsn as u8;
            resp[8..10].copy_from_slice(&info.capability.to_le_bytes());
            let ssid_len = core::cmp::min(info.ssid.len(), resp.len() - 11);
            resp[10] = ssid_len as u8;
            resp[11..11 + ssid_len].copy_from_slice(&info.ssid[..ssid_len]);
            let _ = respond::send(sender_pid, req, E_OK, &resp[..11 + ssid_len], out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        }
    }
}
