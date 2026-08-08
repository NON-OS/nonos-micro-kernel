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
//! Malformed responses a client has to refuse rather than guess at.

use nonos_http::{parse_response, HttpError};

#[test]
fn a_response_without_a_blank_line_is_incomplete() {
    assert_eq!(parse_response(b"HTTP/1.1 200 OK\r\n").err(), Some(HttpError::Incomplete));
}

#[test]
fn a_bad_status_line_is_refused() {
    assert_eq!(parse_response(b"HTTP/1.1 2x0 OK\r\n\r\n").err(), Some(HttpError::StatusLine));
    assert_eq!(parse_response(b"NOTHTTP 200 OK\r\n\r\n").err(), Some(HttpError::StatusLine));
}

#[test]
fn a_header_line_without_a_colon_is_refused() {
    let raw = b"HTTP/1.1 200 OK\r\nnonsense\r\n\r\n";
    assert_eq!(parse_response(raw).err(), Some(HttpError::Header));
}

#[test]
fn a_chunk_size_that_lies_is_refused() {
    // The size says forty bytes follow and only five do.
    let raw = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n28\r\nshort";
    assert_eq!(parse_response(raw).err(), Some(HttpError::Body));
}

#[test]
fn a_chunk_size_that_is_not_hex_is_refused() {
    let raw = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nzz\r\nab\r\n0\r\n\r\n";
    assert_eq!(parse_response(raw).err(), Some(HttpError::Chunk));
}

#[test]
fn chunk_extensions_are_ignored_rather_than_refused() {
    // A size line may carry extensions after a semicolon. They are not for us,
    // but a response carrying them is still valid and must decode.
    let raw = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n3;name=v\r\nabc\r\n0\r\n\r\n";
    assert_eq!(parse_response(raw).expect("parse").body, b"abc");
}

#[test]
fn an_unterminated_chunked_body_is_refused() {
    let raw = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n3\r\nabc\r\n";
    assert_eq!(parse_response(raw).err(), Some(HttpError::Chunk));
}
