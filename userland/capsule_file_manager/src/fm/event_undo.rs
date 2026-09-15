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

use nonos_app_skeleton::clients::vfs::{chmod, rename, rmdir, unlink};
use nonos_app_skeleton::EventOutcome;

use super::perms::{MODE_RO, MODE_RW};
use super::refresh::refresh;
use super::state::State;
use super::undo::Op;

/// Replay one recorded inverse. Every op on the stack is already the inverse of
/// a completed action, so this only has to run it and say whether the server
/// accepted it -- a failure is reported, never papered over.
pub fn undo(state: &mut State) -> EventOutcome {
    let Some(op) = state.undo.pop() else {
        state.status = b"nothing to undo";
        return EventOutcome::Repaint;
    };
    let pid = state.owner_pid;
    let done = match &op {
        Op::Rename { from, to } => {
            rename(pid, from.trim_end_matches('/').as_bytes(), to.as_bytes())
        }
        Op::Rmdir { path } => rmdir(pid, path.trim_end_matches('/').as_bytes(), true),
        Op::Unlink { path } => unlink(pid, path.as_bytes()),
        Op::Chmod { path, writable } => {
            let mode = if *writable { MODE_RW } else { MODE_RO };
            chmod(pid, path.trim_end_matches('/').as_bytes(), mode)
        }
    };
    refresh(state);
    state.status = if done.is_ok() { b"undone" } else { b"undo failed" };
    EventOutcome::Repaint
}
