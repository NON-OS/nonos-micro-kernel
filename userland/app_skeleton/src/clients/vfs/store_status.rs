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

use crate::wire::{read_u32, HDR_LEN};

const ERR_TRANSPORT: i32 = -5;

/// Read the latched capsule-store health code. Zero means the last store
/// operation succeeded; any other value is the server's own status enum.
pub fn store_status() -> Result<u32, i32> {
    let port = super::resolve::vfs_port();
    let mut rx = vec![0u8; HDR_LEN + 12];
    let (status, len) = super::call::call(port, super::types::OP_STORE_STATUS, 19, &[], &mut rx)
        .map_err(|_| ERR_TRANSPORT)?;
    if status != 0 {
        return Err(status);
    }
    if len < HDR_LEN + 8 {
        return Err(ERR_TRANSPORT);
    }
    read_u32(&rx, HDR_LEN + 4).map_err(|_| ERR_TRANSPORT)
}
