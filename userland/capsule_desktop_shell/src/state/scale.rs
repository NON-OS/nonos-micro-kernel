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

const HIDPI_MIN_WIDTH: u32 = 2560;
const HIDPI_MIN_HEIGHT: u32 = 1440;

pub fn scale_for(width: u32, height: u32) -> u32 {
    if width >= HIDPI_MIN_WIDTH && height >= HIDPI_MIN_HEIGHT {
        2
    } else {
        1
    }
}
