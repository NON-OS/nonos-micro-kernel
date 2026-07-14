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

use crate::hid::{valid_descriptor, HID_DESC_LEN};
use crate::i2c_client::write_read;

/// Read and validate the HID descriptor at one (address, register), copying it
/// into `descriptor` on success.
pub(super) fn read_descriptor_at(
    port: u32,
    addr: u8,
    reg: u16,
    descriptor: &mut [u8; HID_DESC_LEN],
) -> bool {
    let mut buf = [0u8; HID_DESC_LEN];
    if write_read(port, addr, &reg.to_le_bytes(), &mut buf).is_some() && valid_descriptor(&buf) {
        descriptor.copy_from_slice(&buf);
        return true;
    }
    false
}
