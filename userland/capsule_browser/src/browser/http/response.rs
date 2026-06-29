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

use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
    pub location: Option<String>,
}

pub fn is_complete(raw: &[u8]) -> bool {
    let Some(sep) = raw.windows(4).position(|w| w == b"\r\n\r\n") else {
        return false;
    };
    let Ok(head) = core::str::from_utf8(&raw[..sep]) else {
        return false;
    };
    let body_len = raw.len() - sep - 4;
    for line in head.lines().skip(1) {
        let lower = line.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            return v.trim().parse::<usize>().map_or(false, |n| body_len >= n);
        }
        if lower.starts_with("transfer-encoding:") && lower.contains("chunked") {
            return raw.ends_with(b"0\r\n\r\n");
        }
    }
    false
}

pub fn parse(raw: &[u8]) -> Option<Response> {
    let sep = raw.windows(4).position(|w| w == b"\r\n\r\n")?;
    let head = core::str::from_utf8(&raw[..sep]).ok()?;
    let status = head.split_whitespace().nth(1)?.parse().ok()?;
    let mut location = None;
    let mut chunked = false;
    let mut encoding = "";
    for line in head.lines().skip(1) {
        let lower = line.to_ascii_lowercase();
        if lower.starts_with("location:") {
            location = Some(line[line.find(':').map(|c| c + 1).unwrap_or(line.len())..].trim().to_string());
        } else if lower.starts_with("transfer-encoding:") && lower.contains("chunked") {
            chunked = true;
        } else if lower.starts_with("content-encoding:") {
            if lower.contains("gzip") { encoding = "gzip"; }
            else if lower.contains("deflate") { encoding = "deflate"; }
        }
    }
    let raw_body = &raw[sep + 4..];
    let body = if chunked {
        super::chunked::decode(raw_body)
    } else {
        raw_body.to_vec()
    };
    let body = match encoding {
        "gzip" => super::inflate::gunzip(&body).unwrap_or_default(),
        "deflate" => super::inflate::zlib(&body).or_else(|| super::inflate::inflate(&body)).unwrap_or_default(),
        _ => body,
    };
    Some(Response { status, body, location })
}
