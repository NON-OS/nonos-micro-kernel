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

//! Build-time embed of the `net.udp` userland capsule. Same shape
//! as the `net.l2` and `net.ip` embeds.

#[cfg(feature = "nonos-capsule-net-udp")]
pub(super) const NET_UDP_ELF: &[u8] =
    include_bytes!(concat!(
    "../../../userland/capsule_net_udp/target/",
    env!("NONOS_USER_TARGET"),
    "/release/net_udp"
));

#[cfg(feature = "nonos-capsule-net-udp")]
pub(super) const NET_UDP_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/net_udp.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-net-udp")]
pub(super) const NET_UDP_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/net_udp.manifest.bin");

#[cfg(feature = "nonos-capsule-net-udp")]
pub(super) const NET_UDP_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/net_udp.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-net-udp"))]
pub(super) const NET_UDP_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-net-udp"))]
pub(super) const NET_UDP_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-net-udp"))]
pub(super) const NET_UDP_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-net-udp"))]
pub(super) const NET_UDP_ATTESTATION_BYTES: &[u8] = &[];
