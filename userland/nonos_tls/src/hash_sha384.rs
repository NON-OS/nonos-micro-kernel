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

pub fn hash_sha384(data: &[u8]) -> Option<[u8; 48]> {
    let port = super::crypto_port::crypto_port()?;
    let mut tx = [0u8; 1536];
    let mut rx = [0u8; 96];
    let len = 20usize.checked_add(data.len())?;
    if len > tx.len() {
        return None;
    }
    tx[0..4].copy_from_slice(&0x4e4f_4358u32.to_le_bytes());
    tx[4..6].copy_from_slice(&1u16.to_le_bytes());
    tx[6..8].copy_from_slice(&20u16.to_le_bytes());
    tx[12..16].copy_from_slice(&8u32.to_le_bytes());
    tx[16..20].copy_from_slice(&(data.len() as u32).to_le_bytes());
    tx[20..len].copy_from_slice(data);
    let rc = nonos_libc::mk_ipc_call(port as u64, tx.as_ptr(), len, rx.as_mut_ptr(), rx.len());
    if rc < 72
        || u32::from_le_bytes([rx[0], rx[1], rx[2], rx[3]]) != 0x4e4f_4358
        || u16::from_le_bytes([rx[6], rx[7]]) != 20
        || i32::from_le_bytes([rx[20], rx[21], rx[22], rx[23]]) != 0
    {
        return None;
    }
    let mut out = [0u8; 48];
    out.copy_from_slice(&rx[24..72]);
    Some(out)
}
