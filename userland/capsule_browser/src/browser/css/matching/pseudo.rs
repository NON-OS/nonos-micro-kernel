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

use crate::browser::css::selector::Pseudo;
use crate::browser::dom::node::NodeKind;
use crate::browser::dom::Dom;

use super::sibling::element_position;
use super::simple::matches_simple;

pub(super) fn pseudo_matches(dom: &Dom, id: usize, p: &Pseudo) -> bool {
    match p {
        Pseudo::Never => false,
        Pseudo::Not(inner) => !matches_simple(dom, id, inner),
        Pseudo::Empty => dom.nodes.get(id).is_some_and(|n| {
            n.children.iter().all(|&ch| {
                dom.nodes
                    .get(ch)
                    .is_none_or(|c| c.kind == NodeKind::Text && c.text.trim().is_empty())
            })
        }),
        _ => {
            let Some((pos, count, pos_ty, count_ty)) = element_position(dom, id) else {
                return false;
            };
            match p {
                Pseudo::FirstChild => pos == 1,
                Pseudo::LastChild => pos == count,
                Pseudo::OnlyChild => count == 1,
                Pseudo::FirstOfType => pos_ty == 1,
                Pseudo::LastOfType => pos_ty == count_ty,
                Pseudo::NthChild(a, b) => nth_matches(*a, *b, pos),
                _ => false,
            }
        }
    }
}

// An+B holds for position i when i = a*k + b for some k >= 0.
fn nth_matches(a: i32, b: i32, i: i32) -> bool {
    if a == 0 {
        return i == b;
    }
    let d = i - b;
    d % a == 0 && d / a >= 0
}
