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

use nonos_attestation_circuit::nox::{circuit_id, hex32, receipt_id, record_evidence_hash};
use nonos_attestation_circuit::{CeremonyTranscript, ContributionRecord, POLICY_EPOCH};

use super::receipt::Receipt;

pub fn build_receipt(
    tx: &CeremonyTranscript,
    rec: &ContributionRecord,
    vk: &[u8; 32],
    tsha: &[u8; 32],
    contributor: &str,
    uri: &str,
) -> Result<Receipt, Box<dyn std::error::Error>> {
    let evidence = record_evidence_hash(rec)?;
    let circuit = circuit_id(&tx.metadata.circuit_name, vk, tsha);
    let receipt = receipt_id(&circuit, &evidence, contributor, rec.round);
    Ok(Receipt {
        schema_version: 1,
        kind: "CEREMONY_ROUND",
        chain_id: 1,
        circuit_name: tx.metadata.circuit_name.clone(),
        circuit_id: hex32(&circuit),
        contributor: rec.contributor_id.clone(),
        contributor_address: contributor.to_ascii_lowercase(),
        round: rec.round,
        policy_epoch: POLICY_EPOCH,
        vk_sha256: hex32(vk),
        transcript_sha256: hex32(tsha),
        transcript_final_vk_hash: final_vk(tx),
        previous_params_hash: hex32(&rec.previous_params_hash),
        new_params_hash: hex32(&rec.new_params_hash),
        randomness_commitment: hex32(&rec.randomness_commitment),
        destruction_attestation_hash: rec
            .destruction_attestation
            .as_ref()
            .map(|d| hex32(&d.attestation_hash)),
        contribution_timestamp: rec.contribution_timestamp,
        evidence_hash: hex32(&evidence),
        receipt_id: hex32(&receipt),
        uri: uri.to_string(),
    })
}

fn final_vk(tx: &CeremonyTranscript) -> String {
    tx.final_vk_hash.as_ref().map(hex32).unwrap_or_default()
}
