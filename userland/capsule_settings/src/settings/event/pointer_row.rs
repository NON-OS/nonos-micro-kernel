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
use nonos_policy_proto::{enum_table, kind_of, KIND_U8};

use crate::settings::schema::read_only;
use crate::settings::state::{current_field, State};

use super::toggle_or_inc::toggle_or_inc;

/// A click on the label selects the row; a click in the control column acts on
/// it. A read-only row is selectable but never changed, which is what keeps the
/// panel from asserting something untrue about the machine.
pub(super) fn activate(state: &mut State, control: bool) -> EventOutcome {
    if !control {
        return EventOutcome::Repaint;
    }
    let Some(field) = current_field(state) else { return EventOutcome::Repaint };
    if read_only(field) {
        return EventOutcome::Repaint;
    }
    if kind_of(field) == KIND_U8 && enum_table(field).is_none() {
        return EventOutcome::Repaint;
    }
    toggle_or_inc(state);
    EventOutcome::Repaint
}
