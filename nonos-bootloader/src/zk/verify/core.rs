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
use super::types::{ZkProof, ZkVerifyResult};
use super::util::zeroize_proof;

#[cfg(feature = "zk-groth16")]
use super::constants::GROTH16_PROOF_LEN;
#[cfg(feature = "zk-groth16")]
use super::groth16::groth16_verify;
#[cfg(feature = "zk-groth16")]
use crate::zk::registry;

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
    if p.public_inputs.len() % 32 != 0 {
        return Err(ZkVerifyResult::Invalid(ZkError::InputsMisaligned.as_str()));
    }
    #[cfg(feature = "zk-groth16")]
    if p.proof_blob.len() != GROTH16_PROOF_LEN {
        return Err(ZkVerifyResult::Invalid(ZkError::ProofSizeInvalid.as_str()));
    }
    Ok(())
}

#[cfg(feature = "zk-groth16")]
fn verify_backend(p: &mut ZkProof) -> ZkVerifyResult {
    let vk_bytes = match registry::lookup(&p.program_hash) {
        Some(v) if !v.is_empty() => v,
        Some(_) => return finalize(p, ZkVerifyResult::Error(ZkError::VerifyingKeyEmpty.as_str())),
        None => {
            return finalize(p, ZkVerifyResult::Unsupported(ZkError::UnknownProgramHash.as_str()))
        }
    };

    let result = match groth16_verify(vk_bytes, &p.proof_blob, &p.public_inputs) {
        Ok(true) => ZkVerifyResult::Valid,
        Ok(false) => ZkVerifyResult::Invalid(ZkError::BackendVerifyFailed.as_str()),
        Err(e) => ZkVerifyResult::Error(e.as_str()),
    };
    finalize(p, result)
}

#[cfg(not(feature = "zk-groth16"))]
fn verify_backend(p: &mut ZkProof) -> ZkVerifyResult {
    finalize(p, ZkVerifyResult::Unsupported(ZkError::BackendUnsupported.as_str()))
}

fn finalize(p: &mut ZkProof, result: ZkVerifyResult) -> ZkVerifyResult {
    zeroize_proof(p);
    result
}
