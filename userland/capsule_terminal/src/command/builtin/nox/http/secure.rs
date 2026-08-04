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

use crate::mixnet::Wire;
use nonos_http::{parse_response, RequestBuilder, Response};
use nonos_tls::{exchange, rtc_now};

use super::io::SocketIo;
use super::url::Url;

const MAX_BODY: usize = 64 * 1024;
const AGENT: &str = "nonos-terminal";

/// TLS 1.3, with the certificate chain checked against the built-in roots
/// before the request is written. A chain that does not verify aborts the
/// request rather than sending it to whoever answered.
pub fn get(url: &Url) -> Result<Response, &'static str> {
    let stream = Wire::connect(&url.host, url.port).map_err(|_| "connect failed")?;
    let mut io = SocketIo::new(stream);
    let request = RequestBuilder::get(&url.host, &url.path).user_agent(AGENT).build();
    let raw = exchange(&mut io, &url.host, &request.bytes, rtc_now(), MAX_BODY)
        .map_err(|_| "tls handshake failed")?;
    parse_response(&raw).map_err(|_| "malformed response")
}
