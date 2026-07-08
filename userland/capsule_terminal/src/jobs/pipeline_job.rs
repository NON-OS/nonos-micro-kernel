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

use crate::command::dispatch::run_stage;
use crate::command::output::Output;
use crate::term::state::State;

use super::table::JobProgress;

// A pipeline whose stages were parsed once at submit time. Each
// `step_pipeline` call runs exactly one stage to completion (a long stage
// such as ping or install blocks for the whole of that stage in v1 — it is
// never sub-stepped) and folds its output into `buffer` for the next
// stage. The final stage's output is written to the live scrollback via
// `out`; earlier stages' buffers stay internal.
pub struct PipelineJob {
    stages: Vec<Vec<Vec<u8>>>,
    cursor: usize,
    buffer: Vec<Vec<u8>>,
}

impl PipelineJob {
    pub fn new(stages: Vec<Vec<Vec<u8>>>) -> Self {
        Self { stages, cursor: 0, buffer: Vec::new() }
    }
}

pub fn step_pipeline(job: &mut PipelineJob, state: &mut State) -> JobProgress {
    if job.cursor >= job.stages.len() {
        return JobProgress::Done(state.last_status);
    }
    let seg: Vec<&[u8]> = job.stages[job.cursor].iter().map(Vec::as_slice).collect();
    let buffer = core::mem::take(&mut job.buffer);
    let lines = run_stage(state, &seg, buffer);
    job.cursor += 1;
    if job.cursor < job.stages.len() {
        job.buffer = lines;
        return JobProgress::Running;
    }
    let mut out = Output::new(&mut state.scrollback);
    for line in &lines {
        out.writeln(line);
    }
    JobProgress::Done(state.last_status)
}
