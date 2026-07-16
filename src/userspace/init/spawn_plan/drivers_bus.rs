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

pub(super) fn spawn() {
    spawn_iwlwifi();
    spawn_rtl8821ce();
    spawn_i2c_pci();
    spawn_i2c_hid();
}

#[cfg(feature = "nonos-capsule-driver-iwlwifi")]
fn spawn_iwlwifi() {
    use crate::hardware::iwlwifi_capsule as c;
    super::boot::capsule(
        "DRIVER-IWLWIFI",
        "driver_iwlwifi",
        c::spawn_driver_iwlwifi_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-iwlwifi"))]
fn spawn_iwlwifi() {}

#[cfg(feature = "nonos-capsule-driver-rtl8821ce")]
fn spawn_rtl8821ce() {
    use crate::hardware::rtl8821ce_capsule as c;
    super::boot::capsule(
        "DRIVER-RTL8821CE",
        "driver_rtl8821ce",
        c::spawn_driver_rtl8821ce_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-rtl8821ce"))]
fn spawn_rtl8821ce() {}

#[cfg(feature = "nonos-capsule-driver-i2c-pci")]
fn spawn_i2c_pci() {
    use crate::hardware::i2c_pci_capsule as c;
    super::boot::capsule(
        "DRIVER-I2C-PCI",
        "driver_i2c_pci",
        c::spawn_driver_i2c_pci_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-i2c-pci"))]
fn spawn_i2c_pci() {}

#[cfg(feature = "nonos-capsule-driver-i2c-hid")]
fn spawn_i2c_hid() {
    use crate::userspace::capsule_driver_i2c_hid as c;
    super::boot::capsule(
        "DRIVER-I2C-HID",
        "driver_i2c_hid",
        c::spawn_driver_i2c_hid_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-driver-i2c-hid"))]
fn spawn_i2c_hid() {}
