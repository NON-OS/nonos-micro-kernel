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

use curve25519_dalek::{
    constants::ED25519_BASEPOINT_TABLE,
    edwards::{CompressedEdwardsY, EdwardsPoint},
    scalar::Scalar,
};
use rand_core::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::collections::BTreeMap;
use zeroize::Zeroize;

pub const DS_FROST: &str = "NONOS:FROST:v1";
pub const DS_FROST_COMMIT: &str = "NONOS:FROST:COMMIT:v1";
pub const DS_FROST_CHALLENGE: &str = "NONOS:FROST:CHALLENGE:v1";

#[derive(Debug)]
pub enum FrostError {
    InvalidThreshold,
    InvalidParticipantCount,
    InvalidParticipantId,
    InsufficientShares,
    InvalidCommitment,
    InvalidSignatureShare,
    VerificationFailed,
    DuplicateParticipant,
    SerializationError(String),
}

impl std::fmt::Display for FrostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidThreshold => write!(f, "invalid threshold"),
            Self::InvalidParticipantCount => write!(f, "invalid participant count"),
            Self::InvalidParticipantId => write!(f, "invalid participant ID"),
            Self::InsufficientShares => write!(f, "insufficient shares"),
            Self::InvalidCommitment => write!(f, "invalid commitment"),
            Self::InvalidSignatureShare => write!(f, "invalid signature share"),
            Self::VerificationFailed => write!(f, "verification failed"),
            Self::DuplicateParticipant => write!(f, "duplicate participant"),
            Self::SerializationError(s) => write!(f, "serialization: {}", s),
        }
    }
}

impl std::error::Error for FrostError {}

pub type ParticipantId = u16;

#[derive(Clone, Serialize, Deserialize)]
pub struct ThresholdConfig {
    pub threshold: u16,
    pub total_signers: u16,
}

impl ThresholdConfig {
    pub fn new(threshold: u16, total_signers: u16) -> Result<Self, FrostError> {
        if threshold == 0 || threshold > total_signers {
            return Err(FrostError::InvalidThreshold);
        }
        if total_signers < 2 {
            return Err(FrostError::InvalidParticipantCount);
        }
        Ok(Self {
            threshold,
            total_signers,
        })
    }
}

/// Key share for a single participant. Contains secret material.
/// Automatically zeroizes secret_share on drop.
#[derive(Clone, Serialize, Deserialize)]
pub struct KeyShare {
    pub participant_id: ParticipantId,
    pub secret_share: [u8; 32],
    pub public_share: [u8; 32],
    pub group_public_key: [u8; 32],
    pub verification_shares: Vec<[u8; 32]>,
    pub config: ThresholdConfig,
}

