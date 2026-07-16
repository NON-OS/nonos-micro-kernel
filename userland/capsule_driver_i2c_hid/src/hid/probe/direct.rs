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

/// Try the exact address and descriptor register the firmware declared through
/// ACPI. Returns the descriptor length on success, so the driver binds without
/// probing a guessed list.
pub fn probe_addr(
    port: u32,
    addr: u8,
    reg: u16,
    descriptor: &mut [u8; HID_DESC_LEN],
) -> Option<usize> {
    read_descriptor_at(port, addr, reg, descriptor).then_some(HID_DESC_LEN)
}
