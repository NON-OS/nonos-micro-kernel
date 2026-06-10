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

use serde_json::Value;

use nonos_attestation_circuit::nox::{hex32, receipt_id, record_evidence_hash};
use nonos_attestation_circuit::CeremonyTranscript;

use super::fields::{expect, number, text};

pub fn verify_ceremony(
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
