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

mod caps;
mod caps_bits;
mod capsule_id;
mod cert_binding;
mod dispatch;
mod endpoint_drift;
mod namespace;
mod payload;
mod signed_region;
mod target_triple;

use crate::security::nonos_id_cert::{NonosIdCertificate, SignaturePolicy, VerifiedNonosId};
use crate::security::nonos_trust_anchor::NonosTrustAnchorPolicy;

use super::decode::decode;
use super::error::ManifestVerifyError;
use super::schema::VerifiedManifest;

pub use endpoint_drift::DeclaredEndpoint;

pub fn verify_with_publisher(
    manifest_bytes: &[u8],
    nonos_id_cert_bytes: &[u8],
    cert: &NonosIdCertificate,
    verified_id: &VerifiedNonosId,
    policy: &NonosTrustAnchorPolicy,
    sig_policy: &SignaturePolicy<'_>,
    payload: &[u8],
    target_triple: &str,
    granted_caps: u64,
    declared_endpoints: &[DeclaredEndpoint<'_>],
) -> Result<(VerifiedManifest, u64), ManifestVerifyError> {
    let manifest = decode(manifest_bytes)?;
    cert_binding::check(&manifest, nonos_id_cert_bytes)?;
    namespace::check(&manifest, cert)?;
    caps::check_ceiling(&manifest, verified_id.allowed_caps_ceiling)?;
    let signed = signed_region::compute(&manifest, manifest_bytes)?;
    for alg in sig_policy.required.iter().copied() {
        dispatch::run(alg, &manifest, cert, policy, signed)?;
    }
    payload::check(&manifest, payload)?;
    target_triple::check(&manifest, target_triple)?;
    endpoint_drift::check(&manifest, declared_endpoints)?;
    let install_caps = caps::check_grant(&manifest, granted_caps)?;
    let capsule_id = capsule_id::derive(&manifest);
    Ok((VerifiedManifest { manifest, capsule_id }, install_caps))
}
