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
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};

use crate::constants::MIN_PCR_ENTROPY_BYTES;

pub fn enforce_pcr_entropy<F: PrimeField>(
    cs: ConstraintSystemRef<F>,
    pcr: &[UInt8<F>],
) -> Result<(), SynthesisError> {
    let mut count = FpVar::<F>::zero();
    let one = FpVar::<F>::one();
    let fp_zero = FpVar::<F>::zero();
    for byte in pcr {
        let zero = UInt8::<F>::new_constant(cs.clone(), 0u8)?;
        let is_nonzero = byte.is_neq(&zero)?;
        count += is_nonzero.select(&one, &fp_zero)?;
    }
    let min = FpVar::<F>::new_constant(cs, F::from(MIN_PCR_ENTROPY_BYTES as u64))?;
    let diff = &count - &min;
    diff.enforce_not_equal(&FpVar::zero())?;
    Ok(())
}
