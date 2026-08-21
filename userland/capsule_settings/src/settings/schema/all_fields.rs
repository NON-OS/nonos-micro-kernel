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

pub const ALL_FIELDS: &[Field] = &[
    Field::Brightness,
    Field::MouseSensitivity,
    Field::SoundEnabled,
    Field::AnonymousMode,
    Field::NymEnabled,
    Field::Theme,
    Field::KeyboardLayout,
    Field::AutoWipe,
    Field::Timezone,
    Field::ScreenTimeout,
    Field::Language,
    Field::DeveloperMode,
    Field::HardwareCrypto,
    Field::ZkAttestation,
    Field::SystemKeysGenerated,
    Field::NotificationsEnabled,
    Field::HighContrast,
    Field::FontSize,
    Field::AutoLockTimeout,
    Field::WifiAutoconnect,
    Field::AnimationsEnabled,
    Field::CursorSize,
    Field::Wallpaper,
    Field::ClockFormat24,
    Field::KernelAslr,
    Field::KernelStackGuard,
    Field::KernelNxBit,
    Field::KernelSmep,
    Field::KernelSmap,
    Field::KernelDebug,
    Field::KernelSerial,
    Field::KernelWatchdog,
    Field::KernelPreempt,
    Field::KernelHugepages,
    Field::KernelIommu,
    Field::KernelSeccomp,
    Field::Hostname,
    Field::DomainName,
    Field::PreferIpv6,
    Field::MeteredConnection,
    Field::ProxyMode,
    Field::WifiRadio,
    Field::WifiAskToJoin,
    Field::Volume,
    Field::AudioBalance,
    Field::AlertSounds,
    Field::StartupChime,
];
