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

// Total bytes the response at the head of `raw` occupies, headers included,
// once it is complete. On a kept-alive connection this is where the next
// response begins. None until the framing is satisfied or when the response
// has no self-delimiting framing at all.
pub fn frame_len(raw: &[u8]) -> Option<usize> {
    let sep = raw.windows(4).position(|w| w == b"\r\n\r\n")?;
    let head = core::str::from_utf8(&raw[..sep]).ok()?;
    let body_at = sep + 4;
    let mut content_len = None;
    let mut chunked = false;
    for line in head.lines().skip(1) {
        let lower = line.to_ascii_lowercase();
        if let Some(v) = lower.strip_prefix("content-length:") {
            content_len = v.trim().parse::<usize>().ok();
        }
        if lower.starts_with("transfer-encoding:") && lower.contains("chunked") {
            chunked = true;
        }
    }
    if chunked {
        return chunked_end(&raw[body_at..]).map(|n| body_at + n);
    }
    let n = content_len?;
    if raw.len() >= body_at + n {
        Some(body_at + n)
    } else {
        None
    }
}

// Bytes a complete chunked body spans, terminator included.
fn chunked_end(body: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    loop {
        let rel = crate::browser::http::chunked::find_crlf(&body[i..])?;
        let size = crate::browser::http::chunked::parse_hex(&body[i..i + rel])?;
        i += rel + 2;
        if size == 0 {
            // Trailer section ends at the next blank line.
            if body.get(i..i + 2) == Some(b"\r\n") {
                return Some(i + 2);
            }
            let end = body[i..].windows(4).position(|w| w == b"\r\n\r\n")?;
            return Some(i + end + 4);
        }
        // `size` is an attacker-supplied hex chunk length up to usize::MAX, so
        // the index math must not overflow (release builds check overflow and
        // abort). Fold the bound into checked arithmetic; any overflow is just
        // a body that does not fit and fails the frame.
        let after = i.checked_add(size).and_then(|v| v.checked_add(2))?;
        if body.len() < after || body.get(after - 2..after) != Some(b"\r\n") {
            return None;
        }
        i = after;
    }
}
