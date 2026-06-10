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

use super::event::{AuditEvent, AUDIT_MSG_LEN};

#[derive(Clone, Copy)]
pub struct AuditEntry {
    pub event: AuditEvent,
    pub timestamp: u64,
    pub message: [u8; AUDIT_MSG_LEN],
    pub msg_len: usize,
    pub chain_hash: [u8; 32],
}

impl AuditEntry {
    pub const fn empty() -> Self {
        Self {
            event: AuditEvent::BootStart,
            timestamp: 0,
            message: [0u8; AUDIT_MSG_LEN],
            msg_len: 0,
            chain_hash: [0u8; 32],
        }
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        let mut buf = [0u8; 64];
        buf[0] = self.event as u8;
        buf[1..9].copy_from_slice(&self.timestamp.to_le_bytes());
        buf[9] = self.msg_len as u8;
        let copy_len = self.msg_len.min(AUDIT_MSG_LEN).min(54);
        buf[10..10 + copy_len].copy_from_slice(&self.message[..copy_len]);
        buf
    }
}
