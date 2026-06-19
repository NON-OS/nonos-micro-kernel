// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

use crate::crypto::asymmetric::alg_id::{verify as alg_verify, AlgId};
use crate::security::nonos_trust_anchor::NonosTrustAnchorPolicy;

use super::super::error::IdCertVerifyError;
use super::super::schema::NonosIdCertificate;

pub(super) fn run(
    alg: AlgId,
    cert: &NonosIdCertificate,
    signed_region: &[u8],
    policy: &NonosTrustAnchorPolicy,
    now_ms: Option<u64>,
) -> Result<(), IdCertVerifyError> {
    let sig = match cert.trust_anchor_signatures.iter().find(|s| s.algorithm == alg) {
        Some(s) => s,
        None => return Err(IdCertVerifyError::TrustAnchorPolicy),
    };
    for key in policy.keys_for(alg) {
        if let Some(ts) = now_ms {
            if ts < key.valid_from_ms || ts >= key.valid_until_ms {
                continue;
            }
        }
        if let Ok(true) = alg_verify(alg, key.pubkey_bytes(), signed_region, sig.sig_bytes()) {
            return Ok(());
        }
    }
    Err(IdCertVerifyError::TrustAnchorBadSig(alg))
}
