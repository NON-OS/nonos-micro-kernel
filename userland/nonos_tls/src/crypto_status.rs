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

pub fn crypto_status(op: u16, body: &[u8]) -> bool {
    let Some(port) = super::crypto_port::crypto_port() else { return false };
    let mut tx = [0u8; 320];
    let mut rx = [0u8; 32];
    let len = 20usize.saturating_add(body.len());
    if len > tx.len() {
        return false;
    }
    tx[0..4].copy_from_slice(&0x4e4f_4358u32.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&op.to_le_bytes());
    tx[12..16].copy_from_slice(&7u32.to_le_bytes());
    tx[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    tx[20..len].copy_from_slice(body);
    let rc = nonos_libc::mk_ipc_call(port as u64, tx.as_ptr(), len, rx.as_mut_ptr(), rx.len());
    rc >= 24
        && u32::from_le_bytes([rx[0], rx[1], rx[2], rx[3]]) == 0x4e4f_4358
        && u16::from_le_bytes([rx[6], rx[7]]) == op
        && i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]) == 0
}
