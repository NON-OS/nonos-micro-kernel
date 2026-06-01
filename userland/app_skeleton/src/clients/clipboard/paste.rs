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

use alloc::vec;

use crate::discover::lookup_port;
use crate::wire::{call_payload, HDR_LEN, NCLP_MAGIC};

const OP_PASTE: u16 = 0x0003;
const CONTENT_TYPE_TEXT: u32 = 1;
const STATUS_LEN: usize = 4;

pub fn clipboard_paste(out: &mut [u8]) -> Result<usize, &'static str> {
    let port = lookup_port(b"clipboard").ok_or("clipboard not available")?;
    let mut rx = vec![0u8; HDR_LEN + STATUS_LEN + 4 + out.len()];
    let ct = CONTENT_TYPE_TEXT.to_le_bytes();
    let total = call_payload(port, NCLP_MAGIC, OP_PASTE, 0, &ct, &mut rx)?;
    let base = HDR_LEN + STATUS_LEN;
    if total < base + 4 {
        return Ok(0);
    }
    let len = u32::from_le_bytes(rx[base..base + 4].try_into().unwrap()) as usize;
    let avail = total - base - 4;
    let n = len.min(avail).min(out.len());
    out[..n].copy_from_slice(&rx[base + 4..base + 4 + n]);
    Ok(n)
}
