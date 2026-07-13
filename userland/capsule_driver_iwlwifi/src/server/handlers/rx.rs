// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Poll the receive ring for the next firmware response or notification and
//! return its command, group, sequence and payload, so a caller can match a
//! reply to the host command it issued.

use crate::driver::Driver;
use crate::protocol::{Request, E_OK, E_TIMEOUT};
use crate::server::respond;

pub fn handle(driver: &mut Driver, sender_pid: u32, req: &Request, out: &mut [u8]) {
    let mut payload = [0u8; 128];
    match driver.poll_response(&mut payload) {
        Some((cmd, group, seq, n)) => {
            let mut resp = [0u8; 133];
            resp[0] = cmd;
            resp[1] = group;
            resp[2..4].copy_from_slice(&seq.to_le_bytes());
            resp[4] = n as u8;
            resp[5..5 + n].copy_from_slice(&payload[..n]);
            let _ = respond::send(sender_pid, req, E_OK, &resp[..5 + n], out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_TIMEOUT, &[], out);
        }
    }
}
