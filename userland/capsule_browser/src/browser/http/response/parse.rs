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

use alloc::string::ToString;

use super::content_encoding::content_encoding;
use super::content_length::content_length;
use super::decode_body::decode_body;
use super::header_line;
use super::header_value::header_value;
use super::status_code::status_code;
use super::types::{ContentKind, Response};

pub fn parse(raw: &[u8]) -> Option<Response> {
    let sep = raw.windows(4).position(|w| w == b"\r\n\r\n")?;
    let head = core::str::from_utf8(&raw[..sep]).ok()?;
    let status = status_code(head.lines().next()?)?;
    let mut location = None;
    let mut chunked = false;
    let mut content_len = None;
    let mut encoding = "";
    let mut content_kind = ContentKind::Html;
    for line in head.lines().skip(1) {
        if !header_line::valid(line) {
            return None;
        }
        if let Some(v) = header_value(line, "location") {
            location = Some(v.to_string());
        } else if let Some(v) = header_value(line, "content-length") {
            content_length(&mut content_len, v)?;
        } else if let Some(v) = header_value(line, "transfer-encoding") {
            chunked = v.eq_ignore_ascii_case("chunked") || v.to_ascii_lowercase().contains("chunked");
        } else if let Some(v) = header_value(line, "content-encoding") {
            encoding = content_encoding(v)?;
        } else if let Some(v) = header_value(line, "content-type") {
            let lower = v.to_ascii_lowercase();
            content_kind = if lower.contains("text/html") || lower.contains("application/xhtml") {
                ContentKind::Html
            } else if lower.starts_with("text/") || lower.contains("application/json") {
                ContentKind::Text
            } else {
                ContentKind::Unsupported
            };
        }
    }
    let body = decode_body(&raw[sep + 4..], chunked, content_len, encoding)?;
    Some(Response { status, body, location, content_kind })
}
