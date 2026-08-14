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

const SLLPS_2MB: u64 = 1 << 34;
const SLLPS_1GB: u64 = 1 << 35;

/// Shallowest level this unit can put a leaf at, leaf level 1. Larger leaves
/// turn an identity map of several gigabytes into a handful of tables.
pub const fn best_leaf_level(cap: u64) -> u8 {
    if cap & SLLPS_1GB != 0 {
        3
    } else if cap & SLLPS_2MB != 0 {
        2
    } else {
        1
    }
}
