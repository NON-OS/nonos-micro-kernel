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

use super::navigate::navigate;
use super::sidebar_model::SideHit;
use super::sidebar_rows::side_hit;
use super::state::State;

/// Route a sidebar click through the one layout pass `paint_sidebar` drew from.
/// A section label carries no hit and is inert, which is why a miss is `Idle`
/// rather than a fall-through to the listing underneath.
pub fn on_side(state: &mut State, y: u32) -> EventOutcome {
    match side_hit(state, y) {
        Some(SideHit::Screen(screen)) => {
            state.screen = screen;
            EventOutcome::Repaint
        }
        Some(SideHit::Path(path)) => {
            navigate(state, path.as_str());
            EventOutcome::Repaint
        }
        None => EventOutcome::Idle,
    }
}
