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

mod app_orchestrator;
mod apps;
mod apps_tools;
mod boot;
mod core;
#[cfg(not(feature = "microkernel-input-probe"))]
mod desktop_fleet;
#[cfg(not(feature = "microkernel-input-probe"))]
mod desktop_services;
mod drivers_bus;
mod drivers_input;
mod drivers_nic;
mod drivers_storage;
mod drivers_usb;
mod drivers_virtio;
mod drivers_virtio_display;
mod drivers_virtio_io;
#[cfg(feature = "microkernel-input-probe")]
mod input_probe_fleet;
mod network;
mod orchestrator;
mod services_audio;
pub(super) use app_orchestrator::spawn_apps;
pub(super) use orchestrator::{
    spawn_core_after_ramfs, spawn_desktop, spawn_display_core, spawn_drivers, spawn_market,
    spawn_network, spawn_ramfs, spawn_vfs,
};

#[cfg(feature = "microkernel-setup-wizard")]
pub(super) use orchestrator::spawn_post_wizard;
