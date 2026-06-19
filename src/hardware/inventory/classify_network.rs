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

use super::family::HardwareFamily;

pub(super) fn classify_network(subclass: u8, vendor: u16, device: u16) -> HardwareFamily {
    match vendor {
        0x1af4 => HardwareFamily::NetworkVirtio,
        0x8086 if subclass == 0x80 => HardwareFamily::NetworkIwlwifi,
        0x8086 => HardwareFamily::NetworkE1000,
        0x10ec if device == 0x8139 => HardwareFamily::NetworkRtl8139,
        0x10ec => HardwareFamily::NetworkRtl8169,
        _ => HardwareFamily::Unknown,
    }
}
