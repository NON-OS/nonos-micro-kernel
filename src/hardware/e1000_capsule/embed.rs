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

//! Build-time embed of the e1000 driver capsule binary. The
//! Makefile target `nonos-mk-driver-e1000` produces the ELF; the
//! kernel feature `nonos-capsule-driver-e1000` selects whether
//! the bytes are pulled into the kernel image or replaced by an
//! empty slice.

#[cfg(feature = "nonos-capsule-driver-e1000")]
pub(super) const DRIVER_E1000_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_e1000/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_e1000"
));

#[cfg(feature = "nonos-capsule-driver-e1000")]
pub(super) const DRIVER_E1000_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_e1000.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-e1000")]
pub(super) const DRIVER_E1000_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_e1000.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-e1000")]
pub(super) const DRIVER_E1000_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_e1000.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-driver-e1000"))]
pub(super) const DRIVER_E1000_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-e1000"))]
pub(super) const DRIVER_E1000_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-e1000"))]
pub(super) const DRIVER_E1000_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-e1000"))]
pub(super) const DRIVER_E1000_ATTESTATION_BYTES: &[u8] = &[];
