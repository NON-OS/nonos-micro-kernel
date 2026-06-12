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

    #[cfg(feature = "zk-groth16")]
    attest_gate(spec, install_caps)?;

    Ok(Preflighted { install_caps })
}

// Per-capsule zero-knowledge attestation gate. After the signatures and manifest
// have verified, a capsule that carries an attestation trailer must prove it.
// During rollout the result is logged but not enforced; under nonos-zk-enforce a
// failed or mismatched proof refuses the spawn before any mapping happens. A
// capsule with no trailer yet is passed through to the existing checks.
#[cfg(feature = "zk-groth16")]
fn attest_gate(spec: &CapsuleSpecVerified, install_caps: u64) -> Result<(), SpawnError> {
    use crate::security::capsule_attest::verify_capsule_attestation;

    let trailer = spec.attestation_trailer;
    if trailer.is_empty() {
        crate::sys::serial::print(b"[ZK-ATTEST] none ");
        crate::sys::serial::print(spec.name.as_bytes());
        crate::sys::serial::print(b"\n");
        #[cfg(feature = "nonos-zk-enforce")]
        return Err(SpawnError::AttestationRejected);
        #[cfg(not(feature = "nonos-zk-enforce"))]
        return Ok(());
    }
    match verify_capsule_attestation(trailer, spec.elf, install_caps) {
        Ok(()) => {
            crate::sys::serial::print(b"[ZK-ATTEST] ok ");
            crate::sys::serial::print(spec.name.as_bytes());
            crate::sys::serial::print(b"\n");
            Ok(())
        }
        Err(e) => {
            crate::sys::serial::print(b"[ZK-ATTEST] FAIL ");
            crate::sys::serial::print(spec.name.as_bytes());
            crate::sys::serial::print(b": ");
            crate::sys::serial::print(e.as_str().as_bytes());
            crate::sys::serial::print(b"\n");
            #[cfg(feature = "nonos-zk-enforce")]
            {
                return Err(SpawnError::AttestationRejected);
            }
            #[cfg(not(feature = "nonos-zk-enforce"))]
            Ok(())
        }
    }
}
