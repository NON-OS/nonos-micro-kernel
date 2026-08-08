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

use super::access::{read16, read8};
use super::device::assign_device;

const VENDOR_ID: u8 = 0x00;
const HEADER_TYPE: u8 = 0x0E;
const HEADER_MULTI_FUNCTION: u8 = 0x80;
const HEADER_TYPE_MASK: u8 = 0x7F;
const HEADER_TYPE_ENDPOINT: u8 = 0x00;
const INVALID_VENDOR: u16 = 0xFFFF;
const MAX_BUS: u16 = 256;
const MAX_DEVICE: u8 = 32;
const MAX_FUNCTION: u8 = 8;

/// Walk config space and assign whatever firmware left unassigned.
///
/// Returns how many functions were given at least one address, which is zero
/// on a machine whose firmware already did the work.
pub fn assign_unassigned() -> usize {
    let mut assigned = 0;
    for bus in 0..MAX_BUS {
        let bus = bus as u8;
        for device in 0..MAX_DEVICE {
            if read16(bus, device, 0, VENDOR_ID) == INVALID_VENDOR {
                continue;
            }
            let header = read8(bus, device, 0, HEADER_TYPE);
            let functions = if header & HEADER_MULTI_FUNCTION != 0 { MAX_FUNCTION } else { 1 };
            for function in 0..functions {
                if read16(bus, device, function, VENDOR_ID) == INVALID_VENDOR {
                    continue;
                }
                // Bridges carry windows rather than BARs, and programming those
                // means deciding the whole bus hierarchy. Nothing behind a
                // bridge is reachable until that is done, so endpoints only.
                let kind = read8(bus, device, function, HEADER_TYPE) & HEADER_TYPE_MASK;
                if kind != HEADER_TYPE_ENDPOINT {
                    continue;
                }
                if assign_device(bus, device, function) {
                    assigned += 1;
                }
            }
        }
    }
    assigned
}
