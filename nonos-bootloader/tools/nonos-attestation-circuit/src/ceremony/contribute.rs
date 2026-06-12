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

use super::constants::DS_CONTRIBUTION;
use super::error::CeremonyError;
use super::hash::hash_params;
use super::params::CeremonyParams;
use super::record::ContributionRecord;
use super::tau::apply_phase2_contribution;
use ark_bls12_381::Fr;
use ark_ff::{UniformRand, Zero};
use ark_std::rand::{rngs::StdRng, SeedableRng};

pub fn contribute_randomness(
    prev: &CeremonyParams,
    id: &str,
    contact: &str,
    loc: &str,
    src: &str,
    ext: &[u8],
) -> Result<(CeremonyParams, ContributionRecord), CeremonyError> {
    let prev_hash = hash_params(&prev.pk);
    if prev_hash != prev.params_hash {
        return Err(CeremonyError::HashMismatch);
    }
    let seed = {
        let mut h = blake3::Hasher::new_derive_key(DS_CONTRIBUTION);
        h.update(&prev_hash);
        h.update(id.as_bytes());
        h.update(ext);
        h.update(
            &std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .to_le_bytes(),
        );
        *h.finalize().as_bytes()
    };
    let rc = blake3::hash(&seed);
    let mut rng = StdRng::from_seed(seed);
    // Phase-2 secret: a single nonzero delta. It is dropped at the end of this
    // function and never persisted; that is the toxic waste being destroyed.
    let mut delta = Fr::rand(&mut rng);
    while delta.is_zero() {
        delta = Fr::rand(&mut rng);
    }
    let new_pk = apply_phase2_contribution(&prev.pk, delta)?;
    let new_hash = hash_params(&new_pk);
    let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let record = ContributionRecord {
        round: prev.round + 1,
        contributor_id: id.to_string(),
        contributor_contact: contact.to_string(),
        location: loc.to_string(),
        randomness_source: src.to_string(),
        previous_params_hash: prev_hash,
        new_params_hash: new_hash,
        randomness_commitment: *rc.as_bytes(),
        contribution_timestamp: ts,
        destruction_attestation: None,
    };
    Ok((CeremonyParams { pk: new_pk, round: prev.round + 1, params_hash: new_hash }, record))
}
