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

use super::error::CeremonyError;
use ark_bls12_381::{Bls12_381, Fr};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::Field;
use ark_groth16::ProvingKey;

/// Apply one Groth16 Phase-2 (BGM17) contribution to the proving key.
///
/// Phase-2 re-randomizes only the secret `delta`. The circuit-fixed elements
/// (`alpha_g1`, `beta_g2`, `gamma_g2`, `gamma_abc_g1`, and the `a_query` /
/// `b_g1_query` / `b_g2_query` vectors) are NOT touched. The delta terms move:
///
///   vk.delta_g2 *= delta
///   pk.delta_g1 *= delta
///   pk.l_query  *= delta^-1     (witness terms are divided by delta)
///   pk.h_query  *= delta^-1     (quotient terms are divided by delta)
///
/// This preserves the verifier equation
///   e(A, B) = e(alpha, beta) * e(sum_public, gamma) * e(C, delta),
/// because C is built from the delta^-1-scaled L and H terms and is checked
/// against the delta-scaled `delta_g2`. The contributor's `delta` is the only
/// secret; destroying it removes that share of the trapdoor, so the composed
/// delta across all rounds is unknown as long as one contributor is honest.
pub fn apply_phase2_contribution(
    pk: &ProvingKey<Bls12_381>,
    delta: Fr,
) -> Result<ProvingKey<Bls12_381>, CeremonyError> {
    let delta_inv = delta.inverse().ok_or(CeremonyError::InvalidContribution)?;

    let mut new_pk = pk.clone();

    new_pk.vk.delta_g2 = (pk.vk.delta_g2.into_group() * delta).into_affine();
    new_pk.delta_g1 = (pk.delta_g1.into_group() * delta).into_affine();

    for g1 in new_pk.l_query.iter_mut() {
        *g1 = (g1.into_group() * delta_inv).into_affine();
    }
    for g1 in new_pk.h_query.iter_mut() {
        *g1 = (g1.into_group() * delta_inv).into_affine();
    }

    Ok(new_pk)
}
