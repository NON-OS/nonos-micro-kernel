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

use crate::browser::state::State;
use crate::browser::url::{self, Url};

use super::import_url::import_url;

const MAX_SHEETS: usize = 16;

// Follow the @import rules of a freshly fetched stylesheet: resolve each
// imported URL against that sheet's own address (not the page base, since the
// imports are relative to the importing file) and queue it for fetching. An
// index sheet that is nothing but @import lines, which several sites ship as
// their main entry point, otherwise contributes no rules and the page loses
// all of its styling.
pub(super) fn enqueue_imports(state: &mut State, css: &str, sheet: &Url) {
    let mut fresh: Vec<String> = Vec::new();
    let mut rest = css;
    while let Some(pos) = rest.find("@import") {
        rest = &rest[pos + "@import".len()..];
        let Some(end) = rest.find(';') else { break };
        if let Some(spec) = import_url(&rest[..end]) {
            let abs = url::join(sheet, &spec);
            if !fresh.contains(&abs) && !state.css_queue.contains(&abs) {
                fresh.push(abs);
            }
        }
        rest = &rest[end + 1..];
        if fresh.len() + state.css_queue.len() >= MAX_SHEETS {
            break;
        }
    }
    state.css_queue.extend(fresh);
}
