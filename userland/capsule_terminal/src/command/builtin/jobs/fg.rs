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

use nonos_libc::mk_time_millis;

use crate::command::output::Output;
use crate::jobs::JobState;
use crate::term::state::State;
use crate::term::util::{copy_into, format_u64};

// Every error return sets `state.last_status = 1` so a chained `fg 99 &&
// rm ...` sees the failure instead of the stale 0 `on_enter` primed before
// dispatch; the success path sets it back to 0 explicitly.
fn err(state: &mut State, msg: &[u8]) {
    Output::new(&mut state.scrollback).writeln(msg);
    state.last_status = 1;
}

// Flips a background job to foreground so `on_tick`'s pump treats it like
// a job submitted fg (prompt suppressed, Ctrl-C applies). Rejects the
// two-fg-jobs case flagged in Task 12's review by refusing when another
// job already owns `state.fg_running`.
pub fn run(state: &mut State, argv: &[&[u8]]) {
    if argv.len() < 2 {
        return err(state, b"usage: fg <id>");
    }
    let id = match core::str::from_utf8(argv[1]).ok().and_then(|s| s.parse::<u32>().ok()) {
        Some(id) => id,
        None => return err(state, b"fg: invalid job id"),
    };
    let fg_running = state.fg_running;
    let now = mk_time_millis();
    let job = match state.jobs.get_mut(id) {
        Some(job) => job,
        None => return err(state, b"fg: no such job"),
    };
    if job.state == JobState::Done {
        return err(state, b"fg: job has already finished");
    }
    if !job.background {
        return err(state, b"fg: job already in foreground");
    }
    if fg_running {
        return err(state, b"fg: a job is already in the foreground");
    }
    job.background = false;
    let mut line = [0u8; 128];
    let mut n = 0;
    line[n] = b'[';
    n += 1;
    let mut num = [0u8; 20];
    let nk = format_u64(job.id as u64, &mut num);
    n += copy_into(&mut line[n..], &num[..nk]);
    n += copy_into(&mut line[n..], b"] ");
    n += copy_into(&mut line[n..], &job.cmdline);
    Output::new(&mut state.scrollback).writeln(&line[..n]);
    state.fg_running = true;
    state.fg_started_ms = now;
    state.last_status = 0;
}
