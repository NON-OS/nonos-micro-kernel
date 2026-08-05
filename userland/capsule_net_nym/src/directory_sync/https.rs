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

use alloc::format;
use alloc::vec::Vec;
use nonos_tls::{exchange, rtc_now};

use super::http::parse;
use super::resolve::resolve;
use super::tls_io::TcpIo;
use crate::tcp_client;

const HTTPS_PORT: u16 = 443;
/// A node list runs to a few hundred kilobytes. This bounds what the far end
/// can make the capsule allocate for one answer.
const MAX_RESPONSE: usize = 512 * 1024;

/// Fetch `path` from `host` over TLS and return the response body.
///
/// The certificate chain is checked before the request is written, so a name
/// that does not verify never sees what was being asked for. Nothing about
/// this fetch is anonymous: it happens before there is a mixnet to be
/// anonymous over.
pub fn fetch_tls(tcp_port: u32, host: &str, path: &str) -> Result<Vec<u8>, u16> {
    crate::trace::say(b"fetch: resolving");
    let ip = resolve(host.as_bytes()).ok_or(21u16)?;
    crate::trace::say(b"fetch: connecting");
    let stream = tcp_client::connect(tcp_port, ip, HTTPS_PORT)?;
    tcp_client::wait_established(tcp_port, stream)?;
    crate::trace::say(b"fetch: handshaking");

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: nonos-nym\r\nAccept: application/json\r\nConnection: close\r\n\r\n",
        path, host
    );
    let mut io = TcpIo::new(tcp_port, stream);
    let raw = exchange(&mut io, host, request.as_bytes(), rtc_now(), MAX_RESPONSE);
    let stage = match &raw {
        Ok(body) => 0u64 + body.len() as u64,
        Err(nonos_tls::SessionError::Init) => 1,
        Err(nonos_tls::SessionError::Io) => 2,
        Err(nonos_tls::SessionError::Handshake) => 3,
        Err(nonos_tls::SessionError::Certificate) => 4,
        Err(nonos_tls::SessionError::TooLarge) => 5,
    };
    crate::trace::say_two(b"fetch: ok-len-or-err", stage, io.overran() as u64);
    let _ = tcp_client::close(tcp_port, stream);
    let body = raw.map_err(|_| if io.overran() { 22u16 } else { 20u16 })?;
    // A short answer is a redirect or an error page, not a node list, and
    // parsing it would find no objects and report an empty directory rather
    // than a wrong address.
    if !status_ok(&body) {
        return Err(20);
    }
    parse::body(&body)
}

/// Whether the response line says 200. Anything else is not a node list.
fn status_ok(resp: &[u8]) -> bool {
    resp.len() > 12 && resp.starts_with(b"HTTP/1.1 200")
}
