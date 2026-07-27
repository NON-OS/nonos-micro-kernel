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

use crate::command::builtin::nox::install;
use crate::command::builtin::ping;
use crate::command::builtin::tool;
use crate::command::dispatch::split_stages;
use crate::command::output::Output;
use crate::term::state::State;

use super::pipeline_job::PipelineJob;
use super::work::JobWork;

// `Instant` statements run through the untouched synchronous dispatch;
// `Job` carries the prepared work to submit; `Handled` means the command
// was job-eligible but its setup already failed and reported (a second
// synchronous run would repeat the error and its wait), so the statement
// is consumed with `state.last_status` left as the setup decided.
pub enum Verdict {
    Instant,
    Job(JobWork),
    Handled,
}

// Long-running builtins that must not block the event loop: `ping` (a
// network round trip) and `install` (an installer call plus stdout
// drain). A plain invocation of either becomes a stepped job directly;
// one piped into a pipeline (`ping x | grep y`) becomes a stepped
// `PipelineStages` job instead, so it does not block behind `command::run`.
// Any other non-plain statement (redirects, or an all-instant pipeline)
// stays on the existing synchronous dispatch.
pub fn is_job_command(state: &mut State, args: &[&[u8]]) -> Verdict {
    if args.is_empty() {
        return Verdict::Instant;
    }
    if !is_plain(args) {
        return match pipeline_stages(args) {
            Some(stages) => Verdict::Job(JobWork::PipelineStages(PipelineJob::new(stages))),
            None => Verdict::Instant,
        };
    }
    match args[0] {
        b"ping" => {
            let mut out = Output::new(&mut state.scrollback);
            match ping::prepare(&mut out, args) {
                Some(job) => Verdict::Job(JobWork::Ping(job)),
                None => Verdict::Handled,
            }
        }
        b"install" => match install::prepare(state, &args[1..]) {
            Some(job) => Verdict::Job(JobWork::InstallDrain(job)),
            None => {
                state.last_status = 1;
                Verdict::Handled
            }
        },
        // Bare-name run of a store tool staged in the vfs (`sd a b`, ...): route
        // it through the same async installer path as `install <name>`, so the
        // tool loads and streams its output without blocking the event loop.
        // The tool name is args[0], which prepare reads as the capsule stem.
        name if STORE_TOOLS.contains(&name) => match install::prepare(state, args) {
            Some(job) => Verdict::Job(JobWork::InstallDrain(job)),
            None => {
                state.last_status = 1;
                Verdict::Handled
            }
        },
        // Bare-name run of a baked, attested tool (`tokei`, `grex foo`, ...): the
        // kernel spawns it parented to this terminal and it streams its stdout
        // through the same drain job.
        name if tool::is_tool(name) => match tool::prepare(state, args) {
            Some(job) => Verdict::Job(JobWork::InstallDrain(job)),
            None => Verdict::Handled,
        },
        _ => Verdict::Instant,
    }
}

// The CLI tools staged in the vfs store. A bare invocation of one of these runs
// it from the store instead of falling through to "unknown verb".
const STORE_TOOLS: &[&[u8]] = &[b"sd", b"tokio-smoke", b"std_proof"];

fn is_plain(args: &[&[u8]]) -> bool {
    !args.iter().any(|a| matches!(*a, b"|" | b">" | b">>" | b"<"))
}

// A pure pipeline (no redirects) whose stages include a long command
// becomes owned, parsed stages for a `PipelineStages` job. Anything with a
// redirect, or a pipeline with no long stage, returns `None` so the caller
// falls back to `Verdict::Instant`.
fn pipeline_stages(args: &[&[u8]]) -> Option<Vec<Vec<Vec<u8>>>> {
    if args.iter().any(|a| matches!(*a, b">" | b">>" | b"<")) {
        return None;
    }
    if !args.iter().any(|a| *a == b"|") {
        return None;
    }
    let segments = split_stages(args);
    if !segments.iter().any(|seg| is_long_stage(seg)) {
        return None;
    }
    let mut stages: Vec<Vec<Vec<u8>>> = Vec::new();
    for seg in &segments {
        let tokens: Vec<Vec<u8>> = seg.iter().map(|tok| tok.to_vec()).collect();
        stages.push(tokens);
    }
    Some(stages)
}

fn is_long_stage(seg: &[&[u8]]) -> bool {
    matches!(seg.first().copied(), Some(b"ping") | Some(b"install"))
}
