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

use super::read_at::read_descriptor_at;
use crate::hid::HID_DESC_LEN;

const CANDIDATE_ADDRS: &[u8] = &[0x10, 0x15, 0x2C, 0x38, 0x4B, 0x4C, 0x20, 0x24];
// 0x0001 is by far the most common HID descriptor register; some touchpads
// place it at 0x0020.
const DESC_REGS: &[u16] = &[0x0001, 0x0020];

/// Blind-probe the common touchpad addresses and descriptor registers. Used as
/// a fallback when the firmware did not declare the device through ACPI.
pub fn probe_bus(port: u32, descriptor: &mut [u8; HID_DESC_LEN]) -> Option<(u8, usize)> {
    for &addr in CANDIDATE_ADDRS {
        for &reg in DESC_REGS {
            if read_descriptor_at(port, addr, reg, descriptor) {
                return Some((addr, HID_DESC_LEN));
            }
        }
    }
    None
}
