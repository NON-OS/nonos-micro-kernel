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

use crate::command::output::Output;
use crate::term::state::State;

use super::pipeline_job::step_pipeline;
use super::reap::reap;
use super::table::{JobProgress, JobState};
use super::work::{step, JobWork};

// Steps every live job one bounded slice into the real scrollback, then
// hands off to `reap`. With no running jobs this collects an empty,
// non-allocating Vec and returns immediately, leaving `on_tick`
// byte-identical to today.
pub fn pump(state: &mut State) -> bool {
    let ids: Vec<u32> =
        state.jobs.iter().filter(|j| j.state == JobState::Running).map(|j| j.id).collect();
    if ids.is_empty() {
        return false;
    }
    for id in ids {
        step_job(state, id);
    }
    reap(state);
    true
}

fn step_job(state: &mut State, id: u32) {
    let cancel = match state.jobs.get(id) {
        Some(job) => job.cancel,
        None => return,
    };
    if !cancel && matches!(state.jobs.get(id).map(|j| &j.work), Some(JobWork::PipelineStages(_))) {
        return step_pipeline_job(state, id);
    }
    if let Some(job) = state.jobs.get_mut(id) {
        let mut out = Output::new(&mut state.scrollback);
        if let JobProgress::Done(status) = step(&mut job.work, &mut out, job.cancel) {
            job.status = status;
            job.state = JobState::Done;
        }
    }
}

// `PipelineStages` needs `&mut State` (to run non-filter stages through
// `exec`), which `step` above does not take. The work is moved out of the
// job record so `state` is fully free for `step_pipeline`, then moved back
// in with the resulting status applied.
fn step_pipeline_job(state: &mut State, id: u32) {
    let mut work = match state.jobs.get_mut(id) {
        Some(job) => core::mem::replace(&mut job.work, JobWork::Noop),
        None => return,
    };
    let progress = match &mut work {
        JobWork::PipelineStages(pj) => step_pipeline(pj, state),
        _ => JobProgress::Running,
    };
    if let Some(job) = state.jobs.get_mut(id) {
        job.work = work;
        if let JobProgress::Done(status) = progress {
            job.status = status;
            job.state = JobState::Done;
        }
    }
}
