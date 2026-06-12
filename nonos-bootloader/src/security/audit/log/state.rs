// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::hash::compute_entry_hash;
use super::verify::verify_log_entries;
use crate::security::audit::types::{AuditEntry, AuditEvent, AUDIT_MSG_LEN};

const MAX_ENTRIES: usize = 64;

pub struct AuditLog {
    entries: [AuditEntry; MAX_ENTRIES],
    count: usize,
    running_hash: [u8; 32],
    sealed: bool,
}

impl AuditLog {
    pub const fn new() -> Self {
        Self {
            entries: [AuditEntry::empty(); MAX_ENTRIES],
            count: 0,
            running_hash: [0u8; 32],
            sealed: false,
        }
    }

    pub fn record(&mut self, event: AuditEvent, ts: u64, msg: &[u8]) {
        if self.sealed || self.count >= MAX_ENTRIES {
            return;
        }
        let mut e = AuditEntry::empty();
        e.event = event;
        e.timestamp = ts;
        e.msg_len = msg.len().min(AUDIT_MSG_LEN);
        e.message[..e.msg_len].copy_from_slice(&msg[..e.msg_len]);
        e.chain_hash = compute_entry_hash(&self.running_hash, &e);
        self.running_hash = e.chain_hash;
        self.entries[self.count] = e;
        self.count += 1;
    }

    pub fn seal(&mut self) {
        self.sealed = true;
    }
    pub fn get_final_hash(&self) -> [u8; 32] {
        self.running_hash
    }
    pub fn get_count(&self) -> usize {
        self.count
    }
    pub fn get_entry(&self, i: usize) -> Option<&AuditEntry> {
        self.entries.get(i).filter(|_| i < self.count)
    }
    pub fn verify_integrity(&self) -> bool {
        verify_log_entries(&self.entries, self.count)
    }
}
