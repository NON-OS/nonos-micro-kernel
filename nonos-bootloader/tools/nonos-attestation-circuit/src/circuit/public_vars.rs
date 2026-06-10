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
use ark_r1cs_std::{alloc::AllocVar, fields::fp::FpVar};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use super::types::NonosAttestationCircuit;

pub struct PublicVars<F: PrimeField> {
    pub capsule_hi: FpVar<F>,
    pub capsule_lo: FpVar<F>,
    pub policy_root: FpVar<F>,
    pub policy_epoch: FpVar<F>,
    pub caps: FpVar<F>,
    pub commit_hi: FpVar<F>,
    pub commit_lo: FpVar<F>,
}

impl<F: PrimeField> PublicVars<F> {
    pub fn new(
        cs: ConstraintSystemRef<F>,
        c: &NonosAttestationCircuit<F>,
    ) -> Result<Self, SynthesisError> {
        Ok(Self {
            capsule_hi: FpVar::new_input(cs.clone(), || {
                c.capsule_hash_hi.ok_or(SynthesisError::AssignmentMissing)
            })?,
            capsule_lo: FpVar::new_input(cs.clone(), || {
                c.capsule_hash_lo.ok_or(SynthesisError::AssignmentMissing)
            })?,
            policy_root: FpVar::new_input(cs.clone(), || {
                c.policy_root.ok_or(SynthesisError::AssignmentMissing)
            })?,
            policy_epoch: FpVar::new_input(cs.clone(), || {
                c.policy_epoch.ok_or(SynthesisError::AssignmentMissing)
            })?,
            caps: FpVar::new_input(cs.clone(), || {
                c.capability_mask.ok_or(SynthesisError::AssignmentMissing)
            })?,
            commit_hi: FpVar::new_input(cs.clone(), || {
                c.commitment_hi.ok_or(SynthesisError::AssignmentMissing)
            })?,
            commit_lo: FpVar::new_input(cs, || {
                c.commitment_lo.ok_or(SynthesisError::AssignmentMissing)
            })?,
        })
    }
}
