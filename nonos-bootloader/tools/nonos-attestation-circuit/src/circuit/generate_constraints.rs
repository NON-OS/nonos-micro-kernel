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

use ark_ff::PrimeField;
use ark_r1cs_std::{
    alloc::AllocVar,
    eq::EqGadget,
    fields::{fp::FpVar, FieldVar},
    uint8::UInt8,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

use super::entropy::enforce_pcr_entropy;
use super::nonzero::enforce_nonzero;
use super::types::NonosAttestationCircuit;
use crate::constants::MIN_HW_LEVEL;

impl<F: PrimeField> ConstraintSynthesizer<F> for NonosAttestationCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let capsule_hi = FpVar::<F>::new_input(cs.clone(), || {
            self.capsule_hash_hi.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let capsule_lo = FpVar::<F>::new_input(cs.clone(), || {
            self.capsule_hash_lo.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let program_hi = FpVar::<F>::new_input(cs.clone(), || {
            self.program_hash_hi.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let program_lo = FpVar::<F>::new_input(cs.clone(), || {
            self.program_hash_lo.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let _caps = FpVar::<F>::new_input(cs.clone(), || {
            self.capability_mask.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let commit_hi = FpVar::<F>::new_input(cs.clone(), || {
            self.commitment_hi.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let commit_lo = FpVar::<F>::new_input(cs.clone(), || {
            self.commitment_lo.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let pcr = self.pcr_preimage.ok_or(SynthesisError::AssignmentMissing)?;
        let hw = self.hardware_attestation.ok_or(SynthesisError::AssignmentMissing)?;
        let pcr_var = UInt8::<F>::new_witness_vec(cs.clone(), &pcr)?;
        let hw_var = FpVar::<F>::new_witness(cs.clone(), || Ok(F::from(hw)))?;
        enforce_nonzero(&capsule_hi)?;
        enforce_nonzero(&capsule_lo)?;
        enforce_nonzero(&program_hi)?;
        enforce_nonzero(&program_lo)?;
        enforce_nonzero(&commit_hi)?;
        enforce_nonzero(&commit_lo)?;
        enforce_pcr_entropy(cs.clone(), &pcr_var)?;
        let min_hw = FpVar::<F>::new_constant(cs, F::from(MIN_HW_LEVEL))?;
        let hw_diff = &hw_var - &min_hw;
        hw_diff.enforce_not_equal(&FpVar::zero())?;
        Ok(())
    }
}
