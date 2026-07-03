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

use crate::browser::js;
use crate::browser::state::State;

use super::relayout::relayout;

// Dispatch a click on a DOM node to its script listeners. Returns whether
// any listener ran (the click is then consumed).
pub fn js_click(state: &mut State, node: usize) -> bool {
    let (fired, dirty) = match (state.page_dom.as_mut(), state.world.as_mut()) {
        (Some(dom), Some(world)) => js::dispatch_event(dom, world, node, "click"),
        _ => return false,
    };
    if dirty {
        relayout(state);
    }
    fired
}
