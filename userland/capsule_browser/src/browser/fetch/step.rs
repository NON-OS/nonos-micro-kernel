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

use crate::browser::fetch::types::Phase;
use crate::browser::fetch::{fail, finish, tls};
use crate::browser::http;
use crate::browser::net;
use crate::browser::state::State;

pub fn step(state: &mut State) -> bool {
    let port = state.sockets_port;
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
        match state.fetch.as_mut() {
            Some(f) => {
                if super::advance::advance(port, f) {
                    return true;
                }
            }
            None => return false,
        }
    }
    let Some(job) = state.fetch.take() else {
        return false;
    };
    let _ = net::socket_close(port, job.handle);
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
