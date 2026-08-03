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

use nonos_libc::mk_time_millis;

use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::net;
use crate::browser::state::State;
use crate::browser::url;

// Start the next script-issued request when the socket is idle. Page
// navigation and its callback delivery both ride the same single-socket
// machine, so only one script request runs at a time.
pub fn js_pump(state: &mut State) -> bool {
    if state.sockets_port == 0 || state.fetch.is_some() {
        return false;
    }
    let (target, cb) = {
        let Some(world) = state.world.as_mut() else {
            return false;
        };
        if world.net.is_empty() || world.net_active.is_some() {
            return false;
        }
        world.net.remove(0)
    };
    let abs = match state.base.as_ref() {
        Some(b) => url::join(b, &target),
        None => target,
    };
    let Some(u) = url::parse(&abs) else {
        return true;
    };
    let proxy = state.proxy.clone();
    let (host, port) = match proxy.as_ref() {
        Some(p) => (p.host.as_str(), p.port),
        None => (u.host.as_str(), u.port),
    };
    let Ok(h) = net::socket_open(state.sockets_port) else {
        return true;
    };
    if net::socket_connect_host(state.sockets_port, h, host, port).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return true;
    }
    let phase = if proxy.is_some() || crate::browser::net::mixnet::is_on() {
        Phase::SocksHello
    } else if u.scheme == url::Scheme::Https {
        Phase::TlsHello
    } else {
        Phase::SendReq
    };
    state.fetch = Some(Fetch {
        url: u,
        handle: h,
        phase,
        buf: Vec::new(),
        socks: Vec::new(),
        tls: None,
        idle: 0,
        started_ms: mk_time_millis(),
        error: None,
        suppress: true,
        image: None,
        last_check: 0,
        post: None,
        js_req: true,
        css: false,
        rx_consumed: 0,
        tx_seq: 0,
        keep_uses: 0,
        font: 0,
        script: false,
    });
    if let Some(world) = state.world.as_mut() {
        world.net_active = Some(cb);
    }
    true
}
