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

use super::super::types::*;
use super::state::PROOF_SYSTEM;
use crate::crypto::util::constant_time::ct_eq_32;

pub fn get_capsule_info(capsule_id: u64) -> Result<CapsuleInfo, &'static str> {
    let capsules = PROOF_SYSTEM.capsules.read();
    match capsules.get(&capsule_id) {
        Some(capsule) => Ok(CapsuleInfo {
            id: capsule.capsule_id,
            start: capsule.memory_region.start.as_u64(),
            end: capsule.memory_region.end.as_u64(),
            tag: capsule.memory_region.tag,
            sealed: capsule.permissions.sealed,
            creation_time: capsule.creation_time,
        }),
        None => Err("Capsule not found"),
    }
}

pub fn get_proof_stats() -> ProofStats {
    ProofStats {
        total_capsules: PROOF_SYSTEM.capsules.read().len(),
        total_proofs: PROOF_SYSTEM.proofs.read().len(),
        audit_entries: PROOF_SYSTEM.audit_log.lock().len(),
    }
}

pub fn destroy_capsule(capsule_id: u64) -> Result<(), &'static str> {
    let mut capsules = PROOF_SYSTEM.capsules.write();
    match capsules.remove(&capsule_id) {
        Some(_) => {
            PROOF_SYSTEM.audit(AuditOperation::Destroy, capsule_id, AuditResult::Success);
            Ok(())
        }
        None => {
            PROOF_SYSTEM.audit(AuditOperation::Destroy, capsule_id, AuditResult::Failure);
            Err("Capsule not found")
        }
    }
}

pub fn unseal_capsule(capsule_id: u64, access_key: &[u8; 32]) -> Result<(), &'static str> {
    let mut capsules = PROOF_SYSTEM.capsules.write();
    match capsules.get_mut(&capsule_id) {
        Some(capsule) if capsule.permissions.sealed => {
            if ct_eq_32(&capsule.access_key, access_key) {
                capsule.permissions.sealed = false;
                PROOF_SYSTEM.audit(AuditOperation::Unseal, capsule_id, AuditResult::Success);
                Ok(())
            } else {
                PROOF_SYSTEM.audit(AuditOperation::Unseal, capsule_id, AuditResult::Violation);
                Err("Invalid access key")
            }
        }
        Some(_) => {
            PROOF_SYSTEM.audit(AuditOperation::Unseal, capsule_id, AuditResult::Failure);
            Err("Capsule not sealed")
        }
        None => {
            PROOF_SYSTEM.audit(AuditOperation::Unseal, capsule_id, AuditResult::Failure);
            Err("Capsule not found")
        }
    }
}
