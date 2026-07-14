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

/// Source clock for an LPSS I2C controller, keyed off its `_HID` vendor. ACPI
/// carries the exact rate in a `_DSD` clock-frequency property this extractor
/// does not evaluate, so these are the common platform defaults: AMD parts run
/// the DesignWare block at 133 MHz, Intel LPSS at 100 MHz.
pub(super) fn source_clock_hz(hid: &[u8; 8]) -> u32 {
    if &hid[..3] == b"AMD" {
        133_000_000
    } else {
        100_000_000
    }
}
