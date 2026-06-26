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

use crate::browser::layout;
use crate::browser::net;
use crate::browser::state::{FetchJob, State, View};
use crate::browser::url::{self, Scheme};
use crate::browser::{html, http};

const MAX: usize = 4 * 1024 * 1024;
const FIRST_WAIT: u32 = 25;
const IDLE_AFTER: u32 = 20;
const MAX_FETCH_MS: i64 = 20000;

pub fn load(state: &mut State, target: &str) {
    if state.sockets_port == 0 {
        state.sockets_port = net::lookup(b"net.sockets");
        state.dns_port = net::lookup(b"net.dns");
    }
    if let Err(msg) = begin(state, target) {
        state.fetch_job = None;
        state.status = String::from(msg);
        state.document = None;
        state.view = View::Page;
    }
}

fn begin(state: &mut State, target: &str) -> Result<(), &'static str> {
    let url = url::parse(target).ok_or("bad url")?;
    if url.scheme == Scheme::Https {
        return Err("https not supported yet (P2)");
    }
    let ip = net::resolve(state.dns_port, url.host.as_bytes()).map_err(|_| "dns failed")?;
    let h = net::socket_open(state.sockets_port).map_err(|_| "socket failed")?;
    if net::socket_connect(state.sockets_port, h, ip, url.port).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return Err("connect failed");
    }
    let req = http::request::build(&url);
    if net::socket_send(state.sockets_port, h, req.as_bytes()).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return Err("send failed");
    }
    state.status = alloc::format!("loading {}", url.host);
    state.document = None;
    state.view = View::Page;
    state.fetch_start_ms = mk_time_millis();
    state.fetch_job = Some(FetchJob { handle: h, buf: Vec::new(), idle: 0 });
    Ok(())
}

pub fn poll(state: &mut State) -> bool {
    if mk_time_millis().wrapping_sub(state.fetch_start_ms) > MAX_FETCH_MS {
        let job = state.fetch_job.take().unwrap();
        let _ = net::socket_close(state.sockets_port, job.handle);
        if job.buf.is_empty() {
            state.status = String::from("timed out");
            state.document = None;
            state.view = View::Page;
        } else {
            finish(state, &job.buf);
        }
        return true;
    }
    let port = state.sockets_port;
    let mut chunk = [0u8; 4096];
    let (handle, finished, failed) = {
        let job = match state.fetch_job.as_mut() {
            Some(j) => j,
            None => return false,
        };
        match net::socket_recv(port, job.handle, &mut chunk) {
            Ok(n) if n > 0 => {
                job.idle = 0;
                job.buf.extend_from_slice(&chunk[..n]);
                let done = job.buf.len() >= MAX || http::response::is_complete(&job.buf);
                (job.handle, done, false)
            }
            _ => {
                job.idle = job.idle.wrapping_add(1);
                let budget = if job.buf.is_empty() { FIRST_WAIT } else { IDLE_AFTER };
                let over = job.idle >= budget;
                (job.handle, over, over && job.buf.is_empty())
            }
        }
    };
    if !finished {
        return false;
    }
    let job = state.fetch_job.take().unwrap();
    let _ = net::socket_close(port, handle);
    if failed {
        state.status = String::from("no response");
        state.document = None;
        state.view = View::Page;
        return true;
    }
    finish(state, &job.buf);
    true
}

fn finish(state: &mut State, raw: &[u8]) {
    match http::response::parse(raw) {
        Some(resp) => {
            let flows = html::parse::parse(&resp.body);
            let doc = layout::build(&flows, crate::browser::manifest::WIDTH, 8);
            state.scroll = 0;
            state.status = alloc::format!("{} ({} bytes)", resp.status, resp.body.len());
            state.document = Some(doc);
        }
        None => {
            state.status = String::from("bad response");
            state.document = None;
        }
    }
    state.view = View::Page;
}
