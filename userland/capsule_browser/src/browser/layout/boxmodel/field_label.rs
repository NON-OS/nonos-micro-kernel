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

use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

// Visible text for a form widget: the value (or hint attribute) for inputs,
// the first option for selects. Empty means the widget shows blank.
pub(super) fn field_label(dom: &Dom, id: usize) -> String {
    let Some(node) = dom.nodes.get(id) else {
        return String::new();
    };
    match node.tag.as_str() {
        "input" => {
            node.attr("value").or_else(|| node.attr("placeholder")).unwrap_or("").to_string()
        }
        "select" => {
            for &c in &node.children {
                let Some(opt) = dom.nodes.get(c) else {
                    continue;
                };
                if opt.kind == NodeKind::Element && opt.tag == "option" {
                    for &t in &opt.children {
                        if let Some(tn) = dom.nodes.get(t) {
                            if tn.kind == NodeKind::Text && !tn.text.trim().is_empty() {
                                return tn.text.trim().to_string();
                            }
                        }
                    }
                }
            }
            String::new()
        }
        _ => String::new(),
    }
}
