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

use super::constants::{DS_CEREMONY, MIN_PARTICIPANTS};
use super::error::CeremonyError;
use super::hash::hash_params;
use super::metadata::CeremonyMetadata;
use super::params::CeremonyParams;
use ark_bls12_381::Fr;
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_snark::SNARK;
use ark_std::rand::{rngs::StdRng, SeedableRng};

pub fn ceremony_init<C>(
    circuit: C,
    ceremony_id: &str,
    circuit_name: &str,
) -> Result<(CeremonyParams, CeremonyMetadata), CeremonyError>
where
    C: ConstraintSynthesizer<Fr> + Clone,
{
    let circuit_hash = {
        let mut h = blake3::Hasher::new_derive_key(DS_CEREMONY);
        h.update(circuit_name.as_bytes());
        *h.finalize().as_bytes()
    };
    let mut rng = StdRng::from_entropy();
    let (pk, _vk) =
        <Groth16<ark_bls12_381::Bls12_381> as SNARK<Fr>>::circuit_specific_setup(circuit, &mut rng)
            .map_err(|e| CeremonyError::SerializationError(e.to_string()))?;
    let params_hash = hash_params(&pk);
    let metadata = CeremonyMetadata {
        ceremony_id: ceremony_id.to_string(),
        circuit_name: circuit_name.to_string(),
        circuit_hash,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        minimum_participants: MIN_PARTICIPANTS,
        current_round: 0,
        finalized: false,
    };
    Ok((CeremonyParams { pk, round: 0, params_hash }, metadata))
}
