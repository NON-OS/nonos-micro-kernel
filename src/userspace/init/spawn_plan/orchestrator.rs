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

pub(in crate::userspace::init) fn spawn_ramfs() {
    super::core::spawn_ramfs();
}

pub(in crate::userspace::init) fn run_ramfs_smoketest() {
    super::smoketests::run_ramfs();
}

pub(in crate::userspace::init) fn spawn_core_after_ramfs() {
    super::core::spawn_after_ramfs();
}

pub(in crate::userspace::init) fn spawn_display_core() {
    super::desktop_fleet::spawn_early_display();
}

pub(in crate::userspace::init) fn spawn_drivers() {
    super::drivers_virtio::spawn();
    super::drivers_bus::spawn();
    super::drivers_input::spawn();
    super::drivers_nic::spawn();
    super::drivers_usb::spawn();
    super::drivers_storage::spawn();
}

pub(in crate::userspace::init) fn spawn_vfs() {
    super::core::spawn_vfs();
}

pub(in crate::userspace::init) fn spawn_network() {
    super::network::spawn();
}

#[cfg(feature = "microkernel-input-probe")]
pub(in crate::userspace::init) fn spawn_desktop() {
    super::input_probe_fleet::spawn();
}

#[cfg(all(feature = "microkernel-setup-wizard", not(feature = "microkernel-input-probe")))]
pub(in crate::userspace::init) fn spawn_desktop() {
    use crate::userspace::capsule_setup_wizard as wiz;
    super::desktop_fleet::spawn_gui_core();
    super::boot::capsule(
        "SETUP-WIZARD",
        "setup_wizard",
        wiz::spawn_setup_wizard_capsule,
        wiz::shared_state,
    );
}

#[cfg(feature = "microkernel-setup-wizard")]
pub(in crate::userspace::init) fn spawn_post_wizard() {
    super::desktop_fleet::spawn();
    super::core::spawn_market();
}

#[cfg(all(not(feature = "microkernel-input-probe"), not(feature = "microkernel-setup-wizard")))]
pub(in crate::userspace::init) fn spawn_desktop() {
    super::desktop_fleet::spawn();
}

#[cfg(not(feature = "microkernel-setup-wizard"))]
pub(in crate::userspace::init) fn spawn_market() {
    super::core::spawn_market();
}
#[cfg(feature = "microkernel-setup-wizard")]
pub(in crate::userspace::init) fn spawn_market() {}

pub(in crate::userspace::init) fn run_smoketests() {
    super::smoketests::run_all();
}
