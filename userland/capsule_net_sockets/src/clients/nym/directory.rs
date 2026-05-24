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

use super::constants::{MAGIC, SYNC_DIRECTORY};
use crate::clients::envelope::call;

pub fn sync_directory(
    port: u32,
    ip: [u8; 4],
    dir_port: u16,
    host: &[u8],
    path: &[u8],
) -> Result<(), u16> {
    let body = directory_body(ip, dir_port, host, path)?;
    call(port, MAGIC, SYNC_DIRECTORY, &body, &mut []).map(|_| ())
}

pub fn resync_directory(port: u32) -> Result<(), u16> {
    call(port, MAGIC, SYNC_DIRECTORY, &[], &mut []).map(|_| ())
}

fn directory_body(ip: [u8; 4], port: u16, host: &[u8], path: &[u8]) -> Result<Vec<u8>, u16> {
    if host.is_empty() || host.len() > 96 || path.is_empty() || path.len() > 96 {
        return Err(4);
    }
    let mut body = Vec::with_capacity(8 + host.len() + path.len());
    body.extend_from_slice(&ip);
    body.extend_from_slice(&port.to_le_bytes());
    body.push(host.len() as u8);
    body.push(path.len() as u8);
    body.extend_from_slice(host);
    body.extend_from_slice(path);
    Ok(body)
}
