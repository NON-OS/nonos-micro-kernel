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
use embed_zk_proof::*;

use crate::summary_image::print_image_summary;
use crate::summary_sidecar::print_sidecar_summary;

pub fn run_attestation(args: &Args) -> Result<()> {
    println!("=== NONOS Transparent Attestation Prover ===\n");
    let kernel = load_signed_kernel(&args.input)?;
    println!("Signed kernel: {} bytes", kernel.raw_bytes.len());
    println!("Kernel code: {} bytes", kernel.kernel_bytes.len());
    let kernel_hash = compute_kernel_hash(&kernel.kernel_bytes);
    println!("Kernel BLAKE3: {}", hex::encode(kernel_hash));

    // STARK self-attestation: embed a pre-made trailer verbatim as the proof
    // region. The trailer is bound to this exact kernel measurement, so the
    // bootloader verifies it against the same BLAKE3 over the same kernel bytes.
    if let Some(proof_path) = &args.proof_file {
        let trailer = fs::read(proof_path)
            .with_context(|| format!("Failed to read proof file: {}", proof_path.display()))?;
        println!("Embedding STARK self-attestation trailer: {} bytes", trailer.len());
        let image = assemble_attested_image(&kernel, trailer);
        fs::write(&args.output, &image.data)
            .with_context(|| format!("Failed to write: {}", args.output.display()))?;
        print_image_summary(args, &image);
        return Ok(());
    }

    let proof = create_transparent_boot_proof(args, &kernel_hash)?;
    let capsule_commitment = compute_capsule_commitment(&kernel_hash, &proof.root);
    println!("Device root: {}", hex::encode(proof.root));
    println!("Capsule commitment: {}", hex::encode(capsule_commitment));
    println!("Boot nonce: {}", hex::encode(&proof.boot_nonce[..8]));
    println!("Machine ID: {}", hex::encode(&proof.machine_id[..8]));
    println!("Proof generated: {} bytes", proof.proof_blob.len());
    let block_params = ZkBlockParams {
        program_hash: &proof.root,
        capsule_commitment: &capsule_commitment,
        kernel_hash: &kernel_hash,
        boot_nonce: &proof.boot_nonce,
        machine_id: &proof.machine_id,
        public_inputs: &proof.public_inputs,
        proof_blob: &proof.proof_blob,
    };
    let zk_block = create_zk_block(&block_params);
    if args.sidecar {
        fs::write(&args.output, &zk_block)
            .with_context(|| format!("Failed to write: {}", args.output.display()))?;
        print_sidecar_summary(args, &zk_block);
        return Ok(());
    }
    let image = assemble_attested_image(&kernel, zk_block);
    fs::write(&args.output, &image.data)
        .with_context(|| format!("Failed to write: {}", args.output.display()))?;
    print_image_summary(args, &image);
    Ok(())
}
