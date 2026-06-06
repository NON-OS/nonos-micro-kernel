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

use super::call_status::call_status;
use super::constants::{OP_SCENE_SUBMIT, SCENE_REQ_LEN};
use super::header::header;

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
    let mut tx = header(OP_SCENE_SUBMIT, request_id, SCENE_REQ_LEN as u32);
    tx.extend_from_slice(&surface_handle.to_le_bytes());
    tx.extend_from_slice(&x.to_le_bytes());
    tx.extend_from_slice(&y.to_le_bytes());
    tx.extend_from_slice(&width.to_le_bytes());
    tx.extend_from_slice(&height.to_le_bytes());
    tx.extend_from_slice(&z.to_le_bytes());
    tx.extend_from_slice(&0u32.to_le_bytes());
    call_status(port, &tx)
}
