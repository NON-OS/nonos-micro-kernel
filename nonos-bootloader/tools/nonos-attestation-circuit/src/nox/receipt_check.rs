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

//! The one verification path for every receipt kind. The CLI checker
//! and the submission server both call this, so a receipt accepted
//! anywhere was accepted by exactly this code.

use std::path::Path;

use serde_json::Value;

use crate::ceremony::CeremonyTranscript;

use super::fields::{expect, number, text};
use super::{
    check_work, circuit_id, hash_file, hex32, receipt_id, record_evidence_hash, verifier_hash,
    work_evidence_hash, work_receipt_id, WorkKind,
};

pub fn check_receipt(
    receipt: &Value,
    vk_path: &Path,
    transcript_path: &Path,
    artifact_path: Option<&Path>,
) -> Result<[u8; 32], String> {
    let vk_sha256 = hash_file(vk_path).map_err(|e| e.to_string())?;
    let transcript_sha256 = hash_file(transcript_path).map_err(|e| e.to_string())?;
    expect("vk_sha256", text(receipt, "vk_sha256")?, &hex32(&vk_sha256))?;
    expect("transcript_sha256", text(receipt, "transcript_sha256")?, &hex32(&transcript_sha256))?;
    let bytes = std::fs::read(transcript_path).map_err(|e| e.to_string())?;
    let transcript: CeremonyTranscript =
        serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let circuit = circuit_id(&transcript.metadata.circuit_name, &vk_sha256, &transcript_sha256);
    expect("circuit_id", text(receipt, "circuit_id")?, &hex32(&circuit))?;
    let kind = text(receipt, "kind")?.to_string();
    let evidence = if kind == "CEREMONY_ROUND" {
        check_ceremony(receipt, &transcript, &circuit)?
    } else {
        let artifact = artifact_path.ok_or("work receipts require the artifact")?;
        check_work_receipt(
            receipt,
            WorkKind::parse(&kind)?,
            artifact,
            vk_path,
            &vk_sha256,
            &circuit,
        )?
    };
    let rid_hex = text(receipt, "receipt_id")?;
    let mut rid = [0u8; 32];
    if rid_hex.len() != 66 || !rid_hex.starts_with("0x") {
        return Err("receipt_id is not a 32-byte 0x hex value".into());
    }
    hex::decode_to_slice(&rid_hex[2..], &mut rid).map_err(|e| e.to_string())?;
    Ok(verifier_hash(&rid, &evidence))
}

fn check_ceremony(
    receipt: &Value,
    transcript: &CeremonyTranscript,
    circuit: &[u8; 32],
) -> Result<[u8; 32], String> {
    if !transcript.metadata.finalized || !transcript.verification_passed {
        return Err("ceremony transcript is not finalized and verified".into());
    }
    let round = number(receipt, "round")? as u32;
    let record = transcript
        .contributions
        .iter()
        .find(|r| r.round == round)
        .ok_or_else(|| format!("round {round} not found in transcript"))?;
    let evidence = record_evidence_hash(record).map_err(|e| e.to_string())?;
    expect("evidence_hash", text(receipt, "evidence_hash")?, &hex32(&evidence))?;
    let contributor = text(receipt, "contributor_address")?;
    let rid = receipt_id(circuit, &evidence, contributor, round);
    expect("receipt_id", text(receipt, "receipt_id")?, &hex32(&rid))?;
    Ok(evidence)
}

fn check_work_receipt(
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
