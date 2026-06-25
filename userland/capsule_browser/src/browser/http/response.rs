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

pub fn parse(raw: &[u8]) -> Option<Response> {
    let sep = raw.windows(4).position(|w| w == b"\r\n\r\n")?;
    let head = core::str::from_utf8(&raw[..sep]).ok()?;
    let status = head.split_whitespace().nth(1)?.parse().ok()?;
    let mut location = None;
    let mut chunked = false;
    for line in head.lines().skip(1) {
        let lower = line.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("location:") {
            location = Some(v.trim().to_string());
        } else if lower.starts_with("transfer-encoding:") && lower.contains("chunked") {
            chunked = true;
        }
    }
    let raw_body = &raw[sep + 4..];
    let body = if chunked {
        super::chunked::decode(raw_body)
    } else {
        raw_body.to_vec()
    };
    Some(Response { status, body, location })
}
