// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

use super::{HDR_LEN, MAGIC, VERSION};

pub fn response(req_op: u16, request_id: u32, errno: i32, payload: &[u8], out: &mut [u8]) -> usize {
    let len = HDR_LEN + payload.len();
    // Never write past the reply buffer: a payload that would overrun it is
    // dropped rather than panicking the driver.
    if out.len() < len {
        return 0;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&req_op.to_le_bytes());
    out[8..12].copy_from_slice(&errno.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    out[HDR_LEN..len].copy_from_slice(payload);
    len
}
