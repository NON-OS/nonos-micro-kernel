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

use super::constants::{PROTOCOL_VERSION, REQ_CONNECT, REQ_SEND};
use super::hostport::write_hostport;
use crate::conn::Dest;

/// Encode a connect request naming `dest`, with no return address: replies
/// come back through reply blocks, so nothing here identifies the sender.
/// Returns the byte count, or `None` if `out` is too small for the address.
pub fn encode_connect(conn_id: u64, dest: &Dest, out: &mut [u8]) -> Option<usize> {
    // [version][flag][conn_id:8][addr_len:2][addr]
    if out.len() < 2 + 8 + 2 {
        return None;
    }
    out[0] = PROTOCOL_VERSION;
    out[1] = REQ_CONNECT;
    out[2..10].copy_from_slice(&conn_id.to_be_bytes());
    let addr_len = write_hostport(dest, out.get_mut(12..)?)?;
    out[10..12].copy_from_slice(&(addr_len as u16).to_be_bytes());
    Some(12 + addr_len)
}

/// Encode a send request carrying `data` for `conn_id` at stream position
/// `seq`. `closed` marks our half of the stream finished, and an empty closing
/// send is how a connection is torn down. Returns the byte count, or `None` if
/// `out` is too small.
pub fn encode_send(
    conn_id: u64,
    seq: u64,
    closed: bool,
    data: &[u8],
    out: &mut [u8],
) -> Option<usize> {
    // [version][flag][conn_id:8][closed:1][seq:8][data]
    let total = 2 + 8 + 1 + 8 + data.len();
    if out.len() < total {
        return None;
    }
    out[0] = PROTOCOL_VERSION;
    out[1] = REQ_SEND;
    out[2..10].copy_from_slice(&conn_id.to_be_bytes());
    out[10] = closed as u8;
    out[11..19].copy_from_slice(&seq.to_be_bytes());
    out[19..total].copy_from_slice(data);
    Some(total)
}
