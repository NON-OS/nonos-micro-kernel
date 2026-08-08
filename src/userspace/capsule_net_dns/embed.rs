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

#[cfg(feature = "nonos-capsule-net-dns")]
pub(super) const NET_DNS_ELF: &[u8] =
    include_bytes!(concat!(
    "../../../userland/capsule_net_dns/target/",
    env!("NONOS_USER_TARGET"),
    "/release/net_dns"
));

#[cfg(feature = "nonos-capsule-net-dns")]
pub(super) const NET_DNS_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/net_dns.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-net-dns")]
pub(super) const NET_DNS_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/net_dns.manifest.bin");

#[cfg(feature = "nonos-capsule-net-dns")]
pub(super) const NET_DNS_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/net_dns.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-net-dns"))]
pub(super) const NET_DNS_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-net-dns"))]
pub(super) const NET_DNS_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-net-dns"))]
pub(super) const NET_DNS_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-net-dns"))]
pub(super) const NET_DNS_ATTESTATION_BYTES: &[u8] = &[];
