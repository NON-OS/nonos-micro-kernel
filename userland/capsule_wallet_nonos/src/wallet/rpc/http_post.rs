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

pub fn http_post(host: &[u8], body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(host.len() + body.len() + 160);
    out.extend_from_slice(b"POST / HTTP/1.1\r\nHost: ");
    out.extend_from_slice(host);
    out.extend_from_slice(b"\r\nContent-Type: application/json\r\nAccept: application/json\r\nContent-Length: ");
    super::append_dec_u64::append_dec_u64(&mut out, body.len() as u64);
    out.extend_from_slice(b"\r\nConnection: close\r\n\r\n");
    out.extend_from_slice(body);
    out
}
