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

pub fn decode_body(
    raw_body: &[u8],
    chunked: bool,
    content_len: Option<usize>,
    encoding: &str,
) -> Option<Vec<u8>> {
    let body = if chunked {
        crate::browser::http::chunked::decode(raw_body)?
    } else if let Some(n) = content_len {
        raw_body.get(..n)?.to_vec()
    } else {
        raw_body.to_vec()
    };
    match encoding {
        "gzip" => nonos_inflate::gunzip(&body),
        "deflate" => nonos_inflate::zlib(&body).or_else(|| nonos_inflate::inflate(&body)),
        _ => Some(body),
    }
}
