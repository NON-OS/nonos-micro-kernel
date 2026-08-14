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

pub const PAGE_SHIFT_4K: u32 = 12;
pub const PAGE_SIZE_4K: usize = 1 << PAGE_SHIFT_4K;
pub const PAGE_MASK_4K: u64 = !((PAGE_SIZE_4K as u64) - 1);

pub const MAX_VTD_DOMAINS: usize = 256;
pub const MAX_VTD_DEVICES: usize = 256;
pub const MAX_VTD_MAPPINGS_PER_DOMAIN: usize = 4096;
