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

pub mod types;
mod plain;
mod tls;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_libc::{mk_time_millis, mk_time_rtc, RtcTime};

use crate::browser::layout;
use crate::browser::net;
use crate::browser::state::{State, View};
use crate::browser::url;
use crate::browser::{html, http};

const MAX: usize = 4 * 1024 * 1024;
const FIRST_WAIT: u32 = 25;
const IDLE_AFTER: u32 = 20;
const MAX_FETCH_MS: i64 = 45000;
const MAX_REDIRECTS: u8 = 5;
const DRAIN_BURST: usize = 64;
const HS_WAIT: u32 = 200;
const CHECK_STRIDE: usize = 16 * 1024;
const MAX_RETRIES: u8 = 2;
const FLIGHT_SETTLE: u32 = 15;

pub fn load(state: &mut State, target: &str) -> Result<(), &'static str> {
    if state.sockets_port == 0 {
        state.sockets_port = net::lookup(b"net.sockets");
        state.dns_port = net::lookup(b"net.dns");
    }
    let url = url::parse(target).ok_or("bad url")?;
    state.base = Some(url.clone());
    let ip = net::resolve(state.dns_port, url.host.as_bytes()).map_err(|_| "dns failed")?;
    let h = net::socket_open(state.sockets_port).map_err(|_| "socket failed")?;
    if net::socket_connect(state.sockets_port, h, ip, url.port).is_err() {
        let _ = net::socket_close(state.sockets_port, h);
        return Err("connect failed");
    }
    let phase = if url.scheme == url::Scheme::Https { types::Phase::TlsHello } else { types::Phase::SendReq };
    state.status = alloc::format!("loading {}", url.host);
    state.document = None;
    state.view = View::Page;
    let suppress = core::mem::take(&mut state.suppress_history_push);
    state.fetch = Some(types::Fetch {
        url, handle: h, phase, buf: Vec::new(), tls: None,
        idle: 0, started_ms: mk_time_millis(), error: None, suppress, last_check: 0,
    });
    Ok(())
}

pub fn step(state: &mut State) -> bool {
    let port = state.sockets_port;
    let now = rtc_packed();
    {
        let Some(f) = state.fetch.as_mut() else { return false; };
        if mk_time_millis().wrapping_sub(f.started_ms) > MAX_FETCH_MS {
            if f.buf.is_empty() {
                if f.error.is_none() { f.error = Some("timed out"); }
                f.phase = types::Phase::Error;
            } else {
                f.phase = if f.tls.is_some() { types::Phase::Decrypt } else { types::Phase::Done };
            }
        }
        match f.phase {
            types::Phase::TlsHello => { tls::hello(port, f, now); return true; }
            types::Phase::TlsFlight => { tls::read_flight(port, f); return true; }
            types::Phase::TlsVerify => { tls::verify_and_send(port, f); return true; }
            types::Phase::SendReq => { plain::send_req(port, f); return true; }
            types::Phase::ReadBody => { plain::read_body(port, f, f.tls.is_some()); return true; }
            types::Phase::Decrypt | types::Phase::Done | types::Phase::Error => {}
        }
    }
    let job = state.fetch.take().unwrap();
    let _ = net::socket_close(port, job.handle);
    match job.phase {
        types::Phase::Decrypt => match tls::decrypt(&job) {
            Some(p) => finish(state, &p, job.suppress),
            None => fail(state, "decrypt failed"),
        },
        types::Phase::Done => finish(state, &job.buf, job.suppress),
        _ => fail(state, job.error.unwrap_or("error")),
    }
    true
}

fn fail(state: &mut State, msg: &str) {
    if state.retries < MAX_RETRIES && !state.address.is_empty() {
        state.retries += 1;
        state.status = alloc::format!("retry {} — {}", state.retries, msg);
        state.pending_nav = Some(state.address.clone());
    } else {
        state.retries = 0;
        state.status = String::from(msg);
        state.document = None;
        state.view = View::Page;
    }
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

fn finish(state: &mut State, raw: &[u8], suppress: bool) {
    state.retries = 0;
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
            state.status = alloc::format!(
                "{} raw={} body={} fl={}",
                resp.status, raw.len(), resp.body.len(), flows.len()
            );
            state.document = if flows.is_empty() { None } else { Some(doc) };
            record_history(state, suppress);
        }
        None => {
            state.redirect_count = 0;
            state.status = alloc::format!("bad resp raw={}", raw.len());
            state.document = None;
        }
    }
    state.view = View::Page;
}

fn record_history(state: &mut State, suppress: bool) {
    if suppress {
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
