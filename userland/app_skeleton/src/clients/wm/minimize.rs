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

use crate::wire::{call_status, NWMP_MAGIC};

const OP_MINIMIZE: u16 = 0x0009;
const OP_RESTORE: u16 = 0x000A;
const BODY_LEN: usize = 8;

fn window_op(port: u32, request_id: u32, op: u16, window_id: u32) -> Result<(), &'static str> {
    let mut body = [0u8; BODY_LEN];
    body[0..4].copy_from_slice(&window_id.to_le_bytes());
    let status = call_status(port, NWMP_MAGIC, op, request_id, &body)?;
    if status != 0 {
        return Err("wm rejected window state op");
    }
    Ok(())
}

pub fn window_minimize(port: u32, request_id: u32, window_id: u32) -> Result<(), &'static str> {
    window_op(port, request_id, OP_MINIMIZE, window_id)
}

pub fn window_restore(port: u32, request_id: u32, window_id: u32) -> Result<(), &'static str> {
    window_op(port, request_id, OP_RESTORE, window_id)
}
