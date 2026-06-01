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

use crate::catalog::get_bytes;
use crate::protocol::{CHUNK_MAX, E_NOT_FOUND, E_RANGE, OP_GET_CHUNK};

use super::super::respond;

pub fn handle(pid: u32, index: u32, offset: u32) {
    let bytes = match get_bytes(index) {
        Some(b) => b,
        None => return respond::err(pid, OP_GET_CHUNK, index, E_NOT_FOUND),
    };
    let start = offset as usize;
    if start > bytes.len() {
        return respond::err(pid, OP_GET_CHUNK, index, E_RANGE);
    }
    let end = core::cmp::min(start + CHUNK_MAX, bytes.len());
    respond::ok(pid, OP_GET_CHUNK, index, offset, &bytes[start..end]);
}
