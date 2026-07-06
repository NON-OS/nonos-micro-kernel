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

use nonos_app_skeleton::{EventOutcome, InputEvent};

use super::filter;
use super::help;
use super::preview_key;
use super::prompt;
use super::state::{Mode, State};

pub fn route(state: &mut State, event: InputEvent) -> Option<EventOutcome> {
    match state.mode {
        Mode::Filter => Some(filter::on_key(state, event)),
        Mode::Help => Some(help::on_key(state, event)),
        Mode::Prompt(_) => Some(prompt::on_key(state, event)),
        Mode::Preview => Some(preview_key::on_key(state, event)),
        Mode::Browse => None,
    }
}
