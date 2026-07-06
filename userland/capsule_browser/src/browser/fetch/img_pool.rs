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

use crate::browser::fetch::types::{Fetch, Phase};
use crate::browser::fetch::{advance, tls};
use crate::browser::http;
use crate::browser::net;
use crate::browser::state::State;
use crate::browser::{image, url};

// Advance every in-flight image fetch one step and ingest the ones that
// finished, on their own sockets, so an image-heavy page loads in parallel
// with the page fetch instead of one raster at a time. Returns true if any
// image fetch progressed.
pub fn step_images(state: &mut State) -> bool {
    if state.img_fetches.is_empty() {
        return false;
    }
    let port = state.sockets_port;
    let mut i = 0;
    while i < state.img_fetches.len() {
        if advance::advance(port, &mut state.img_fetches[i]) {
            i += 1;
            continue;
        }
        let job = state.img_fetches.remove(i);
        complete(state, job);
    }
    true
}

// Fold a finished image fetch into the store, chasing a redirect on its own
// budget while keeping the pixels keyed to the URL the boxes reference.
fn complete(state: &mut State, job: Fetch) {
    let _ = net::socket_close(state.sockets_port, job.handle);
    let src = job.image.clone().unwrap_or_default();
    let raw = match job.phase {
        Phase::Decrypt => tls::decrypt(&job),
        Phase::Done => Some(job.buf.clone()),
        _ => None,
    };
    let resp = raw.as_deref().and_then(http::response::parse);
    if let Some(r) = resp.as_ref() {
        if matches!(r.status, 301 | 302 | 303 | 307 | 308) && job.redirects < image::REDIRECT_LIMIT {
            if let Some(loc) = r.location.as_deref() {
                let abs = url::join(&job.url, loc);
                if let Some(f) = image::begin(state, &abs, &src, job.redirects + 1) {
                    state.img_fetches.push(f);
                    return;
                }
            }
        }
    }
    let body = resp.filter(|r| r.status == 200).map(|r| r.body).unwrap_or_default();
    image::ingest(&mut state.images, &src, &body);
}
