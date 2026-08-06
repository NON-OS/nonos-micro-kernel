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

use alloc::string::String;
use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::net;
use crate::browser::state::State;
use crate::browser::url;

const MAX_IMG_REDIRECTS: u8 = 4;

// Start the next queued image fetch when the socket is idle. Returns true if a
// fetch was launched so the run loop keeps ticking. Page navigation always
// wins the socket; this only runs once the page's own fetch has finished.
// data: sources carry their bytes inline and ingest without a socket.
pub fn pump(state: &mut State) -> bool {
    if state.sockets_port == 0 || state.fetch.is_some() {
        return false;
    }
    let Some(target) = next_pending(state) else { return false };
    if target.starts_with("data:") {
        match super::data_uri::data_uri_bytes(&target) {
            Some(bytes) => super::ingest(&mut state.images, &target, &bytes),
            None => state.images.set_failed(&target),
        }
        return true;
    }
    state.image_redirects = 0;
    if begin(state, &target, &target).is_err() {
        state.images.set_failed(&target);
    }
    true
}

// Chase a 3xx on an image fetch: the redirect target is fetched but the
// decoded pixels stay keyed to the original URL the boxes reference. Returns
// false once the hop budget is spent so the caller records a failure.
pub fn follow_redirect(state: &mut State, location: &str, key: &str) -> bool {
    if state.image_redirects >= MAX_IMG_REDIRECTS {
        return false;
    }
    state.image_redirects += 1;
    begin(state, location, key).is_ok()
}

fn next_pending(state: &mut State) -> Option<String> {
    while !state.image_queue.is_empty() {
        let u = state.image_queue.remove(0);
        if state.images.ready(&u).is_none() {
            return Some(u);
        }
    }
    None
}

fn begin(state: &mut State, target: &str, key: &str) -> Result<(), &'static str> {
    let url = url::parse(target).ok_or("bad url")?;
    // A kept-alive connection to the same host skips connect and handshake;
    // a refused reuse falls through to a fresh connection.
    if state.proxy.is_none() {
        if let Some(f) = crate::browser::fetch::try_reuse(state, &url, key) {
            state.fetch = Some(f);
            return Ok(());
        }
    }
    let proxy = state.proxy.clone();
    let (host, port) = match proxy.as_ref() {
        Some(p) => (p.host.as_str(), p.port),
        None => (url.host.as_str(), url.port),
    };
    let h = net::socket_open(state.sockets_port).map_err(|_| "socket failed")?;
    if net::socket_connect_host(state.sockets_port, h, host, port).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return Err("connect failed");
    }
    let phase = if proxy.is_some() || crate::browser::net::mixnet::is_on() {
        Phase::SocksHello
    } else if url.scheme == url::Scheme::Https {
        Phase::TlsHello
    } else {
        Phase::SendReq
    };
    state.fetch = Some(Fetch {
        url,
        handle: h,
        phase,
        buf: Vec::new(),
        socks: Vec::new(),
        tls: None,
        idle: 0,
        started_ms: mk_time_millis(),
        error: None,
        suppress: true,
        image: Some(String::from(key)),
        last_check: 0,
        post: None,
        js_req: false,
        css: false,
        rx_consumed: 0,
        tx_seq: 0,
        keep_uses: 0,
        font: 0,
        script: false,
    });
    Ok(())
}
