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

use crate::browser::layout;
use crate::browser::net;
use crate::browser::state::State;
use crate::browser::url::{self, Scheme};
use crate::browser::{html, http};

pub fn load(state: &mut State, target: &str) {
    if state.sockets_port == 0 {
        state.sockets_port = net::lookup(b"net.sockets");
        state.dns_port = net::lookup(b"net.dns");
    }
    match fetch(state, target) {
        Ok(()) => {}
        Err(msg) => {
            state.status = alloc::string::String::from(msg);
            state.document = None;
        }
    }
}

fn fetch(state: &mut State, target: &str) -> Result<(), &'static str> {
    let url = url::parse(target).ok_or("bad url")?;
    if url.scheme == Scheme::Https {
        return Err("https not supported yet (P2)");
    }
    let ip = net::resolve(state.dns_port, url.host.as_bytes()).map_err(|_| "dns failed")?;
    let h = net::socket_open(state.sockets_port).map_err(|_| "socket failed")?;
    let result = transact(state, h, &url, ip);
    let _ = net::socket_close(state.sockets_port, h);
    let raw = result?;
    let resp = http::response::parse(&raw).ok_or("bad response")?;
    let flows = html::parse::parse(&resp.body);
    let doc = layout::build(&flows, crate::browser::manifest::WIDTH, 8);
    state.scroll = 0;
    state.status = alloc::format!("{} ({} bytes)", resp.status, resp.body.len());
    state.document = Some(doc);
    Ok(())
}

fn transact(
    state: &mut State,
    h: u32,
    url: &url::Url,
    ip: [u8; 4],
) -> Result<alloc::vec::Vec<u8>, &'static str> {
    net::socket_connect(state.sockets_port, h, ip, url.port).map_err(|_| "connect failed")?;
    let req = http::request::build(url);
    net::socket_send(state.sockets_port, h, req.as_bytes()).map_err(|_| "send failed")?;
    net::recv_all(state.sockets_port, h, 4 * 1024 * 1024).map_err(|_| "recv failed")
}
