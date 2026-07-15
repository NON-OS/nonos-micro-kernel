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

use alloc::vec::Vec;
use nonos_libc::mk_service_lookup;

pub(super) const HDR: usize = 20;
pub(super) const OP_CONNECT: u16 = 3;
pub(super) const OP_SEND: u16 = 5;
pub(super) const OP_RECV: u16 = 6;
pub(super) const OP_CLOSE: u16 = 7;
pub(super) const OP_STATE: u16 = 9;
pub(super) const SEG_MAX: usize = 1460;
pub(super) const RX_CAP: usize = HDR + 1524;

const MAGIC: u32 = 0x4E54_4350;

pub(super) fn frame(op: u16, body: &[u8]) -> Vec<u8> {
    let mut r = Vec::with_capacity(HDR + body.len());
    r.extend_from_slice(&MAGIC.to_le_bytes());
    r.extend_from_slice(&1u16.to_le_bytes());
    r.extend_from_slice(&op.to_le_bytes());
    r.extend_from_slice(&0u32.to_le_bytes());
    r.extend_from_slice(&1u32.to_le_bytes());
    r.extend_from_slice(&(body.len() as u32).to_le_bytes());
    r.extend_from_slice(body);
    r
}

pub(super) fn status(rx: &[u8]) -> u16 {
    u16::from_le_bytes([rx[8], rx[9]])
}

pub(super) fn payload_len(rx: &[u8]) -> usize {
    u32::from_le_bytes([rx[16], rx[17], rx[18], rx[19]]) as usize
}

pub(super) fn lookup() -> Option<u32> {
    let name = b"net.tcp";
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        None
    } else {
        Some(port)
    }
}
