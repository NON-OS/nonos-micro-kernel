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

#[cfg(feature = "nonos-capsule-wallpaper-catalog")]
pub(crate) const WALLPAPER_CATALOG_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_wallpaper_catalog/target/",
    env!("NONOS_USER_TARGET"),
    "/release/wallpaper_catalog"
));

#[cfg(feature = "nonos-capsule-wallpaper-catalog")]
pub(crate) const WALLPAPER_CATALOG_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/wallpaper_catalog.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-wallpaper-catalog")]
pub(crate) const WALLPAPER_CATALOG_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/wallpaper_catalog.manifest.bin");

#[cfg(feature = "nonos-capsule-wallpaper-catalog")]
pub(crate) const WALLPAPER_CATALOG_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/wallpaper_catalog.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-wallpaper-catalog"))]
pub(crate) const WALLPAPER_CATALOG_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-wallpaper-catalog"))]
pub(crate) const WALLPAPER_CATALOG_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-wallpaper-catalog"))]
pub(crate) const WALLPAPER_CATALOG_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-wallpaper-catalog"))]
pub(crate) const WALLPAPER_CATALOG_ATTESTATION_BYTES: &[u8] = &[];
