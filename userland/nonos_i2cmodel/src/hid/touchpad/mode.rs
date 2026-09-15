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

//! Feature report 2: the input mode a host writes to ask for raw touch (3),
//! and the surface and button switches beside it.

use super::report_desc::MODE_REPORT_ID;

pub(super) const MODE: &[&[u8]] = &[
    &[0x05, 0x0D],           // Usage Page (Digitizer)
    &[0x85, MODE_REPORT_ID], // Report ID
    &[0x09, 0x52],           // Usage (Input Mode)
    &[0x25, 0x03],           // Logical Maximum (3)
    &[0x75, 0x08],           // Report Size (8)
    &[0x95, 0x01],           // Report Count (1)
    &[0xB1, 0x02],           // Feature: input mode at bit 0
    &[0x09, 0x57],           // Usage (Surface Switch)
    &[0x09, 0x58],           // Usage (Button Switch)
    &[0x25, 0x01],           // Logical Maximum (1)
    &[0x75, 0x01],           // Report Size (1)
    &[0x95, 0x02],           // Report Count (2)
    &[0xB1, 0x02],           // Feature: surface switch at bit 8, button at 9
    &[0x95, 0x06],           // Report Count (6)
    &[0xB1, 0x03],           // Feature, constant: padding to the byte
];
