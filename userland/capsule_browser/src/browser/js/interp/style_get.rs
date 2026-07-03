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

use alloc::string::{String, ToString};

use crate::browser::dom::Dom;

use super::css_name::css_name;

// Read one property out of the element's inline style attribute.
pub(super) fn style_get(dom: &Dom, id: usize, prop: &str) -> String {
    let want = css_name(prop);
    let Some(node) = dom.nodes.get(id) else {
        return String::new();
    };
    let style = node.attr("style").unwrap_or("");
    for d in style.split(';') {
        if let Some(colon) = d.find(':') {
            if d[..colon].trim().eq_ignore_ascii_case(&want) {
                return d[colon + 1..].trim().to_string();
            }
        }
    }
    String::new()
}
