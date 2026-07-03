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
use crate::browser::dom::Dom;
use crate::browser::http::urlencode;

const MAX_FIELDS: usize = 64;

// The form's submittable fields as one urlencoded body. Buttons and unset
// checkables stay out, matching what a submit would carry.
pub(super) fn form_fields(dom: &Dom, form: usize) -> String {
    let mut body = String::new();
    let mut stack: Vec<usize> = dom.nodes.get(form).map(|n| n.children.clone()).unwrap_or_default();
    let mut fields = 0usize;
    while let Some(id) = stack.pop() {
        let Some(n) = dom.nodes.get(id) else {
            continue;
        };
        if n.kind != NodeKind::Element {
            continue;
        }
        stack.extend(n.children.iter().copied());
        if fields >= MAX_FIELDS {
            break;
        }
        let name = n.attr("name").unwrap_or("");
        if name.is_empty() {
            continue;
        }
        let value = match n.tag.as_str() {
            "input" => {
                let ty = n.attr("type").unwrap_or("text").to_ascii_lowercase();
                match ty.as_str() {
                    "submit" | "button" | "image" | "file" => continue,
                    "checkbox" | "radio" => {
                        if n.attr("checked").is_none() {
                            continue;
                        }
                        n.attr("value").unwrap_or("on")
                    }
                    _ => n.attr("value").unwrap_or(""),
                }
            }
            "textarea" => n.attr("value").unwrap_or(""),
            "select" => n.attr("value").unwrap_or(""),
            _ => continue,
        };
        if !body.is_empty() {
            body.push('&');
        }
        body.push_str(&urlencode(name));
        body.push('=');
        body.push_str(&urlencode(value));
        fields += 1;
    }
    body
}
