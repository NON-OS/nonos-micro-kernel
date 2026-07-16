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

use crate::hardware::broker::class::ids;
use crate::hardware::broker::DeviceRecord;

// A short human name for the broker class, focused on the input controllers
// this census exists to confirm.
pub(super) fn class_label(rec: &DeviceRecord) -> &'static str {
    match rec.class {
        ids::USB_HOST_XHCI => "xHCI USB host",
        ids::USB_HOST => "USB host",
        ids::I2C_HID => "i2c-HID input",
        ids::SERIAL => "i2c/serial ctrl",
        ids::GPIO_CTRL => "gpio community",
        ids::INPUT => "PS2/legacy input",
        ids::BLOCK => "block",
        ids::NETWORK => "network",
        ids::DISPLAY => "display",
        ids::AUDIO => "audio",
        ids::RNG => "rng",
        _ => "other",
    }
}
