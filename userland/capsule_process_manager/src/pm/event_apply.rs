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

//! Carrying out one keyboard action.

use crate::pm::state::{Filter, Sort, State, SCREENS, SIGKILL, SIGTERM};
use crate::pm::ui::keys::Act;

// One arm per action, so a binding added to the table without a case here fails
// to compile rather than reading as a key that quietly does nothing.
pub fn apply(state: &mut State, act: Act) {
    match act {
        Act::Screen(i) => state.set_screen(SCREENS[i.min(SCREENS.len() - 1)]),
        Act::NextScreen => state.set_screen(state.screen.next()),
        Act::Security => state.toggle_security(),
        Act::Terminate => state.kill_selected(SIGTERM),
        Act::ForceKill => state.kill_selected(SIGKILL),
        Act::SortCpu => state.set_sort(Sort::Cpu),
        Act::SortMem => state.set_sort(Sort::Mem),
        Act::SortName => state.set_sort(Sort::Name),
        Act::SortPid => state.set_sort(Sort::Pid),
        Act::SortIpc => state.set_sort(Sort::Ipc),
        Act::SortSysc => state.set_sort(Sort::Sysc),
        Act::Refresh => state.refresh(),
        Act::FilterAll => state.set_filter(Filter::All),
        Act::FilterElevated => state.set_filter(Filter::Elevated),
        Act::FilterProtected => state.set_filter(Filter::Protected),
        Act::FilterFlagged => state.set_filter(Filter::Flagged),
        Act::Search => state.focus_search(true),
        Act::Help => state.help_open = true,
        Act::Close => state.help_open = false,
    }
}
