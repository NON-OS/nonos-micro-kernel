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

use alloc::vec::Vec;

use crate::discover::lookup_port;
use crate::wire::{call_status, NCLP_MAGIC};

const OP_COPY: u16 = 0x0002;
const CONTENT_TYPE_TEXT: u32 = 1;

pub fn clipboard_copy(text: &[u8]) -> Result<(), &'static str> {
    let port = lookup_port(b"clipboard").ok_or("clipboard not available")?;
    let mut payload = Vec::with_capacity(4 + text.len());
    payload.extend_from_slice(&CONTENT_TYPE_TEXT.to_le_bytes());
    payload.extend_from_slice(text);
    let status = call_status(port, NCLP_MAGIC, OP_COPY, 0, &payload)?;
    if status != 0 {
        return Err("clipboard rejected copy");
    }
    Ok(())
}
