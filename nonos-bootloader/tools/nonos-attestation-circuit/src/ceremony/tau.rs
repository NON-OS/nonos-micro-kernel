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
use ark_groth16::ProvingKey;

pub fn apply_powers_of_tau(
    pk: &ProvingKey<Bls12_381>,
    tau: Fr,
    alpha: Fr,
    beta: Fr,
) -> Result<ProvingKey<Bls12_381>, CeremonyError> {
    let mut new_pk = pk.clone();
    let max_degree = pk
        .vk
        .gamma_abc_g1
        .len()
        .max(pk.a_query.len())
        .max(pk.b_g1_query.len())
        .max(pk.b_g2_query.len())
        .max(pk.h_query.len())
        .max(pk.l_query.len())
        + 10;
    let mut tau_powers = Vec::with_capacity(max_degree);
    tau_powers.push(Fr::from(1u64));
    for i in 1..max_degree {
        tau_powers.push(tau_powers[i - 1] * tau);
    }
    new_pk.vk.alpha_g1 = (pk.vk.alpha_g1.into_group() * alpha).into_affine();
    new_pk.vk.beta_g2 = (pk.vk.beta_g2.into_group() * beta).into_affine();
    let alpha_beta = alpha * beta;
    new_pk.beta_g1 = (pk.beta_g1.into_group() * beta).into_affine();
    for (i, g1) in new_pk.vk.gamma_abc_g1.iter_mut().enumerate() {
        if i < tau_powers.len() {
            *g1 = (g1.into_group() * tau_powers[i]).into_affine();
        }
    }
    for (i, g1) in new_pk.a_query.iter_mut().enumerate() {
        if i < tau_powers.len() {
            *g1 = (g1.into_group() * tau_powers[i]).into_affine();
        }
    }
    for (i, g1) in new_pk.b_g1_query.iter_mut().enumerate() {
        if i < tau_powers.len() {
            *g1 = (g1.into_group() * tau_powers[i]).into_affine();
        }
    }
    for (i, g2) in new_pk.b_g2_query.iter_mut().enumerate() {
        if i < tau_powers.len() {
            *g2 = (g2.into_group() * tau_powers[i]).into_affine();
        }
    }
    for (i, g1) in new_pk.h_query.iter_mut().enumerate() {
        if i < tau_powers.len() {
            let ti = tau_powers[i];
            let tn = if i + 1 < tau_powers.len() { tau_powers[i + 1] } else { tau_powers[i] * tau };
            *g1 = (g1.into_group() * (tn - ti)).into_affine();
        }
    }
    for (i, g1) in new_pk.l_query.iter_mut().enumerate() {
        if i < tau_powers.len() {
            *g1 = (g1.into_group() * (alpha_beta * tau_powers[i])).into_affine();
        }
    }
    Ok(new_pk)
}
