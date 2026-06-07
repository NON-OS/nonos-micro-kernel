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

use std::fs;

use anyhow::{Context, Result};
use clap::Parser;

use embed_zk_proof::*;

fn main() -> Result<()> {
    let args = Args::parse();
    run_attestation(&args)
}

fn run_attestation(args: &Args) -> Result<()> {
    println!("=== NONOS ZK Attestation Prover ===\n");

    let kernel = load_signed_kernel(&args.input)?;
    println!("Signed kernel: {} bytes", kernel.raw_bytes.len());
    println!("Kernel code: {} bytes", kernel.kernel_bytes.len());

    let kernel_hash = compute_kernel_hash(&kernel.kernel_bytes);
    println!("Kernel BLAKE3: {}", hex::encode(kernel_hash));

    let boot_nonce = generate_boot_nonce(&args.seed, &kernel_hash);
    let machine_id = generate_machine_id(&args.seed);
    println!("Boot nonce: {}", hex::encode(&boot_nonce[..8]));
    println!("Machine ID: {}", hex::encode(&machine_id[..8]));

    let pk = load_proving_key(&args.proving_key)?;
    println!("Proving key loaded\n");

    let params = create_circuit_params(&kernel.kernel_bytes, &args.seed, &boot_nonce, &machine_id);
    println!("Program hash: {}", hex::encode(params.program_hash));
    println!("Capsule commitment: {}", hex::encode(params.capsule_commitment));

    println!("\nGenerating Groth16 proof...");
    let proof_bytes = generate_proof(&pk, &params)?;
    println!("Proof generated: {} bytes", proof_bytes.len());

    let public_inputs = extract_public_inputs(&params)?;
    println!("Public inputs: {} bytes\n", public_inputs.len());

    let block_params = ZkBlockParams {
        program_hash: &params.program_hash,
        capsule_commitment: &params.capsule_commitment,
        kernel_hash: &params.kernel_hash,
        boot_nonce: &params.boot_nonce,
        machine_id: &params.machine_id,
        public_inputs: &public_inputs,
        proof_blob: &proof_bytes,
    };
    let zk_block = create_zk_block(&block_params);

    let image = assemble_attested_image(&kernel, zk_block);
    fs::write(&args.output, &image.data)
        .with_context(|| format!("Failed to write: {}", args.output.display()))?;

    print_summary(&args, &image);
    Ok(())
}

fn generate_boot_nonce(seed: &str, kernel_hash: &[u8; 32]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:BOOT:NONCE:v1");
    hasher.update(seed.as_bytes());
    hasher.update(kernel_hash);
    hasher.update(
        &std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_le_bytes(),
    );
    *hasher.finalize().as_bytes()
}

fn generate_machine_id(seed: &str) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new_derive_key("NONOS:MACHINE:ID:v1");
    hasher.update(seed.as_bytes());
    hasher.update(b"build-machine");
    *hasher.finalize().as_bytes()
}

fn print_summary(args: &Args, image: &AttestedImage) {
    println!("=== Output ===");
    println!("Written: {} ({} bytes)", args.output.display(), image.data.len());
    println!("\nBreakdown:");
    println!("  Kernel:      {} bytes", image.kernel_size);
    println!("  Signature:   {} bytes", image.signature_size);
    println!("  ZK block:    {} bytes", image.proof_size);
    println!("  Footer:      {} bytes", FOOTER_SIZE);
    println!("  Total:       {} bytes", image.data.len());
}
