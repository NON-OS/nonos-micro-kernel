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

use crate::command::builtin::nox::install;
use crate::command::builtin::ping;
use crate::command::output::Output;
use crate::term::state::State;

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
// drain). Everything else, and any `ping`/`install` invocation that is
// piped or redirected, stays on the existing synchronous dispatch.
// External processes and long pipelines extend this match once their
// `JobWork` variants exist.
pub fn is_job_command(state: &mut State, args: &[&[u8]]) -> Verdict {
    if args.is_empty() || !is_plain(args) {
        return Verdict::Instant;
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
                state.last_status = false;
                Verdict::Handled
            }
        },
        _ => Verdict::Instant,
    }
}

fn is_plain(args: &[&[u8]]) -> bool {
    !args.iter().any(|a| matches!(*a, b"|" | b">" | b">>" | b"<"))
}
