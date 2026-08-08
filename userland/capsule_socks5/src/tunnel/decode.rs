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

use super::constants::{PROTOCOL_VERSION, RESP_CONNECTION_ERROR, RESP_NETWORK_DATA};
use super::provider::inner_response;

/// A response from the exit: which connection it belongs to, its stream
/// position, whether the remote closed, and the bytes for the SOCKS5 client.
/// A connection error decodes as a close with no data.
pub struct Response<'a> {
    pub conn_id: u64,
    pub seq: u64,
    pub closed: bool,
    pub data: &'a [u8],
}

/// Decode a `Socks5Response`, or `None` when the version is not the one we
/// speak, the flag is unknown, or the body is too short for its fields.
pub fn decode_response(buf: &[u8]) -> Option<Response<'_>> {
    let raw = inner_response(buf)?;
    if raw.len() < 2 || raw[0] != PROTOCOL_VERSION {
        return None;
    }
    let body = &raw[2..];
    match raw[1] {
        RESP_NETWORK_DATA => {
            // local_closed(1) | conn_id(8) | seq(8) | data
            //
            // The same three fields travel in a different order each way: a
            // request leads with the connection id, a response leads with the
            // close flag. Reading a response the way a request is written
            // shifts the id by a byte, so the stream is filed against a
            // connection nobody is waiting on and the reader is never handed
            // anything.
            if body.len() < 17 {
                return None;
            }
            let closed = body[0] != 0;
            let conn_id = u64::from_be_bytes(body[1..9].try_into().ok()?);
            let seq = u64::from_be_bytes(body[9..17].try_into().ok()?);
            Some(Response { conn_id, seq, closed, data: &body[17..] })
        }
        RESP_CONNECTION_ERROR => {
            // conn_id(8) | utf8 message. The message is not stream data, so it
            // is reported as a close with an empty payload.
            if body.len() < 8 {
                return None;
            }
            let conn_id = u64::from_be_bytes(body[0..8].try_into().ok()?);
            Some(Response { conn_id, seq: 0, closed: true, data: &[] })
        }
        _ => None,
    }
}
