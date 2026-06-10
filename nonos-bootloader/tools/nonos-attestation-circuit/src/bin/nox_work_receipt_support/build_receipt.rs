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

use nonos_attestation_circuit::nox::{
    circuit_id, hex32, work_evidence_hash, work_receipt_id, WorkKind,
};
use nonos_attestation_circuit::POLICY_EPOCH;
use sha2::{Digest, Sha256};

use super::args::Args;
use super::receipt::WorkReceipt;

pub fn build_receipt(
    args: &Args,
    kind: WorkKind,
    circuit_name: &str,
    artifact: &[u8],
    vk_sha256: &[u8; 32],
    transcript_sha256: &[u8; 32],
    attested_count: u64,
) -> WorkReceipt {
    let evidence = work_evidence_hash(kind.as_str(), artifact);
    let circuit = circuit_id(circuit_name, vk_sha256, transcript_sha256);
    let receipt =
        work_receipt_id(&circuit, &evidence, &args.contributor, kind.as_str(), args.epoch);
    let mut artifact_sha = [0u8; 32];
    artifact_sha.copy_from_slice(&Sha256::digest(artifact));
    WorkReceipt {
        schema_version: 1,
        kind: kind.as_str(),
        chain_id: 1,
        circuit_name: circuit_name.to_string(),
        circuit_id: hex32(&circuit),
        contributor_address: args.contributor.to_ascii_lowercase(),
        claim_epoch: args.epoch,
        policy_epoch: POLICY_EPOCH,
        vk_sha256: hex32(vk_sha256),
        transcript_sha256: hex32(transcript_sha256),
        artifact_name: artifact_name(&args.artifact),
        artifact_sha256: hex32(&artifact_sha),
        attested_count,
        evidence_hash: hex32(&evidence),
        receipt_id: hex32(&receipt),
        uri: args.uri.clone(),
    }
}

fn artifact_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string())
}
