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

// The nth field's usage; when a report declares more fields than usages, the
// last usage applies to the remainder (per the HID spec).
pub(super) fn usage_for(usages: &[u16], n: usize, index: usize) -> u16 {
    if n == 0 {
        0
    } else if index < n {
        usages[index]
    } else {
        usages[n - 1]
    }
}
