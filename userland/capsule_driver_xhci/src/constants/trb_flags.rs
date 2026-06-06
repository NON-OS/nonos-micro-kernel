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
pub const TRB_CYCLE: u32 = 1 << 0;
pub const TRB_IOC: u32 = 1 << 5;
pub const TRB_IDT: u32 = 1 << 6;
pub const TRB_TYPE_SHIFT: u32 = 10;
pub const TRB_TYPE_MASK: u32 = 0x3F << 10;
pub const LINK_TC: u32 = 1 << 1;
pub const TRB_DIR_IN: u32 = 1 << 16;
pub const TRT_NO_DATA: u32 = 0;
pub const TRT_OUT_DATA: u32 = 2 << 16;
pub const TRT_IN_DATA: u32 = 3 << 16;
pub const TRB_ISP: u32 = 1 << 2;
pub const TRB_CH: u32 = 1 << 4;
