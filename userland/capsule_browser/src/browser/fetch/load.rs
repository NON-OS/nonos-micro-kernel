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
use crate::browser::state::{State, View};
use crate::browser::url;

pub fn load(state: &mut State, target: &str) -> Result<(), &'static str> {
    if super::about_page::about_page(state, target) {
        return Ok(());
    }
    let url = url::parse(target).ok_or("bad url")?;
    let proxy = state.proxy.clone();
    let (connect_host, connect_port) = match proxy.as_ref() {
        Some(p) => (p.host.as_str(), p.port),
        None => (url.host.as_str(), url.port),
    };
    super::services::services(state)?;
    state.base = Some(url.clone());
    let h = net::socket_open(state.sockets_port).map_err(|_| "socket failed")?;
    if net::socket_connect_host(state.sockets_port, h, connect_host, connect_port).is_err() {
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
    state.status = alloc::format!("loading {}", url.host);
    state.document = None;
    state.box_doc = None;
    state.engine = None;
    state.page_dom = None;
    state.world = None;
    state.css_queue.clear();
    state.script_queue.clear();
    state.page_css.clear();
    state.image_queue.clear();
    state.images.reset();
    if let Some(keep) = state.keep.take() {
        let _ = crate::browser::net::socket_close(state.sockets_port, keep.handle);
    }
    state.font_queue.clear();
    state.font_seen.clear();
    crate::browser::fonts::clear();
    state.css_cache = None;
    state.scroll = 0;
    state.focus = None;
    state.view = View::Page;
    let suppress = core::mem::take(&mut state.suppress_history_push);
    let post = core::mem::take(&mut state.pending_post);
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
        suppress,
        image: None,
        last_check: 0,
        post,
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
