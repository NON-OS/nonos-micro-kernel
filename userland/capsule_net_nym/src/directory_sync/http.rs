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

mod number;
pub(super) mod parse;
mod read;
mod request;

use alloc::vec::Vec;

use super::source::DirectorySource;
use crate::tcp_client;

pub fn fetch(tcp_port: u32, source: &DirectorySource) -> Result<Vec<u8>, u16> {
    let stream = tcp_client::connect(tcp_port, source.ip, source.port)?;
    // connect returns as soon as the SYN is queued, so a write issued
    // straight after it races the handshake and net.tcp refuses it while the
    // socket is still opening. Every node asked this way looked dead.
    tcp_client::wait_established(tcp_port, stream)?;
    let req = request::build(source);
    let sent = tcp_client::send_all(tcp_port, stream, &req);
    let read = sent.and_then(|()| read::response(tcp_port, stream));
    let close = tcp_client::close(tcp_port, stream);
    match (read, close) {
        (Ok(resp), Ok(())) => parse::body(&resp),
        (Ok(resp), Err(_)) => parse::body(&resp),
        (Err(e), _) => Err(e),
    }
}
