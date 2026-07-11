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

use crate::term::state::State;

use super::env::JobEnv;
use super::work::JobWork;

// Snapshot the shell environment the job needs and register it in the
// table. Every job, background or foreground, gets its own `JobEnv`; only
// a foreground job's env is merged back into `State` once it reaps, so a
// background job's `cd`/`set` never leaks into the interactive shell.
pub fn submit(state: &mut State, cmdline: &[u8], background: bool, work: JobWork) -> u32 {
    let env = JobEnv::snapshot(state);
    state.jobs.add(cmdline, background, work, env)
}
