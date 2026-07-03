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

use crate::browser::dom::Dom;

// What a click landed on, walking up from the hit node.
pub(super) enum Field {
    Edit(usize),
    Submit(usize),
    None,
}

pub(super) fn field_at(dom: &Dom, node: usize) -> Field {
    let mut cur = node;
    let mut hops = 0u32;
    while cur != 0 && hops < 64 {
        let Some(n) = dom.nodes.get(cur) else {
            return Field::None;
        };
        match n.tag.as_str() {
            "textarea" => return Field::Edit(cur),
            "button" => return Field::Submit(cur),
            "input" => {
                let ty = n.attr("type").unwrap_or("text").to_ascii_lowercase();
                return match ty.as_str() {
                    "submit" | "button" | "image" => Field::Submit(cur),
                    "hidden" => Field::None,
                    _ => Field::Edit(cur),
                };
            }
            _ => {}
        }
        cur = n.parent;
        hops += 1;
    }
    Field::None
}
