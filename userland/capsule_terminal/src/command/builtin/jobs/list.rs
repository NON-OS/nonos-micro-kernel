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

use crate::command::output::Output;
use crate::jobs::{JobState, JobTable};
use crate::term::util::{copy_into, format_u64};

pub fn run(out: &mut Output<'_>, jobs: &JobTable, _argv: &[&[u8]]) {
    for job in jobs.iter() {
        let mut line = [0u8; 128];
        let mut n = 0;
        line[n] = b'[';
        n += 1;
        let mut num = [0u8; 20];
        let nk = format_u64(job.id as u64, &mut num);
        n += copy_into(&mut line[n..], &num[..nk]);
        n += copy_into(&mut line[n..], b"] ");
        let label: &[u8] = if job.state == JobState::Running { b"Running " } else { b"Done " };
        n += copy_into(&mut line[n..], label);
        n += copy_into(&mut line[n..], &job.cmdline);
        out.writeln(&line[..n]);
    }
}
