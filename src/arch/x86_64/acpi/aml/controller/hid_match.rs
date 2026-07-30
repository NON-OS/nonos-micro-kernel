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

/// Match a decoded `_HID` against the known LPSS I2C host controllers: the
/// Intel Lynx Point and Wildcat Point EISAIDs, the Bay Trail and Braswell
/// string identifiers, and the AMD I2C controllers. Seven-character EISAIDs
/// carry a trailing zero byte; eight-character string IDs fill all eight.
pub(super) fn hid_is_i2c_controller(hid: &[u8; 8]) -> bool {
    const IDS: [&[u8]; 15] = [
        b"INT33C2",
        b"INT33C3",
        b"INT3432",
        b"INT3433",
        b"INT3442",
        b"INT3443",
        b"INT3444",
        b"INT3445",
        b"INT3446",
        b"INT3447",
        b"80860F41",
        b"808622C1",
        b"AMDI0010",
        b"AMDI0510",
        b"AMD0010",
    ];
    IDS.iter().any(
        |&id| {
            if id.len() == 7 {
                &hid[..7] == id && hid[7] == 0
            } else {
                &hid[..8] == id
            }
        },
    )
}
