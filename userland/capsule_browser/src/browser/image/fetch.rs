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
// Images fetched at once. Enough to hide per-request latency on an image-heavy
// page without opening an unbounded number of sockets.
const MAX_CONCURRENT: usize = 6;

// Fill the concurrent image pool from the queue while a socket slot is free.
// data: sources carry their bytes inline and ingest without a socket. Returns
// true if the pool advanced so the run loop keeps ticking.
pub fn pump(state: &mut State) -> bool {
    if state.sockets_port == 0 {
        return false;
    }
    let mut did = false;
    while state.img_fetches.len() < MAX_CONCURRENT {
        let Some(target) = next_pending(state) else { break };
        did = true;
        if target.starts_with("data:") {
            match super::data_uri::data_uri_bytes(&target) {
                Some(bytes) => super::ingest(&mut state.images, &target, &bytes),
                None => state.images.set_failed(&target),
            }
            continue;
        }
        match begin(state, &target, &target, 0) {
            Some(f) => state.img_fetches.push(f),
            None => state.images.set_failed(&target),
        }
    }
    did
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

// Open a connection for one image and return the fetch to add to the pool. The
// decoded pixels stay keyed to `key` even when `target` is a redirect target.
pub(crate) fn begin(state: &State, target: &str, key: &str, redirects: u8) -> Option<Fetch> {
    let url = url::parse(target)?;
    let proxy = state.proxy.clone();
    let (host, port) = match proxy.as_ref() {
        Some(p) => (p.host.as_str(), p.port),
        None => (url.host.as_str(), url.port),
    };
    let h = net::socket_open(state.sockets_port).ok()?;
    if net::socket_connect_host(state.sockets_port, h, host, port).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return None;
    }
    let phase = if proxy.is_some() {
        Phase::SocksHello
    } else if url.scheme == url::Scheme::Https {
        Phase::TlsHello
    } else {
        Phase::SendReq
    };
    Some(Fetch {
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
        redirects,
    })
}

pub(crate) const REDIRECT_LIMIT: u8 = MAX_IMG_REDIRECTS;
