// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Issue a host command to the alive firmware over the command queue. The body
//! is `[cmd][group][payload...]`; the driver composes it in the DMA command
//! area, fills the slot's TFD, advances the ring and rings the doorbell, then
//! returns the new ring write pointer.

use crate::driver::Driver;
use crate::protocol::{Request, E_INVAL, E_OK};
use crate::server::respond;

pub fn handle(driver: &mut Driver, sender_pid: u32, req: &Request, body: &[u8], out: &mut [u8]) {
    if body.len() < 2 {
        let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        return;
    }
    match driver.issue_command(body[0], body[1], &body[2..]) {
        Some(wp) => {
            let resp = (wp as u32).to_le_bytes();
            let _ = respond::send(sender_pid, req, E_OK, &resp, out);
        }
        None => {
            let _ = respond::send(sender_pid, req, E_INVAL, &[], out);
        }
    }
}
