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

use crate::proto::{call_status, header};

const OP_SCENE_SUBMIT: u16 = 0x0002;
const OP_DAMAGE_COMMIT: u16 = 0x0003;
const OP_SCENE_REMOVE: u16 = 0x0007;
const SCENE_REQ_LEN: u32 = 32;
const DAMAGE_REQ_LEN: u32 = 16;
const SCENE_REMOVE_REQ_LEN: u32 = 8;
const OVERLAY_Z: u32 = 4_000_000;

pub(crate) fn submit(port: u32, rid: u32, handle: u64, w: u32, h: u32) -> Result<(), i32> {
    let mut tx = header(OP_SCENE_SUBMIT, rid, SCENE_REQ_LEN);
    tx.extend_from_slice(&handle.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&w.to_le_bytes());
    tx.extend_from_slice(&h.to_le_bytes());
    tx.extend_from_slice(&OVERLAY_Z.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    call_status(port, &tx)
}

pub(crate) fn damage(port: u32, rid: u32, w: u32, h: u32) -> Result<(), i32> {
    let mut tx = header(OP_DAMAGE_COMMIT, rid, DAMAGE_REQ_LEN);
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&w.to_le_bytes());
    tx.extend_from_slice(&h.to_le_bytes());
    call_status(port, &tx)
}

pub(crate) fn remove(port: u32, rid: u32) -> Result<(), i32> {
    let mut tx = header(OP_SCENE_REMOVE, rid, SCENE_REMOVE_REQ_LEN);
    tx.extend_from_slice(&0u32.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    call_status(port, &tx)
}
