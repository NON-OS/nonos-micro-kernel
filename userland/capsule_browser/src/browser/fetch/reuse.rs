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
use crate::browser::state::State;
use crate::browser::url::{Scheme, Url};
use crate::browser::{http, net, tls13};

// Send an image request over the kept-alive connection when it matches the
// target host, skipping connect and handshake entirely. Anything off about
// the kept connection closes it and returns None, so the caller falls back
// to a fresh connection and the worst case is exactly the old behaviour.
pub(crate) fn try_reuse(state: &mut State, url: &Url, key: &str) -> Option<Fetch> {
    let keep = state.keep.take()?;
    if keep.host != url.host || keep.port != url.port || url.scheme != Scheme::Https {
        let _ = net::socket_close(state.sockets_port, keep.handle);
        return None;
    }
    let sealed = keep
        .tls
        .server_app
        .as_ref()
        .and_then(|app| {
            let req = http::request::build_keep_alive(url);
            tls13::application_request(app, keep.tx_seq, req.as_bytes())
        })
        .filter(|rec| net::socket_send(state.sockets_port, keep.handle, rec).is_ok());
    if sealed.is_none() {
        let _ = net::socket_close(state.sockets_port, keep.handle);
        return None;
    }
    Some(Fetch {
        url: url.clone(),
        handle: keep.handle,
        phase: Phase::ReadBody,
        buf: keep.buf,
        socks: Vec::new(),
        tls: Some(keep.tls),
        idle: 0,
        started_ms: mk_time_millis(),
        error: None,
        suppress: true,
        image: Some(String::from(key)),
        last_check: 0,
        post: None,
        js_req: false,
        css: false,
        rx_consumed: keep.consumed,
        tx_seq: keep.tx_seq,
        keep_uses: keep.used,
    })
}
