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

#[cfg(feature = "nonos-capsule-sd")]
pub(crate) const SD_ELF: &[u8] =
    include_bytes!(concat!(
    "../../../userland/capsule_sd/target/",
    env!("NONOS_USER_TARGET"),
    "/release/sd"
));

#[cfg(feature = "nonos-capsule-sd")]
pub(crate) const SD_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/sd.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-sd")]
pub(crate) const SD_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/sd.manifest.bin");

#[cfg(feature = "nonos-capsule-sd")]
pub(crate) const SD_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/sd.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-sd"))]
pub(crate) const SD_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-sd"))]
pub(crate) const SD_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-sd"))]
pub(crate) const SD_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-sd"))]
pub(crate) const SD_ATTESTATION_BYTES: &[u8] = &[];
