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

use alloc::vec::Vec;

use crate::term::state::State;

// A job's private view of the shell environment it needs to run and, if it
// is a foreground job, hand changes back from. Background jobs are
// subshells: their snapshot is used to run but never merged back, so a `cd`
// or `set` inside a `&` job never leaks into the interactive shell.
pub struct JobEnv {
    pub cwd: Vec<u8>,
    pub vars: Vec<(Vec<u8>, Vec<u8>)>,
    pub aliases: Vec<(Vec<u8>, Vec<u8>)>,
}

impl JobEnv {
    pub fn snapshot(state: &State) -> Self {
        Self {
            cwd: state.cwd.as_bytes().to_vec(),
            vars: state.vars.clone(),
            aliases: state.aliases.clone(),
        }
    }

    pub fn merge_back(self, state: &mut State) {
        state.cwd.set(self.cwd);
        state.vars = self.vars;
        // Aliases were snapshotted with the rest and have to come back with
        // it, or one defined inside a foreground job is lost on return.
        state.aliases = self.aliases;
    }
}
