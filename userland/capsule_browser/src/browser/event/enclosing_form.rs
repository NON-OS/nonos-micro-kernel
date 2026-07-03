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

use crate::browser::state::State;

// The nearest <form> ancestor of `from`, if any.
pub(super) fn enclosing_form(state: &State, from: usize) -> Option<usize> {
    let dom = state.page_dom.as_ref()?;
    let mut cur = from;
    let mut hops = 0u32;
    while cur != 0 && hops < 512 {
        let n = dom.nodes.get(cur)?;
        if n.tag == "form" {
            return Some(cur);
        }
        cur = n.parent;
        hops += 1;
    }
    None
}
