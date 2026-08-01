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

#[cfg(feature = "nonos-capsule-driver-usb-hid")]
pub(super) const DRIVER_USB_HID_ELF: &[u8] = include_bytes!(concat!(
    "../../../userland/capsule_driver_usb_hid/target/",
    env!("NONOS_USER_TARGET"),
    "/release/driver_usb_hid"
));

#[cfg(feature = "nonos-capsule-driver-usb-hid")]
pub(super) const DRIVER_USB_HID_NONOS_ID_CERT_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_usb_hid.nonos_id_cert.bin");

#[cfg(feature = "nonos-capsule-driver-usb-hid")]
pub(super) const DRIVER_USB_HID_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_usb_hid.manifest.bin");

#[cfg(feature = "nonos-capsule-driver-usb-hid")]
pub(super) const DRIVER_USB_HID_ATTESTATION_BYTES: &[u8] =
    include_bytes!("../../../nonos-data/trust/capsules/driver_usb_hid.zk_trailer.bin");

#[cfg(not(feature = "nonos-capsule-driver-usb-hid"))]
pub(super) const DRIVER_USB_HID_ELF: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-usb-hid"))]
pub(super) const DRIVER_USB_HID_NONOS_ID_CERT_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-usb-hid"))]
pub(super) const DRIVER_USB_HID_MANIFEST_BYTES: &[u8] = &[];

#[cfg(not(feature = "nonos-capsule-driver-usb-hid"))]
pub(super) const DRIVER_USB_HID_ATTESTATION_BYTES: &[u8] = &[];
