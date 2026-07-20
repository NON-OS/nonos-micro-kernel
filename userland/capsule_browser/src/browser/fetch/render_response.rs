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

use crate::browser::fetch::unsupported_content;
use crate::browser::http::response::{ContentKind, Response};
use crate::browser::{dom, html, layout};

use super::render_lines::render_lines;

// HTML hands back the parsed DOM: its scripts run through QuickJS and it is laid
// out only after it is homed in the page state, so the retained engine's pointer
// into that DOM stays valid for later event dispatch. Plain text and error
// surfaces use the line renderer directly.
pub enum Rendered {
    Html(dom::Dom),
    Lines(layout::doc::RenderDocument),
    Nothing,
}

pub fn render_response(resp: &Response) -> (Rendered, usize) {
    match resp.content_kind {
        ContentKind::Html => (Rendered::Html(dom::parse(&resp.body)), 0),
        ContentKind::Text => render_lines(html::text::parse_text(&resp.body)),
        ContentKind::Unsupported => {
            render_lines(unsupported_content::unsupported_content(resp.status, resp.body.len()))
        }
    }
}
