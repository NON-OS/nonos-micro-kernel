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

use super::constants::*;
use crate::clients::envelope::call;

pub fn send(port: u32, session: u32, payload: &[u8]) -> Result<(), u16> {
    let mut body = vec![0u8; 4 + payload.len()];
    body[0..4].copy_from_slice(&session.to_le_bytes());
    body[4..].copy_from_slice(payload);
    call(port, MAGIC, SEND, &body, &mut []).map(|_| ())
}

pub fn recv(port: u32, session: u32, out: &mut [u8]) -> Result<usize, u16> {
    call(port, MAGIC, RECV, &session.to_le_bytes(), out)
}

pub fn cover(port: u32, session: u32) -> Result<(), u16> {
    call(port, MAGIC, COVER, &session.to_le_bytes(), &mut []).map(|_| ())
}
