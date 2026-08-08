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

#[cfg(feature = "nonos-capsule-std-proof")]
pub(crate) const STD_PROOF_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_std_proof/target/",
    env!("NONOS_USER_TARGET"),
    "/release/std_proof"
));

#[cfg(feature = "nonos-capsule-std-proof")]
pub(crate) const STD_PROOF_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/std_proof.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-std-proof")]
pub(crate) const STD_PROOF_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/std_proof.manifest.bin");

#[cfg(feature = "nonos-capsule-std-proof")]
pub(crate) const STD_PROOF_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/std_proof.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-std-proof"))]
pub(crate) const STD_PROOF_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-std-proof"))]
pub(crate) const STD_PROOF_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-std-proof"))]
pub(crate) const STD_PROOF_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-std-proof"))]
pub(crate) const STD_PROOF_ATTESTATION_BYTES: &[u8] = &[];
