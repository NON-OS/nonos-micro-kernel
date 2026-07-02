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

use anyhow::{bail, Result};
use ed25519_dalek::{Verifier, VerifyingKey};
use nonos_capsule_sign::algs::mldsa65::MlDsa65;
use nonos_capsule_sign::algs::traits::Verifier as PqVerifier;

use crate::args::Args;
use crate::constants::{FOOTER_MAGIC, FOOTER_SIZE};
use crate::message::signed_message;

pub fn verify_signed_kernel(
    args: &Args,
    signature_len: usize,
    verifying_key: &VerifyingKey,
    pq_pub: &[u8],
) -> Result<()> {
    println!();
    println!("=== Verification ===");
    let signed_data = fs::read(&args.output)?;
    if signed_data.len() < signature_len + FOOTER_SIZE {
        bail!("Signed file too small");
    }
    let footer_start = signed_data.len() - FOOTER_SIZE;
    if signed_data[footer_start..footer_start + 8] == FOOTER_MAGIC {
        println!("NONOSIMG footer: PRESENT");
    } else {
        bail!("NONOSIMG footer: MISSING");
    }
    let sig_offset = signed_data.len() - FOOTER_SIZE - signature_len;
    let payload = &signed_data[..sig_offset];
    let sig_bytes = &signed_data[sig_offset..sig_offset + signature_len];
    let mut sig_arr = [0u8; 64];
    sig_arr.copy_from_slice(&sig_bytes[40..104]);
    let sig_read = ed25519_dalek::Signature::from_bytes(&sig_arr);
    let check = signed_message(payload, args.rollback_index);
    verifying_key.verify(&check, &sig_read)?;
    println!("Signature verification: PASSED");
    let pq = &sig_bytes[136..];
    let ok = MlDsa65::verify(pq_pub, &check, pq).map_err(|e| anyhow::anyhow!("{}", e))?;
    if !ok {
        bail!("ML-DSA-65 verification failed");
    }
    println!("ML-DSA-65 verification: PASSED");
    Ok(())
}
