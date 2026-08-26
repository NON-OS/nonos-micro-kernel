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

pub const STRING_CAP: usize = 64;

#[derive(Clone, Copy)]
pub struct StringField {
    pub bytes: [u8; STRING_CAP],
    pub len: usize,
}

#[derive(Clone, Copy)]
pub struct Store {
    pub brightness: u8,
    pub mouse_sensitivity: u8,
    pub sound_enabled: bool,
    pub anonymous_mode: bool,
    pub nym_enabled: bool,
    pub theme: u8,
    pub keyboard_layout: u8,
    pub auto_wipe: bool,
    pub timezone: i8,
    pub screen_timeout: u8,
    pub language: u8,
    pub developer_mode: bool,
    pub hardware_crypto: bool,
    pub zk_attestation: bool,
    pub system_keys_generated: bool,
    pub notifications_enabled: bool,
    pub high_contrast: bool,
    pub font_size: u8,
    pub auto_lock_timeout: u8,
    pub wifi_autoconnect: bool,
    pub animations_enabled: bool,
    pub cursor_size: u8,
    pub wallpaper: u8,
    pub clock_format24: bool,
    pub prefer_ipv6: bool,
    pub metered_connection: bool,
    pub proxy_mode: u8,
    pub wifi_radio: bool,
    pub wifi_ask_to_join: bool,
    pub volume: u8,
    pub audio_balance: u8,
    pub alert_sounds: bool,
    pub startup_chime: bool,
    pub kernel_aslr: bool,
    pub kernel_stack_guard: bool,
    pub kernel_nx_bit: bool,
    pub kernel_smep: bool,
    pub kernel_smap: bool,
    pub kernel_debug: bool,
    pub kernel_serial: bool,
    pub kernel_watchdog: bool,
    pub kernel_preempt: bool,
    pub kernel_hugepages: bool,
    pub kernel_iommu: bool,
    pub kernel_seccomp: bool,
    pub hostname: StringField,
    pub domainname: StringField,
}
