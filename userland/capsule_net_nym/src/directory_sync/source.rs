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

const MAX_HOST: usize = 96;
const MAX_PATH: usize = 96;

#[derive(Clone)]
pub struct DirectorySource {
    pub ip: [u8; 4],
    pub port: u16,
    pub host: Vec<u8>,
    pub path: Vec<u8>,
}

pub fn parse(body: &[u8]) -> Result<DirectorySource, u16> {
    if body.len() < 8 {
        return Err(4);
    }
    let host_len = body[6] as usize;
    let path_len = body[7] as usize;
    if bad_lengths(body, host_len, path_len) {
        return Err(4);
    }
    let mut ip = [0u8; 4];
    ip.copy_from_slice(&body[..4]);
    let port = u16::from_le_bytes([body[4], body[5]]);
    if port == 0 {
        return Err(4);
    }
    Ok(source(ip, port, &body[8..8 + host_len], &body[8 + host_len..]))
}

fn bad_lengths(body: &[u8], host_len: usize, path_len: usize) -> bool {
    host_len == 0
        || host_len > MAX_HOST
        || path_len == 0
        || path_len > MAX_PATH
        || body.len() != 8 + host_len + path_len
        || body[8 + host_len] != b'/'
}

fn source(ip: [u8; 4], port: u16, host_src: &[u8], path_src: &[u8]) -> DirectorySource {
    let mut host = Vec::with_capacity(host_src.len());
    let mut path = Vec::with_capacity(path_src.len());
    host.extend_from_slice(host_src);
    path.extend_from_slice(path_src);
    DirectorySource { ip, port, host, path }
}
