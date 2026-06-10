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

use spin::Mutex;

use super::state::AuditLog;
use crate::security::audit::types::AuditEvent;

pub static AUDIT_LOG: Mutex<AuditLog> = Mutex::new(AuditLog::new());

pub fn audit(event: AuditEvent, timestamp: u64, message: &[u8]) {
    let mut log = AUDIT_LOG.lock();
    log.record(event, timestamp, message);
}

pub fn audit_alert(timestamp: u64, message: &[u8]) {
    audit(AuditEvent::SecurityAlert, timestamp, message);
}

pub fn seal_audit_log() {
    let mut log = AUDIT_LOG.lock();
    log.seal();
}
pub fn verify_audit_integrity() -> bool {
    let log = AUDIT_LOG.lock();
    log.verify_integrity()
}
pub fn get_audit_hash() -> [u8; 32] {
    let log = AUDIT_LOG.lock();
    log.get_final_hash()
}
