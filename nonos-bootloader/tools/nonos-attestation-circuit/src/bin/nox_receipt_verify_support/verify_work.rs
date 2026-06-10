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

use std::path::Path;

use serde_json::Value;

use nonos_attestation_circuit::nox::{
    check_work, hex32, work_evidence_hash, work_receipt_id, WorkKind,
};

use super::fields::{expect, number, text};

pub fn verify_work(
    receipt: &Value,
    kind: WorkKind,
    artifact_path: &Path,
    vk_path: &Path,
    vk_sha256: &[u8; 32],
    circuit: &[u8; 32],
) -> Result<[u8; 32], String> {
    let artifact = std::fs::read(artifact_path).map_err(|e| format!("read artifact: {e}"))?;
    let attested = check_work(kind, artifact_path, &artifact, vk_path, vk_sha256)?;
    if attested != number(receipt, "attested_count")? {
        return Err("attested_count mismatch against the artifact".into());
    }
    let evidence = work_evidence_hash(kind.as_str(), &artifact);
    expect("evidence_hash", text(receipt, "evidence_hash")?, &hex32(&evidence))?;
    let contributor = text(receipt, "contributor_address")?;
    let epoch = number(receipt, "claim_epoch")?;
    let rid = work_receipt_id(circuit, &evidence, contributor, kind.as_str(), epoch);
    expect("receipt_id", text(receipt, "receipt_id")?, &hex32(&rid))?;
    Ok(evidence)
}
