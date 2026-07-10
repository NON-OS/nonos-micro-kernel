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

//! Delete a desktop entry: a directory is removed with rmdir, a file with
//! unlink.

use alloc::vec;

use super::call::call;
use super::constants::{OP_RMDIR, OP_UNLINK};
use super::owner_body::owner_body;
use super::path;

pub fn remove(path: &[u8], is_dir: bool) -> bool {
    if !path::is_valid(path) {
        return false;
    }
    let op = if is_dir { OP_RMDIR } else { OP_UNLINK };
    let mut rx = vec![0u8; 64];
    call(op, &owner_body(path), &mut rx).is_some()
}
