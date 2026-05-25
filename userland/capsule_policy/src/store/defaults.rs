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

use super::types::{Store, StringField, STRING_CAP};

const DEFAULT_HOSTNAME: &[u8] = b"nonos";

pub const fn store() -> Store {
    Store {
        brightness: 80,
        mouse_sensitivity: 5,
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
        wallpaper: 0,
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

const fn default_hostname() -> StringField {
    let mut bytes = [0u8; STRING_CAP];
    let src = DEFAULT_HOSTNAME;
    let mut i = 0;
    while i < src.len() {
        bytes[i] = src[i];
        i += 1;
    }
    StringField { bytes, len: src.len() }
}

const fn empty_string() -> StringField {
    StringField { bytes: [0u8; STRING_CAP], len: 0 }
}
