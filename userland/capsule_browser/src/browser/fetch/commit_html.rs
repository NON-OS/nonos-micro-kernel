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

//! Commit a freshly homed HTML DOM: run its scripts through QuickJS, retain the
//! engine for event dispatch, and lay the mutated tree out.

use crate::browser::event::relayout;
use crate::browser::js::World;
use crate::browser::qjs_run::run_scripts;
use crate::browser::state::State;
use crate::browser::url::join;

/// Run the page DOM's scripts through QuickJS and lay the result out. The DOM
/// must already sit in `state.page_dom`, its address for the page's life, so the
/// engine's pointer into it stays valid; the caller drops any prior engine
/// before replacing the DOM. The inert tree-walk world is set so the timer and
/// script-fetch pumps have something to read while the engine owns page state.
pub fn commit_html(state: &mut State) {
    // The document has to know where it came from before a script runs.
    // `location` is read during setup on most pages, and a relative href
    // resolved against the wrong address points somewhere else entirely.
    let base = state.base.as_ref().map(|u| join(u, "")).unwrap_or_default();
    let engine = match state.page_dom.as_mut() {
        Some(dom) => {
            dom.base = base;
            run_scripts(dom)
        }
        None => None,
    };
    state.engine = engine;
    state.world = Some(World::empty());
    relayout(state);
}
