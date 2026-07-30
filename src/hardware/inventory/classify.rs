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

use super::classify_display::classify_display;
use super::classify_network::classify_network;
use super::classify_serial_bus::classify_serial_bus;
use super::classify_storage::classify_storage;
use super::family::HardwareFamily;

pub fn classify_family(
    class: u8,
    subclass: u8,
    progif: u8,
    vendor: u16,
    device: u16,
) -> HardwareFamily {
    match class {
        0x01 => classify_storage(subclass, vendor),
        0x02 => classify_network(subclass, vendor, device),
        0x03 => classify_display(vendor),
        0x04 => match subclass {
            0x01 | 0x03 => HardwareFamily::AudioHda,
            _ => HardwareFamily::Unknown,
        },
        0x06 => HardwareFamily::BridgePci,
        0x08 => HardwareFamily::SystemPeripheral,
        0x0c => classify_serial_bus(subclass, progif),
        _ => HardwareFamily::Unknown,
    }
}
