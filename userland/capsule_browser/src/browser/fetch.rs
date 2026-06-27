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

use nonos_libc::{mk_time_millis, mk_time_rtc, RtcTime};

use crate::browser::layout;
use crate::browser::net;
use crate::browser::state::{FetchJob, State, View};
use crate::browser::url::{self, Scheme};
use crate::browser::{html, http, tls13};

const MAX: usize = 4 * 1024 * 1024;
const FIRST_WAIT: u32 = 25;
const IDLE_AFTER: u32 = 20;
const MAX_FETCH_MS: i64 = 20000;
const MAX_REDIRECTS: u8 = 5;

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
    state.base = Some(url.clone());
    if url.scheme == Scheme::Https {
        return https_fetch(state, &url);
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

fn rtc_packed() -> u64 {
    let mut t = RtcTime::default();
    if mk_time_rtc(&mut t as *mut RtcTime) != 0 {
        return 0;
    }
    (t.year as u64) * 10_000_000_000
        + (t.month as u64) * 100_000_000
        + (t.day as u64) * 1_000_000
        + (t.hour as u64) * 10_000
        + (t.minute as u64) * 100
        + t.second as u64
}

fn https_fetch(state: &mut State, url: &url::Url) -> Result<(), &'static str> {
    let host = url.host.as_bytes();
    let now = rtc_packed();
    let ip = net::resolve(state.dns_port, host).map_err(|_| "dns failed")?;
    let cf = tls13::client_flight(host).ok_or("tls init failed")?;
    let h = net::socket_open(state.sockets_port).map_err(|_| "socket failed")?;
    if net::socket_connect(state.sockets_port, h, ip, url.port).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return Err("connect failed");
    }
    if net::socket_send(state.sockets_port, h, &cf.record).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return Err("send failed");
    }
    let flight = net::read_tls_flight(state.sockets_port, h).map_err(|_| "tls handshake failed")?;
    let req = http::request::build(url);
    let out = tls13::application_write(&cf, &flight, req.as_bytes(), host, now).ok_or("cert verify failed")?;
    if net::socket_send(state.sockets_port, h, &out).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return Err("send failed");
    }
    let enc = tls_read_response(state.sockets_port, h);
    let _ = net::socket_close(state.sockets_port, h);
    let plain = tls13::application_plaintext(&cf, &flight, &enc, host, now).ok_or("cert verify failed")?;
    state.fetch_job = None;
    finish(state, &plain);
    Ok(())
}

fn tls_read_response(port: u32, h: u32) -> Vec<u8> {
    let mut enc = Vec::new();
    let mut idle = 0u32;
    let mut chunk = [0u8; 4096];
    loop {
        match net::socket_recv(port, h, &mut chunk) {
            Ok(n) if n > 0 => {
                idle = 0;
                enc.extend_from_slice(&chunk[..n]);
                if enc.len() >= MAX {
                    break;
                }
            }
            _ => {
                idle += 1;
                let budget = if enc.is_empty() { FIRST_WAIT } else { IDLE_AFTER };
                if idle >= budget {
                    break;
                }
            }
        }
    }
    enc
}

fn finish(state: &mut State, raw: &[u8]) {
    match http::response::parse(raw) {
        Some(resp) => {
            if matches!(resp.status, 301 | 302 | 303 | 307 | 308) {
                if let Some(loc) = resp.location {
                    return redirect(state, loc);
                }
            }
            state.redirect_count = 0;
            let flows = html::parse::parse(&resp.body);
            let doc = layout::build(&flows, crate::browser::manifest::WIDTH, nonos_app_skeleton::font_advance());
            state.scroll = 0;
            state.status = alloc::format!("{} ({} bytes)", resp.status, resp.body.len());
            state.document = Some(doc);
            record_history(state);
        }
        None => {
            state.redirect_count = 0;
            state.status = String::from("bad response");
            state.document = None;
        }
    }
    state.view = View::Page;
}

fn record_history(state: &mut State) {
    if state.suppress_history_push {
        state.suppress_history_push = false;
        return;
    }
    let url = state.address.clone();
    if url.is_empty() {
        return;
    }
    let trunc = (state.hist_index + 1).max(0) as usize;
    state.history.truncate(trunc);
    state.history.push(url);
    state.hist_index = state.history.len() as i32 - 1;
}

fn redirect(state: &mut State, location: String) {
    state.view = View::Page;
    if state.redirect_count >= MAX_REDIRECTS {
        state.redirect_count = 0;
        state.status = String::from("too many redirects");
        state.document = None;
        return;
    }
    state.redirect_count += 1;
    let next = match &state.base {
        Some(b) => url::join(b, &location),
        None => location,
    };
    state.status = alloc::format!("redirecting to {}", next);
    state.address = next.clone();
    state.pending_nav = Some(next);
}
