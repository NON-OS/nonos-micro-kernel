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

mod checks;
mod dispatch;
mod signed_region;

use crate::security::nonos_trust_anchor::NonosTrustAnchorPolicy;

use super::decode::decode;
use super::error::IdCertVerifyError;
use super::policy::SignaturePolicy;
use super::schema::VerifiedNonosId;

pub fn verify(
    bytes: &[u8],
    policy: &NonosTrustAnchorPolicy,
    sig_policy: &SignaturePolicy<'_>,
    now_ms: Option<u64>,
) -> Result<VerifiedNonosId, IdCertVerifyError> {
    let cert = decode(bytes)?;
    checks::run(&cert, policy, now_ms)?;
    let signed = signed_region::compute(&cert, bytes)?;
    for alg in sig_policy.required.iter().copied() {
        dispatch::run(alg, &cert, signed, policy, now_ms)?;
    }
    Ok(VerifiedNonosId {
        nonos_id: cert.nonos_id,
        cert_serial: cert.cert_serial,
        allowed_caps_ceiling: cert.allowed_caps_ceiling,
    })
}
