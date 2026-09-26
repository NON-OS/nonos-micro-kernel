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

//! A GET, over the socket service this capsule already uses.

use alloc::vec::Vec;
use alloc::{format, string::String};

use crate::linux::net::raw::{connect_host, open_stream};
use crate::linux::net::raw_io::{close, recv_all, send_all};

/// Enough for the largest package index; a reply beyond it is refused
/// rather than truncated into a half-parsed index.
const MAX_BODY: usize = 64 << 20;

pub fn get(host: &str, port: u16, path: &str) -> Option<Vec<u8>> {
    let handle = open_stream()?;
    if connect_host(handle, host, port).is_none() {
        close(handle);
        return None;
    }
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: nonos\r\nConnection: close\r\n\r\n"
    );
    if send_all(handle, req.as_bytes()).is_none() {
        close(handle);
        return None;
    }
    let raw = recv_all(handle, MAX_BODY);
    close(handle);
    body(&raw?)
}

/// The bytes after the header block. A reply whose status is not 200 is
/// nothing: an error page parsed as a package is the worst outcome here.
fn body(raw: &[u8]) -> Option<Vec<u8>> {
    let head_end = find(raw, b"\r\n\r\n")? + 4;
    let head = String::from_utf8_lossy(&raw[..head_end]);
    let first = head.lines().next()?;
    if !first.contains(" 200 ") {
        return None;
    }
    Some(raw[head_end..].to_vec())
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}
