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

use crate::browser::dom::node::NodeKind;
use crate::browser::state::State;
use crate::browser::url;

const MAX_SHEETS: usize = 16;

// Queue every <link rel="stylesheet"> href in the page, resolved against the
// page base, for fetching. The page renders first with inline <style> only;
// these arrive after and restyle it.
pub fn enqueue_css(state: &mut State) {
    let Some(base) = state.base.clone() else { return };
    let Some(dom) = state.page_dom.as_ref() else { return };
    let mut fresh: Vec<String> = Vec::new();
    for node in &dom.nodes {
        if node.kind != NodeKind::Element || node.tag != "link" {
            continue;
        }
        let is_sheet = node
            .attr("rel")
            .map(|r| r.split_whitespace().any(|t| t.eq_ignore_ascii_case("stylesheet")))
            .unwrap_or(false);
        if !is_sheet {
            continue;
        }
        let Some(href) = node.attr("href") else { continue };
        if href.is_empty() {
            continue;
        }
        let abs = url::join(&base, href);
        if !fresh.contains(&abs) && !state.css_queue.contains(&abs) {
            fresh.push(abs);
        }
        if fresh.len() + state.css_queue.len() >= MAX_SHEETS {
            break;
        }
    }
    // Stylesheets the WebFont loader would have requested count like any
    // other sheet; their @font-face rules feed the font pipeline.
    for u in super::webfont_shim::webfont_css_urls(dom) {
        if !fresh.contains(&u)
            && !state.css_queue.contains(&u)
            && fresh.len() + state.css_queue.len() < MAX_SHEETS
        {
            fresh.push(u);
        }
    }
    state.css_queue.extend(fresh);
}
