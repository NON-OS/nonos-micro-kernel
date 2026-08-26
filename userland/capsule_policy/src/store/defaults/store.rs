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

use super::default_hostname::default_hostname;
use super::empty_string::empty_string;
use crate::store::types::Store;

pub const fn store() -> Store {
    Store {
        brightness: 80,
        mouse_sensitivity: 2,
        sound_enabled: true,
        anonymous_mode: true,
        nym_enabled: false,
        theme: 0,
        keyboard_layout: 0,
        auto_wipe: true,
        timezone: 0,
        screen_timeout: 0,
        language: 0,
        developer_mode: false,
        hardware_crypto: true,
        zk_attestation: true,
        system_keys_generated: false,
        notifications_enabled: true,
        high_contrast: false,
        font_size: 1,
        auto_lock_timeout: 5,
        wifi_autoconnect: true,
        animations_enabled: true,
        cursor_size: 1,
        wallpaper: 48,
        clock_format24: true,
        prefer_ipv6: false,
        metered_connection: false,
        proxy_mode: 0,
        wifi_radio: true,
        wifi_ask_to_join: true,
        volume: 64,
        audio_balance: 50,
        alert_sounds: false,
        startup_chime: false,
        kernel_aslr: true,
        kernel_stack_guard: true,
        kernel_nx_bit: true,
        kernel_smep: true,
        kernel_smap: true,
        kernel_debug: false,
        kernel_serial: true,
        kernel_watchdog: false,
        kernel_preempt: true,
        kernel_hugepages: false,
        kernel_iommu: true,
        kernel_seccomp: true,
        hostname: default_hostname(),
        domainname: empty_string(),
    }
}
