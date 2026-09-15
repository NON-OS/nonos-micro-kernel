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

use nonos_app_skeleton::EventOutcome;

use super::event_crumb::crumb_nav;
use super::event_parent::open_parent;
use super::event_undo::undo;
use super::header_slots::HeadHit;
use super::prompt_start::start_prompt;
use super::screen::Screen;
use super::state::{PromptKind, State, ViewKind};
use super::store_meta::save_meta;
use super::view::rebuild_view;

/// Act on a header control. The view and sort controls write through to
/// `prefs` as well as to the live state, so the drawn control and the persisted
/// preference can never disagree across a restart.
pub fn on_head(state: &mut State, hit: HeadHit) -> EventOutcome {
    match hit {
        HeadHit::Crumb(index) => crumb_nav(state, index),
        HeadHit::ViewList => set_view(state, ViewKind::List),
        HeadHit::ViewGrid => set_view(state, ViewKind::Grid),
        HeadHit::Sort => {
            state.sort_mode = state.sort_mode.next();
            state.prefs.sort = state.sort_mode;
            rebuild_view(state);
            save_meta(state);
        }
        HeadHit::Search => {
            state.screen = Screen::Search;
            state.status = b"type to search, Enter to run";
        }
        HeadHit::SearchClear => {
            state.query.clear();
            state.hits.clear();
        }
        HeadHit::Undo => return undo(state),
        HeadHit::New => return start_prompt(state, PromptKind::NewFile, b"new file: "),
        HeadHit::NavBack => return open_parent(state),
        HeadHit::NavFwd => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}

fn set_view(state: &mut State, view: ViewKind) {
    state.view = view;
    state.prefs.view = view;
    save_meta(state);
}
