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

use super::number;

pub fn body(resp: &[u8]) -> Result<Vec<u8>, u16> {
    let Some(off) = header_end(resp) else {
        return Err(20);
    };
    if !ok_status(&resp[..off]) {
        return Err(20);
    }
    let payload = &resp[off..];
    let n = content_length(&resp[..off]).unwrap_or(payload.len());
    if n == 0 || payload.len() < n {
        return Err(20);
    }
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&payload[..n]);
    Ok(out)
}

pub fn content_length(headers: &[u8]) -> Option<usize> {
    for line in headers.split(|b| *b == b'\n') {
        let line = trim_cr(line);
        if prefixed(line, b"content-length:") {
            return number::decimal(number::trim_left(&line[15..]));
        }
    }
    None
}

fn header_end(resp: &[u8]) -> Option<usize> {
    resp.windows(4).position(|w| w == b"\r\n\r\n").map(|i| i + 4)
}

fn ok_status(headers: &[u8]) -> bool {
    headers.starts_with(b"HTTP/1.1 200") || headers.starts_with(b"HTTP/1.0 200")
}

fn prefixed(line: &[u8], prefix: &[u8]) -> bool {
    line.len() >= prefix.len()
        && line[..prefix.len()].iter().zip(prefix).all(|(a, b)| a.to_ascii_lowercase() == *b)
}

fn trim_cr(line: &[u8]) -> &[u8] {
    line.strip_suffix(b"\r").unwrap_or(line)
}
