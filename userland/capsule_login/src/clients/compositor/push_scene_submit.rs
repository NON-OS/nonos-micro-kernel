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
use alloc::vec::Vec;

use nonos_libc::mk_ipc_call;

use super::constants::{HDR_LEN, MAGIC, OP_SCENE_SUBMIT, SCENE_REQ_LEN, VERSION};
use super::status;

pub fn push_scene_submit(
    port: u32,
    request_id: u32,
    surface_handle: u64,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    z: u32,
) -> Result<(), i32> {
    let mut tx = Vec::with_capacity(HDR_LEN + SCENE_REQ_LEN);
    tx.extend_from_slice(&MAGIC.to_le_bytes());
    tx.extend_from_slice(&VERSION.to_le_bytes());
    tx.extend_from_slice(&OP_SCENE_SUBMIT.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&0u16.to_le_bytes());
    tx.extend_from_slice(&request_id.to_le_bytes());
    tx.extend_from_slice(&(SCENE_REQ_LEN as u32).to_le_bytes());
    tx.extend_from_slice(&surface_handle.to_le_bytes());
    tx.extend_from_slice(&x.to_le_bytes());
    tx.extend_from_slice(&y.to_le_bytes());
    tx.extend_from_slice(&width.to_le_bytes());
    tx.extend_from_slice(&height.to_le_bytes());
    tx.extend_from_slice(&z.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());

    let mut rx = vec![0u8; HDR_LEN + 4];
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    status::check(rc, &rx)
}
