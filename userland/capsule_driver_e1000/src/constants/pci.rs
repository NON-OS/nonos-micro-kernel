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

pub const INTEL_VENDOR_ID: u16 = 0x8086;

pub const E1000_DEVICE_IDS: &[u16] = &[
    0x100E, 0x1015, 0x1016, 0x1017, 0x101E, 0x100F, 0x1011, 0x1026, 0x1027, 0x1028, 0x1010, 0x1012,
    0x101D, 0x1079, 0x107A, 0x107B, 0x1099, 0x10B5, 0x1013, 0x1014, 0x1018, 0x1076, 0x1077, 0x1078,
    0x107C, 0x1019, 0x101A, 0x1075,
];

pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
