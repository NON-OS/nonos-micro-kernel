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

use crate::browser::http::response::parse;
use crate::browser::state::State;

const MAX_PAGE_CSS: usize = 2_097_152;

// Fold a completed stylesheet fetch into the page and re-lay-out. The body
// (when a 200) cascades after inline <style>; a failed or empty sheet still
// triggers the relayout so the render-blocking hold in finish resolves once
// every queued sheet has been attempted. `raw` is None when the fetch failed.
pub(super) fn apply_css(state: &mut State, raw: Option<&[u8]>) {
    if let Some(raw) = raw {
        let body = parse(raw).filter(|r| r.status == 200).map(|r| r.body).unwrap_or_default();
        if !body.is_empty() {
            let text = alloc::string::String::from_utf8_lossy(&body);
            if state.page_css.len() + text.len() <= MAX_PAGE_CSS {
                state.page_css.push('\n');
                state.page_css.push_str(&text);
            }
        }
    }
    crate::browser::event::relayout(state);
}
