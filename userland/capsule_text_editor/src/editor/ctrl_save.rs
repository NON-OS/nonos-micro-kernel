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
use super::state::State;

pub(super) fn ctrl_save(state: &mut State) -> EventOutcome {
    if !resolve_owner_pid(state) {
        state.status = b"save failed";
        return EventOutcome::Repaint;
    }
    let path = state.path[..state.path_len].to_vec();
    state.status = if vfs::write_file(state.owner_pid, &path, &state.buf[..state.len]).is_ok() {
        b"saved"
    } else {
        b"save failed"
    };
    EventOutcome::Repaint
}
