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

/// Act on a navigation a script asked for while it was running.
///
/// `location.assign`, `location.replace` and `location.reload` cannot take
/// effect where they are called: the tree the script is still executing
/// against would be torn down under it. The engine parks the address instead,
/// and this collects it once the run is over.
///
/// A page already going somewhere is left alone. The reader's own click
/// started that one, and letting a script's request overwrite it would take
/// them somewhere they did not ask to go.
pub fn take_script_nav(state: &mut State) {
    if state.pending_nav.is_some() {
        return;
    }
    let Some(engine) = state.engine.as_ref() else {
        return;
    };
    let Some(next) = engine.take_navigation() else {
        return;
    };
    if next.is_empty() {
        return;
    }
    state.address = next.clone();
    state.pending_nav = Some(next);
}
