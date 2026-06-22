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

use anyhow::{bail, Result};
use nonos_attestation_circuit::transparent::{proof_path, prove, root, verify, EnrolledSecret};

use crate::Args;

use super::challenge_file::challenge_file;
use super::explicit_challenge::explicit_challenge;
use super::types::TransparentBootProof;
use super::{commitments::commitments, ctx::ctx_runtime, hex32::hex32};
use super::{public_inputs::public_inputs, root_file::root_file, scalar::scalar};

pub fn runtime_boot_proof(args: &Args, kernel_hash: &[u8; 32]) -> Result<TransparentBootProof> {
    let enrolled = commitments(&args.commitments)?;
    let expected_root = root_file(&args.root)?;
    if root(&enrolled) != expected_root || args.index >= enrolled.len() {
        bail!("runtime enrollment invalid");
    }
    let (boot_nonce, machine_id, timestamp) = match &args.challenge {
        Some(path) => challenge_file(path, kernel_hash)?,
        None => explicit_challenge(args)?,
    };
    let nonce_seed = hex32("nonce_seed", &args.nonce_seed)?;
    let secret = EnrolledSecret {
        x: scalar("secret_x", &args.secret_x)?,
        r: scalar("secret_r", &args.secret_r)?,
    };
    let proof_ctx = ctx_runtime(kernel_hash, &boot_nonce, &machine_id, timestamp);
    let (siblings, dirs) = proof_path(&enrolled, args.index);
    let proof_blob = prove(&secret, &siblings, &dirs, &expected_root, &proof_ctx, &nonce_seed)
        .map_err(|e| anyhow::anyhow!(e))?;
    verify(&expected_root, &proof_ctx, &proof_blob).map_err(|e| anyhow::anyhow!(e))?;
    Ok(TransparentBootProof {
        root: expected_root,
        public_inputs: public_inputs(kernel_hash, &boot_nonce, &machine_id, timestamp).to_vec(),
        proof_blob,
        boot_nonce,
        machine_id,
    })
}
