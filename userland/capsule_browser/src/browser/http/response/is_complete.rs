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

pub fn is_complete(raw: &[u8]) -> bool {
    let Some(sep) = raw.windows(4).position(|w| w == b"\r\n\r\n") else {
        return false;
    };
    let Ok(head) = core::str::from_utf8(&raw[..sep]) else {
        return false;
    };
    let body_len = raw.len() - sep - 4;
    let mut content_len = None;
    let mut chunked = false;
    for line in head.lines().skip(1) {
        if !super::header_line::valid(line) {
            return false;
        }
        let lower = line.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            let Ok(n) = v.trim().parse::<usize>() else { return false };
            if content_len.is_some_and(|old| old != n) {
                return false;
            }
            content_len = Some(n);
        }
        if lower.starts_with("transfer-encoding:") && lower.contains("chunked") {
            chunked = true;
        }
    }
    if chunked {
        crate::browser::http::chunked::complete(&raw[sep + 4..])
    } else {
        content_len.is_some_and(|n| body_len >= n)
    }
}
