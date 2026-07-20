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

//! The message a SOCKS5 connection carries through the Nym mixnet to a network
//! requester (the exit gateway that makes the real TCP connection and returns the
//! data through single-use reply blocks). This is the Nym `Socks5Request` and
//! `Socks5Response` wire format, protocol version 3, reimplemented so a live Nym
//! exit reads our traffic:
//!
//!   request  = [version 3][flag][body]
//!     Connect (flag 0): conn_id(u64 be) | addr_len(u16 be) | "host:port" | []
//!     Send    (flag 1): conn_id(u64 be) | local_closed(1) | seq(u64 be) | data
//!   response = [version 3][flag][body]
//!     NetworkData (flag 1): conn_id(u64 be) | local_closed(1) | seq(u64 be) | data
//!     ConnectionError (flag 2): conn_id(u64 be) | utf8 message
//!
//! The connect request carries no return address: replies come back anonymously
//! through reply blocks the mixnet client attaches when it sends, so the exit
//! never learns who asked. The `seq` numbers let both ends reassemble a stream
//! that the mixnet may reorder. Every encoder is bounds-checked and every decoder
//! rejects a short or unknown message rather than trusting it.

use crate::conn::Dest;

/// Nym request/response protocol version this speaks.
pub const PROTOCOL_VERSION: u8 = 3;

/// Request that the exit open a TCP connection to the named host.
pub const REQ_CONNECT: u8 = 0;
/// Request that the exit forward stream bytes (or, with the closed flag, that it
/// close its half of the connection).
pub const REQ_SEND: u8 = 1;

/// A response carrying stream bytes back from the exit.
pub const RESP_NETWORK_DATA: u8 = 1;
/// A response reporting that the exit could not or can no longer serve the
/// connection.
pub const RESP_CONNECTION_ERROR: u8 = 2;

/// Encode a connect request naming `dest`, with no return address (anonymous,
/// reply-block replies). Returns the byte count, or `None` if `out` is too small
/// or the rendered address does not fit. rtw88 has nothing to do with this; the
/// address is rendered as the ASCII `host:port` a Nym exit parses.
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

/// Encode a send request carrying `data` for `conn_id` at stream position `seq`.
/// `closed` marks the client's half of the stream as finished (an empty closing
/// send is how a connection is torn down). Returns the byte count, or `None` if
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

/// A response from the exit: which connection it belongs to, its stream position,
/// whether the remote closed, and the bytes to hand back to the SOCKS5 client. A
/// connection error decodes as a closed connection with no data.
pub struct Response<'a> {
    pub conn_id: u64,
    pub seq: u64,
    pub closed: bool,
    pub data: &'a [u8],
}

/// Decode a `Socks5Response` from the exit, or `None` if the version is not the
/// one we speak, the flag is unknown, or the body is too short for its fields.
pub fn decode_response(buf: &[u8]) -> Option<Response<'_>> {
    if buf.len() < 2 || buf[0] != PROTOCOL_VERSION {
        return None;
    }
    let body = &buf[2..];
    match buf[1] {
        RESP_NETWORK_DATA => {
            // conn_id(8) | local_closed(1) | seq(8) | data
            if body.len() < 17 {
                return None;
            }
            let conn_id = u64::from_be_bytes(body[0..8].try_into().ok()?);
            let closed = body[8] != 0;
            let seq = u64::from_be_bytes(body[9..17].try_into().ok()?);
            Some(Response { conn_id, seq, closed, data: &body[17..] })
        }
        RESP_CONNECTION_ERROR => {
            // conn_id(8) | utf8 message. The message is not stream data, so it is
            // reported as a close with an empty payload.
            if body.len() < 8 {
                return None;
            }
            let conn_id = u64::from_be_bytes(body[0..8].try_into().ok()?);
            Some(Response { conn_id, seq: 0, closed: true, data: &[] })
        }
        _ => None,
    }
}

/// Render `dest` as the ASCII `host:port` string a Nym exit parses as its remote
/// address (IPv6 bracketed, full groups). Returns the byte count, or `None` if it
/// does not fit `out`.
fn write_hostport(dest: &Dest, out: &mut [u8]) -> Option<usize> {
    let mut w = Writer { out, pos: 0 };
    match dest {
        Dest::V4(a, port) => {
            for (i, b) in a.iter().enumerate() {
                if i > 0 {
                    w.byte(b'.')?;
                }
                w.dec(*b as u16)?;
            }
            w.byte(b':')?;
            w.dec(*port)?;
        }
        Dest::V6(a, port) => {
            w.byte(b'[')?;
            for i in 0..8 {
                if i > 0 {
                    w.byte(b':')?;
                }
                let group = ((a[i * 2] as u16) << 8) | a[i * 2 + 1] as u16;
                w.hex(group)?;
            }
            w.byte(b']')?;
            w.byte(b':')?;
            w.dec(*port)?;
        }
        Dest::Domain { name, len, port } => {
            w.bytes(&name[..*len as usize])?;
            w.byte(b':')?;
            w.dec(*port)?;
        }
    }
    Some(w.pos)
}

/// A bounds-checked cursor for rendering an address into a caller buffer without
/// allocating.
struct Writer<'a> {
    out: &'a mut [u8],
    pos: usize,
}

impl Writer<'_> {
    fn byte(&mut self, b: u8) -> Option<()> {
        *self.out.get_mut(self.pos)? = b;
        self.pos += 1;
        Some(())
    }

    fn bytes(&mut self, bs: &[u8]) -> Option<()> {
        self.out.get_mut(self.pos..self.pos + bs.len())?.copy_from_slice(bs);
        self.pos += bs.len();
        Some(())
    }

    // Write `v` in decimal with no leading zeros.
    fn dec(&mut self, v: u16) -> Option<()> {
        let mut buf = [0u8; 5];
        let mut i = buf.len();
        let mut n = v;
        loop {
            i -= 1;
            buf[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        self.bytes(&buf[i..])
    }

    // Write `v` as lowercase hex with no leading zeros.
    fn hex(&mut self, v: u16) -> Option<()> {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut buf = [0u8; 4];
        let mut i = buf.len();
        let mut n = v;
        loop {
            i -= 1;
            buf[i] = DIGITS[(n & 0xF) as usize];
            n >>= 4;
            if n == 0 {
                break;
            }
        }
        self.bytes(&buf[i..])
    }
}
