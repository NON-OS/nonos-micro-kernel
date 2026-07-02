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
use nonos_sign_kernel::constants::FOOTER_SIZE;
use nonos_sign_kernel::{
    load_mldsa65_pub, print_summary, release_signature_blob, sign_ed25519, sign_mldsa65,
    signed_message, verify_signed_kernel, write_signed_kernel, Args,
};

fn main() -> Result<()> {
    let args = Args::parse();
    println!("=== NONOS Kernel Signing Tool ===");
    println!();
    let kernel_data = fs::read(&args.input)
        .with_context(|| format!("Failed to read kernel: {}", args.input.display()))?;
    println!("Kernel size: {} bytes", kernel_data.len());
    let kernel_hash = blake3::hash(&kernel_data);
    println!("Kernel BLAKE3: {}", kernel_hash.to_hex());
    println!("Rollback index: {}", args.rollback_index);
    println!();

    let message = signed_message(&kernel_data, args.rollback_index);
    let ed = sign_ed25519(&args, &message)?;
    let pq_pub = load_mldsa65_pub(&args)?;
    let pq_sig = sign_mldsa65(&args, &message)?;
    let signature_blob =
        release_signature_blob(ed.verifying_key.as_bytes(), &ed.sig, &pq_pub, &pq_sig);
    println!("Ed25519 signature (R): {}", hex::encode(&ed.sig[..32]));
    println!("Ed25519 signature (S): {}", hex::encode(&ed.sig[32..]));
    println!("ML-DSA-65 signature: {} bytes", pq_sig.len());
    println!();

    let (kernel_size, signature_size, output_size) =
        write_signed_kernel(&args, &kernel_data, &signature_blob)?;
    println!("Wrote signed kernel: {} ({} bytes)", args.output.display(), output_size);
    println!("  Kernel:    {} bytes", kernel_size);
    println!("  Signature: {} bytes", signature_size);
    println!("  Footer:    {} bytes", FOOTER_SIZE);
    if args.verify {
        verify_signed_kernel(&args, signature_blob.len(), &ed.verifying_key, &pq_pub)?;
    }
    print_summary(&args, &ed.verifying_key);
    Ok(())
}
