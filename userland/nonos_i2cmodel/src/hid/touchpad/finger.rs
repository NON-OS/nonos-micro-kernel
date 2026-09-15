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

//! Input report 1: one finger, then the contact count and the click button.
//! Bit positions are what the driver's parser has to arrive at.

use super::report_desc::TOUCH_REPORT_ID;

pub(super) const FINGER: &[&[u8]] = &[
    &[0x85, TOUCH_REPORT_ID], // Report ID
    &[0x09, 0x22],            // Usage (Finger)
    &[0xA1, 0x02],            // Collection (Logical)
    &[0x09, 0x47],            // Usage (Confidence)
    &[0x09, 0x42],            // Usage (Tip Switch)
    &[0x15, 0x00],            // Logical Minimum (0)
    &[0x25, 0x01],            // Logical Maximum (1)
    &[0x75, 0x01],            // Report Size (1)
    &[0x95, 0x02],            // Report Count (2)
    &[0x81, 0x02],            // Input: confidence at bit 0, tip at bit 1
    &[0x95, 0x06],            // Report Count (6)
    &[0x81, 0x03],            // Input, constant: padding to the byte
    &[0x05, 0x01],            // Usage Page (Generic Desktop)
    &[0x09, 0x30],            // Usage (X)
    &[0x26, 0xB0, 0x04],      // Logical Maximum (1200)
    &[0x75, 0x10],            // Report Size (16)
    &[0x95, 0x01],            // Report Count (1)
    &[0x81, 0x02],            // Input: X at bit 8
    &[0x09, 0x31],            // Usage (Y)
    &[0x26, 0x20, 0x03],      // Logical Maximum (800)
    &[0x81, 0x02],            // Input: Y at bit 24
    &[0xC0],                  // End Collection
    &[0x05, 0x0D],            // Usage Page (Digitizer)
    &[0x09, 0x54],            // Usage (Contact Count)
    &[0x25, 0x05],            // Logical Maximum (5)
    &[0x75, 0x08],            // Report Size (8)
    &[0x81, 0x02],            // Input: contact count at bit 40
    &[0x05, 0x09],            // Usage Page (Button)
    &[0x09, 0x01],            // Usage (Button 1)
    &[0x25, 0x01],            // Logical Maximum (1)
    &[0x75, 0x01],            // Report Size (1)
    &[0x81, 0x02],            // Input: button at bit 48
    &[0x95, 0x07],            // Report Count (7)
    &[0x81, 0x03],            // Input, constant: padding to the byte
];
