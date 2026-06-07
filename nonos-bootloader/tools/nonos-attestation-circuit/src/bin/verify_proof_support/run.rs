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

use super::args::Args;
use super::binding::binding;
use super::constants::GROTH16_PROOF_SIZE;
use super::parse_public_inputs::parse_public_inputs;
use super::read_capsule::read_capsule;
use super::read_file_input::read_file_input;
use super::read_vk::read_vk;
use super::verify_groth16::verify_groth16;

pub fn run() -> Result<(), String> {
    let args = Args::parse();
    let vk = read_vk(&args.verifying_key)?;
    let input = match &args.capsule {
        Some(path) => read_capsule(path)?,
        None => read_file_input(&args)?,
    };
    if input.proof.len() != GROTH16_PROOF_SIZE {
        return Err(format!("proof size {} != {}", input.proof.len(), GROTH16_PROOF_SIZE));
    }
    let fields = parse_public_inputs(&input.public_inputs)?;
    binding(&input.public_inputs, input.body_hash, input.trailer_commitment)?;
    if !verify_groth16(&vk, &input.proof, &fields)? {
        println!("RESULT: FAIL");
        return Err("groth16 verification failed".into());
    }
    println!("RESULT: PASS");
    println!("proof_bytes: {}", input.proof.len());
    println!("public_inputs: {}", fields.len());
    Ok(())
}
