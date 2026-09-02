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
    // A large node list comes back chunked, and until this stripped the framing
    // the chunk-size lines landed inside the JSON, so every record straddling a
    // chunk boundary picked up "\r\n<hex>\r\n" and dropped out of the parse.
    if transfer_encoding_chunked(&resp[..off]) {
        return dechunk(payload).ok_or(20);
    }
    let n = content_length(&resp[..off]).unwrap_or(payload.len());
    if n == 0 || payload.len() < n {
        return Err(20);
    }
    let mut out = Vec::with_capacity(n);
    out.extend_from_slice(&payload[..n]);
    Ok(out)
}

fn transfer_encoding_chunked(headers: &[u8]) -> bool {
    for line in headers.split(|b| *b == b'\n') {
        let line = trim_cr(line);
        if prefixed(line, b"transfer-encoding:") {
            let lower: Vec<u8> = line.iter().map(|c| c.to_ascii_lowercase()).collect();
            return lower.windows(7).any(|w| w == b"chunked");
        }
    }
    false
}

/// Strip chunked framing: repeated `<hex-size>CRLF <bytes> CRLF`, ended by a
/// zero-size chunk. A `;`-delimited chunk extension is skipped. Returns None on
/// a malformed frame rather than guessing a boundary.
fn dechunk(payload: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut at = 0usize;
    loop {
        let mut size = 0usize;
        let start = at;
        while at < payload.len() && payload[at] != b'\r' && payload[at] != b'\n' {
            let c = payload[at];
            let d = match c {
                b'0'..=b'9' => c - b'0',
                b'a'..=b'f' => c - b'a' + 10,
                b'A'..=b'F' => c - b'A' + 10,
                b';' => break,
                _ => return None,
            };
            size = size.checked_mul(16)?.checked_add(d as usize)?;
            at += 1;
        }
        if at == start && size == 0 && payload.get(at) != Some(&b'\r') {
            return None;
        }
        while at < payload.len() && payload[at] != b'\n' {
            at += 1;
        }
        if at < payload.len() {
            at += 1;
        }
        if size == 0 {
            return Some(out);
        }
        // `at <= payload.len()` is invariant here, so the subtraction cannot
        // underflow. Comparing this way avoids the `at + size` overflow a
        // hostile response could force with a 16-nibble size line.
        if size > payload.len() - at {
            return None;
        }
        out.extend_from_slice(&payload[at..at + size]);
        at += size;
        if payload.get(at..at + 2) == Some(b"\r\n") {
            at += 2;
        }
    }
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
