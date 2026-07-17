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

use super::scan;

pub struct Parsed {
    pub status: u16,
    pub body_off: usize,
    pub content_length: Option<usize>,
    pub chunked: bool,
    pub gzip: bool,
}

pub fn build_get(host: &[u8], path: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(b"GET ");
    r.extend_from_slice(path);
    r.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    r.extend_from_slice(host);
    r.extend_from_slice(b"\r\nConnection: close\r\nUser-Agent: nonos-nox\r\nAccept-Encoding: gzip\r\n\r\n");
    r
}

pub fn build_get_ka(host: &[u8], path: &[u8]) -> Vec<u8> {
    let mut r = Vec::new();
    r.extend_from_slice(b"GET ");
    r.extend_from_slice(path);
    r.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    r.extend_from_slice(host);
    r.extend_from_slice(b"\r\nConnection: keep-alive\r\nUser-Agent: nonos-nox\r\nAccept-Encoding: gzip\r\n\r\n");
    r
}

pub fn parse_head(buf: &[u8]) -> Option<Parsed> {
    let sep = scan::find(buf, b"\r\n\r\n")?;
    let head = &buf[..sep];
    let mut lines = scan::split_lines(head).into_iter();
    let status = parse_status(lines.next()?)?;
    let mut content_length = None;
    let mut chunked = false;
    let mut gzip = false;
    for line in lines {
        if let Some(v) = header(line, b"content-length") {
            content_length = scan::parse_usize(v);
        } else if let Some(v) = header(line, b"transfer-encoding") {
            chunked = scan::find(v, b"chunked").is_some();
        } else if let Some(v) = header(line, b"content-encoding") {
            gzip = scan::find(v, b"gzip").is_some();
        }
    }
    Some(Parsed { status, body_off: sep + 4, content_length, chunked, gzip })
}

fn parse_status(line: &[u8]) -> Option<u16> {
    let mut parts = line.split(|&c| c == b' ');
    let _http = parts.next()?;
    let code = parts.next()?;
    scan::parse_usize(code).map(|n| n as u16)
}

fn header<'a>(line: &'a [u8], name: &[u8]) -> Option<&'a [u8]> {
    let colon = line.iter().position(|&c| c == b':')?;
    if scan::eq_ci(&line[..colon], name) {
        Some(&line[colon + 1..])
    } else {
        None
    }
}
