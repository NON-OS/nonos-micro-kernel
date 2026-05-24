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

extern crate alloc;

use spin::Mutex;

#[derive(Debug, Clone)]
pub struct SyscallAuditEntry {
    pub timestamp_ms: u64,
    pub syscall_num: u64,
    pub syscall_name: &'static str,
    pub pid: u32,
    pub result: i64,
    pub args: [u64; 4],
    pub success: bool,
}

pub(super) static AUDIT_LOG: Mutex<AuditLog> = Mutex::new(AuditLog::new());

pub(super) struct AuditLog {
    entries: [Option<SyscallAuditEntry>; 256],
    head: usize,
    count: usize,
}

impl AuditLog {
    pub(super) const fn new() -> Self {
        const NONE: Option<SyscallAuditEntry> = None;
        Self { entries: [NONE; 256], head: 0, count: 0 }
    }

    pub(super) fn push(&mut self, entry: SyscallAuditEntry) {
        self.entries[self.head] = Some(entry);
        self.head = (self.head + 1) % 256;
        if self.count < 256 {
            self.count += 1;
        }
    }
}

// Returns up to `max_entries` of the most recently appended records,
// newest first.
pub fn get_audit_log(max_entries: usize) -> alloc::vec::Vec<SyscallAuditEntry> {
    let log = AUDIT_LOG.lock();
    let mut result = alloc::vec::Vec::with_capacity(max_entries.min(log.count));
    let start = if log.count >= 256 { log.head } else { 0 };
    for i in 0..log.count.min(max_entries) {
        let idx = (start + log.count - 1 - i) % 256;
        if let Some(entry) = &log.entries[idx] {
            result.push(entry.clone());
        }
    }
    result
}

pub fn clear_audit_log() {
    let mut log = AUDIT_LOG.lock();
    *log = AuditLog::new();
}
