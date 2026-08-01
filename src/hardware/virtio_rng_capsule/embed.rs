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

//! Build-time embed of the virtio-rng driver capsule binary. The
//! Makefile target `nonos-mk-virtio-rng` produces the ELF; the
//! kernel feature `nonos-capsule-driver-virtio-rng` selects whether
//! the bytes are pulled into the kernel image or replaced by an
//! empty slice (no driver capsule available).

#[cfg(feature = "nonos-capsule-driver-virtio-rng")]
pub(super) const DRIVER_VIRTIO_RNG_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_virtio_rng/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_virtio_rng"
));

#[cfg(feature = "nonos-capsule-driver-virtio-rng")]
pub(super) const DRIVER_VIRTIO_RNG_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_virtio_rng.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-virtio-rng")]
pub(super) const DRIVER_VIRTIO_RNG_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_virtio_rng.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-virtio-rng")]
pub(super) const DRIVER_VIRTIO_RNG_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_virtio_rng.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-driver-virtio-rng"))]
pub(super) const DRIVER_VIRTIO_RNG_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-virtio-rng"))]
pub(super) const DRIVER_VIRTIO_RNG_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-virtio-rng"))]
pub(super) const DRIVER_VIRTIO_RNG_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-virtio-rng"))]
pub(super) const DRIVER_VIRTIO_RNG_ATTESTATION_BYTES: &[u8] = &[];
