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
use super::state::{State, CAPACITY, PATH};

pub(super) fn ctrl_open(state: &mut State) -> EventOutcome {
    if !resolve_owner_pid(state) {
        state.status = b"open failed";
        return EventOutcome::Repaint;
    }
    match vfs::read_file(state.owner_pid, PATH, CAPACITY as u32) {
        Ok(bytes) if core::str::from_utf8(&bytes).is_ok() && bytes.len() <= CAPACITY => {
            state.buf[..bytes.len()].copy_from_slice(&bytes);
            state.len = bytes.len();
            state.status = b"opened /notes.txt";
        }
        Ok(_) => state.status = b"file is not valid utf-8",
        Err(_) => state.status = b"open failed",
    }
    EventOutcome::Repaint
}
