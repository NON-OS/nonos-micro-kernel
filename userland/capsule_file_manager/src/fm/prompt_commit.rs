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

use alloc::format;

use super::prompt_run_op::run_op;
use super::refresh::refresh;
use super::state::{Mode, PromptKind, State};

pub fn commit(state: &mut State, kind: PromptKind) {
    let name = core::mem::take(&mut state.input);
    state.mode = Mode::Browse;
    if name.is_empty() && !matches!(kind, PromptKind::Delete) {
        state.status = b"empty name";
        return;
    }
    let target = format!("{}{}", state.prefix, name);
    let msg = match run_op(state, kind, &name, &target) {
        Ok(msg) => msg,
        Err(e) => e.as_bytes(),
    };
    refresh(state);
    state.status = msg;
}
