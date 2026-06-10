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

use nonos_attestation_circuit::nox::{circuit_id, hash_file, hex32, write_json};
use nonos_attestation_circuit::{
    compute_cargo_lock_hash, compute_source_tree_hash, CeremonyTranscript, POLICY_EPOCH,
};

use super::args::parse;
use super::entry::RegistryEntry;
use super::git_hash::git_hash32;
use super::layout_hash::layout_hash;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();
    let circuit_dir = Path::new(&args.circuit_dir);
    let commit = git_hash32(circuit_dir, "HEAD")?;
    let tree = git_hash32(circuit_dir, "HEAD^{tree}")?;
    let source_tree = compute_source_tree_hash(&commit, &tree);
    let cargo_lock = compute_cargo_lock_hash(&std::fs::read(&args.cargo_lock)?);
    let vk_sha256 = hash_file(Path::new(&args.verifying_key))?;
    let transcript_path = Path::new(&args.transcript);
    let transcript_sha256 = hash_file(transcript_path)?;
    let transcript: CeremonyTranscript = serde_json::from_slice(&std::fs::read(transcript_path)?)?;
    if !transcript.metadata.finalized || !transcript.verification_passed {
        return Err("ceremony transcript is not finalized and verified".into());
    }
    let circuit = circuit_id(&transcript.metadata.circuit_name, &vk_sha256, &transcript_sha256);
    let entry = RegistryEntry {
        schema_version: 1,
        circuit_name: transcript.metadata.circuit_name.clone(),
        circuit_id: hex32(&circuit),
        source_tree_hash: hex32(&source_tree),
        cargo_lock_hash: hex32(&cargo_lock),
        public_input_layout_hash: hex32(&layout_hash()),
        vk_sha256: hex32(&vk_sha256),
        transcript_sha256: hex32(&transcript_sha256),
        policy_epoch: POLICY_EPOCH,
        status: "active",
        uri: args.uri.clone(),
    };
    write_json(Path::new(&args.out), &entry)?;
    println!("{}", entry.circuit_id);
    Ok(())
}
