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

use super::types::{NetworkSettings, SavedNetwork};
use crate::network::boot_config::PrivacyMode;
use crate::sys::sync::IrqMutex;
use alloc::vec::Vec;
use core::sync::atomic::AtomicBool;

pub static NETWORK_SETTINGS: IrqMutex<NetworkSettings> = IrqMutex::new(NetworkSettings {
    privacy_mode: PrivacyMode::TorOnly,
    dhcp_enabled: true,
    static_ip: [0, 0, 0, 0],
    subnet_prefix: 24,
    gateway: [0, 0, 0, 0],
    dns_primary: [8, 8, 8, 8],
    dns_secondary: [8, 8, 4, 4],
    dns_over_onion: true,
    onion_enabled: true,
    onion_auto_connect: true,
    onion_prebuild_circuits: 3,
    onion_relay_mode: false,
    socks_enabled: true,
    socks_port: 9050,
    transparent_proxy: true,
    strict_onion: true,
    randomize_mac: true,
    firewall_enabled: true,
    block_inbound: true,
    log_connections: true,
});

pub static SAVED_NETWORKS: IrqMutex<Vec<SavedNetwork>> = IrqMutex::new(Vec::new());
pub static SETTINGS_MODIFIED: AtomicBool = AtomicBool::new(false);
