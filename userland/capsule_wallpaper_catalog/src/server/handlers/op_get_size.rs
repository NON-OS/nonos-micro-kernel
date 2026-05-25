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

use crate::catalog::get_size;
use crate::protocol::{E_NOT_FOUND, OP_GET_SIZE};

use super::super::respond;

pub fn handle(pid: u32, index: u32) {
    match get_size(index) {
        Some(sz) => respond::ok(pid, OP_GET_SIZE, index, 0, &sz.to_le_bytes()),
        None => respond::err(pid, OP_GET_SIZE, index, E_NOT_FOUND),
    }
}
