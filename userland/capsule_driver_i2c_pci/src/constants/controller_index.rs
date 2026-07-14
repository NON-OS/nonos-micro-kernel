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
/// Index of an LPSS I2C function among its family's controllers (the `n` in
/// the firmware's I2Cn device name), derived from the PCI device id layout.
/// Lets the driver match the ACPI `_CRS` ResourceSource name against an
/// enumerated PCI function. Only families whose id blocks are index-ordered
/// are mapped; None means the name cannot be matched and probing decides.
pub fn controller_index(device: u16) -> Option<u8> {
    match device {
        // Gemini Lake: I2C0..I2C7 = 0x31AC, 0x31AE, .. 0x31BA (even ids).
        0x31AC..=0x31BA if device & 1 == 0 => Some(((device - 0x31AC) / 2) as u8),
        _ => None,
    }
}
