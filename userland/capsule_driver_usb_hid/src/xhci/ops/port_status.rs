// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! PORT_STATUS call. Request: empty body. Reply payload (after
//! status): `[0]=port_count, [1..4]=resv`, then `port_count` 8-byte
//! entries each `[0]=port_id,[1..4]=resv,[4..8]=portsc_raw`.
//! Mirrors `xhci/server/handlers/port_status.rs`.

use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{HDR_LEN, OP_PORT_STATUS, STATUS_LEN};

const HDR_AND_STATUS: usize = HDR_LEN + STATUS_LEN;
const REPLY_HDR: usize = 4;
const ENTRY: usize = 8;
const MAX_PORTS: usize = 255;

#[derive(Clone, Copy, Debug)]
pub struct PortSnapshot {
    pub port_id: u8,
    pub portsc_raw: u32,
}

pub fn port_status(
    xhci_port: u32,
    out: &mut [PortSnapshot; MAX_PORTS],
) -> Result<usize, XhciClientError> {
    let mut resp = [0u8; HDR_AND_STATUS + REPLY_HDR + MAX_PORTS * ENTRY];
    let (status, data_len) = call(xhci_port, OP_PORT_STATUS, &[], &mut resp)?;
    if status != 0 || data_len < REPLY_HDR {
        return Err(XhciClientError::BadResponse);
    }
    let base = HDR_AND_STATUS;
    let count = resp[base] as usize;
    if count > MAX_PORTS || data_len < REPLY_HDR + count * ENTRY {
        return Err(XhciClientError::BadResponse);
    }
    let mut o = base + REPLY_HDR;
    for i in 0..count {
        out[i] = PortSnapshot {
            port_id: resp[o],
            portsc_raw: u32::from_le_bytes([resp[o+4], resp[o+5], resp[o+6], resp[o+7]]),
        };
        o += ENTRY;
    }
    Ok(count)
}
