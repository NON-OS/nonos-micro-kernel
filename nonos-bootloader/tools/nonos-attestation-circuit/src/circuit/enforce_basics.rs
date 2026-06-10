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
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use super::entropy::enforce_pcr_entropy;
use super::nonzero::enforce_nonzero;
use super::public_vars::PublicVars;
use super::witness_vars::WitnessVars;
use crate::constants::MIN_HW_LEVEL;

pub fn enforce_basics<F: PrimeField>(
    cs: ConstraintSystemRef<F>,
    public: &PublicVars<F>,
    witness: &WitnessVars<F>,
) -> Result<(), SynthesisError> {
    enforce_nonzero(&public.capsule_hi)?;
    enforce_nonzero(&public.capsule_lo)?;
    enforce_nonzero(&public.policy_root)?;
    enforce_nonzero(&public.policy_epoch)?;
    enforce_nonzero(&public.commit_hi)?;
    enforce_nonzero(&public.commit_lo)?;
    enforce_pcr_entropy(cs.clone(), &witness.pcr)?;
    let min_hw = FpVar::<F>::new_constant(cs, F::from(MIN_HW_LEVEL))?;
    let hw_diff = &witness.hw - &min_hw;
    hw_diff.enforce_not_equal(&FpVar::zero())
}
