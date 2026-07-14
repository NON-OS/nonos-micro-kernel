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
pub fn device_info(device: u16) -> Option<(&'static str, u32)> {
    match device {
        0x9D60..=0x9D65 => Some(("Sunrise Point-LP", 120_000_000)),
        0xA160..=0xA163 => Some(("Sunrise Point-H", 120_000_000)),
        0x9DE8..=0x9DEB => Some(("Cannon Point-LP", 120_000_000)),
        0xA368..=0xA36B => Some(("Cannon Lake-H", 120_000_000)),
        0x02E8..=0x02EB => Some(("Comet Lake", 120_000_000)),
        0x06E8..=0x06EB => Some(("Comet Lake-H", 120_000_000)),
        0xA0E8..=0xA0EB | 0xA0C5 | 0xA0C6 => Some(("Tiger Lake-LP", 100_000_000)),
        0x43E8..=0x43EB => Some(("Tiger Lake-H", 100_000_000)),
        0x51E8..=0x51EB | 0x51C5 | 0x51C6 => Some(("Alder Lake-P", 100_000_000)),
        0x7AE8..=0x7AEB | 0x7AF8 | 0x7AF9 => Some(("Alder Lake-S", 100_000_000)),
        0xA0D8..=0xA0DD => Some(("Raptor Lake-P", 100_000_000)),
        0x7A4C..=0x7A4F | 0x7A7C | 0x7A7D => Some(("Raptor Lake-S", 100_000_000)),
        0x54E8..=0x54EB => Some(("Alder Lake-N", 100_000_000)),
        0x7E50..=0x7E52 | 0x7E78..=0x7E7A => Some(("Meteor Lake-P", 100_000_000)),
        0x34E8..=0x34EB | 0x34C5 | 0x34C6 => Some(("Ice Lake-LP", 100_000_000)),
        0x4DE8..=0x4DEB | 0x4DC5 | 0x4DC6 => Some(("Jasper Lake", 100_000_000)),
        // Broxton and Gemini Lake I2C runs from a 133 MHz input clock (Linux
        // intel-lpss-pci bxt_i2c_info). Deriving the SCL counts from 100 MHz
        // clocks the bus a third faster than programmed, out of fast-mode spec.
        0x5AC2 | 0x5AC4 | 0x5AC6 | 0x5AEE => Some(("Broxton", 133_000_000)),
        0x1AC2 | 0x1AC4 | 0x1AC6 | 0x1AEE => Some(("Broxton-P", 133_000_000)),
        0x31AC | 0x31AE | 0x31B0 | 0x31B2 => Some(("Gemini Lake", 133_000_000)),
        0x31B4 | 0x31B6 | 0x31B8 | 0x31BA => Some(("Gemini Lake", 133_000_000)),
        _ => None,
    }
}
