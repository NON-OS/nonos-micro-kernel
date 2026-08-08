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

use super::env::JobEnv;
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
    pub env: JobEnv,
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

    pub fn add(&mut self, cmdline: &[u8], background: bool, work: JobWork, env: JobEnv) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.jobs.push(JobRecord {
            id,
            cmdline: cmdline.to_vec(),
            background,
            cancel: false,
            status: 0,
            state: JobState::Running,
            env,
            work,
        });
        id
    }

    pub fn foreground(&self) -> Option<u32> {
        self.jobs.iter().rev().find(|j| !j.background && j.state == JobState::Running).map(|j| j.id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut JobRecord> {
        self.jobs.iter_mut().find(|j| j.id == id)
    }

    pub fn get(&self, id: u32) -> Option<&JobRecord> {
        self.jobs.iter().find(|j| j.id == id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &JobRecord> {
        self.jobs.iter()
    }

    pub fn drop_done(&mut self) -> Vec<(u32, Vec<u8>, i32, bool)> {
        let mut done = Vec::new();
        self.jobs.retain(|j| {
            if j.state == JobState::Done {
                done.push((j.id, j.cmdline.clone(), j.status, j.background));
                false
            } else {
                true
            }
        });
        done
    }
}