impl Drop for KeyShare {
    fn drop(&mut self) {
        self.secret_share.zeroize();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PublicKeyPackage {
    pub group_public_key: [u8; 32],
    pub verification_shares: BTreeMap<ParticipantId, [u8; 32]>,
    pub config: ThresholdConfig,
}

/// Signing nonces for a participant. Contains secret material.
/// Automatically zeroizes nonces on drop.
#[derive(Clone, Serialize, Deserialize)]
pub struct SigningNonces {
    pub participant_id: ParticipantId,
    pub hiding_nonce: [u8; 32],
    pub binding_nonce: [u8; 32],
}

impl Drop for SigningNonces {
    fn drop(&mut self) {
        self.hiding_nonce.zeroize();
        self.binding_nonce.zeroize();
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SigningCommitments {
    pub participant_id: ParticipantId,
    pub hiding_commitment: [u8; 32],
    pub binding_commitment: [u8; 32],
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SigningPackage {
    pub message: Vec<u8>,
    pub commitments: BTreeMap<ParticipantId, SigningCommitments>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SignatureShare {
    pub participant_id: ParticipantId,
    pub share: [u8; 32],
}

fn scalar_from_bytes(bytes: &[u8; 32]) -> Scalar {
    let mut wide = [0u8; 64];
    wide[..32].copy_from_slice(bytes);
    Scalar::from_bytes_mod_order_wide(&wide)
}

fn point_from_bytes(bytes: &[u8; 32]) -> Option<EdwardsPoint> {
    CompressedEdwardsY(*bytes).decompress()
}

fn point_to_bytes(point: &EdwardsPoint) -> [u8; 32] {
    point.compress().to_bytes()
}

fn hash_to_scalar(domain: &str, data: &[u8]) -> Scalar {
    let mut hasher = Sha512::new();
    hasher.update(domain.as_bytes());
    hasher.update(data);
    let hash = hasher.finalize();
    let mut wide = [0u8; 64];
    wide.copy_from_slice(&hash);
    Scalar::from_bytes_mod_order_wide(&wide)
}

pub fn keygen<R: RngCore + CryptoRng>(
    config: &ThresholdConfig,
    rng: &mut R,
) -> Result<(Vec<KeyShare>, PublicKeyPackage), FrostError> {
    let mut coefficients = Vec::with_capacity(config.threshold as usize);
    for _ in 0..config.threshold {
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        coefficients.push(scalar_from_bytes(&bytes));
    }

    let group_public_key = &coefficients[0] * ED25519_BASEPOINT_TABLE;

    let mut verification_commitments = Vec::with_capacity(config.threshold as usize);
    for coef in &coefficients {
        verification_commitments.push(coef * ED25519_BASEPOINT_TABLE);
    }

    let mut key_shares = Vec::with_capacity(config.total_signers as usize);
    let mut verification_shares = BTreeMap::new();

    for i in 1..=config.total_signers {
        let x = Scalar::from(i as u64);

        let mut secret_share = Scalar::ZERO;
        let mut x_power = Scalar::ONE;
        for coef in &coefficients {
            secret_share += coef * x_power;
            x_power *= x;
        }

        let public_share = &secret_share * ED25519_BASEPOINT_TABLE;

        let share = KeyShare {
            participant_id: i,
            secret_share: secret_share.to_bytes(),
            public_share: point_to_bytes(&public_share),
            group_public_key: point_to_bytes(&group_public_key),
            verification_shares: verification_commitments
                .iter()
                .map(point_to_bytes)
                .collect(),
            config: config.clone(),
        };

        key_shares.push(share);
        verification_shares.insert(i, point_to_bytes(&public_share));
    }

    let pubkey_package = PublicKeyPackage {
        group_public_key: point_to_bytes(&group_public_key),
        verification_shares,
        config: config.clone(),
    };

    Ok((key_shares, pubkey_package))
}

pub fn round1_commit<R: RngCore + CryptoRng>(
    key_share: &KeyShare,
    rng: &mut R,
) -> (SigningNonces, SigningCommitments) {
    let mut hiding_bytes = [0u8; 32];
    let mut binding_bytes = [0u8; 32];
    rng.fill_bytes(&mut hiding_bytes);
    rng.fill_bytes(&mut binding_bytes);

    let hiding_nonce = scalar_from_bytes(&hiding_bytes);
    let binding_nonce = scalar_from_bytes(&binding_bytes);

    let hiding_commitment = &hiding_nonce * ED25519_BASEPOINT_TABLE;
    let binding_commitment = &binding_nonce * ED25519_BASEPOINT_TABLE;

    let nonces = SigningNonces {
        participant_id: key_share.participant_id,
        hiding_nonce: hiding_nonce.to_bytes(),
        binding_nonce: binding_nonce.to_bytes(),
    };

    let commitments = SigningCommitments {
        participant_id: key_share.participant_id,
        hiding_commitment: point_to_bytes(&hiding_commitment),
        binding_commitment: point_to_bytes(&binding_commitment),
    };

    (nonces, commitments)
}

fn compute_binding_factor(
    participant_id: ParticipantId,
    message: &[u8],
    all_commitments: &BTreeMap<ParticipantId, SigningCommitments>,
) -> Scalar {
    let mut data = Vec::new();
    data.extend_from_slice(&participant_id.to_le_bytes());
    data.extend_from_slice(message);

    for (id, commit) in all_commitments {
        data.extend_from_slice(&id.to_le_bytes());
        data.extend_from_slice(&commit.hiding_commitment);
        data.extend_from_slice(&commit.binding_commitment);
    }

    hash_to_scalar(DS_FROST_COMMIT, &data)
}

fn compute_group_commitment(
    commitments: &BTreeMap<ParticipantId, SigningCommitments>,
    binding_factors: &BTreeMap<ParticipantId, Scalar>,
) -> Result<EdwardsPoint, FrostError> {
    let mut group_commitment = EdwardsPoint::default();

    for (id, commit) in commitments {
        let hiding =
            point_from_bytes(&commit.hiding_commitment).ok_or(FrostError::InvalidCommitment)?;
        let binding =
            point_from_bytes(&commit.binding_commitment).ok_or(FrostError::InvalidCommitment)?;

        let rho = binding_factors
            .get(id)
            .ok_or(FrostError::InvalidParticipantId)?;

        group_commitment += hiding + (rho * binding);
    }

    Ok(group_commitment)
}

fn compute_challenge(
    group_commitment: &EdwardsPoint,
    group_public_key: &EdwardsPoint,
    message: &[u8],
) -> Scalar {
    let mut data = Vec::new();
    data.extend_from_slice(&point_to_bytes(group_commitment));
    data.extend_from_slice(&point_to_bytes(group_public_key));
    data.extend_from_slice(message);

    hash_to_scalar(DS_FROST_CHALLENGE, &data)
}

fn lagrange_coefficient(
    participant_id: ParticipantId,
    participant_ids: &[ParticipantId],
) -> Scalar {
    let x_i = Scalar::from(participant_id as u64);

    let mut numerator = Scalar::ONE;
    let mut denominator = Scalar::ONE;

    for &other_id in participant_ids {
        if other_id == participant_id {
            continue;
        }
        let x_j = Scalar::from(other_id as u64);
        numerator *= x_j;
        denominator *= x_j - x_i;
    }

    numerator * denominator.invert()
}

pub fn round2_sign(
    signing_package: &SigningPackage,
    nonces: &SigningNonces,
    key_share: &KeyShare,
) -> Result<SignatureShare, FrostError> {
    let mut binding_factors = BTreeMap::new();
    for id in signing_package.commitments.keys() {
        let factor =
            compute_binding_factor(*id, &signing_package.message, &signing_package.commitments);
        binding_factors.insert(*id, factor);
    }

    let group_commitment =
        compute_group_commitment(&signing_package.commitments, &binding_factors)?;

    let group_public_key =
        point_from_bytes(&key_share.group_public_key).ok_or(FrostError::VerificationFailed)?;

    let challenge = compute_challenge(
        &group_commitment,
        &group_public_key,
        &signing_package.message,
    );

    let hiding_nonce = scalar_from_bytes(&nonces.hiding_nonce);
    let binding_nonce = scalar_from_bytes(&nonces.binding_nonce);
    let secret_share = scalar_from_bytes(&key_share.secret_share);

    let rho = binding_factors
        .get(&key_share.participant_id)
        .ok_or(FrostError::InvalidParticipantId)?;

    let participant_ids: Vec<_> = signing_package.commitments.keys().copied().collect();
    let lambda = lagrange_coefficient(key_share.participant_id, &participant_ids);

    let signature_share =
        hiding_nonce + (rho * binding_nonce) + (lambda * secret_share * challenge);

    Ok(SignatureShare {
        participant_id: key_share.participant_id,
        share: signature_share.to_bytes(),
    })
}

pub fn aggregate_signatures(
    signing_package: &SigningPackage,
    signature_shares: &BTreeMap<ParticipantId, SignatureShare>,
    pubkey_package: &PublicKeyPackage,
) -> Result<[u8; 64], FrostError> {
    if signature_shares.len() < pubkey_package.config.threshold as usize {
        return Err(FrostError::InsufficientShares);
    }

    let mut binding_factors = BTreeMap::new();
    for id in signing_package.commitments.keys() {
        let factor =
            compute_binding_factor(*id, &signing_package.message, &signing_package.commitments);
        binding_factors.insert(*id, factor);
    }

    let group_commitment =
        compute_group_commitment(&signing_package.commitments, &binding_factors)?;

    let mut z = Scalar::ZERO;
    for share in signature_shares.values() {
        z += scalar_from_bytes(&share.share);
    }

    let mut signature = [0u8; 64];
    signature[..32].copy_from_slice(&point_to_bytes(&group_commitment));
    signature[32..].copy_from_slice(&z.to_bytes());

    let _group_public_key =
        point_from_bytes(&pubkey_package.group_public_key).ok_or(FrostError::VerificationFailed)?;

    verify_signature(
        &signing_package.message,
        &signature,
        &pubkey_package.group_public_key,
    )?;

    Ok(signature)
}

pub fn verify_signature(
    message: &[u8],
    signature: &[u8; 64],
    public_key: &[u8; 32],
) -> Result<(), FrostError> {
    let r_bytes: [u8; 32] = signature[..32].try_into().unwrap();
    let s_bytes: [u8; 32] = signature[32..].try_into().unwrap();

    let r = point_from_bytes(&r_bytes).ok_or(FrostError::VerificationFailed)?;
    let s = scalar_from_bytes(&s_bytes);
    let pk = point_from_bytes(public_key).ok_or(FrostError::VerificationFailed)?;

    let challenge = compute_challenge(&r, &pk, message);

    let expected = &s * ED25519_BASEPOINT_TABLE;
    let actual = r + (challenge * pk);

    if expected.compress() == actual.compress() {
        Ok(())
    } else {
        Err(FrostError::VerificationFailed)
    }
}

