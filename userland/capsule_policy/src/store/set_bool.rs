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

use nonos_policy_proto::Field;

use super::state::STORE;

pub fn set(field: Field, value: bool) -> bool {
    let mut s = STORE.lock();
    match field {
        Field::SoundEnabled => s.sound_enabled = value,
        Field::AnonymousMode => s.anonymous_mode = value,
        Field::NymEnabled => s.nym_enabled = value,
        Field::AutoWipe => s.auto_wipe = value,
        Field::DeveloperMode => s.developer_mode = value,
        Field::HardwareCrypto => s.hardware_crypto = value,
        Field::ZkAttestation => s.zk_attestation = value,
        Field::SystemKeysGenerated => s.system_keys_generated = value,
        Field::NotificationsEnabled => s.notifications_enabled = value,
        Field::HighContrast => s.high_contrast = value,
        Field::WifiAutoconnect => s.wifi_autoconnect = value,
        Field::AnimationsEnabled => s.animations_enabled = value,
        Field::ClockFormat24 => s.clock_format24 = value,
        Field::PreferIpv6 => s.prefer_ipv6 = value,
        Field::MeteredConnection => s.metered_connection = value,
        Field::WifiRadio => s.wifi_radio = value,
        Field::WifiAskToJoin => s.wifi_ask_to_join = value,
        Field::AlertSounds => s.alert_sounds = value,
        Field::StartupChime => s.startup_chime = value,
        Field::KernelAslr => s.kernel_aslr = value,
        Field::KernelStackGuard => s.kernel_stack_guard = value,
        Field::KernelNxBit => s.kernel_nx_bit = value,
        Field::KernelSmep => s.kernel_smep = value,
        Field::KernelSmap => s.kernel_smap = value,
        Field::KernelDebug => s.kernel_debug = value,
        Field::KernelSerial => s.kernel_serial = value,
        Field::KernelWatchdog => s.kernel_watchdog = value,
        Field::KernelPreempt => s.kernel_preempt = value,
        Field::KernelHugepages => s.kernel_hugepages = value,
        Field::KernelIommu => s.kernel_iommu = value,
        Field::KernelSeccomp => s.kernel_seccomp = value,
        _ => return false,
    }
    true
}
