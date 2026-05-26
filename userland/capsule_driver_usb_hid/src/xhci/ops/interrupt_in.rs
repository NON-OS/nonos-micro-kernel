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

//! INTERRUPT_IN call. Request body (4 bytes):
//! `[0]=slot,[1]=dci,[2..4]=length_le`. Reply payload (after
//! status): `[0..2]=actual_len_le`, then report bytes.
//! `status == E_AGAIN(-11)` means no report yet (Ok(None)).
//! Mirrors `xhci/server/handlers/interrupt_in/reply.rs`.

use crate::xhci::call::{call, XhciClientError};
use crate::xhci::wire::{HDR_LEN, OP_INTERRUPT_IN, STATUS_LEN, E_AGAIN};

const HDR_AND_STATUS: usize = HDR_LEN + STATUS_LEN;
const REPLY_PREFIX: usize = 2;
const MAX_REPORT: usize = 8;
const RESP_BUF: usize = HDR_AND_STATUS + REPLY_PREFIX + MAX_REPORT;

pub fn interrupt_in(
    xhci_port: u32,
    slot: u8,
    dci: u8,
    length: u16,
    out: &mut [u8],
) -> Result<Option<usize>, XhciClientError> {
    let mut body = [0u8; 4];
    body[0] = slot;
    body[1] = dci;
    body[2..4].copy_from_slice(&length.to_le_bytes());
    let mut resp = [0u8; RESP_BUF];
    let (status, data_len) = call(xhci_port, OP_INTERRUPT_IN, &body, &mut resp)?;
    if status == E_AGAIN {
        return Ok(None);
    }
    if status != 0 || data_len < REPLY_PREFIX {
        return Err(XhciClientError::BadResponse);
    }
    let o = HDR_AND_STATUS;
    let actual = u16::from_le_bytes([resp[o], resp[o + 1]]) as usize;
    if data_len < REPLY_PREFIX + actual || out.len() < actual {
        return Err(XhciClientError::BadResponse);
    }
    out[..actual].copy_from_slice(&resp[o + REPLY_PREFIX..o + REPLY_PREFIX + actual]);
    Ok(Some(actual))
}
