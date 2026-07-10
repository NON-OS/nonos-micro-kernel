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

use nonos_app_skeleton::EventOutcome;

use crate::command::output::Output;
use crate::jobs::JobWork;
use crate::term::state::State;

// A key typed while a foreground job runs: if that job is an interactive
// external capsule, queue the byte on its stdin buffer (the on_tick pump feeds
// it to the child via mk_proc_input) and echo it locally so the typist sees
// what they entered. Any other foreground job swallows the key exactly as
// Task 13's gate did, returning `Idle`.
pub fn forward(state: &mut State, byte: u8) -> EventOutcome {
    let mut fed = false;
    if let Some(id) = state.jobs.foreground() {
        if let Some(job) = state.jobs.get_mut(id) {
            if let JobWork::ExternalStage { in_buf, .. } = &mut job.work {
                in_buf.push(byte);
                fed = true;
            }
        }
    }
    if fed {
        Output::new(&mut state.scrollback).feed_raw(&[byte]);
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}
