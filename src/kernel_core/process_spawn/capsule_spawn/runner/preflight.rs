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

use crate::security::capsule_manifest::{verify_with_publisher, DeclaredEndpoint, EndpointKind};
use crate::security::nonos_id_cert::{
    decode as decode_id_cert, verify as verify_id_cert, IdCertVerifyError, NONOS_PRODUCTION_POLICY,
};
use crate::security::nonos_trust_anchor::NonosTrustAnchorPolicy;

use super::super::spec::{CapsuleSpecVerified, SpawnError};

pub(crate) struct Preflighted {
    pub install_caps: u64,
}

pub(crate) fn run(
    spec: &CapsuleSpecVerified,
    trust_anchor: &NonosTrustAnchorPolicy,
    now_ms: Option<u64>,
) -> Result<Preflighted, SpawnError> {
    let cert = decode_id_cert(spec.nonos_id_cert_bytes)
        .map_err(|e| SpawnError::NonosIdCertRejected(IdCertVerifyError::Decode(e)))?;
    let verified_id =
        verify_id_cert(spec.nonos_id_cert_bytes, trust_anchor, &NONOS_PRODUCTION_POLICY, now_ms)?;

    let declared = [
        DeclaredEndpoint { kind: EndpointKind::Service, port: spec.service_port, name: spec.name },
        DeclaredEndpoint {
            kind: EndpointKind::Reply,
            port: spec.reply_port,
            name: spec.reply_inbox,
        },
    ];

    let verification = verify_with_publisher(
        spec.manifest_bytes,
        spec.nonos_id_cert_bytes,
        &cert,
        &verified_id,
        trust_anchor,
        &NONOS_PRODUCTION_POLICY,
        spec.elf,
        spec.target_triple,
        spec.requested_caps,
        &declared,
    )?;
    let install_caps = verification.1;
    let required_caps = verification.0.manifest.required_caps;
    let namespace = verification.0.manifest.namespace_str();
    match super::tier::classify(namespace) {
        super::tier::Tier::Enrolled => super::attest_gate::attest_gate(spec, required_caps)?,
        super::tier::Tier::Publisher => {
            super::publisher_gate::publisher_gate(spec, namespace, required_caps)?
        }
    }

    Ok(Preflighted { install_caps })
}
