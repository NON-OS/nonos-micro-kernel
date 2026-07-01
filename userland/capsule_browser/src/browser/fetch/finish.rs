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

use crate::browser::fetch::{record_history, redirect, render_error, render_response};
use crate::browser::state::{State, View};
use crate::browser::http;

pub(super) fn finish(state: &mut State, raw: &[u8], suppress: bool) {
    state.retries = 0;
    match http::response::parse(raw) {
        Some(resp) => {
            if matches!(resp.status, 301 | 302 | 303 | 307 | 308) {
                if let Some(loc) = resp.location { return redirect::redirect(state, loc); }
            }
            state.redirect_count = 0;
            let (doc, flows) = render_response::render_response(&resp);
            state.scroll = 0;
            state.status = alloc::format!(
                "{} raw={} body={} fl={}",
                resp.status, raw.len(), resp.body.len(), flows
            );
            state.document = doc;
            record_history::record_history(state, suppress);
        }
        None => {
            state.redirect_count = 0;
            state.status = alloc::format!("bad resp raw={}", raw.len());
            state.document = Some(render_error::render_error("bad response"));
        }
    }
    state.view = View::Page;
}
