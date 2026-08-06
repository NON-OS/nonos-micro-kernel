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

// Fetch the next queued external script when the socket is idle. Rides the same
// single-socket machine as navigation, stylesheets and images, so only one
// request runs at a time and scripts arrive in the order they were queued,
// which is document order. Returns true when a fetch was launched.
pub fn script_pump(state: &mut State) -> bool {
    if state.sockets_port == 0 || state.fetch.is_some() || state.script_queue.is_empty() {
        return false;
    }
    // A script whose fetch cannot even launch is dropped; the page keeps
    // whatever the other scripts built.
    let target = state.script_queue.remove(0);
    let Some(u) = url::parse(&target) else {
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
        js_req: false,
        css: false,
        rx_consumed: 0,
        tx_seq: 0,
        keep_uses: 0,
        font: 0,
        script: true,
    });
    true
}
