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

use crate::zk::errors::ZkError;

use super::constants::{MAX_INPUT_SIZE, MAX_PROOF_SIZE};
use super::transparent::verify_transparent;
use super::types::{ZkProof, ZkVerifyResult};
use super::util::zeroize_proof;

pub fn verify_proof(p: &mut ZkProof) -> ZkVerifyResult {
    if let Err(e) = validate_proof_bounds(p) {
        return e;
    }

    verify_backend(p)
}

fn validate_proof_bounds(p: &ZkProof) -> Result<(), ZkVerifyResult> {
    if p.proof_blob.len() > MAX_PROOF_SIZE {
        return Err(ZkVerifyResult::Unsupported(ZkError::ProofTooLarge.as_str()));
    }
    if p.public_inputs.len() > MAX_INPUT_SIZE {
        return Err(ZkVerifyResult::Unsupported(ZkError::InputsTooLarge.as_str()));
    }
    Ok(())
}

fn verify_backend(p: &mut ZkProof) -> ZkVerifyResult {
    let result = match verify_transparent(&p.program_hash, &p.public_inputs, &p.proof_blob) {
        Ok(()) => ZkVerifyResult::Valid,
        Err(e) => ZkVerifyResult::Invalid(e),
    };
    finalize(p, result)
}

fn finalize(p: &mut ZkProof, result: ZkVerifyResult) -> ZkVerifyResult {
    zeroize_proof(p);
    result
}
