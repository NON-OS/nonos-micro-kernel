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
    check_work, hash_file, validate_address, write_json, WorkKind,
};

use super::args::parse;
use super::build_receipt::build_receipt;
use super::load_circuit_name::load_circuit_name;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();
    validate_address(&args.contributor)?;
    let kind = WorkKind::parse(&args.kind)?;
    let artifact_path = Path::new(&args.artifact);
    let vk_path = Path::new(&args.verifying_key);
    let artifact = std::fs::read(artifact_path)?;
    let vk_sha256 = hash_file(vk_path)?;
    let transcript_sha256 = hash_file(Path::new(&args.transcript))?;
    let circuit_name = load_circuit_name(Path::new(&args.transcript))?;
    let attested_count = check_work(kind, artifact_path, &artifact, vk_path, &vk_sha256)?;
    let receipt = build_receipt(
        &args,
        kind,
        &circuit_name,
        &artifact,
        &vk_sha256,
        &transcript_sha256,
        attested_count,
    );
    write_json(Path::new(&args.out), &receipt)?;
    println!("{}", receipt.receipt_id);
    Ok(())
}
