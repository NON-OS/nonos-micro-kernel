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

use nonos_libc::PROC_NAME_LEN;

// Severity of a finding, worst last so `max()` picks the headline level.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Info,
    Warn,
    Critical,
}

impl Level {
    pub fn label(self) -> &'static [u8] {
        match self {
            Level::Info => b"INFO",
            Level::Warn => b"WARN",
            Level::Critical => b"CRIT",
        }
    }
}

// One security finding about the running system. Every field is derived from
// the live kernel process table, never invented: `pid`/`name` point at the
// process the finding is about (pid 0 = system-wide), `msg` states the fact.
#[derive(Clone)]
pub struct Alert {
    pub level: Level,
    pub pid: u32,
    pub name: [u8; PROC_NAME_LEN],
    pub name_len: u8,
    pub msg: &'static [u8],
}

impl Alert {
    // Build a finding about `pid`/`name`. Use pid 0 with a service name for a
    // system-wide finding (for example a service that stopped running).
    pub fn about(level: Level, pid: u32, name: &[u8], msg: &'static [u8]) -> Self {
        let mut n = [0u8; PROC_NAME_LEN];
        let len = name.len().min(PROC_NAME_LEN);
        n[..len].copy_from_slice(&name[..len]);
        Alert { level, pid, name: n, name_len: len as u8, msg }
    }

    pub fn name(&self) -> &[u8] {
        &self.name[..(self.name_len as usize).min(PROC_NAME_LEN)]
    }
}
