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

use nonos_libc::mk_ipc_call;

use super::constants::OP_HEALTHCHECK;

pub fn health(port: u32, magic: u32) -> bool {
    let mut tx = [0u8; 20];
    let mut rx = [0u8; 20];
    tx[0..4].copy_from_slice(&magic.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&OP_HEALTHCHECK.to_le_bytes());
    tx[12..16].copy_from_slice(&1u32.to_le_bytes());
    let rc = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    rc >= 20
        && u32::from_le_bytes([rx[0], rx[1], rx[2], rx[3]]) == magic
        && rx[8] == 0
        && rx[9] == 0
}
