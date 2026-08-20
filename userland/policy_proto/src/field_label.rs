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

pub fn label_of(field: Field) -> &'static [u8] {
    match field {
        Field::Brightness => b"Display brightness",
        Field::MouseSensitivity => b"Pointer speed",
        Field::SoundEnabled => b"System sound",
        Field::AnonymousMode => b"Anonymous mode",
        Field::NymEnabled => b"Nym routing",
        Field::Theme => b"Color theme",
        Field::Wallpaper => b"Wallpaper",
        Field::KeyboardLayout => b"Keyboard layout",
        Field::AutoWipe => b"Auto-wipe on shutdown",
        Field::Timezone => b"Timezone (hours from UTC)",
        Field::ScreenTimeout => b"Screen blank timeout (min)",
        Field::Language => b"Language",
        Field::DeveloperMode => b"Developer mode",
        Field::HardwareCrypto => b"Hardware crypto offload",
        Field::ZkAttestation => b"Zero-knowledge attestation",
        Field::SystemKeysGenerated => b"System keys provisioned",
        Field::NotificationsEnabled => b"Notifications",
        Field::HighContrast => b"High contrast mode",
        Field::FontSize => b"Text size",
        Field::AutoLockTimeout => b"Auto-lock after idle (min)",
        Field::WifiAutoconnect => b"Wi-Fi auto-connect",
        Field::AnimationsEnabled => b"UI animations",
        Field::CursorSize => b"Cursor size",
        Field::ClockFormat24 => b"24-hour clock",
        Field::PreferIpv6 => b"Prefer IPv6",
        Field::MeteredConnection => b"Metered connection",
        Field::ProxyMode => b"Proxy",
        Field::WifiRadio => b"Wi-Fi",
        Field::WifiAskToJoin => b"Ask to join networks",
        Field::Volume => b"Volume",
        Field::AudioBalance => b"Balance",
        Field::AlertSounds => b"Alert sounds",
        Field::StartupChime => b"Startup chime",
        Field::KernelAslr => b"Kernel ASLR",
        Field::KernelStackGuard => b"Stack guard pages",
        Field::KernelNxBit => b"NX bit enforcement",
        Field::KernelSmep => b"SMEP (supervisor exec prevention)",
        Field::KernelSmap => b"SMAP (supervisor access prevention)",
        Field::KernelDebug => b"Kernel debug output",
        Field::KernelSerial => b"Serial console",
        Field::KernelWatchdog => b"Hard-lockup watchdog",
        Field::KernelPreempt => b"Preemptive scheduling",
        Field::KernelHugepages => b"Huge page allocator",
        Field::KernelIommu => b"IOMMU isolation",
        Field::KernelSeccomp => b"Seccomp syscall filter",
        Field::Hostname => b"Hostname",
        Field::DomainName => b"Domain name",
    }
}
