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

use super::envelope::call;

pub const RX_EMPTY: u16 = 8;

const MAGIC: u32 = 0x4E55_4450;
const BIND: u16 = 2;
const UNBIND: u16 = 3;
const SEND: u16 = 4;
const RECV: u16 = 5;

pub fn bind(port: u32, local: u16) -> Result<(), u16> {
    call(port, MAGIC, BIND, &local.to_le_bytes(), &mut []).map(|_| ())
}

pub fn unbind(port: u32, local: u16) -> Result<(), u16> {
    call(port, MAGIC, UNBIND, &local.to_le_bytes(), &mut []).map(|_| ())
}

pub fn send(port: u32, local: u16, dst: [u8; 4], dst_port: u16, payload: &[u8]) -> Result<(), u16> {
    let mut body = vec![0u8; 8 + payload.len()];
    body[0..2].copy_from_slice(&local.to_le_bytes());
    body[2..6].copy_from_slice(&dst);
    body[6..8].copy_from_slice(&dst_port.to_le_bytes());
    body[8..].copy_from_slice(payload);
    call(port, MAGIC, SEND, &body, &mut []).map(|_| ())
}

pub fn recv(port: u32, local: u16, out: &mut [u8]) -> Result<usize, u16> {
    call(port, MAGIC, RECV, &local.to_le_bytes(), out)
}
