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

use nonos_libc::mk_time_millis;

use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

use super::table::{JobRecord, JobState};
use super::JobEnv;

// Reaps jobs that finished this tick: a done background job gets a "[n]
// Done cmdline" notice; the one done foreground job (if any) closes the
// open block, merges its env snapshot back, and clears `fg_running`.
pub(super) fn reap(state: &mut State) {
    let fg_id =
        state.jobs.iter().find(|j| !j.background && j.state == JobState::Done).map(|j| j.id);
    let mut fg_env = fg_id.and_then(|id| state.jobs.get_mut(id)).map(take_env);
    for (id, cmdline, status, background) in state.jobs.drop_done() {
        if background {
            notice_bg_done(state, id, &cmdline);
        } else if let Some(env) = fg_env.take() {
            let elapsed = (mk_time_millis() - state.fg_started_ms).clamp(0, u32::MAX as i64) as u32;
            state.close_block(status == 0, elapsed);
            env.merge_back(state);
            state.fg_running = false;
            state.last_status = status;
        }
    }
}

fn take_env(job: &mut JobRecord) -> JobEnv {
    let empty = JobEnv { cwd: Vec::new(), vars: Vec::new(), aliases: Vec::new() };
    core::mem::replace(&mut job.env, empty)
}

fn notice_bg_done(state: &mut State, id: u32, cmdline: &[u8]) {
    let mut num = [0u8; 20];
    let nk = format_u64(id as u64, &mut num);
    let mut msg = [0u8; 128];
    let mut k = 0;
    msg[k] = b'[';
    k += 1;
    k += copy_into(&mut msg[k..], &num[..nk]);
    k += copy_into(&mut msg[k..], b"] Done ");
    k += copy_into(&mut msg[k..], cmdline);
    state.scrollback.push_line(&msg[..k]);
}
