// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use clap::Parser;
use nonos_attestation_circuit::{NonosAttestationCircuit, MIN_HW_LEVEL};

use super::args::Args;
use super::commitment::commitment;
use super::hash_file::hash_file;
use super::parse_caps::parse_caps;
use super::parse_hash::parse_hash;
use super::pcr::pcr;
use super::proof::proof;
use super::public_inputs::public_inputs;
use super::read_key::read_key;
use super::trailer::trailer;
use super::write_outputs::write_outputs;

pub fn run() -> Result<(), String> {
    let args = Args::parse();
    let key = read_key(&args.proving_key)?;
    let (capsule_hash, capsule_bytes) = hash_file(&args.capsule)?;
    let program_hash = match &args.program_hash {
        Some(value) => parse_hash(value)?,
        None => capsule_hash,
    };
    let caps = parse_caps(&args.capability_mask)?;
    let commitment = commitment(&capsule_hash, &program_hash, caps);
    let pcr = pcr(&args.seed, &capsule_hash, &commitment);
    let circuit = NonosAttestationCircuit::new(
        capsule_hash,
        program_hash,
        caps,
        commitment,
        pcr,
        MIN_HW_LEVEL + 0x1000,
    );
    let seed = public_inputs(&capsule_hash, &program_hash, caps, &commitment);
    let proof = proof(&key, circuit, &seed)?;
    let trailer = trailer(&proof, &seed, &commitment);
    write_outputs(&args, &proof, &seed, &trailer, &capsule_bytes)?;
    println!("RESULT: PASS");
    println!("proof_bytes: {}", proof.len());
    println!("public_inputs: {}", seed.len() / 32);
    println!("capsule_hash: {}", hex::encode(capsule_hash));
    println!("program_hash: {}", hex::encode(program_hash));
    println!("commitment: {}", hex::encode(commitment));
    Ok(())
}
