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

pub fn get(field: Field) -> Option<bool> {
    let s = STORE.lock();
    Some(match field {
        Field::SoundEnabled => s.sound_enabled,
        Field::AnonymousMode => s.anonymous_mode,
        Field::NymEnabled => s.nym_enabled,
        Field::AutoWipe => s.auto_wipe,
        Field::DeveloperMode => s.developer_mode,
        Field::HardwareCrypto => s.hardware_crypto,
        Field::ZkAttestation => s.zk_attestation,
        Field::SystemKeysGenerated => s.system_keys_generated,
        Field::NotificationsEnabled => s.notifications_enabled,
        Field::HighContrast => s.high_contrast,
        Field::WifiAutoconnect => s.wifi_autoconnect,
        Field::AnimationsEnabled => s.animations_enabled,
        Field::ClockFormat24 => s.clock_format24,
        Field::PreferIpv6 => s.prefer_ipv6,
        Field::MeteredConnection => s.metered_connection,
        Field::WifiRadio => s.wifi_radio,
        Field::WifiAskToJoin => s.wifi_ask_to_join,
        Field::AlertSounds => s.alert_sounds,
        Field::StartupChime => s.startup_chime,
        Field::KernelAslr => s.kernel_aslr,
        Field::KernelStackGuard => s.kernel_stack_guard,
        Field::KernelNxBit => s.kernel_nx_bit,
        Field::KernelSmep => s.kernel_smep,
        Field::KernelSmap => s.kernel_smap,
        Field::KernelDebug => s.kernel_debug,
        Field::KernelSerial => s.kernel_serial,
        Field::KernelWatchdog => s.kernel_watchdog,
        Field::KernelPreempt => s.kernel_preempt,
        Field::KernelHugepages => s.kernel_hugepages,
        Field::KernelIommu => s.kernel_iommu,
        Field::KernelSeccomp => s.kernel_seccomp,
        _ => return None,
    })
}
