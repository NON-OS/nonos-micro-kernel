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

const MAX_SCRIPTS: usize = 24;

// Queue every external `<script src>` in document order for fetching. Inline
// scripts already ran in commit_html; these arrive after and are evaluated in
// the page engine as they land, so a framework bundle runs and builds the DOM.
// Module scripts are included: the engine treats them as scripts, which runs
// the common non-module-only bundles even if import graphs are not resolved.
pub fn enqueue_scripts(state: &mut State) {
    let Some(base) = state.base.clone() else { return };
    let Some(dom) = state.page_dom.as_ref() else { return };
    let mut fresh: Vec<String> = Vec::new();
    for node in &dom.nodes {
        if node.kind != NodeKind::Element || node.tag != "script" {
            continue;
        }
        let Some(src) = node.attr("src") else { continue };
        if src.is_empty() {
            continue;
        }
        let abs = url::join(&base, src);
        if !fresh.contains(&abs) && !state.script_queue.contains(&abs) {
            fresh.push(abs);
        }
        if fresh.len() + state.script_queue.len() >= MAX_SCRIPTS {
            break;
        }
    }
    state.script_queue.extend(fresh);
}
