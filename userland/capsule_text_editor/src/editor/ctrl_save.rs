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

use nonos_app_skeleton::{clients::vfs, EventOutcome};

use super::resolve_owner_pid::resolve_owner_pid;
use super::state::{State, PATH};

pub(super) fn ctrl_save(state: &mut State) -> EventOutcome {
    if !resolve_owner_pid(state) {
        state.status = b"save failed";
        return EventOutcome::Repaint;
    }
    state.status = if vfs::write_file(state.owner_pid, PATH, &state.buf[..state.len]).is_ok() {
        b"saved /notes.txt"
    } else {
        b"save failed"
    };
    EventOutcome::Repaint
}
