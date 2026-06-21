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

use super::family::HardwareFamily;

pub(super) fn classify_display(vendor: u16) -> HardwareFamily {
    match vendor {
        0x1af4 => HardwareFamily::DisplayVirtioGpu,
        0x1234 => HardwareFamily::DisplayBga,
        0x8086 => HardwareFamily::DisplayNativeIntel,
        0x1002 => HardwareFamily::DisplayNativeAmd,
        0x10de => HardwareFamily::DisplayNativeNvidia,
        _ => HardwareFamily::Unknown,
    }
}
