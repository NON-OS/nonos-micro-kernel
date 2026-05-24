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

use crate::directory_sync::DirectorySource;

pub fn build(source: &DirectorySource) -> Vec<u8> {
    let mut req = Vec::with_capacity(source.host.len() + source.path.len() + 96);
    req.extend_from_slice(b"GET ");
    req.extend_from_slice(&source.path);
    req.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    req.extend_from_slice(&source.host);
    req.extend_from_slice(b"\r\nAccept: application/octet-stream\r\n");
    req.extend_from_slice(b"Connection: close\r\n\r\n");
    req
}
