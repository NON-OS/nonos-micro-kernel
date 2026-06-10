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

use nonos_attestation_circuit::nox::{circuit_id, hash_file, hex32, verifier_hash, WorkKind};
use nonos_attestation_circuit::CeremonyTranscript;

use super::args::parse;
use super::fields::{expect, text};
use super::verify_ceremony::verify_ceremony;
use super::verify_work::verify_work;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();
    let receipt: serde_json::Value = serde_json::from_slice(&std::fs::read(&args.receipt)?)?;
    let vk_path = Path::new(&args.verifying_key);
    let transcript_path = Path::new(&args.transcript);
    let vk_sha256 = hash_file(vk_path)?;
    let transcript_sha256 = hash_file(transcript_path)?;
    expect("vk_sha256", text(&receipt, "vk_sha256")?, &hex32(&vk_sha256))?;
    expect("transcript_sha256", text(&receipt, "transcript_sha256")?, &hex32(&transcript_sha256))?;
    let transcript: CeremonyTranscript = serde_json::from_slice(&std::fs::read(transcript_path)?)?;
    let circuit = circuit_id(&transcript.metadata.circuit_name, &vk_sha256, &transcript_sha256);
    expect("circuit_id", text(&receipt, "circuit_id")?, &hex32(&circuit))?;
    let kind = text(&receipt, "kind")?.to_string();
    let evidence = if kind == "CEREMONY_ROUND" {
        verify_ceremony(&receipt, &transcript, &circuit)?
    } else {
        let artifact = args.artifact.as_ref().ok_or("work receipts require --artifact")?;
        let work = WorkKind::parse(&kind)?;
        verify_work(&receipt, work, Path::new(artifact), vk_path, &vk_sha256, &circuit)?
    };
    let rid_hex = text(&receipt, "receipt_id")?;
    let mut rid = [0u8; 32];
    hex::decode_to_slice(&rid_hex[2..], &mut rid)?;
    println!("{}", hex32(&verifier_hash(&rid, &evidence)));
    Ok(())
}
