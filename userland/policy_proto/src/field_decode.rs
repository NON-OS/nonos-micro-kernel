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

use super::field::Field;

pub fn decode(id: u32) -> Option<Field> {
    Some(match id {
        0x0101 => Field::Brightness,
        0x0102 => Field::MouseSensitivity,
        0x0103 => Field::SoundEnabled,
        0x0104 => Field::AnonymousMode,
        0x0105 => Field::NymEnabled,
        0x0106 => Field::Theme,
        0x0107 => Field::KeyboardLayout,
        0x0108 => Field::AutoWipe,
        0x0109 => Field::Timezone,
        0x010A => Field::ScreenTimeout,
        0x010B => Field::Language,
        0x010C => Field::DeveloperMode,
        0x010D => Field::HardwareCrypto,
        0x010E => Field::ZkAttestation,
        0x010F => Field::SystemKeysGenerated,
        0x0110 => Field::NotificationsEnabled,
        0x0111 => Field::HighContrast,
        0x0112 => Field::FontSize,
        0x0113 => Field::AutoLockTimeout,
        0x0114 => Field::WifiAutoconnect,
        0x0115 => Field::AnimationsEnabled,
        0x0116 => Field::CursorSize,
        0x0117 => Field::Wallpaper,
        0x0118 => Field::ClockFormat24,
        0x0119 => Field::PreferIpv6,
        0x011A => Field::MeteredConnection,
        0x011B => Field::ProxyMode,
        0x011C => Field::WifiRadio,
        0x011D => Field::WifiAskToJoin,
        0x011E => Field::Volume,
        0x011F => Field::AudioBalance,
        0x0120 => Field::AlertSounds,
        0x0121 => Field::StartupChime,
        0x0201 => Field::KernelAslr,
        0x0202 => Field::KernelStackGuard,
        0x0203 => Field::KernelNxBit,
        0x0204 => Field::KernelSmep,
        0x0205 => Field::KernelSmap,
        0x0206 => Field::KernelDebug,
        0x0207 => Field::KernelSerial,
        0x0208 => Field::KernelWatchdog,
        0x0209 => Field::KernelPreempt,
        0x020A => Field::KernelHugepages,
        0x020B => Field::KernelIommu,
        0x020C => Field::KernelSeccomp,
        0x0301 => Field::Hostname,
        0x0302 => Field::DomainName,
        _ => return None,
    })
}
