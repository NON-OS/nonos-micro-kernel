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
//! Responses, parsed from what a server actually sends.
//!
//! `live_chunked.bin` is the raw TLS payload github.com returned for
//! `GET /octocat/Hello-World.git/info/refs`, captured off the wire rather
//! than through a client that would have decoded it: the header block, the
//! first chunk exactly as GitHub framed it, and a terminating zero chunk.

use nonos_http::{parse_response, HttpError};

const LIVE: &[u8] = include_bytes!("data/live_chunked.bin");

#[test]
fn a_real_chunked_response_decodes() {
    let r = parse_response(LIVE).expect("real response must parse");
    assert_eq!(r.status, 200);
    assert_eq!(r.header("content-type"), Some("application/x-git-upload-pack-advertisement"));
    assert_eq!(r.header("transfer-encoding"), Some("chunked"));
    // The chunk header said 0x36BE, and the framing is stripped from the body.
    assert_eq!(r.body.len(), 0x36BE);
    assert!(r.body.starts_with(b"001e# service=git-upload-pack\n"));
}

#[test]
fn header_names_are_matched_without_case() {
    let r = parse_response(LIVE).expect("parse");
    // GitHub sends Content-Type capitalised and expires lowercase.
    assert!(r.header("content-type").is_some());
    assert!(r.header("expires").is_some());
    assert_eq!(r.header("Content-Type"), None);
}

#[test]
fn a_length_bodied_response_is_cut_to_that_length() {
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhellotrailing junk";
    let r = parse_response(raw).expect("parse");
    assert_eq!(r.body, b"hello");
}

#[test]
fn a_short_body_is_an_error_rather_than_a_truncation() {
    // Handing back what arrived would pass a truncated pack to the caller.
    let raw = b"HTTP/1.1 200 OK\r\nContent-Length: 40\r\n\r\nshort";
    assert_eq!(parse_response(raw).err(), Some(HttpError::Body));
}
