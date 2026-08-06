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

use super::relayout::relayout;

// Dispatch a click on a DOM node to the page engine's listeners. Returns
// whether any listener ran (the click is then consumed). A listener may mutate
// the DOM through the engine's node bindings, so the page relays out when one
// fires.
pub fn js_click(state: &mut State, node: usize) -> bool {
    let fired = match state.engine.as_ref() {
        Some(engine) => engine.dispatch_event(node as i32, "click") > 0,
        None => return false,
    };
    if fired {
        relayout(state);
    }
    // A handler may have asked to go somewhere. It could not be acted on
    // while the script still held the tree, so it is collected here.
    super::script_nav::take_script_nav(state);
    fired
}
