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
use core::ops::Range;

use super::http;

pub struct Framed {
    pub body: Range<usize>,
    pub gzip: bool,
    pub status: u16,
    pub location: Option<Vec<u8>>,
}

pub fn frame(raw: &[u8]) -> Result<Option<Framed>, (bool, &'static str)> {
    let head = match http::parse_head(raw) {
        Some(h) => h,
        None => return Ok(None),
    };
    if head.chunked {
        return Err((true, "keep-alive needs content-length"));
    }
    let len = match head.content_length {
        Some(l) => l,
        None => return Err((true, "keep-alive needs content-length")),
    };
    let end = head.body_off + len;
    if raw.len() < end {
        return Ok(None);
    }
    Ok(Some(Framed {
        body: head.body_off..end,
        gzip: head.gzip,
        status: head.status,
        location: head.location,
    }))
}
