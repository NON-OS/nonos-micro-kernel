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

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Brightness = 0x0101,
    MouseSensitivity = 0x0102,
    SoundEnabled = 0x0103,
    AnonymousMode = 0x0104,
    NymEnabled = 0x0105,
    Theme = 0x0106,
    KeyboardLayout = 0x0107,
    AutoWipe = 0x0108,
    Timezone = 0x0109,
    ScreenTimeout = 0x010A,
    Language = 0x010B,
    DeveloperMode = 0x010C,
    HardwareCrypto = 0x010D,
    ZkAttestation = 0x010E,
    SystemKeysGenerated = 0x010F,
    NotificationsEnabled = 0x0110,
    HighContrast = 0x0111,
    FontSize = 0x0112,
    AutoLockTimeout = 0x0113,
    WifiAutoconnect = 0x0114,
    AnimationsEnabled = 0x0115,
    CursorSize = 0x0116,
    Wallpaper = 0x0117,
    ClockFormat24 = 0x0118,
    PreferIpv6 = 0x0119,
    MeteredConnection = 0x011A,
    ProxyMode = 0x011B,
    WifiRadio = 0x011C,
    WifiAskToJoin = 0x011D,
    Volume = 0x011E,
    AudioBalance = 0x011F,
    AlertSounds = 0x0120,
    StartupChime = 0x0121,
    KernelAslr = 0x0201,
    KernelStackGuard = 0x0202,
    KernelNxBit = 0x0203,
    KernelSmep = 0x0204,
    KernelSmap = 0x0205,
    KernelDebug = 0x0206,
    KernelSerial = 0x0207,
    KernelWatchdog = 0x0208,
    KernelPreempt = 0x0209,
    KernelHugepages = 0x020A,
    KernelIommu = 0x020B,
    KernelSeccomp = 0x020C,
    Hostname = 0x0301,
    DomainName = 0x0302,
}
