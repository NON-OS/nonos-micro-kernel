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

use anyhow::{bail, Context, Result};
use nonos_attestation_circuit::transparent::{proof_path, prove, root, verify, EnrolledSecret};

use crate::Args;

use super::types::TransparentBootProof;
use super::{commitments::commitments, ctx_static::ctx_static, hex32::hex32};
use super::{public_inputs_static::public_inputs_static, root_file::root_file, scalar::scalar};

pub fn static_boot_proof(args: &Args, kernel_hash: &[u8; 32]) -> Result<TransparentBootProof> {
    let enrolled = commitments(
        args.commitments.as_deref().context("--commitments is required for the curve boot proof")?,
    )?;
    let expected_root =
        root_file(args.root.as_deref().context("--root is required for the curve boot proof")?)?;
    if root(&enrolled) != expected_root {
        bail!("commitments file does not fold to root");
    }
    if args.index >= enrolled.len() {
        bail!("enrollment index out of range");
    }
    let nonce_seed = hex32("nonce_seed", &args.nonce_seed)?;
    let secret = EnrolledSecret {
        x: scalar("secret_x", &args.secret_x)?,
        r: scalar("secret_r", &args.secret_r)?,
    };
    let proof_ctx = ctx_static(&expected_root, kernel_hash);
    let (siblings, dirs) = proof_path(&enrolled, args.index);
    let proof_blob = prove(&secret, &siblings, &dirs, &expected_root, &proof_ctx, &nonce_seed)
        .map_err(|e| anyhow::anyhow!(e))?;
    verify(&expected_root, &proof_ctx, &proof_blob).map_err(|e| anyhow::anyhow!(e))?;
    Ok(TransparentBootProof {
        root: expected_root,
        public_inputs: public_inputs_static(kernel_hash).to_vec(),
        proof_blob,
        boot_nonce: [0u8; 32],
        machine_id: [0u8; 32],
    })
}
