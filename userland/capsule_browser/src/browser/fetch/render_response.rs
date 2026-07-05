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
use crate::browser::{css, dom, html, js, layout};

use super::render_lines::render_lines;

// HTML renders through the box engine and keeps its DOM and script world
// alive for events; plain text and error surfaces use the line renderer.
pub enum Rendered {
    Boxes(layout::boxmodel::BoxDocument, dom::Dom, js::World),
    Lines(layout::doc::RenderDocument),
    Nothing,
}

pub fn render_response(resp: &Response) -> (Rendered, usize) {
    match resp.content_kind {
        ContentKind::Html => {
            let mut tree = dom::parse(&resp.body);
            let world = js::run(&mut tree);
            let styled = css::compute(&tree, &css::collect_css(&tree));
            let root = layout::boxmodel::build(&tree, &styled.styles, &styled.bg_images);
            if root.children.is_empty() {
                return (Rendered::Nothing, 0);
            }
            let doc = layout::boxmodel::layout(&root, crate::browser::manifest::WIDTH);
            let n = doc.frags.len();
            (Rendered::Boxes(doc, tree, world), n)
        }
        ContentKind::Text => render_lines(html::text::parse_text(&resp.body)),
        ContentKind::Unsupported => {
            render_lines(unsupported_content::unsupported_content(resp.status, resp.body.len()))
        }
    }
}
