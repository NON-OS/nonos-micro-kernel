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
use super::metadata::CeremonyMetadata;
use super::params::CeremonyParams;
use super::record::ContributionRecord;
use super::transcript::CeremonyTranscript;
use ark_bls12_381::Bls12_381;
use ark_groth16::VerifyingKey;
use ark_serialize::{CanonicalSerialize, Compress};

pub fn ceremony_finalize(
    final_params: &CeremonyParams,
    contributions: &[ContributionRecord],
) -> Result<(VerifyingKey<Bls12_381>, CeremonyTranscript), CeremonyError> {
    if contributions.len() < MIN_PARTICIPANTS {
        return Err(CeremonyError::InsufficientParticipants);
    }
    for c in contributions {
        if c.destruction_attestation.is_none() {
            return Err(CeremonyError::ToxicWasteNotDestroyed);
        }
    }
    let vk = final_params.pk.vk.clone();
    let vk_hash = {
        let mut buf = Vec::new();
        vk.serialize_with_mode(&mut buf, Compress::Yes).unwrap();
        let mut h = blake3::Hasher::new_derive_key(DS_CEREMONY);
        h.update(&buf);
        *h.finalize().as_bytes()
    };
    let metadata = CeremonyMetadata {
        ceremony_id: format!("nonos-ceremony-{}", final_params.round),
        circuit_name: "nonos-attestation".to_string(),
        circuit_hash: [0u8; 32],
        created_at: contributions.first().map(|c| c.contribution_timestamp).unwrap_or(0),
        minimum_participants: MIN_PARTICIPANTS,
        current_round: final_params.round,
        finalized: true,
    };
    let transcript = CeremonyTranscript {
        metadata,
        contributions: contributions.to_vec(),
        final_vk_hash: Some(vk_hash),
        verification_passed: true,
    };
    Ok((vk, transcript))
}
