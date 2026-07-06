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

use nonos_libc::mk_time_millis;

use crate::browser::fetch::types::Phase;
use crate::browser::fetch::{constants, fail, finish, plain, rtc_packed, socks, tls};
use crate::browser::http;
use crate::browser::net;
use crate::browser::state::State;

pub fn step(state: &mut State) -> bool {
    let port = state.sockets_port;
    let now = rtc_packed::rtc_packed();
    {
        // Surface page-fetch progress on the loading screen. Sub-resource
        // fetches (stylesheets, images) are suppressed so they don't clobber
        // the label while the page itself is already rendered.
        let progress =
            state.fetch.as_ref().filter(|f| !f.suppress).map(|f| {
                alloc::format!("{} {}", super::progress::phase_label(f.phase), f.url.host)
            });
        if let Some(s) = progress {
            state.status = s;
        }
        let Some(f) = state.fetch.as_mut() else {
            return false;
        };
        if mk_time_millis().wrapping_sub(f.started_ms) > constants::MAX_FETCH_MS {
            if f.tls.is_some() {
                f.phase = if tls::decrypt(f).is_some_and(|p| http::response::has_headers(&p)) {
                    Phase::Decrypt
                } else {
                    Phase::Error
                };
            } else {
                f.phase = if f.buf.is_empty() { Phase::Error } else { Phase::Done };
            }
            if f.error.is_none() && matches!(f.phase, Phase::Error) {
                f.error = Some("timed out");
            }
        }
        match f.phase {
            Phase::SocksHello => {
                socks::hello(port, f);
                return true;
            }
            Phase::SocksMethod => {
                socks::method(port, f);
                return true;
            }
            Phase::SocksConnect => {
                socks::connect(port, f);
                return true;
            }
            Phase::TlsHello => {
                tls::hello(port, f, now);
                return true;
            }
            Phase::TlsFlight => {
                tls::read_flight(port, f);
                return true;
            }
            Phase::TlsVerify => {
                tls::verify_and_send(port, f);
                return true;
            }
            Phase::SendReq => {
                plain::send_req(port, f);
                return true;
            }
            Phase::ReadBody => {
                plain::read_body(port, f, f.tls.is_some());
                return true;
            }
            Phase::Decrypt | Phase::Done | Phase::Error => {}
        }
    }
    let Some(job) = state.fetch.take() else {
        return false;
    };
    let _ = net::socket_close(port, job.handle);
    if let Some(src) = job.image.clone() {
        let raw = match job.phase {
            Phase::Decrypt => tls::decrypt(&job),
            Phase::Done => Some(job.buf.clone()),
            _ => None,
        };
        let resp = raw.as_deref().and_then(http::response::parse);
        // CDNs answer image hits with 3xx routinely; chase the hop while
        // keeping the pixels keyed to the URL the boxes reference.
        if let Some(r) = resp.as_ref() {
            if matches!(r.status, 301 | 302 | 303 | 307 | 308) {
                if let Some(loc) = r.location.as_deref() {
                    let abs = crate::browser::url::join(&job.url, loc);
                    if crate::browser::image::follow_redirect(state, &abs, &src) {
                        return true;
                    }
                }
            }
        }
        // Route through ingest either way: a non-200 or empty body fails to
        // decode and is recorded as such, so the box keeps its fallback look.
        let body = resp.filter(|resp| resp.status == 200).map(|resp| resp.body).unwrap_or_default();
        crate::browser::image::ingest(&mut state.images, &src, &body);
        return true;
    }
    if job.css {
        let raw = match job.phase {
            Phase::Decrypt => tls::decrypt(&job),
            Phase::Done => Some(job.buf.clone()),
            _ => None,
        };
        super::apply_css::apply_css(state, raw.as_deref(), Some(&job.url));
        return true;
    }
    if job.js_req {
        let raw = match job.phase {
            Phase::Decrypt => tls::decrypt(&job),
            Phase::Done => Some(job.buf.clone()),
            _ => None,
        };
        // Failed requests still call back with status 0 and an empty body
        // so page code can branch on ok.
        let (status, body) = raw
            .as_deref()
            .and_then(http::response::parse)
            .map(|r| (r.status, alloc::string::String::from_utf8_lossy(&r.body).into_owned()))
            .unwrap_or((0, alloc::string::String::new()));
        if let (Some(dom), Some(world)) = (state.page_dom.as_mut(), state.world.as_mut()) {
            if let Some(cb) = world.net_active.take() {
                let dirty = crate::browser::js::deliver_net(dom, world, cb, status, body);
                if dirty {
                    crate::browser::event::relayout(state);
                }
            }
        }
        return true;
    }
    match job.phase {
        Phase::Decrypt => match tls::decrypt(&job) {
            Some(p) => finish::finish(state, &p, job.suppress),
            None => fail::fail(state, "decrypt failed"),
        },
        Phase::Done => finish::finish(state, &job.buf, job.suppress),
        _ => fail::fail(
            state,
            match job.error {
                Some(err) => err,
                None => "error",
            },
        ),
    }
    true
}
