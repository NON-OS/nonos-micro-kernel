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
use super::script_nav::take_script_nav;

// One app tick for the page's timers. Returns whether anything ran and the
// screen needs repainting.
pub fn js_tick(state: &mut State) -> bool {
    // The page's own timers, in the engine that ran its scripts. Nothing was
    // draining this queue, so every callback a page deferred sat in it: the
    // work a page does after its first paint never happened at all.
    let ran = match state.engine.as_ref() {
        Some(engine) => engine.flush_timers(nonos_libc::mk_uptime_ms() as u64) > 0,
        None => false,
    };
    if ran {
        relayout(state);
        take_script_nav(state);
    }
    let (fired, dirty) = match (state.page_dom.as_mut(), state.world.as_mut()) {
        (Some(dom), Some(world)) => js::pump_timers(dom, world),
        _ => return ran,
    };
    if dirty {
        relayout(state);
    }
    fired || ran
}
