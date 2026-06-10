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

use nonos_attestation_circuit::nox::{hash_file, validate_address, write_json};
use nonos_attestation_circuit::CeremonyTranscript;

use super::args::parse;
use super::build_receipt::build_receipt;
use super::find_record::find_record;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse();
    validate_address(&args.contributor)?;
    let transcript_path = Path::new(&args.transcript);
    let transcript_bytes = std::fs::read(transcript_path)?;
    let transcript: CeremonyTranscript = serde_json::from_slice(&transcript_bytes)?;
    let record = find_record(&transcript, args.round)?;
    let vk_hash = hash_file(Path::new(&args.verifying_key))?;
    let transcript_hash = hash_file(transcript_path)?;
    let receipt = build_receipt(
        &transcript,
        &record,
        &vk_hash,
        &transcript_hash,
        &args.contributor,
        &args.uri,
    )?;
    write_json(Path::new(&args.out), &receipt)?;
    println!("{}", receipt.receipt_id);
    Ok(())
}
