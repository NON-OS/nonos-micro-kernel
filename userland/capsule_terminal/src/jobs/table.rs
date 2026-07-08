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

use super::work::JobWork;

// Result of stepping a job's work by one bounded slice.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JobProgress {
    Running,
    Done(i32),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Running,
    Done,
}

pub struct JobRecord {
    pub id: u32,
    pub cmdline: Vec<u8>,
    pub background: bool,
    pub cancel: bool,
    pub status: i32,
    pub state: JobState,
    pub work: JobWork,
}

// Every job lives on the single event-loop thread; the table is stepped
// from `on_tick`, never shared across threads or interrupt context.
pub struct JobTable {
    jobs: Vec<JobRecord>,
    next_id: u32,
}

impl JobTable {
    pub fn new() -> Self {
        Self { jobs: Vec::new(), next_id: 1 }
    }

    pub fn add(&mut self, cmdline: &[u8], background: bool, work: JobWork) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.jobs.push(JobRecord {
            id,
            cmdline: cmdline.to_vec(),
            background,
            cancel: false,
            status: 0,
            state: JobState::Running,
            work,
        });
        id
    }
}
